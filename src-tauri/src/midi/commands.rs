use tauri::ipc::Channel;

use super::MidiState;
use crate::{
    frontend::{Midi, Recorder},
    midi::{message::MidiMessage, playback::TrackInfo},
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
pub async fn play_midi_recording<'a>(
    index: usize,
    state: tauri::State<'a, MidiState>,
) -> Result<(), String> {
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

    // FIXME: create frontend DTO for playback
    Ok(())
}
