#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use crate::timer::TimerState;

mod database;
mod models;
mod timer;
mod commands;
mod stats;
mod sound;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let timer_state = TimerState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(timer_state)
        .setup(|app| {
            // Initialize database
            let app_data_dir = app.path().app_data_dir()
                .expect("Failed to get app data dir");
            database::init_db(app_data_dir)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::start_session,
            commands::pause_session,
            commands::resume_session,
            commands::stop_session,
            commands::get_sessions,
            commands::delete_session,
            commands::get_stats,
            commands::get_tag_stats,
            commands::export_data,
            commands::import_data,
            commands::play_completion_sound,
            commands::play_tick_sound,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
