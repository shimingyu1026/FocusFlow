use crate::timer::{TimerState, start_timer, pause_timer, resume_timer, stop_timer};
use crate::models::FocusSession;
use crate::database;
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
    tags: Vec<String>,
    _app_handle: tauri::AppHandle,
) -> Result<(), String> {
    if !(1..=480).contains(&duration) {
        return Err("专注时长必须在 1 到 480 分钟之间".to_string());
    }

    if state.start_time.lock().map_err(|e| e.to_string())?.is_some() {
        return Err("当前已有一轮专注正在进行".to_string());
    }

    let normalized_task = task.trim().to_string();
    let normalized_tags = tags
        .into_iter()
        .map(|tag| tag.trim().to_string())
        .filter(|tag| !tag.is_empty())
        .collect();

    start_timer(&state, duration, normalized_task, normalized_tags);
    Ok(())
}

#[tauri::command]
pub async fn pause_session(state: State<'_, TimerState>) -> Result<(), String> {
    if state.start_time.lock().map_err(|e| e.to_string())?.is_none() {
        return Err("没有正在进行的专注记录".to_string());
    }

    if !*state.is_running.lock().map_err(|e| e.to_string())? {
        return Err("当前专注已经暂停".to_string());
    }

    pause_timer(&state);
    Ok(())
}

#[tauri::command]
pub async fn resume_session(state: State<'_, TimerState>) -> Result<(), String> {
    if state.start_time.lock().map_err(|e| e.to_string())?.is_none() {
        return Err("没有可以继续的专注记录".to_string());
    }

    if *state.is_running.lock().map_err(|e| e.to_string())? {
        return Err("当前专注已经在运行".to_string());
    }

    resume_timer(&state);
    Ok(())
}

#[tauri::command]
pub async fn stop_session(
    state: State<'_, TimerState>,
    completed: bool,
    elapsed_seconds: i32,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let start_time = state
        .start_time
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or_else(|| "没有正在进行的专注记录".to_string())?;
    let task = state
        .current_task
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .unwrap_or_default();
    let tags = state.current_tags.lock().map_err(|e| e.to_string())?.clone();
    let safe_elapsed = elapsed_seconds.max(0);
    let duration_minutes = if safe_elapsed == 0 {
        0
    } else {
        (safe_elapsed + 59) / 60
    };

    let session = FocusSession {
        id: Uuid::new_v4().to_string(),
        task,
        duration: duration_minutes,
        start_time,
        end_time: Utc::now().to_rfc3339(),
        completed,
        tags,
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
pub async fn clear_all_data(
    app_handle: tauri::AppHandle,
) -> Result<usize, String> {
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = database::get_db_path(app_data_dir);

    database::clear_all_sessions(&db_path)
        .map_err(|e| e.to_string())
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
