use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusSession {
    pub id: String,
    pub task: String,
    pub duration: i32,        // 分钟
    #[serde(alias = "start_time")]
    pub start_time: String,   // ISO 8601
    #[serde(alias = "end_time")]
    pub end_time: String,     // ISO 8601
    pub completed: bool,
    pub tags: Vec<String>,
}
