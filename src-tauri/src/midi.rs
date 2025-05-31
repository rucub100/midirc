use std::{fmt::Debug, sync::Mutex};

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
pub struct MidiInputConnection {
    pub port: MidiInputPort,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MidiOutputConnection {
    pub port: MidiOutputPort,
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MidiStateInner {
    available_input_ports: Vec<MidiInputPort>,
    available_output_ports: Vec<MidiOutputPort>,
    #[serde(skip)]
    _input_connection: Option<midir::MidiInputConnection<()>>,
    input_connection: Option<MidiInputConnection>,
    #[serde(skip)]
    _output_connection: Option<midir::MidiOutputConnection>,
    output_connection: Option<MidiOutputConnection>,
}

impl Debug for MidiStateInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MidiStateInner")
            .field("available_input_ports", &self.available_input_ports)
            .field("available_output_ports", &self.available_output_ports)
            .field("input_connection", &self.input_connection)
            .finish()
    }
}

impl Clone for MidiStateInner {
    fn clone(&self) -> Self {
        MidiStateInner {
            available_input_ports: self.available_input_ports.clone(),
            available_output_ports: self.available_output_ports.clone(),
            _input_connection: None,
            input_connection: self.input_connection.clone(),
            _output_connection: None,
            output_connection: self.output_connection.clone(),
        }
    }
}

impl MidiStateInner {
    pub fn scan_input(&mut self) -> Result<(), String> {
        self.available_input_ports.clear();

        let input = MidiInput::new("midirc")
            .map_err(|e| format!("Failed to scan for input ports: {}", e))?;

        for port in input.ports().iter() {
            let id = port.id();
            let name = input.port_name(port);

            if let Ok(name) = name {
                self.available_input_ports.push(MidiInputPort { name, id });
            }
        }

        Ok(())
    }

    pub fn scan_output(&mut self) -> Result<(), String> {
        self.available_output_ports.clear();

        let output = midir::MidiOutput::new("midirc")
            .map_err(|e| format!("Failed to scan for output ports: {}", e))?;

        for port in output.ports().iter() {
            let id = port.id();
            let name = output.port_name(port);

            if let Ok(name) = name {
                self.available_output_ports
                    .push(MidiOutputPort { name, id });
            }
        }

        Ok(())
    }

    pub fn disconnect_input(&mut self) {
        self.input_connection = None;
        self._input_connection = None;
    }

    pub fn connect_input(&mut self, index: usize) -> Result<(), String> {
        if self.input_connection.is_some() || self._input_connection.is_some() {
            return Err("Input connection already exists. Disconnect first.".to_string());
        }

        let input = MidiInput::new("midirc")
            .map_err(|e| format!("Failed to connect for input ports: {}", e))?;

        let port = self.available_input_ports.get(index).ok_or_else(|| {
            format!(
                "Input port index out of bounds: {}. Available ports: {}",
                index,
                self.available_input_ports.len()
            )
        })?;
        let midi_port = input.find_port_by_id(port.id.clone());
        let midi_port =
            midi_port.ok_or_else(|| format!("Input port not found: {}", port.name.as_str()))?;

        let connection = input
            .connect(
                &midi_port,
                port.name.as_str(),
                |_, message, _| {
                    println!("Received MIDI message: {:?}", message);
                },
                (),
            )
            .map_err(|e| format!("Failed to connect to input port: {}", e))?;

        self._input_connection = Some(connection);
        self.input_connection = Some(MidiInputConnection {
            port: port.to_owned(),
        });

        Ok(())
    }

    pub fn disconnect_output(&mut self) {
        self.output_connection = None;
        self._output_connection = None;
    }

    pub fn connect_output(&mut self, index: usize) -> Result<(), String> {
        if self.output_connection.is_some() || self._output_connection.is_some() {
            return Err("Output connection already exists. Disconnect first.".to_string());
        }

        let output = midir::MidiOutput::new("midirc")
            .map_err(|e| format!("Failed to connect for output ports: {}", e))?;

        let port = self.available_output_ports.get(index).ok_or_else(|| {
            format!(
                "Output port index out of bounds: {}. Available ports: {}",
                index,
                self.available_output_ports.len()
            )
        })?;
        let midi_port = output.find_port_by_id(port.id.clone());
        let midi_port =
            midi_port.ok_or_else(|| format!("Output port not found: {}", port.name.as_str()))?;

        let connection = output
            .connect(&midi_port, port.name.as_str())
            .map_err(|e| format!("Failed to connect to output port: {}", e))?;

        self._output_connection = Some(connection);
        self.output_connection = Some(MidiOutputConnection {
            port: port.to_owned(),
        });

        Ok(())
    }
}

pub type MidiState = Mutex<MidiStateInner>;
