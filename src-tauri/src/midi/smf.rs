use crate::midi::message::{ChannelMessage, MidiChannel};

const MIDI_HEADER_CHUNK_ASCII_TYPE: &[u8; 4] = b"MThd";
const MIDI_TRACK_CHUNK_ASCII_TYPE: &[u8; 4] = b"MTrk";

pub enum MidiFormat {
    SingleMultiChannelTrack,
    MultiTrackSequence,
    MultiSequence,
}

pub enum MidiDivision {
    TicksPerQuarterNote(u16),
    TimeCode(u8, u8),
}

pub enum MusicalScale {
    Major,
    Minor,
}

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
}

pub enum Event {
    MidiEvent(ChannelMessage),
    SysExEvent,
    MetaEvent(MetaEvent),
}

pub struct MidiTrackEvent {
    delta_time: u32,
    event: Event,
}

pub struct MidiHeader {
    format: MidiFormat,
    num_tracks: u16,
    division: MidiDivision,
}

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

#[cfg(test)]
mod tests {
    use super::*;

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
