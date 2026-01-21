#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod database;
mod models;
mod timer;
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Initialize database
            database::init_db(app.path().app_data_dir())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::start_session,
            commands::pause_session,
            commands::resume_session,
            commands::stop_session,
            commands::get_sessions,
            commands::get_stats,
            commands::export_data,
            commands::import_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
