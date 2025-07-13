use std::time::Duration;

use crate::midi::{MidiStateInner, playback::MidiPlayback};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MidiInputPort {
    pub id: String,
    pub name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MidiOutputPort {
    pub id: String,
    pub name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Midi {
    pub available_input_ports: Vec<MidiInputPort>,
    pub available_output_ports: Vec<MidiOutputPort>,
    pub input_connection: Option<MidiInputPort>,
    pub output_connection: Option<MidiOutputPort>,
}

impl From<&MidiStateInner> for Midi {
    fn from(state: &MidiStateInner) -> Self {
        Midi {
            available_input_ports: state
                .available_input_ports
                .iter()
                .map(|port| MidiInputPort {
                    id: port.id.clone(),
                    name: port.name.clone(),
                })
                .collect(),
            available_output_ports: state
                .available_output_ports
                .iter()
                .map(|port| MidiOutputPort {
                    id: port.id.clone(),
                    name: port.name.clone(),
                })
                .collect(),
            input_connection: state.input_connection.as_ref().map(|c| MidiInputPort {
                id: c.port.id.clone(),
                name: c.port.name.clone(),
            }),
            output_connection: state.output_connection.as_ref().map(|c| MidiOutputPort {
                id: c.port.id.clone(),
                name: c.port.name.clone(),
            }),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum RecorderState {
    Stopped,
    Recording,
    Paused,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Recording {
    index: usize,
    duration_milliseconds: u32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Recorder {
    pub state: RecorderState,
    pub recordings: Vec<Recording>,
}

impl From<&crate::midi::recorder::MidiRecorder> for Recorder {
    fn from(value: &crate::midi::recorder::MidiRecorder) -> Self {
        let state = match value.get_state() {
            crate::midi::recorder::RecorderState::Stopped => RecorderState::Stopped,
            crate::midi::recorder::RecorderState::Recording => RecorderState::Recording,
        };

        let calc_duration_milliseconds =
            |recording: &Vec<crate::midi::message::TimeStampedMidiMessage>| {
                if recording.is_empty() {
                    0
                } else if recording.len() == 1 {
                    (recording.first().unwrap().timestamp_microseconds / 1000) as u32
                } else {
                    let start_time = recording.first().unwrap().timestamp_microseconds;
                    let end_time = recording.last().unwrap().timestamp_microseconds;
                    ((end_time - start_time) / 1000) as u32
                }
            };
        let recordings = value
            .get_recordings()
            .iter()
            .enumerate()
            .map(|(index, recording)| Recording {
                index,
                duration_milliseconds: calc_duration_milliseconds(recording),
            })
            .collect();

        Recorder { state, recordings }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum PlaybackIdentifier {
    Recording { index: usize },
    MidiFile { path: String },
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    index: usize,
    duration_milliseconds: u32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Playback {
    pub state: PlaybackState,
    pub identifier: Option<PlaybackIdentifier>,
    pub tracks: Vec<Track>,
    pub duration_milliseconds: Option<u32>,
    pub position_milliseconds: u32,
}

impl From<&MidiPlayback> for Playback {
    fn from(playback: &MidiPlayback) -> Self {
        let src_state = playback.get_state();
        let (state, identifier) = match src_state {
            crate::midi::playback::PlaybackState::Stopped => (PlaybackState::Stopped, None),
            crate::midi::playback::PlaybackState::Playing(track_info) => (
                PlaybackState::Playing,
                match track_info {
                    crate::midi::playback::TrackInfo::Recording(index) => {
                        Some(PlaybackIdentifier::Recording { index })
                    }
                    crate::midi::playback::TrackInfo::StandardMidiFile(index) => {
                        Some(PlaybackIdentifier::MidiFile {
                            path: format!("Track {}", index),
                        })
                    }
                },
            ),
            crate::midi::playback::PlaybackState::Paused(track_info) => (
                PlaybackState::Paused,
                match track_info {
                    crate::midi::playback::TrackInfo::Recording(index) => {
                        Some(PlaybackIdentifier::Recording { index })
                    }
                    crate::midi::playback::TrackInfo::StandardMidiFile(index) => {
                        Some(PlaybackIdentifier::MidiFile {
                            path: format!("Track {}", index),
                        })
                    }
                },
            ),
        };
        let src_tracks = playback.get_tracks();
        let tracks = src_tracks
            .iter()
            .enumerate()
            .map(|(index, track)| Track {
                duration_milliseconds: (track.iter().map(|(delta, _)| delta).sum::<u64>() / 1000)
                    as u32,
                index,
            })
            .collect();
        let duration_milliseconds = playback.get_duration().map(|d| d.as_millis() as u32);
        let position_milliseconds = playback.get_position().as_millis() as u32;

        Playback {
            state,
            identifier,
            tracks,
            duration_milliseconds,
            position_milliseconds,
        }
    }
}
