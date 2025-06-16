use std::sync::{Arc, Mutex};

use midir::MidiInput;
use tauri::ipc::Channel;

use crate::midi::message::MidiMessage;

pub mod commands;
pub mod message;
pub mod smf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MidiInputPort {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MidiOutputPort {
    pub name: String,
    pub id: String,
}

pub struct MidiInputConnection {
    pub port: MidiInputPort,
    _connection: midir::MidiInputConnection<()>,
}

pub struct MidiOutputConnection {
    pub port: MidiOutputPort,
    _connection: Arc<Mutex<midir::MidiOutputConnection>>,
}

#[derive(Default)]
pub struct MidiStateInner {
    pub available_input_ports: Vec<MidiInputPort>,
    pub available_output_ports: Vec<MidiOutputPort>,
    pub input_connection: Option<MidiInputConnection>,
    pub output_connection: Option<MidiOutputConnection>,
    pub frontend_channel: Arc<Mutex<Option<Channel<MidiMessage>>>>,
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
    }

    pub fn connect_input(&mut self, index: usize) -> Result<(), String> {
        if self.input_connection.is_some() {
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

        let frontend_channel = self.frontend_channel.clone();
        let connection = input
            .connect(
                &midi_port,
                port.name.as_str(),
                move |_, message, _| {
                    let midi_message_result: Result<MidiMessage, String> = message.try_into();
                    let message = match midi_message_result {
                        Ok(msg) => msg,
                        Err(e) => {
                            eprintln!("Error parsing MIDI message: {}", e);
                            return;
                        }
                    };

                    let frontend_channel = frontend_channel.lock().unwrap();
                    if let Some(ref ch) = *frontend_channel {
                        ch.send(message).unwrap_or_else(|e| {
                            eprintln!("Failed to send MIDI message to frontend: {}", e);
                        });
                    }
                },
                (),
            )
            .map_err(|e| format!("Failed to connect to input port: {}", e))?;

        self.input_connection = Some(MidiInputConnection {
            port: port.to_owned(),
            _connection: connection,
        });

        Ok(())
    }

    pub fn disconnect_output(&mut self) {
        self.output_connection = None;
    }

    pub fn connect_output(&mut self, index: usize) -> Result<(), String> {
        if self.output_connection.is_some() {
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

        self.output_connection = Some(MidiOutputConnection {
            port: port.to_owned(),
            _connection: Arc::new(Mutex::new(connection)),
        });

        Ok(())
    }

    pub fn send_message(&self, message: MidiMessage) -> Result<(), String> {
        if self.output_connection.is_none() {
            return Err(
                "No output connection established. Connect to an output port first.".to_string(),
            );
        }

        let data: Vec<u8> = message.into();

        let connection = self.output_connection.as_ref().unwrap()._connection.clone();
        let mut connection = connection.lock().unwrap();
        connection
            .send(data.as_slice())
            .map_err(|e| format!("Failed to send MIDI message: {}", e))?;
        Ok(())
    }

    pub fn set_frontend_channel(&mut self, channel: Channel<MidiMessage>) {
        let mut frontend_channel = self.frontend_channel.lock().unwrap();
        *frontend_channel = Some(channel);
    }
}

pub type MidiState = Mutex<MidiStateInner>;
