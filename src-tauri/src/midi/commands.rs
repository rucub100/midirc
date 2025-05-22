use crate::midi::MidiStateInner;

use super::MidiState;

#[tauri::command]
pub async fn scan_midi<'a>(state: tauri::State<'a, MidiState>) -> Result<MidiStateInner, String> {
    let mut midi = state.lock().unwrap();

    midi.scan_input()?;
    midi.scan_output()?;

    Ok(midi.clone())
}
