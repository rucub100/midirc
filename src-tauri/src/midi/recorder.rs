use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use crate::midi::message::{MidiMessage, TimeStampedMidiMessage};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RecorderState {
    Stopped,
    Recording { start: Instant },
    Paused { elapsed: Duration },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MidiRecorder {
    state: RecorderState,
    buffer: VecDeque<TimeStampedMidiMessage>,
}

impl Default for MidiRecorder {
    fn default() -> Self {
        Self {
            state: RecorderState::Stopped,
            buffer: VecDeque::new(),
        }
    }
}

impl MidiRecorder {
    pub fn get_state(&self) -> RecorderState {
        self.state.clone()
    }

    pub fn start_recording(&mut self) -> Result<(), String> {
        if let RecorderState::Stopped = self.state {
            self.buffer.clear();
            self.state = RecorderState::Recording {
                start: Instant::now(),
            };
            Ok(())
        } else {
            Err("Recorder is already recording or paused".to_string())
        }
    }

    pub fn pause_recording(&mut self) -> Result<(), String> {
        if let RecorderState::Recording { start } = self.state {
            self.state = RecorderState::Paused {
                elapsed: start.elapsed(),
            };
            Ok(())
        } else {
            Err("Recorder is not currently recording".to_string())
        }
    }

    pub fn resume_recording(&mut self) -> Result<(), String> {
        if let RecorderState::Paused { elapsed } = self.state {
            self.state = RecorderState::Recording {
                start: Instant::now().checked_sub(elapsed).unwrap(),
            };
            Ok(())
        } else {
            Err("Recorder is not currently paused".to_string())
        }
    }

    pub fn stop_recording(&mut self) -> Result<(), String> {
        if let RecorderState::Stopped = self.state {
            return Err("Recorder is not currently recording".to_string());
        }
        self.state = RecorderState::Stopped;
        Ok(())
    }

    pub fn add_message(&mut self, message: MidiMessage) -> Result<(), String> {
        if let RecorderState::Recording { start } = self.state {
            self.buffer.push_back(TimeStampedMidiMessage {
                timestamp_microseconds: start
                    .elapsed()
                    .as_micros()
                    .try_into()
                    .map_err(|_| "Timestamp overflow")?,
                message,
            });
            Ok(())
        } else {
            return Err("Recorder is not currently recording".to_string());
        }
    }
}
