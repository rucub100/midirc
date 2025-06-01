use tauri::Manager;

use midi::{
    commands::{connect_midi, disconnect_midi, scan_midi},
    MidiState,
};

mod midi;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(MidiState::default())
        .invoke_handler(tauri::generate_handler![
            scan_midi,
            connect_midi,
            disconnect_midi
        ])
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let main_window = app.get_webview_window("main");
                if let Some(main_window) = main_window {
                    main_window.open_devtools();
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
