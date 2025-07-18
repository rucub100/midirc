use std::{
    ops::{Deref, DerefMut},
    slice::Iter,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    thread::{self},
    time::{Duration, Instant},
};

use tauri::async_runtime::JoinHandle;

use crate::midi::{
    message::{MidiChannel, MidiMessage, TimeStampedMidiMessage},
    smf::{Event, MidiFile, calc_delta_time_microseconds},
};

type MidiPlayerFn = Arc<dyn Fn(&[u8]) -> Result<(), String> + Sync + Send + 'static>;

const MAX_SLEEP_DURATION: Duration = Duration::from_millis(50);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Track(Vec<(u64, Vec<u8>)>);

impl Deref for Track {
    type Target = Vec<(u64, Vec<u8>)>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Track {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for Track {
    type Item = (u64, Vec<u8>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Track {
    type Item = &'a (u64, Vec<u8>);
    type IntoIter = std::slice::Iter<'a, (u64, Vec<u8>)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Track {
    type Item = &'a mut (u64, Vec<u8>);
    type IntoIter = std::slice::IterMut<'a, (u64, Vec<u8>)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TrackInfo {
    Recording(usize),
    StandardMidiFile(usize),
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
    tracks: Vec<Track>,
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
                tracks: Vec::new(),
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

    pub fn get_tracks(&self) -> Vec<Track> {
        let inner = self.inner.lock().unwrap();
        inner.tracks.clone()
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

    pub fn load_track(&mut self, file: MidiFile) -> Result<(), String> {
        let mut inner = self.inner.lock().unwrap();

        if file.get_tracks().is_empty() {
            return Err("Cannot load empty MIDI file".to_string());
        }

        for track in file.get_tracks() {
            inner.tracks.push(Track(
                track
                    .iter()
                    .filter_map(|msg| match msg.event {
                        Event::MidiEvent(ref midi_message) => {
                            // FIXME: handle tempo changes and other meta events
                            Some((
                                calc_delta_time_microseconds(
                                    msg.delta_time,
                                    500_000,
                                    file.get_header().get_division(),
                                ),
                                midi_message.clone().into(),
                            ))
                        }
                        _ => None,
                    })
                    .collect(),
            ));
        }

        Ok(())
    }

    pub async fn eject_track(&mut self, index: usize) -> Result<(), String> {
        let mut inner = self.inner.lock().unwrap();
        if index < inner.tracks.len() {
            inner.tracks.remove(index);
            // FIXME: reset state?
            Ok(())
        } else {
            Err("Track index out of bounds".to_string())
        }
    }

    pub async fn play_track(&mut self, index: usize) -> Result<(), String> {
        let buffer = self._load_track(index)?;
        self._play(buffer, TrackInfo::StandardMidiFile(index)).await
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

    fn _load_track(&mut self, index: usize) -> Result<Track, String> {
        let mut inner = self.inner.lock().unwrap();
        if index < inner.tracks.len() {
            let track = inner.tracks[index].clone();
            inner.duration_milliseconds = Some(Arc::new(AtomicUsize::new(
                (track.iter().map(|(delta, _)| delta).sum::<u64>() / 1000) as usize,
            )));
            Ok(track)
        } else {
            Err("Track index out of bounds".to_string())
        }
    }

    fn load_timestamped_data(
        &mut self,
        data: &Vec<TimeStampedMidiMessage>,
    ) -> Result<Track, String> {
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

        Ok(Track(normalized_data))
    }

    async fn _play(&mut self, buffer: Track, track_info: TrackInfo) -> Result<(), String> {
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
