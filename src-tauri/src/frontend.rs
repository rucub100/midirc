use std::time::Duration;

use crate::midi::MidiStateInner;

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
pub struct Recorder {
    pub state: RecorderState,
    pub elapsed: Option<Duration>,
}

impl From<&crate::midi::recorder::RecorderState> for Recorder {
    fn from(value: &crate::midi::recorder::RecorderState) -> Self {
        let (state, elapsed) = {
            match value {
                crate::midi::recorder::RecorderState::Stopped => (RecorderState::Stopped, None),
                crate::midi::recorder::RecorderState::Recording { start } => {
                    (RecorderState::Recording, Some(start.elapsed()))
                }
                crate::midi::recorder::RecorderState::Paused { elapsed } => {
                    (RecorderState::Paused, Some(elapsed.clone()))
                }
            }
        };

        Recorder { state, elapsed }
    }
}
