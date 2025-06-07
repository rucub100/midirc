// https://midi.org/midi-1-0-detailed-specification

use std::time::Instant;

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum MidiChannel {
    Channel1,
    Channel2,
    Channel3,
    Channel4,
    Channel5,
    Channel6,
    Channel7,
    Channel8,
    Channel9,
    Channel10,
    Channel11,
    Channel12,
    Channel13,
    Channel14,
    Channel15,
    Channel16,
}

impl From<MidiChannel> for u8 {
    fn from(channel: MidiChannel) -> Self {
        match channel {
            MidiChannel::Channel1 => 0,
            MidiChannel::Channel2 => 1,
            MidiChannel::Channel3 => 2,
            MidiChannel::Channel4 => 3,
            MidiChannel::Channel5 => 4,
            MidiChannel::Channel6 => 5,
            MidiChannel::Channel7 => 6,
            MidiChannel::Channel8 => 7,
            MidiChannel::Channel9 => 8,
            MidiChannel::Channel10 => 9,
            MidiChannel::Channel11 => 10,
            MidiChannel::Channel12 => 11,
            MidiChannel::Channel13 => 12,
            MidiChannel::Channel14 => 13,
            MidiChannel::Channel15 => 14,
            MidiChannel::Channel16 => 15,
        }
    }
}

