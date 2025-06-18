use std::{
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use crate::midi::message::{MidiMessage, TimeStampedMidiMessage};

type MidiPlayerFn = Box<dyn Fn(Vec<u8>) -> Result<(), String> + Send + 'static>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

pub struct MidiPlayback {
    state: PlaybackState,
    buffer: Option<Vec<TimeStampedMidiMessage>>,
    player: Arc<Mutex<Option<MidiPlayerFn>>>,
    position: Arc<AtomicUsize>,
    // Thread management
    thread_handle: Option<JoinHandle<()>>,
    signal_stop: Option<Arc<AtomicBool>>,
    signal_pause: Option<Arc<AtomicBool>>,
}

impl Default for MidiPlayback {
    fn default() -> Self {
        Self {
            state: PlaybackState::Stopped,
            buffer: None,
            player: Arc::new(Mutex::new(None)),
            position: Arc::new(AtomicUsize::new(0)),
            thread_handle: None,
            signal_stop: None,
            signal_pause: None,
        }
    }
}

impl MidiPlayback {
    pub fn set_player<F>(&mut self, player: F) -> Result<(), String>
    where
        F: Fn(Vec<u8>) -> Result<(), String> + Send + 'static,
    {
        if self.state != PlaybackState::Stopped {
            return Err("Cannot set player while playback is in progress".to_string());
        }
        let mut player_lock = self.player.lock().unwrap();
        *player_lock = Some(Box::new(player));
        Ok(())
    }
    pub fn load_data(&mut self, data: &Vec<TimeStampedMidiMessage>) -> Result<(), String> {
        // Validation checks
        if self.state != PlaybackState::Stopped {
            return Err("First stop playback before loading new data".to_string());
        }
        if data.is_empty() {
            return Err("Cannot load empty data".to_string());
        }

        // Normalize data for playback
        let mut normalized_data = data.clone();
        normalized_data.sort_by_key(|msg| msg.timestamp_microseconds);
        let start_timestamp = normalized_data.first().unwrap().timestamp_microseconds;
        if start_timestamp > 0 {
            normalized_data.iter_mut().for_each(|msg| {
                msg.timestamp_microseconds -= start_timestamp;
            });
        }
        // convert absolute timestamps to relative timestamps (delta)
        normalized_data = normalized_data
            .iter()
            .enumerate()
            .map(|(index, msg)| {
                if index > 0 {
                    TimeStampedMidiMessage {
                        timestamp_microseconds: msg.timestamp_microseconds
                            - normalized_data[index - 1].timestamp_microseconds,
                        message: msg.message.clone(),
                    }
                } else {
                    msg.clone()
                }
            })
            .collect(); // FIXME: also convert structured MIDI messages to bytes

        self.buffer = Some(normalized_data);
        self.position.store(0, Ordering::SeqCst);

        Ok(())
    }

    pub fn play(&mut self) -> Result<(), String> {
        // FIXME: cleanup previous playback is thread already finished (we may need to do this for multiple playback commands)
        // Validation checks
        if self.state != PlaybackState::Stopped {
            return Err("Playback is already in progress or paused".to_string());
        }
        if self.buffer.is_none() {
            return Err("No data loaded for playback".to_string());
        }
        if self.position.load(Ordering::SeqCst) >= self.buffer.as_ref().unwrap().len() {
            return Err("Playback position is out of bounds".to_string());
        }
        if self.thread_handle.is_some() {
            return Err("Playback thread is already running".to_string());
        }
        if self.signal_stop.is_some() || self.signal_pause.is_some() {
            return Err("Playback signals are already initialized".to_string());
        }

        // Initialize playback
        let signal_stop = Arc::new(AtomicBool::new(false));
        let signal_pause = Arc::new(AtomicBool::new(false));
        self.signal_stop = Some(signal_stop.clone());
        self.signal_pause = Some(signal_pause.clone());
        let playback_data = self.buffer.as_ref().unwrap().clone();
        let position = self.position.clone();
        let player = self.player.clone();
        let playback_thread = thread::spawn(move || {
            let start = Instant::now();
            let mut time = Duration::ZERO;
            for (index, msg) in playback_data.iter().enumerate() {
                while signal_pause.load(Ordering::SeqCst) {
                    thread::sleep(Duration::from_millis(100));
                }
                if signal_stop.load(Ordering::SeqCst) {
                    break;
                }

                time += Duration::from_micros(msg.timestamp_microseconds);
                let elapsed = start.elapsed();
                if elapsed < time {
                    thread::sleep(time - elapsed);
                }

                if let Some(player) = player.lock().unwrap().as_ref() {
                    // FIXME: make sure message is already in bytes format at this point (performance)
                    if let Err(error) = player(msg.message.clone().into()) {
                        eprintln!("{error}");
                        break;
                    }
                } else {
                    break;
                }

                position.store(index + 1, Ordering::SeqCst);
            }
        });
        self.thread_handle = Some(playback_thread);

        Ok(())
    }

    pub fn pause(&mut self) -> Result<(), String> {
        todo!("Implement pause functionality");
        Ok(())
    }

    pub fn resume(&mut self) -> Result<(), String> {
        todo!("Implement resume functionality");
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), String> {
        todo!("Implement stop functionality");
        Ok(())
    }
}
