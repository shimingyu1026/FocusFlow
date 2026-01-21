use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct FocusSession {
    pub id: String,
    pub task: String,
    pub duration: i32,        // 分钟
    pub start_time: String,   // ISO 8601
    pub end_time: String,     // ISO 8601
    pub completed: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsData {
    pub today_total: i32,
    pub today_count: i32,
    pub week_total: i32,
    pub week_avg: i32,
    pub month_total: i32,
    pub month_count: i32,
}
