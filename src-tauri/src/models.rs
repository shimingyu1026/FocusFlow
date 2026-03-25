use serde::{Deserialize, Serialize};

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
