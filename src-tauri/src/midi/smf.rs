use crate::midi::message::{MidiChannel, MidiMessage};

const MIDI_HEADER_CHUNK_ASCII_TYPE: &[u8; 4] = b"MThd";
const MIDI_TRACK_CHUNK_ASCII_TYPE: &[u8; 4] = b"MTrk";

#[derive(Debug, Clone, PartialEq)]
pub enum MidiFormat {
    SingleMultiChannelTrack,
    MultiTrackSequence,
    MultiSequence,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FramesPerSecond {
    Fps24,
    Fps25,
    Fps30,
    Fps30DropFrame,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MidiDivision {
    TicksPerQuarterNote(u16),
    TimeCode(FramesPerSecond, u8),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MusicalScale {
    Major,
    Minor,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MetaEvent {
    SequenceNumber(u16),
    TextEvent(String),
    CopyrightNotice(String),
    SequenceName(String),
    TrackName(String),
    InstrumentName(String),
    Lyric(String),
    Marker(String),
    CuePoint(String),
    MidiChannelPrefix(MidiChannel),
    EndOfTrack,
    SetTempo(u32),
    SmpteOffset {
        hour: u8,
        minute: u8,
        second: u8,
        frame: u8,
        sub_frame: u8,
    },
    TimeSignature {
        numerator: u8,
        denominator: u8,
        clocks_per_click: u8,
        notated_32nd_notes_per_quarter_note: u8,
    },
    KeySignature {
        key: i8,
        scale: MusicalScale,
    },
    SequencerSpecific(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    MidiEvent(MidiMessage),
    SysExEvent,
    MetaEvent(MetaEvent),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MidiTrackEvent {
    delta_time: u32,
    event: Event,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MidiHeader {
    format: MidiFormat,
    num_tracks: u16,
    division: MidiDivision,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MidiFile {
    header: MidiHeader,
    tracks: Vec<Vec<MidiTrackEvent>>,
}

fn to_var_length_bytes(value: u32) -> Result<Vec<u8>, String> {
    if value < 0x80 {
        return Ok(vec![value as u8]);
    }

    if value > 0x0F_FF_FF_FF {
        return Err(
            "Value exceeds larges number allowed (0x0FFFFFFF) for variable-length quantity"
                .to_string(),
        );
    }

    let mut bytes: Vec<u8> = Vec::new();

    let byte_0 = (value & 0x7F) as u8;
    let byte_1 = (value >> 7) as u8 | 0x80;
    let byte_2 = (value >> 14) as u8 | 0x80;
    let byte_3 = (value >> 21) as u8 | 0x80;

    if byte_3 > 0x80 {
        bytes.push(byte_3);
        bytes.push(byte_2);
    } else if byte_2 > 0x80 {
        bytes.push(byte_2);
    }

    bytes.push(byte_1);
    bytes.push(byte_0);

    Ok(bytes)
}

fn from_var_length_bytes(bytes: &[u8]) -> Result<u32, String> {
    if bytes.is_empty() {
        return Err("Input byte slice is empty".to_string());
    }

    if bytes.len() > 4 {
        return Err("Input byte slice exceeds maximum length of 4 bytes".to_string());
    }

    let last_byte = *bytes.last().unwrap();
    if last_byte > 0x7F {
        return Err("Last byte must not have the MSB set".to_string());
    }

    let mut value: u32 = 0;
    let mut shif: usize = 0;

    for byte in bytes.iter().rev() {
        let mask = (byte & 0x7F) as u32;
        let mask = mask << shif;
        shif += 7;
        value |= mask;
    }

    Ok(value)
}

fn get_var_length_bytes_length(bytes: &[u8]) -> Result<usize, String> {
    let mut length = 0;
    let mut last_byte_valid = false;

    for &byte in bytes {
        length += 1;
        if byte & 0x80 == 0 {
            last_byte_valid = true;
            break;
        }
    }

    if length == 0 || !last_byte_valid {
        return Err("No valid variable-length bytes found".to_string());
    } else if length > 4 {
        return Err("Variable-length bytes exceed maximum length of 4 bytes".to_string());
    }

    Ok(length)
}

fn get_event_length(data: &[u8], running_status_length: Option<u8>) -> Result<usize, String> {
    if data.len() == 0 {
        return Err("Data is empty, cannot determine event length".to_string());
    }

    let first_byte = data[0];

    Ok(match first_byte {
        // Running status
        0x00..=0x7F => {
            if let Some(length) = running_status_length {
                length as usize
            } else {
                return Err("Running status not set, cannot determine event length".to_string());
            }
        }
        // Channel messages
        0x80..=0xEF => {
            if data.len() < 2 {
                return Err("Channel message data is too short".to_string());
            }

            match first_byte & 0xF0 {
                0x80 | 0x90 | 0xA0 | 0xB0 | 0xE0 => {
                    if data.len() < 3 {
                        return Err(
                            "Channel message data is too short for expected length".to_string()
                        );
                    }

                    3
                }
                0xC0 | 0xD0 => 2,
                _ => unreachable!(),
            }
        }
        // Meta event
        0xFF => {
            if data.len() < 2 {
                return Err("Meta event data is too short".to_string());
            }

            let meta_type = data[1];
            match meta_type {
                0x00 => {
                    if data.len() < 5 {
                        return Err("Sequence number meta event data is too short".to_string());
                    }

                    5
                }
                0x20 => {
                    if data.len() < 4 {
                        return Err("MIDI Channel Prefix meta event data is too short".to_string());
                    }

                    4
                }
                0x2F => {
                    if data.len() < 3 {
                        return Err("End of Track meta event data is too short".to_string());
                    }

                    3
                }
                0x51 => {
                    if data.len() < 6 {
                        return Err("Set Tempo meta event data is too short".to_string());
                    }

                    6
                }
                0x54 => {
                    if data.len() < 8 {
                        return Err("SMPTE Offset meta event data is too short".to_string());
                    }

                    8
                }
                0x58 => {
                    if data.len() < 7 {
                        return Err("Time Signature meta event data is too short".to_string());
                    }

                    7
                }
                0x59 => {
                    if data.len() < 5 {
                        return Err("Key Signature meta event data is too short".to_string());
                    }

                    5
                }
                // Variable length meta events
                _ => {
                    let var_length = get_var_length_bytes_length(&data[2..])?;
                    let length = from_var_length_bytes(&data[2..2 + var_length])?;

                    if data.len() < 2 + var_length + length as usize {
                        return Err("Meta event data length exceeds available data".to_string());
                    }

                    2 + var_length + length as usize
                }
            }
        }
        // SysEx event
        0xF0 | 0xF7 => {
            if data.len() < 2 {
                return Err("SysEx event data is too short".to_string());
            }

            let var_length = get_var_length_bytes_length(&data[1..])?;
            let length = from_var_length_bytes(&data[1..1 + var_length])?;

            if data.len() < 1 + var_length + length as usize {
                return Err("SysEx event data length exceeds available data".to_string());
            }

            // ignore fragmented SysEx events for now
            1 + var_length + length as usize
        }
        _ => return Err("Unsupported event type or running status not handled".to_string()),
    })
}

fn calc_delta_time_microseconds(delta: u32, tempo: u32, division: &MidiDivision) -> u64 {
    let delta_u64 = delta as u64;
    let tempo_u64 = tempo as u64;

    match division {
        MidiDivision::TicksPerQuarterNote(ticks) => delta_u64 * tempo_u64 / (*ticks as u64),
        // Time code is less common and should be avoided if possible.
        MidiDivision::TimeCode(frames_per_second, ticks) => {
            let ticks_u64 = *ticks as u64;

            match frames_per_second {
                FramesPerSecond::Fps25 => delta_u64 * 1_000_000 / (25 * ticks_u64),
                FramesPerSecond::Fps24 => delta_u64 * 1_000_000 / (24 * ticks_u64),
                FramesPerSecond::Fps30 => delta_u64 * 100_000 / (3 * ticks_u64),
                FramesPerSecond::Fps30DropFrame => delta_u64 * 100_100 / (3 * ticks_u64),
            }
        }
    }
}

fn parse_midi_file_header(data: &[u8]) -> Result<MidiHeader, String> {
    if data.len() < 14 {
        return Err("Data is too short to be a valid MIDI file".to_string());
    }

    let header_chunk_type = &data[0..4];

    if header_chunk_type != MIDI_HEADER_CHUNK_ASCII_TYPE {
        return Err("Invalid MIDI header chunk type".to_string());
    }

    let header_length = u32::from_be_bytes(data[4..8].try_into().unwrap());
    let format = u16::from_be_bytes(data[8..10].try_into().unwrap());
    let ntrks = u16::from_be_bytes(data[10..12].try_into().unwrap());
    let division = u16::from_be_bytes(data[12..14].try_into().unwrap());

    if header_length != 6 {
        return Err("Invalid MIDI header length".to_string());
    }

    let midi_format = match format {
        0 => MidiFormat::SingleMultiChannelTrack,
        1 => MidiFormat::MultiTrackSequence,
        2 => MidiFormat::MultiSequence,
        _ => return Err("Unsupported MIDI format".to_string()),
    };

    if ntrks == 0 {
        return Err("MIDI file must contain at least one track".to_string());
    } else if midi_format == MidiFormat::SingleMultiChannelTrack && ntrks != 1 {
        return Err("Single multi-channel track format must have exactly one track".to_string());
    }

    let midi_division = if division & 0x8000 != 0 {
        MidiDivision::TimeCode(
            match (division >> 8) as i8 {
                -24 => FramesPerSecond::Fps24,
                -25 => FramesPerSecond::Fps25,
                -30 => FramesPerSecond::Fps30,
                -29 => FramesPerSecond::Fps30DropFrame,
                _ => return Err("Unsupported frames per second in time code".to_string()),
            },
            (division & 0xFF) as u8,
        )
    } else {
        MidiDivision::TicksPerQuarterNote(division)
    };

    Ok(MidiHeader {
        format: midi_format,
        num_tracks: ntrks,
        division: midi_division,
    })
}

fn parse_midi_track_event(data: &[u8], running_status: Option<u8>) -> Result<(Event, u8), String> {
    let first_byte = data[0];

    match first_byte {
        // Running status
        0x00..=0x7F => {
            if running_status.is_none() {
                return Err("Missing running status for MIDI event".to_string());
            }
            let status = running_status.unwrap();
            let mut message_data = vec![status];
            message_data.extend_from_slice(data);
            Ok((
                Event::MidiEvent(MidiMessage::try_from(message_data.as_slice())?),
                0,
            ))
        }
        // Channel messages
        0x80..=0xEF => Ok((Event::MidiEvent(MidiMessage::try_from(data)?), first_byte)),
        // Meta event
        0xFF => {
            let event = Event::MetaEvent(match data[1] {
                0x00 => {
                    if data[2] != 0x02 {
                        return Err("Invalid sequence number meta event".to_string());
                    }

                    MetaEvent::SequenceNumber(u16::from_be_bytes(
                        data[3..=4]
                            .try_into()
                            .map_err(|_| "Invalid sequence number bytes".to_string())?,
                    ))
                }
                0x01 => {
                    let var_length = get_var_length_bytes_length(&data[2..])?;
                    let length = from_var_length_bytes(&data[2..2 + var_length])? as usize;

                    MetaEvent::TextEvent(
                        String::from_utf8(data[2 + var_length..2 + var_length + length].to_vec())
                            .map_err(|_| "Invalid UTF-8 in text event".to_string())?,
                    )
                }
                0x02 => {
                    let var_length = get_var_length_bytes_length(&data[2..])?;
                    let length = from_var_length_bytes(&data[2..2 + var_length])? as usize;

                    MetaEvent::CopyrightNotice(
                        String::from_utf8(data[2 + var_length..2 + var_length + length].to_vec())
                            .map_err(|_| "Invalid UTF-8 in text event".to_string())?,
                    )
                }
                0x03 => {
                    let var_length = get_var_length_bytes_length(&data[2..])?;
                    let length = from_var_length_bytes(&data[2..2 + var_length])? as usize;

                    // FIXME: we need to differentiate between sequence name and track name
                    // for now we assume it's a sequence name (format 0 or first track in format 1)
                    MetaEvent::SequenceName(
                        String::from_utf8(data[2 + var_length..2 + var_length + length].to_vec())
                            .map_err(|_| "Invalid UTF-8 in text event".to_string())?,
                    )
                }
                0x04 => {
                    let var_length = get_var_length_bytes_length(&data[2..])?;
                    let length = from_var_length_bytes(&data[2..2 + var_length])? as usize;

                    MetaEvent::InstrumentName(
                        String::from_utf8(data[2 + var_length..2 + var_length + length].to_vec())
                            .map_err(|_| "Invalid UTF-8 in text event".to_string())?,
                    )
                }
                0x05 => {
                    let var_length = get_var_length_bytes_length(&data[2..])?;
                    let length = from_var_length_bytes(&data[2..2 + var_length])? as usize;

                    MetaEvent::Lyric(
                        String::from_utf8(data[2 + var_length..2 + var_length + length].to_vec())
                            .map_err(|_| "Invalid UTF-8 in text event".to_string())?,
                    )
                }
                0x06 => {
                    let var_length = get_var_length_bytes_length(&data[2..])?;
                    let length = from_var_length_bytes(&data[2..2 + var_length])? as usize;

                    MetaEvent::Marker(
                        String::from_utf8(data[2 + var_length..2 + var_length + length].to_vec())
                            .map_err(|_| "Invalid UTF-8 in text event".to_string())?,
                    )
                }
                0x07 => {
                    let var_length = get_var_length_bytes_length(&data[2..])?;
                    let length = from_var_length_bytes(&data[2..2 + var_length])? as usize;

                    MetaEvent::CuePoint(
                        String::from_utf8(data[2 + var_length..2 + var_length + length].to_vec())
                            .map_err(|_| "Invalid UTF-8 in text event".to_string())?,
                    )
                }
                0x20 => {
                    if data[2] != 0x01 {
                        return Err("Invalid MIDI Channel Prefix meta event".to_string());
                    }

                    let channel = MidiChannel::try_from(data[3])?;
                    MetaEvent::MidiChannelPrefix(channel)
                }
                0x2F => {
                    if data[2] != 0x00 {
                        return Err("Invalid End of Track meta event".to_string());
                    }

                    MetaEvent::EndOfTrack
                }
                0x51 => {
                    if data[2] != 0x03 {
                        return Err("Invalid End of Track meta event".to_string());
                    }

                    let mut tempo_bytes: Vec<u8> = vec![0x00];
                    tempo_bytes.extend_from_slice(&data[3..=5]);
                    let tempo = u32::from_be_bytes(
                        tempo_bytes
                            .try_into()
                            .map_err(|_| "Invalid tempo bytes".to_string())?,
                    );

                    MetaEvent::SetTempo(tempo)
                }
                0x54 => {
                    if data[2] != 0x05 {
                        return Err("Invalid SMPTE Offset meta event".to_string());
                    }

                    let hour = data[3];
                    let minute = data[4];
                    let second = data[5];
                    let frame = data[6];
                    let sub_frame = data[7];

                    // FIXME: add validation for SMPTE offset values

                    MetaEvent::SmpteOffset {
                        hour,
                        minute,
                        second,
                        frame,
                        sub_frame,
                    }
                }
                0x58 => {
                    if data[2] != 0x04 {
                        return Err("Invalid SMPTE Offset meta event".to_string());
                    }

                    let numerator = data[3];
                    let denominator = data[4];
                    let clocks_per_click = data[5];
                    let notated_32nd_notes_per_quarter_note = data[6];

                    // FIXME: add validation for time signature values
                    if notated_32nd_notes_per_quarter_note != 8 {
                        eprintln!(
                            "Warning: Notated 32nd notes per quarter note is not 8, got {}",
                            notated_32nd_notes_per_quarter_note
                        );
                    }

                    MetaEvent::TimeSignature {
                        numerator,
                        denominator,
                        clocks_per_click,
                        notated_32nd_notes_per_quarter_note,
                    }
                }
                0x59 => {
                    if data[2] != 0x02 {
                        return Err("Invalid SMPTE Offset meta event".to_string());
                    }

                    let key = data[3] as i8;

                    if key < -7 || key > 7 {
                        return Err("Key signature must be between -7 and 7".to_string());
                    }

                    let scale = match data[4] {
                        0 => MusicalScale::Major,
                        1 => MusicalScale::Minor,
                        _ => return Err("Invalid key signature scale".to_string()),
                    };

                    MetaEvent::KeySignature { key, scale }
                }
                0x7F => {
                    let var_length = get_var_length_bytes_length(&data[2..])?;
                    let length = from_var_length_bytes(&data[2..2 + var_length])? as usize;

                    MetaEvent::SequencerSpecific(
                        data[2 + var_length..2 + var_length + length].to_vec(),
                    )
                }
                _ => return Err("Unsupported meta event type".to_string()),
            });
            Ok((event, 0))
        }
        // SysEx event
        0xF0 | 0xF7 => Ok((Event::SysExEvent, 0)),
        _ => return Err("Invalid MIDI event".to_string()),
    }
}

fn parse_midi_file_track(data: &[u8], offset: &mut usize) -> Result<Vec<MidiTrackEvent>, String> {
    if data.len() <= *offset + 8 {
        return Err("Data is too short for MIDI track chunk".to_string());
    }

    let track_chunk_type = &data[*offset..*offset + 4];
    *offset += 4;

    if track_chunk_type != MIDI_TRACK_CHUNK_ASCII_TYPE {
        return Err("Invalid MIDI track chunk type".to_string());
    }

    let track_length = u32::from_be_bytes(data[*offset..*offset + 4].try_into().unwrap());
    *offset += 4;

    if track_length == 0 {
        return Err("Track length cannot be zero".to_string());
    }

    if data.len() < *offset + track_length as usize {
        return Err("Track length exceeds available data".to_string());
    }

    let track_data = &data[*offset..*offset + track_length as usize];
    *offset += track_length as usize;

    let mut track: Vec<MidiTrackEvent> = Vec::new();

    let mut track_offset = 0;
    let mut running_status: Option<u8> = None;
    let mut running_status_length: Option<u8> = None;
    loop {
        let var_length = get_var_length_bytes_length(&track_data[track_offset..])?;
        let delta = from_var_length_bytes(&track_data[track_offset..track_offset + var_length])?;
        track_offset += var_length;
        let event_length = get_event_length(&track_data[track_offset..], running_status_length)?;
        let (event, new_status) = parse_midi_track_event(
            &track_data[track_offset..track_offset + event_length],
            running_status,
        )?;
        track_offset += event_length;

        if !matches!(event, Event::MidiEvent(_)) {
            running_status = None;
            running_status_length = None;
        } else if new_status != 0 {
            running_status = Some(new_status);
            running_status_length = Some((event_length - 1) as u8);
        }

        // SysEx events are not supported
        let is_sysex = match event {
            Event::SysExEvent => true,
            _ => false,
        };

        if is_sysex {
            track.push(MidiTrackEvent {
                delta_time: delta,
                event: event.clone(),
            });
        }

        if event == Event::MetaEvent(MetaEvent::EndOfTrack) {
            break;
        } else if track_data[track_offset..].len() < 3 {
            eprintln!("Not enough data for next event, breaking...");
            // try to fix corrupted track data and add end of track event
            track.push(MidiTrackEvent {
                delta_time: 0,
                event: Event::MetaEvent(MetaEvent::EndOfTrack),
            });
            break;
        }
    }

    Ok(track)
}

impl TryFrom<&[u8]> for MidiFile {
    type Error = String;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let header = parse_midi_file_header(data)?;

        let mut offset: usize = 14;
        let mut tracks = Vec::new();
        for _ in 0..header.num_tracks {
            let track = parse_midi_file_track(data, &mut offset)?;
            tracks.push(track);
        }

        Ok(MidiFile { header, tracks })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_midi_file() {
        let midi_data: &[u8] = &[
            0x4D, 0x54, 0x68, 0x64, // MThd
            0x00, 0x00, 0x00, 0x06, // chunk length
            0x00, 0x00, // format 0
            0x00, 0x01, // one track
            0x00, 0x60, // division: 96 ticks per quarter note
            0x4D, 0x54, 0x72, 0x6B, // MTrk
            0x00, 0x00, 0x00, 0x3B, // chunk length
            0x00, 0xFF, 0x58, 0x04, 0x04, 0x02, 0x18,
            0x08, // time signature: 4/4, 24 clocks per click, 8 notated 32nd notes per quarter note
            0x00, 0xFF, 0x51, 0x03, 0x07, 0xA1,
            0x20, // set tempo: 500000 microseconds per quarter note = 120 BPM
            0x00, 0xC0, 0x05, // program change: channel 0, program 5
            0x00, 0xC1, 0x2E, // program change: channel 1, program 46
            0x00, 0xC2, 0x46, // program change: channel 2, program 70
            0x00, 0x92, 0x30, 0x60, // note on: channel 2, note 48, velocity 96
            0x00, 0x3C, 0x60, // note on: channel 2, note 60, velocity 96 (running status)
            0x60, 0x91, 0x43, 0x40, // ...
            0x60, 0x90, 0x4C, 0x20, // ...
            0x81, 0x40, 0x82, 0x30, 0x40, // ...
            0x00, 0x3C, 0x40, // ...
            0x00, 0x81, 0x43, 0x40, // ...
            0x00, 0x80, 0x4C, 0x40, // ...
            0x00, 0xFF, 0x2F, 0x00, // end of track
        ];

        let midi_file = MidiFile::try_from(midi_data);

        if midi_file.is_err() {
            eprintln!("Failed to parse MIDI file: {:?}", midi_file.err());
            assert!(false, "MIDI file parsing failed");
        }
    }

    #[test]
    fn calc_delta_time_microseconds_time_code_25_40() {
        let delta = 1234;
        let tempo = 500_000; // 120 BPM (500,000 microseconds per quarter note)
        let division = MidiDivision::TimeCode(FramesPerSecond::Fps25, 40);

        let result = calc_delta_time_microseconds(delta, tempo, &division);
        assert_eq!(result, 1_234_000);
    }

    #[test]
    fn calc_delta_time_microseconds_time_code_30_80() {
        let delta = 2400;
        let tempo = 500_000; // 120 BPM (500,000 microseconds per quarter note)
        let division = MidiDivision::TimeCode(FramesPerSecond::Fps30, 80);

        let result = calc_delta_time_microseconds(delta, tempo, &division);
        assert_eq!(result, 1_000_000);
    }

    #[test]
    fn calc_delta_time_microseconds_time_code_30_drop_frame() {
        let delta = 2400;
        let tempo = 500_000; // 120 BPM (500,000 microseconds per quarter note)
        let division = MidiDivision::TimeCode(FramesPerSecond::Fps30DropFrame, 80);

        let result = calc_delta_time_microseconds(delta, tempo, &division);
        assert_eq!(result, 1_001_000);
    }

    #[test]
    fn calc_delta_time_microseconds_ticks_per_quarter_note() {
        let delta = 6144;
        let tempo = 500_000; // 120 BPM (500,000 microseconds per quarter note)
        let division = MidiDivision::TicksPerQuarterNote(96);

        let result = calc_delta_time_microseconds(delta, tempo, &division);
        assert_eq!(result, 32_000_000);
    }

    #[test]
    fn to_var_length_bytes_spec_examples() {
        assert_eq!(to_var_length_bytes(0x00).unwrap(), vec![0x00]);
        assert_eq!(to_var_length_bytes(0x40).unwrap(), vec![0x40]);
        assert_eq!(to_var_length_bytes(0x7F).unwrap(), vec![0x7F]);
        assert_eq!(to_var_length_bytes(0x80).unwrap(), vec![0x81, 0x00]);
        assert_eq!(to_var_length_bytes(0x2000).unwrap(), vec![0xC0, 0x00]);
        assert_eq!(to_var_length_bytes(0x3FFF).unwrap(), vec![0xFF, 0x7F]);
        assert_eq!(to_var_length_bytes(0x4000).unwrap(), vec![0x81, 0x80, 0x00]);
        assert_eq!(
            to_var_length_bytes(0x100000).unwrap(),
            vec![0xC0, 0x80, 0x00]
        );
        assert_eq!(
            to_var_length_bytes(0x1FFFFF).unwrap(),
            vec![0xFF, 0xFF, 0x7F]
        );
        assert_eq!(
            to_var_length_bytes(0x200000).unwrap(),
            vec![0x81, 0x80, 0x80, 0x00]
        );
        assert_eq!(
            to_var_length_bytes(0x8000000).unwrap(),
            vec![0xC0, 0x80, 0x80, 0x00]
        );
        assert_eq!(
            to_var_length_bytes(0xFFFFFFF).unwrap(),
            vec![0xFF, 0xFF, 0xFF, 0x7F]
        );
    }

    #[test]
    fn to_var_length_bytes_too_large() {
        let result = to_var_length_bytes(0x10000000);
        assert!(result.is_err());
    }

    #[test]
    fn from_var_length_bytes_spec_examples() {
        assert_eq!(from_var_length_bytes(vec![0x00].as_slice()).unwrap(), 0x00);
        assert_eq!(from_var_length_bytes(vec![0x40].as_slice()).unwrap(), 0x40);
        assert_eq!(from_var_length_bytes(vec![0x7F].as_slice()).unwrap(), 0x7F);
        assert_eq!(
            from_var_length_bytes(vec![0x81, 0x00].as_slice()).unwrap(),
            0x80
        );
        assert_eq!(
            from_var_length_bytes(vec![0xC0, 0x00].as_slice()).unwrap(),
            0x2000
        );
        assert_eq!(
            from_var_length_bytes(vec![0xFF, 0x7F].as_slice()).unwrap(),
            0x3FFF
        );
        assert_eq!(
            from_var_length_bytes(vec![0x81, 0x80, 0x00].as_slice()).unwrap(),
            0x4000
        );
        assert_eq!(
            from_var_length_bytes(vec![0xC0, 0x80, 0x00].as_slice()).unwrap(),
            0x100000
        );
        assert_eq!(
            from_var_length_bytes(vec![0xFF, 0xFF, 0x7F].as_slice()).unwrap(),
            0x1FFFFF
        );
        assert_eq!(
            from_var_length_bytes(vec![0x81, 0x80, 0x80, 0x00].as_slice()).unwrap(),
            0x200000
        );
        assert_eq!(
            from_var_length_bytes(vec![0xC0, 0x80, 0x80, 0x00].as_slice()).unwrap(),
            0x8000000
        );
        assert_eq!(
            from_var_length_bytes(vec![0xFF, 0xFF, 0xFF, 0x7F].as_slice()).unwrap(),
            0xFFFFFFF
        );
    }
}
