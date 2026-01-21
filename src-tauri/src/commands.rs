use crate::timer::{TimerState, start_timer, pause_timer, resume_timer, stop_timer};
use crate::models::{FocusSession, StatsData};
use crate::database::{self, get_db_path};
use crate::stats::{self, DailyStats, TagStats};
use crate::sound;
use tauri::{State, Manager};
use chrono::Utc;
use uuid::Uuid;

#[tauri::command]
pub async fn start_session(
    state: State<'_, TimerState>,
    duration: i32,
    task: String,
    _app_handle: tauri::AppHandle,
) -> Result<(), String> {
    start_timer(&state, duration, task);
    Ok(())
}

#[tauri::command]
pub async fn pause_session(state: State<'_, TimerState>) -> Result<(), String> {
    pause_timer(&state);
    Ok(())
}

#[tauri::command]
pub async fn resume_session(state: State<'_, TimerState>) -> Result<(), String> {
    resume_timer(&state);
    Ok(())
}

#[tauri::command]
pub async fn stop_session(
    state: State<'_, TimerState>,
    completed: bool,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let task = state.current_task.lock().unwrap().clone().unwrap_or_default();
    let start_time = state.start_time.lock().unwrap().clone().unwrap_or_default();
    let remaining = *state.remaining_seconds.lock().unwrap();

    let session = FocusSession {
        id: Uuid::new_v4().to_string(),
        task,
        duration: (remaining / 60).max(1),
        start_time,
        end_time: Utc::now().to_rfc3339(),
        completed,
        tags: vec![],
    };

    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = database::get_db_path(app_data_dir);

    database::save_session(&session, &db_path)
        .map_err(|e| e.to_string())?;

    stop_timer(&state);
    Ok(())
}

#[tauri::command]
pub async fn get_sessions(
    limit: Option<i32>,
    app_handle: tauri::AppHandle,
) -> Result<Vec<FocusSession>, String> {
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = database::get_db_path(app_data_dir);

    database::get_sessions(&db_path, limit)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_session(
    id: String,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = database::get_db_path(app_data_dir);

    database::delete_session(&db_path, &id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn export_data(
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = database::get_db_path(app_data_dir);

    database::export_data(&db_path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn import_data(
    json_data: String,
    app_handle: tauri::AppHandle,
) -> Result<usize, String> {
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = database::get_db_path(app_data_dir);

    database::import_data(&db_path, &json_data)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_stats(
    app_handle: tauri::AppHandle,
) -> Result<Vec<DailyStats>, String> {
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = database::get_db_path(app_data_dir);

    let sessions = database::get_sessions(&db_path, None)
        .map_err(|e| e.to_string())?;

    Ok(stats::calculate_daily_stats(&sessions, 30))
}

#[tauri::command]
pub async fn get_tag_stats(
    app_handle: tauri::AppHandle,
) -> Result<Vec<TagStats>, String> {
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = database::get_db_path(app_data_dir);

    let sessions = database::get_sessions(&db_path, None)
        .map_err(|e| e.to_string())?;

    Ok(stats::calculate_tag_stats(&sessions))
}

#[tauri::command]
pub async fn play_completion_sound() -> Result<(), String> {
    sound::play_completion_sound()
}

#[tauri::command]
pub async fn play_tick_sound() -> Result<(), String> {
    sound::play_tick_sound()
}
