use std::sync::Mutex;
use tauri::State;
use chrono::Utc;

pub struct TimerState {
    pub is_running: Mutex<bool>,
    pub remaining_seconds: Mutex<i32>,
    pub current_task: Mutex<Option<String>>,
    pub start_time: Mutex<Option<String>>,
}

impl TimerState {
    pub fn new() -> Self {
        Self {
            is_running: Mutex::new(false),
            remaining_seconds: Mutex::new(0),
            current_task: Mutex::new(None),
            start_time: Mutex::new(None),
        }
    }
}

pub fn start_timer(state: &TimerState, duration_minutes: i32, task: String) {
    let mut is_running = state.is_running.lock().unwrap();
    let mut remaining = state.remaining_seconds.lock().unwrap();
    let mut current_task = state.current_task.lock().unwrap();
    let mut start_time = state.start_time.lock().unwrap();

    *is_running = true;
    *remaining = duration_minutes * 60;
    *current_task = Some(task);
    *start_time = Some(Utc::now().to_rfc3339());
}

pub fn pause_timer(state: &TimerState) {
    let mut is_running = state.is_running.lock().unwrap();
    *is_running = false;
}

pub fn resume_timer(state: &TimerState) {
    let mut is_running = state.is_running.lock().unwrap();
    *is_running = true;
}

pub fn stop_timer(state: &TimerState) {
    let mut is_running = state.is_running.lock().unwrap();
    let mut remaining = state.remaining_seconds.lock().unwrap();
    let mut current_task = state.current_task.lock().unwrap();
    let mut start_time = state.start_time.lock().unwrap();

    *is_running = false;
    *remaining = 0;
    *current_task = None;
    *start_time = None;
}
