use std::sync::Mutex;

use midir::MidiInput;

pub mod commands;

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MidiInputPort {
    pub name: String,
    pub id: String,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MidiOutputPort {
    pub name: String,
    pub id: String,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MidiStateInner {
    input: Vec<MidiInputPort>,
    output: Vec<MidiOutputPort>,
}

impl MidiStateInner {
    pub fn scan_input(&mut self) -> Result<(), String> {
        self.input.clear();

        let input = MidiInput::new("midirc")
            .map_err(|e| format!("Failed to scan for input ports: {}", e))?;

        for port in input.ports().iter() {
            let id = port.id();
            let name = input.port_name(port);

            if let Ok(name) = name {
                self.input.push(MidiInputPort { name, id });
            }
        }

        Ok(())
    }

    pub fn scan_output(&mut self) -> Result<(), String> {
        self.output.clear();

        let output = midir::MidiOutput::new("midirc")
            .map_err(|e| format!("Failed to scan for output ports: {}", e))?;

        for port in output.ports().iter() {
            let id = port.id();
            let name = output.port_name(port);

            if let Ok(name) = name {
                self.output.push(MidiOutputPort { name, id });
            }
        }

        Ok(())
    }
}

pub type MidiState = Mutex<MidiStateInner>;
