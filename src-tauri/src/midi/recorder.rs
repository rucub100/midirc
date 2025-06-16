use std::{collections::VecDeque, time::Instant};

use crate::midi::message::TimeStampedMidiMessage;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RecorderState {
    Stopped,
    Recording { start: Instant },
    RecordingPaused { pause: Instant },
    Playing,
    PlayingPaused { index: usize },
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
    pub fn start_recording(&mut self) -> Result<(), String> {
        if self.state != RecorderState::Stopped {
            return Err("First stop the recorder before starting a new recording".to_string());
        }

        self.state = RecorderState::Recording {
            start: Instant::now(),
        };
        self.buffer.clear();

        Ok(())
    }

    pub fn pause_recording(&mut self) -> Result<(), String> {
        if let RecorderState::Recording { start: _ } = self.state {
            return Err("Recorder is not currently recording".to_string());
        }

        self.state = RecorderState::RecordingPaused {
            pause: Instant::now(),
        };

        Ok(())
    }
}
