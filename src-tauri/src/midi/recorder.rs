use crate::midi::message::{MidiMessage, TimeStampedMidiMessage};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RecorderState {
    Stopped,
    Recording,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MidiRecorder {
    state: RecorderState,
    buffer: Vec<TimeStampedMidiMessage>,
    recordings: Vec<Vec<TimeStampedMidiMessage>>,
}

impl Default for MidiRecorder {
    fn default() -> Self {
        Self {
            state: RecorderState::Stopped,
            buffer: Vec::new(),
            recordings: Vec::new(),
        }
    }
}

impl MidiRecorder {
    pub fn get_state(&self) -> RecorderState {
        self.state.clone()
    }

    pub fn start_recording(&mut self) -> Result<(), String> {
        if self.state == RecorderState::Stopped {
            self.buffer.clear();
            self.state = RecorderState::Recording;
            Ok(())
        } else {
            Err("Recorder is already recording".to_string())
        }
    }

    pub fn stop_recording(&mut self) -> Result<(), String> {
        if self.state == RecorderState::Stopped {
            return Err("Recorder is not currently recording".to_string());
        }

        self.state = RecorderState::Stopped;

        if !self.buffer.is_empty() {
            self.recordings.push(self.buffer.clone());
            self.buffer.clear();
        }

        Ok(())
    }

    pub fn get_recordings(&self) -> &[Vec<TimeStampedMidiMessage>] {
        self.recordings.as_slice()
    }

    pub fn remove_recording(&mut self, index: usize) -> Result<(), String> {
        if index < self.recordings.len() {
            self.recordings.remove(index);
            Ok(())
        } else {
            Err("Recording index out of bounds".to_string())
        }
    }

    pub fn add_message(
        &mut self,
        message: MidiMessage,
        timestamp_microseconds: u64,
    ) -> Result<(), String> {
        // FIXME: We may want to filter messages here (e.g. skip active sensing messages)
        if self.state == RecorderState::Recording {
            self.buffer.push(TimeStampedMidiMessage {
                timestamp_microseconds,
                message,
            });
            Ok(())
        } else {
            return Err("Recorder is not currently recording".to_string());
        }
    }
}
