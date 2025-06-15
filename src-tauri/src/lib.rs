use tauri::Manager;

use midi::{
    commands::{
        connect_midi_input, connect_midi_output, disconnect_midi_input, disconnect_midi_output,
        get_midi, scan_midi_input, scan_midi_output,
    },
    MidiState,
};

use crate::midi::commands::{register_midi_channel, send_midi_message};

mod frontend;
mod midi;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(MidiState::default())
        .invoke_handler(tauri::generate_handler![
            get_midi,
            scan_midi_input,
            scan_midi_output,
            connect_midi_input,
            connect_midi_output,
            disconnect_midi_input,
            disconnect_midi_output,
            register_midi_channel,
            send_midi_message
        ])
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let main_window = app.get_webview_window("main");
                if let Some(main_window) = main_window {
                    main_window.open_devtools();
                }
            }
            // initialize MIDI state
            {
                let midi = app.state::<MidiState>();
                let mut midi = midi.lock().unwrap();
                let _ = midi.scan_input();
                let _ = midi.scan_output();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
