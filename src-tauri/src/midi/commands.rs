use tauri::ipc::Channel;
use tauri_plugin_dialog::{DialogExt, FilePath};

use super::MidiState;
use crate::{
    frontend::{Midi, Playback, Recorder},
    midi::{
        message::MidiMessage,
        playback::TrackInfo,
        smf::{MidiFile, MidiHeader, MidiTrack},
    },
};

#[tauri::command]
pub async fn get_midi<'a>(state: tauri::State<'a, MidiState>) -> Result<Midi, String> {
    let midi = state.lock().await;
    Ok((&*midi).into())
}

#[tauri::command]
pub async fn scan_midi_input<'a>(state: tauri::State<'a, MidiState>) -> Result<Midi, String> {
    let mut midi = state.lock().await;
    midi.scan_input()?;
    Ok((&*midi).into())
}

#[tauri::command]
pub async fn scan_midi_output<'a>(state: tauri::State<'a, MidiState>) -> Result<Midi, String> {
    let mut midi = state.lock().await;
    midi.scan_output()?;
    Ok((&*midi).into())
}

#[tauri::command]
pub async fn connect_midi_input<'a>(
    input_id: String,
    state: tauri::State<'a, MidiState>,
) -> Result<Midi, String> {
    let mut midi = state.lock().await;
    let input_index = midi
        .available_input_ports
        .iter()
        .position(|port| port.id == input_id)
        .ok_or_else(|| format!("Input port with ID {} not found", input_id))?;
    midi.connect_input(input_index)?;
    Ok((&*midi).into())
}

#[tauri::command]
pub async fn connect_midi_output<'a>(
    output_id: String,
    state: tauri::State<'a, MidiState>,
) -> Result<Midi, String> {
    let mut midi = state.lock().await;
    let output_index = midi
        .available_output_ports
        .iter()
        .position(|port| port.id == output_id)
        .ok_or_else(|| format!("Output port with ID {} not found", output_id))?;
    midi.connect_output(output_index).await?;
    Ok((&*midi).into())
}

#[tauri::command]
pub async fn disconnect_midi_input<'a>(state: tauri::State<'a, MidiState>) -> Result<Midi, String> {
    let mut midi = state.lock().await;
    midi.disconnect_input();
    Ok((&*midi).into())
}

#[tauri::command]
pub async fn disconnect_midi_output<'a>(
    state: tauri::State<'a, MidiState>,
) -> Result<Midi, String> {
    let mut midi = state.lock().await;
    midi.disconnect_output();
    Ok((&*midi).into())
}

#[tauri::command]
pub async fn register_midi_channel<'a>(
    state: tauri::State<'a, MidiState>,
    channel: Channel<MidiMessage>,
) -> Result<(), String> {
    let mut midi = state.lock().await;
    midi.set_frontend_channel(channel);
    Ok(())
}

#[tauri::command]
pub async fn send_midi_message<'a>(
    state: tauri::State<'a, MidiState>,
    midi_message: MidiMessage,
) -> Result<(), String> {
    let midi = state.lock().await;
    midi.send_message(midi_message)?;
    Ok(())
}

#[tauri::command]
pub async fn get_midi_recorder<'a>(state: tauri::State<'a, MidiState>) -> Result<Recorder, String> {
    let midi = state.lock().await;
    let recorder = midi.recorder.lock().unwrap();
    Ok((&*recorder).into())
}

#[tauri::command]
pub async fn start_midi_recording<'a>(
    state: tauri::State<'a, MidiState>,
) -> Result<Recorder, String> {
    let midi = state.lock().await;

    let mut recorder = midi.recorder.lock().unwrap();
    recorder.start_recording()?;

    Ok((&*recorder).into())
}

#[tauri::command]
pub async fn stop_midi_recording<'a>(
    state: tauri::State<'a, MidiState>,
) -> Result<Recorder, String> {
    let midi = state.lock().await;

    let mut recorder = midi.recorder.lock().unwrap();
    recorder.stop_recording()?;

    Ok((&*recorder).into())
}

