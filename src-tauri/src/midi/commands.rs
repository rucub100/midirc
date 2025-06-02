use crate::midi::MidiStateInner;

use super::MidiState;

#[tauri::command]
pub async fn get_midi<'a>(state: tauri::State<'a, MidiState>) -> Result<MidiStateInner, String> {
    let midi = state.lock().unwrap();
    Ok(midi.clone())
}

#[tauri::command]
pub async fn scan_midi_input<'a>(
    state: tauri::State<'a, MidiState>,
) -> Result<MidiStateInner, String> {
    let mut midi = state.lock().unwrap();
    midi.scan_input()?;
    Ok(midi.clone())
}

#[tauri::command]
pub async fn scan_midi_output<'a>(
    state: tauri::State<'a, MidiState>,
) -> Result<MidiStateInner, String> {
    let mut midi = state.lock().unwrap();
    midi.scan_output()?;
    Ok(midi.clone())
}

#[tauri::command]
pub async fn connect_midi_input<'a>(
    input_id: String,
    state: tauri::State<'a, MidiState>,
) -> Result<MidiStateInner, String> {
    let mut midi = state.lock().unwrap();
    let input_index = midi
        .available_input_ports
        .iter()
        .position(|port| port.id == input_id)
        .ok_or_else(|| format!("Input port with ID {} not found", input_id))?;
    midi.connect_input(input_index)?;
    Ok(midi.clone())
}

#[tauri::command]
pub async fn connect_midi_output<'a>(
    output_id: String,
    state: tauri::State<'a, MidiState>,
) -> Result<MidiStateInner, String> {
    let mut midi = state.lock().unwrap();
    let output_index = midi
        .available_output_ports
        .iter()
        .position(|port| port.id == output_id)
        .ok_or_else(|| format!("Output port with ID {} not found", output_id))?;
    midi.connect_output(output_index)?;
    Ok(midi.clone())
}

#[tauri::command]
pub async fn disconnect_midi_input<'a>(
    state: tauri::State<'a, MidiState>,
) -> Result<MidiStateInner, String> {
    let mut midi = state.lock().unwrap();
    midi.disconnect_input();
    Ok(midi.clone())
}

#[tauri::command]
pub async fn disconnect_midi_output<'a>(
    state: tauri::State<'a, MidiState>,
) -> Result<MidiStateInner, String> {
    let mut midi = state.lock().unwrap();
    midi.disconnect_output();
    Ok(midi.clone())
}