impl TryFrom<u8> for MidiChannel {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MidiChannel::Channel1),
            1 => Ok(MidiChannel::Channel2),
            2 => Ok(MidiChannel::Channel3),
            3 => Ok(MidiChannel::Channel4),
            4 => Ok(MidiChannel::Channel5),
            5 => Ok(MidiChannel::Channel6),
            6 => Ok(MidiChannel::Channel7),
            7 => Ok(MidiChannel::Channel8),
            8 => Ok(MidiChannel::Channel9),
            9 => Ok(MidiChannel::Channel10),
            10 => Ok(MidiChannel::Channel11),
            11 => Ok(MidiChannel::Channel12),
            12 => Ok(MidiChannel::Channel13),
            13 => Ok(MidiChannel::Channel14),
            14 => Ok(MidiChannel::Channel15),
            15 => Ok(MidiChannel::Channel16),
            _ => Err(format!(
                "Invalid MIDI channel: {}. Must be between 0 and 15 for internal representation.",
                value
            )),
        }
    }
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ChannelVoiceMessage {
    NoteOff { note: u8, velocity: u8 },
    NoteOn { note: u8, velocity: u8 },
    PolyphonicKeyPressure { note: u8, pressure: u8 },
    ControlChange { controller: u8, value: u8 },
    ProgramChange(u8),
    ChannelPressure(u8),
    PitchBendChange(u16),
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ChannelModeMessage {
    AllSoundOff,
    ResetAllControllers,
    LocalControlOff,
    LocalControlOn,
    AllNotesOff,
    OmniModeOff,
    OmniModeOn,
    MonoMode { number_of_voices: u8 },
    PolyMode,
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ChannelMessage {
    Voice(ChannelVoiceMessage),
    Mode(ChannelModeMessage),
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SystemCommonMessage {
    MidiTimeCodeQuarterFrame,
    SongPositionPointer(u16),
    SongSelect(u8),
    TuneRequest,
    EndOfSystemExclusive,
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SystemRealTimeMessage {
    TimingClock,
    Start,
    Continue,
    Stop,
    ActiveSensing,
    SystemReset,
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SystemExclusiveSubId {
    ManufacturerIdentification(Vec<u8>),
    NonCommercial,
    NonRealTime,
    RealTime,
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SystemExclusiveMessage {
    sub_id: SystemExclusiveSubId,
    data: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SystemMessage {
    Common(SystemCommonMessage),
    RealTime(SystemRealTimeMessage),
    Exclusive(SystemExclusiveMessage),
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MidiMessage {
    Channel {
        channel: MidiChannel,
        message: ChannelMessage,
    },
    System(SystemMessage),
}

fn validate_midi_data_byte(value: u8) -> Result<u8, String> {
    if value <= 127 {
        Ok(value)
    } else {
        Err(format!(
            "Invalid MIDI data byte: {}. Must be between 0 and 127.",
            value
        ))
    }
}

impl TryFrom<&[u8]> for MidiMessage {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("MIDI message cannot be empty".to_string());
        }

        let status_byte = value[0];
        let data_bytes = &value[1..];

        let msg = match status_byte {
            // Channel Message
            0x80..=0xEF => {
                let channel = MidiChannel::try_from(status_byte & 0x0F)?;
                MidiMessage::Channel {
                    channel,
                    message: match status_byte & 0xF0 {
                        // Note Off
                        0x80 => {
                            if data_bytes.len() != 2 {
                                return Err(
                                    "Note Off message requires exactly 2 data bytes".to_string()
                                );
                            }
                            ChannelMessage::Voice(ChannelVoiceMessage::NoteOff {
                                note: validate_midi_data_byte(data_bytes[0])?,
                                velocity: validate_midi_data_byte(data_bytes[1])?,
                            })
                        }
                        // Note On
                        0x90 => {
                            if data_bytes.len() != 2 {
                                return Err(
                                    "Note On message requires exactly 2 data bytes".to_string()
                                );
                            }
                            ChannelMessage::Voice(ChannelVoiceMessage::NoteOn {
                                note: validate_midi_data_byte(data_bytes[0])?,
                                velocity: validate_midi_data_byte(data_bytes[1])?,
                            })
                        }
                        // Polyphonic Key Pressure
                        0xA0 => {
                            if data_bytes.len() != 2 {
                                return Err(
                                    "Polyphonic Key Pressure message requires exactly 2 data bytes"
                                        .to_string(),
                                );
                            }
                            ChannelMessage::Voice(ChannelVoiceMessage::PolyphonicKeyPressure {
                                note: validate_midi_data_byte(data_bytes[0])?,
                                pressure: validate_midi_data_byte(data_bytes[1])?,
                            })
                        }
                        // Control Change OR Mode Change
                        0xB0 => {
                            if data_bytes.len() != 2 {
                                return Err(
                                    "Control/Mode Change message requires exactly 2 data bytes"
                                        .to_string(),
                                );
                            }
                            let data_byte_1 = validate_midi_data_byte(data_bytes[0])?;
                            let data_byte_2 = validate_midi_data_byte(data_bytes[1])?;

                            match data_byte_1 {
                                // Mode Change
                                0x78.. => ChannelMessage::Mode(match data_byte_1 & 0x07 {
                                    0x00 => ChannelModeMessage::AllSoundOff,
                                    0x01 => ChannelModeMessage::ResetAllControllers,
                                    0x02 => match data_byte_2 {
                                        0x00 => ChannelModeMessage::LocalControlOff,
                                        0x7F => ChannelModeMessage::LocalControlOn,
                                        _ => {
                                            return Err(format!(
                                                "Invalid Local Control value: {}; must be 0 or 127",
                                                data_byte_2
                                            ));
                                        }
                                    },
                                    0x03 => ChannelModeMessage::AllNotesOff,
                                    0x04 => ChannelModeMessage::OmniModeOff,
                                    0x05 => ChannelModeMessage::OmniModeOn,
                                    0x06 => ChannelModeMessage::MonoMode {
                                        number_of_voices: data_byte_2,
                                    },
                                    0x07 => ChannelModeMessage::PolyMode,
                                    _ => unreachable!(),
                                }),
                                // Control Change
                                _ => ChannelMessage::Voice(ChannelVoiceMessage::ControlChange {
                                    controller: data_byte_1,
                                    value: data_byte_2,
                                }),
                            }
                        }
                        // Program Change
                        0xC0 => {
                            if data_bytes.len() != 1 {
                                return Err("Program Change message requires exactly 1 data byte"
                                    .to_string());
                            }
                            ChannelMessage::Voice(ChannelVoiceMessage::ProgramChange(
                                validate_midi_data_byte(data_bytes[0])?,
                            ))
                        }
                        // Channel Pressure
                        0xD0 => {
                            if data_bytes.len() != 1 {
                                return Err(
                                    "Channel Pressure message requires exactly 1 data byte"
                                        .to_string(),
                                );
                            }
                            ChannelMessage::Voice(ChannelVoiceMessage::ChannelPressure(
                                validate_midi_data_byte(data_bytes[0])?,
                            ))
                        }
                        // Pitch Bend Change
                        0xE0 => {
                            if data_bytes.len() != 2 {
                                return Err(
                                    "Pitch Bend Change message requires exactly 2 data bytes"
                                        .to_string(),
                                );
                            }
                            let lsb = validate_midi_data_byte(data_bytes[0])?;
                            let msb = validate_midi_data_byte(data_bytes[1])?;
                            let value = ((msb as u16) << 7) | (lsb as u16);
                            ChannelMessage::Voice(ChannelVoiceMessage::PitchBendChange(value))
                        }
                        _ => unreachable!(),
                    },
                }
            }
            // System Exclusive Message
            0xF0 => {
                if data_bytes.is_empty() {
                    return Err("System Exclusive Message cannot be empty".to_string());
                }

                let sub_id = validate_midi_data_byte(data_bytes[0])?;

                match data_bytes[0] {
                    ..=0x7C => {
                        /* FIXME: System Real Time messages (0xF8-0xFF) may appear within data bytes
                         * we may want to filter them out or handle them differently
                         * any other status byte (i.e. > 127) will be considered as EOX and terminate the message
                         */
                        let mut manufacturer_id = data_bytes[0..1].to_vec();
                        if manufacturer_id[0] == 0x00 {
                            if data_bytes.len() < 3 {
                                return Err(
                                    "Manufacturer Identification System Exclusive message requires at least 3 data bytes"
                                        .to_string(),
                                );
                            }
                            manufacturer_id.extend_from_slice(&data_bytes[1..3]);
                        }

                        let data_bytes_offset = manufacturer_id.len();
                        MidiMessage::System(SystemMessage::Exclusive(SystemExclusiveMessage {
                            sub_id: SystemExclusiveSubId::ManufacturerIdentification(
                                manufacturer_id,
                            ),
                            data: data_bytes[data_bytes_offset..].to_vec(),
                        }))
                    }
                    0x7D => MidiMessage::System(SystemMessage::Exclusive(SystemExclusiveMessage {
                        sub_id: SystemExclusiveSubId::NonCommercial,
                        data: data_bytes[1..].to_vec(),
                    })),
                    0x7E => MidiMessage::System(SystemMessage::Exclusive(SystemExclusiveMessage {
                        sub_id: SystemExclusiveSubId::NonRealTime,
                        data: data_bytes[1..].to_vec(),
                    })),
                    0x7F => MidiMessage::System(SystemMessage::Exclusive(SystemExclusiveMessage {
                        sub_id: SystemExclusiveSubId::RealTime,
                        data: data_bytes[1..].to_vec(),
                    })),
                    _ => unreachable!("Invalid System Exclusive sub ID: {}", sub_id),
                }
            }
            // System Common Messages
            0xF1 => {
                if data_bytes.len() != 1 {
                    return Err(
                        "System Common Message: MIDI Time Code Quarter Frame requires exactly 1 data byte"
                            .to_string(),
                    );
                }
                // TODO: add data byte to the structured message when needed
                let _data_byte = validate_midi_data_byte(data_bytes[0])?;
                MidiMessage::System(SystemMessage::Common(
                    SystemCommonMessage::MidiTimeCodeQuarterFrame,
                ))
            }
            0xF2 => {
                if data_bytes.len() != 2 {
                    return Err(
                        "System Common Message: Song Position Pointer requires exactly 2 data bytes"
                            .to_string(),
                    );
                }
                let lsb = validate_midi_data_byte(data_bytes[0])?;
                let msb = validate_midi_data_byte(data_bytes[1])?;
                let position = ((msb as u16) << 7) | (lsb as u16);
                MidiMessage::System(SystemMessage::Common(
                    SystemCommonMessage::SongPositionPointer(position),
                ))
            }
            0xF3 => {
                if data_bytes.len() != 1 {
                    return Err(
                        "System Common Message: Song Select requires exactly 1 data byte"
                            .to_string(),
                    );
                }
                let song_number = validate_midi_data_byte(data_bytes[0])?;
                MidiMessage::System(SystemMessage::Common(SystemCommonMessage::SongSelect(
                    song_number,
                )))
            }
            0xF6 => {
                if !data_bytes.is_empty() {
                    return Err(
                        "System Common Message: Tune Request does not require any data bytes"
                            .to_string(),
                    );
                }
                MidiMessage::System(SystemMessage::Common(SystemCommonMessage::TuneRequest))
            }
            // Stand-alone End of System Exclusive Message
            0xF7 => MidiMessage::System(SystemMessage::Common(
                SystemCommonMessage::EndOfSystemExclusive,
            )),
            // System Real-Time Messages
            // FIXME: these messages may appear within System Exclusive data bytes
            // FIXME: we did not check the length of data bytes for these messages
            0xF8 => {
                MidiMessage::System(SystemMessage::RealTime(SystemRealTimeMessage::TimingClock))
            }
            0xFA => MidiMessage::System(SystemMessage::RealTime(SystemRealTimeMessage::Start)),
            0xFB => MidiMessage::System(SystemMessage::RealTime(SystemRealTimeMessage::Continue)),
            0xFC => MidiMessage::System(SystemMessage::RealTime(SystemRealTimeMessage::Stop)),
            0xFE => MidiMessage::System(SystemMessage::RealTime(
                SystemRealTimeMessage::ActiveSensing,
            )),
            0xFF => {
                MidiMessage::System(SystemMessage::RealTime(SystemRealTimeMessage::SystemReset))
            }
            _ => {
                return Err(format!(
                    "Invalid MIDI status byte: {}. Must be between 0x80 and 0xFF.",
                    status_byte
                ));
            }
        };

        Ok(msg)
    }
}

impl From<MidiMessage> for Vec<u8> {
    fn from(value: MidiMessage) -> Self {
        let mut result = Vec::new();
        match value {
            MidiMessage::Channel { channel, message } => {
                let channel_u8 = u8::from(channel);
                match message {
                    ChannelMessage::Voice(voice_msg) => match voice_msg {
                        ChannelVoiceMessage::NoteOff { note, velocity } => {
                            result.push(0x80 | channel_u8);
                            result.push(note);
                            result.push(velocity);
                        }
                        ChannelVoiceMessage::NoteOn { note, velocity } => {
                            result.push(0x90 | channel_u8);
                            result.push(note);
                            result.push(velocity);
                        }
                        ChannelVoiceMessage::PolyphonicKeyPressure { note, pressure } => {
                            result.push(0xA0 | channel_u8);
                            result.push(note);
                            result.push(pressure);
                        }
                        ChannelVoiceMessage::ControlChange { controller, value } => {
                            result.push(0xB0 | channel_u8);
                            result.push(controller);
                            result.push(value);
                        }
                        ChannelVoiceMessage::ProgramChange(program) => {
                            result.push(0xC0 | channel_u8);
                            result.push(program);
                        }
                        ChannelVoiceMessage::ChannelPressure(pressure) => {
                            result.push(0xD0 | channel_u8);
                            result.push(pressure);
                        }
                        ChannelVoiceMessage::PitchBendChange(value) => {
                            let lsb = (value & 0x7F) as u8;
                            let msb = ((value >> 7) & 0x7F) as u8;
                            result.push(0xE0 | channel_u8);
                            result.push(lsb);
                            result.push(msb);
                        }
                    },
                    ChannelMessage::Mode(mode_msg) => match mode_msg {
                        ChannelModeMessage::AllSoundOff => {
                            result.push(0xB0 | channel_u8);
                            result.push(0x78);
                            result.push(0x00);
                        }
                        ChannelModeMessage::ResetAllControllers => {
                            result.push(0xB0 | channel_u8);
                            result.push(0x79);
                            result.push(0x00);
                        }
                        ChannelModeMessage::LocalControlOff => {
                            result.push(0xB0 | channel_u8);
                            result.push(0x7A);
                            result.push(0x00);
                        }
                        ChannelModeMessage::LocalControlOn => {
                            result.push(0xB0 | channel_u8);
                            result.push(0x7A);
                            result.push(0x7F);
                        }
                        ChannelModeMessage::AllNotesOff => {
                            result.push(0xB0 | channel_u8);
                            result.push(0x7B);
                            result.push(0x00);
                        }
                        ChannelModeMessage::OmniModeOff => {
                            result.push(0xB0 | channel_u8);
                            result.push(0x7C);
                            result.push(0x00);
                        }
                        ChannelModeMessage::OmniModeOn => {
                            result.push(0xB0 | channel_u8);
                            result.push(0x7D);
                            result.push(0x00);
                        }
                        ChannelModeMessage::MonoMode { number_of_voices } => {
                            result.push(0xB0 | channel_u8);
                            result.push(0x7E);
                            result.push(number_of_voices);
                        }
                        ChannelModeMessage::PolyMode => {
                            result.push(0xB0 | channel_u8);
                            result.push(0x7F);
                            result.push(0x00);
                        }
                    },
                }
            }
            MidiMessage::System(system_msg) => match system_msg {
                SystemMessage::Common(common_msg) => {
                    match common_msg {
                        SystemCommonMessage::MidiTimeCodeQuarterFrame => {
                            // FIXME: not implemented yet
                            todo!("MIDI Time Code Quarter Frame not implemented yet");
                        }
                        SystemCommonMessage::SongPositionPointer(position) => {
                            result.push(0xF2);
                            result.push((position & 0x7F) as u8); // LSB
                            result.push(((position >> 7) & 0x7F) as u8); // MSB
                        }
                        SystemCommonMessage::SongSelect(song_number) => {
                            result.push(0xF3);
                            result.push(song_number);
                        }
                        SystemCommonMessage::TuneRequest => {
                            result.push(0xF6);
                        }
                        SystemCommonMessage::EndOfSystemExclusive => {
                            result.push(0xF7);
                        }
                    }
                }
                SystemMessage::Exclusive(exclusive_msg) => {
                    result.push(0xF0);
                    match exclusive_msg.sub_id {
                        SystemExclusiveSubId::ManufacturerIdentification(manufacturer_id) => {
                            result.extend_from_slice(&manufacturer_id);
                        }
                        SystemExclusiveSubId::NonCommercial => {
                            result.push(0x7D);
                        }
                        SystemExclusiveSubId::NonRealTime => {
                            result.push(0x7E);
                        }
                        SystemExclusiveSubId::RealTime => {
                            result.push(0x7F);
                        }
                    }
                    result.extend_from_slice(&exclusive_msg.data);
                }
                SystemMessage::RealTime(real_time_msg) => match real_time_msg {
                    SystemRealTimeMessage::TimingClock => result.push(0xF8),
                    SystemRealTimeMessage::Start => result.push(0xFA),
                    SystemRealTimeMessage::Continue => result.push(0xFB),
                    SystemRealTimeMessage::Stop => result.push(0xFC),
                    SystemRealTimeMessage::ActiveSensing => result.push(0xFE),
                    SystemRealTimeMessage::SystemReset => result.push(0xFF),
                },
            },
        }

        result
    }
}

impl MidiMessage {
    // Channel Voice Messages
    pub fn note_off(channel: MidiChannel, note: u8, velocity: u8) -> Result<Self, String> {
        Ok(MidiMessage::Channel {
            channel,
            message: ChannelMessage::Voice(ChannelVoiceMessage::NoteOff {
                note: validate_midi_data_byte(note)?,
                velocity: validate_midi_data_byte(velocity)?,
            }),
        })
    }

    pub fn note_on(channel: MidiChannel, note: u8, velocity: u8) -> Result<Self, String> {
        Ok(MidiMessage::Channel {
            channel,
            message: ChannelMessage::Voice(ChannelVoiceMessage::NoteOn {
                note: validate_midi_data_byte(note)?,
                velocity: validate_midi_data_byte(velocity)?,
            }),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TimeStampedMidiMessage {
    pub timestamp: Instant,
    pub message: MidiMessage,
}