#[tauri::command]
pub async fn save_midi_recording<'a>(
    index: usize,
    app: tauri::AppHandle,
    state: tauri::State<'a, MidiState>,
) -> Result<(), String> {
    let file_path = app
        .dialog()
        .file()
        .add_filter("Standard MIDI Files", &["mid"])
        .blocking_save_file();

    if let Some(path) = file_path
        && let FilePath::Path(path_buf) = path
    {
        let recording = {
            let midi = state.lock().await;
            let recorder = midi.recorder.lock().unwrap();
            let recording = recorder
                .get_recordings()
                .get(index)
                .ok_or_else(|| format!("Recording with index {} not found", index))?;
            recording.clone()
        };
        let tempo = 500_000;
        let midi_header = MidiHeader::single_multi_channel_track();
        let track =
            MidiTrack::from_time_stamped_messages(recording, tempo, midi_header.get_division());

        let midi_file = MidiFile::new(midi_header, vec![track]);
        let midi_bytes: Vec<u8> = (&midi_file).try_into()?;

        std::fs::write(path_buf, midi_bytes)
            .map_err(|e| format!("Failed to write MIDI file: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn delete_midi_recording<'a>(
    index: usize,
    state: tauri::State<'a, MidiState>,
) -> Result<Recorder, String> {
    let midi = state.lock().await;

    let mut recorder = midi.recorder.lock().unwrap();
    recorder.remove_recording(index)?;

    Ok((&*recorder).into())
}

#[tauri::command]
pub async fn get_midi_playback<'a>(state: tauri::State<'a, MidiState>) -> Result<Playback, String> {
    let midi = state.lock().await;
    let playback = midi.playback.lock().await;

    Ok((&*playback).into())
}

#[tauri::command]
pub async fn play_midi_recording<'a>(
    index: usize,
    state: tauri::State<'a, MidiState>,
) -> Result<Playback, String> {
    let midi = state.lock().await;

    let recording = {
        let recorder = midi.recorder.lock().unwrap();
        let recording = recorder
            .get_recordings()
            .get(index)
            .ok_or_else(|| format!("Recording with index {} not found", index))?;
        recording.clone()
    };

    let mut playback = midi.playback.lock().await;
    playback
        .play(&recording, TrackInfo::Recording(index))
        .await?;

    Ok((&*playback).into())
}

#[tauri::command]
pub async fn load_midi_track<'a>(
    app: tauri::AppHandle,
    state: tauri::State<'a, MidiState>,
) -> Result<Playback, String> {
    let file_path = app
        .dialog()
        .file()
        .add_filter("Standard MIDI Files", &["mid"])
        .blocking_pick_file();

    let midi = state.lock().await;
    let mut playback = midi.playback.lock().await;

    if let Some(path) = file_path
        && let FilePath::Path(path_buf) = path
    {
        let result =
            std::fs::read(path_buf).map_err(|e| format!("Failed to read MIDI file: {}", e));
        let midi_bytes = result.unwrap();
        let midi_file: MidiFile = midi_bytes
            .as_slice()
            .try_into()
            .map_err(|e| format!("Failed to parse MIDI file: {}", e))?;
        playback.load_track(midi_file)?;
    }

    Ok((&*playback).into())
}

#[tauri::command]
pub async fn play_midi_track<'a>(
    index: usize,
    state: tauri::State<'a, MidiState>,
) -> Result<Playback, String> {
    let midi = state.lock().await;

    let mut playback = midi.playback.lock().await;
    playback.play_track(index).await?;

    Ok((&*playback).into())
}

#[tauri::command]
pub async fn eject_midi_track<'a>(
    index: usize,
    state: tauri::State<'a, MidiState>,
) -> Result<Playback, String> {
    let midi = state.lock().await;

    let mut playback = midi.playback.lock().await;
    playback.eject_track(index).await?;

    Ok((&*playback).into())
}

#[tauri::command]
pub async fn pause_midi_playback<'a>(
    state: tauri::State<'a, MidiState>,
) -> Result<Playback, String> {
    let midi = state.lock().await;
    let mut playback = midi.playback.lock().await;
    playback.pause()?;

    Ok((&*playback).into())
}

#[tauri::command]
pub async fn resume_midi_playback<'a>(
    state: tauri::State<'a, MidiState>,
) -> Result<Playback, String> {
    let midi = state.lock().await;
    let mut playback = midi.playback.lock().await;
    playback.resume()?;

    Ok((&*playback).into())
}

#[tauri::command]
pub async fn stop_midi_playback<'a>(
    state: tauri::State<'a, MidiState>,
) -> Result<Playback, String> {
    let midi = state.lock().await;
    let mut playback = midi.playback.lock().await;
    playback.stop().await?;

    Ok((&*playback).into())
}
