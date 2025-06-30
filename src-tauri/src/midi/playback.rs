use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    thread::{self},
    time::{Duration, Instant},
};

use tauri::async_runtime::JoinHandle;

use crate::midi::message::{MidiChannel, MidiMessage, TimeStampedMidiMessage};

type MidiPlayerFn = Arc<dyn Fn(&[u8]) -> Result<(), String> + Sync + Send + 'static>;

const MAX_SLEEP_DURATION: Duration = Duration::from_millis(50);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TrackInfo {
    Recording(usize),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PlaybackState {
    Stopped,
    Playing(TrackInfo),
    Paused(TrackInfo),
}

pub struct MidiPlaybackInner {
    state: PlaybackState,
    player: Option<MidiPlayerFn>,
    position_milliseconds: Arc<AtomicUsize>,
    duration_milliseconds: Option<Arc<AtomicUsize>>,
    // Thread management
    thread_handle: Option<JoinHandle<()>>,
    signal_stop: Option<Arc<AtomicBool>>,
    signal_pause: Option<Arc<AtomicBool>>,
}

pub struct MidiPlayback {
    inner: Arc<Mutex<MidiPlaybackInner>>,
}

impl Default for MidiPlayback {
    fn default() -> Self {
        Self {
            inner: Arc::new(Mutex::new(MidiPlaybackInner {
                state: PlaybackState::Stopped,
                player: None,
                position_milliseconds: Arc::new(AtomicUsize::new(0)),
                duration_milliseconds: None,
                thread_handle: None,
                signal_stop: None,
                signal_pause: None,
            })),
        }
    }
}

impl MidiPlayback {
    pub fn get_state(&self) -> PlaybackState {
        let inner = self.inner.lock().unwrap();
        inner.state.clone()
    }

    pub fn get_duration(&self) -> Option<Duration> {
        let inner = self.inner.lock().unwrap();
        match &inner.duration_milliseconds {
            Some(duration) => Some(Duration::from_millis(duration.load(Ordering::SeqCst) as u64)),
            None => None,
        }
    }

    pub fn get_position(&self) -> Duration {
        let inner = self.inner.lock().unwrap();
        Duration::from_millis(inner.position_milliseconds.load(Ordering::SeqCst) as u64)
    }

    pub fn set_player<F>(&mut self, player: F) -> Result<(), String>
    where
        F: Fn(&[u8]) -> Result<(), String> + Sync + Send + 'static,
    {
        let mut inner = self.inner.lock().unwrap();

        if inner.state != PlaybackState::Stopped {
            return Err("Cannot set player while playback is in progress".to_string());
        }

        inner.player = Some(Arc::new(player));

        Ok(())
    }

    pub async fn play(
        &mut self,
        data: &Vec<TimeStampedMidiMessage>,
        track_info: TrackInfo,
    ) -> Result<(), String> {
        let buffer = self.load_timestamped_data(data)?;
        self._play(buffer, track_info).await
    }

    pub fn pause(&mut self) -> Result<(), String> {
        let mut inner = self.inner.lock().unwrap();

        inner.state = match inner.state {
            PlaybackState::Playing(ref track_info) => PlaybackState::Paused(track_info.clone()),
            _ => return Err("Playback is not active, cannot pause".to_string()),
        };

        if let Some(signal) = &inner.signal_pause {
            signal.store(true, Ordering::SeqCst);
        }

        Ok(())
    }

    pub fn resume(&mut self) -> Result<(), String> {
        let mut inner = self.inner.lock().unwrap();

        inner.state = match inner.state {
            PlaybackState::Paused(ref track_info) => PlaybackState::Playing(track_info.clone()),
            _ => return Err("Playback is not paused, cannot resume".to_string()),
        };

        if let Some(signal) = &inner.signal_pause {
            signal.store(false, Ordering::SeqCst);
        }

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), String> {
        let handle = {
            let mut inner = self.inner.lock().unwrap();

            match inner.thread_handle.take() {
                Some(handle) => {
                    if let Some(signal_stop) = inner.signal_stop.take() {
                        signal_stop.store(true, Ordering::SeqCst);
                    }

                    Some(handle)
                }
                _ => None,
            }
        };

        if let Some(handle) = handle {
            handle
                .await
                .map_err(|e| format!("Failed to join playback thread: {e}"))?;
        }

        Ok(())
    }

    fn load_timestamped_data(
        &mut self,
        data: &Vec<TimeStampedMidiMessage>,
    ) -> Result<Vec<(u64, Vec<u8>)>, String> {
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

        // Set duration
        {
            let mut inner = self.inner.lock().unwrap();
            inner.duration_milliseconds = Some(Arc::new(AtomicUsize::new(
                (normalized_data.last().unwrap().timestamp_microseconds / 1000) as usize,
            )));
        }

        // Convert absolute timestamps to relative timestamps (delta)
        let normalized_data = normalized_data
            .iter()
            .enumerate()
            .map(|(index, msg)| {
                if index > 0 {
                    (
                        msg.timestamp_microseconds
                            - normalized_data[index - 1].timestamp_microseconds,
                        msg.message.clone().into(),
                    )
                } else {
                    (msg.timestamp_microseconds, msg.message.clone().into())
                }
            })
            .collect();

        Ok(normalized_data)
    }

    async fn _play(
        &mut self,
        buffer: Vec<(u64, Vec<u8>)>,
        track_info: TrackInfo,
    ) -> Result<(), String> {
        self.stop().await?;

        let signal_stop = Arc::new(AtomicBool::new(false));
        let signal_pause = Arc::new(AtomicBool::new(false));

        let mut inner = self.inner.lock().unwrap();
        inner.state = PlaybackState::Playing(track_info);
        inner.position_milliseconds.store(0, Ordering::SeqCst);
        inner.signal_stop = Some(signal_stop.clone());
        inner.signal_pause = Some(signal_pause.clone());

        let position = inner.position_milliseconds.clone();
        let player = if let Some(player) = inner.player.as_ref() {
            player.clone()
        } else {
            return Err("No MIDI player set".to_string());
        };
        // FIXME: set priority to high for real-time playback
        // use accurate sleeping (e.g. https://crates.io/crates/spin_sleep)
        let playback_thread = thread::spawn(move || {
            let play_stop = || {
                // FIXME: Make channel configurable
                let msg_all_notes_off: Vec<u8> =
                    MidiMessage::all_notes_off(MidiChannel::Channel1).into();
                let msg_all_sound_off: Vec<u8> =
                    MidiMessage::all_sound_off(MidiChannel::Channel1).into();
                let _ = player(msg_all_notes_off.as_slice()).map_err(|e| e.to_string());
                let _ = player(&msg_all_sound_off.as_slice()).map_err(|e| e.to_string());
            };
            let mut start = Instant::now();
            let mut time = Duration::ZERO;
            for msg in buffer.iter() {
                time += Duration::from_micros(msg.0);
                let mut elapsed = start.elapsed();
                position.store(elapsed.as_millis() as usize, Ordering::SeqCst);
                while elapsed < time {
                    if signal_pause.load(Ordering::SeqCst) {
                        let elapsed = start.elapsed();
                        while signal_pause.load(Ordering::SeqCst) {
                            if signal_stop.load(Ordering::SeqCst) {
                                play_stop();
                                return;
                            }
                            thread::sleep(MAX_SLEEP_DURATION);
                        }
                        start = Instant::now() - elapsed;
                    }

                    if signal_stop.load(Ordering::SeqCst) {
                        play_stop();
                        break;
                    }

                    let duration = time - elapsed;
                    // sleep no more than 50ms to be able to handle pause/stop signals quickly
                    let sleep_duration = MAX_SLEEP_DURATION.min(duration);
                    thread::sleep(sleep_duration);
                    elapsed = start.elapsed();
                }

                if let Err(error) = player(msg.1.as_slice()) {
                    eprintln!("{error}");
                    break;
                }
            }
        });

        let inner_clone = self.inner.clone();
        let handle = tauri::async_runtime::spawn_blocking(move || {
            let result = playback_thread.join();

            if let Err(e) = result {
                eprintln!("{e:?}");
            }

            let mut inner_clone = inner_clone.lock().unwrap();
            inner_clone.state = PlaybackState::Stopped;
            inner_clone.position_milliseconds.store(0, Ordering::SeqCst);
            inner_clone.signal_pause = None;
            inner_clone.signal_stop = None;
            inner_clone.thread_handle = None;
        });
        inner.thread_handle = Some(handle);

        Ok(())
    }
}
