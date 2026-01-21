use crate::models::FocusSession;
use chrono::{Utc, Duration};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyStats {
    pub date: String,
    pub total_minutes: i32,
    pub count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagStats {
    pub tag: String,
    pub total_minutes: i32,
    pub percentage: f64,
}

pub fn calculate_daily_stats(sessions: &[FocusSession], days: i32) -> Vec<DailyStats> {
    let mut stats: std::collections::HashMap<String, (i32, i32)> = std::collections::HashMap::new();
    let now = Utc::now();

    for i in 0..days {
        let date = now - Duration::days(i as i64);
        let date_key = date.format("%Y-%m-%d").to_string();
        stats.insert(date_key, (0, 0));
    }

    for session in sessions {
        if let Ok(start_time) = session.start_time.parse::<chrono::DateTime<Utc>>() {
            let date_key = start_time.format("%Y-%m-%d").to_string();
            if let Some((total, count)) = stats.get_mut(&date_key) {
                *total += session.duration;
                *count += 1;
            }
        }
    }

    let mut result: Vec<DailyStats> = stats.into_iter()
        .map(|(date, (total, count))| DailyStats {
            date,
            total_minutes: total,
            count,
        })
        .collect();

    result.sort_by(|a, b| a.date.cmp(&b.date));
    result
}

pub fn calculate_tag_stats(sessions: &[FocusSession]) -> Vec<TagStats> {
    let mut tag_totals: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
    let mut total_all: i32 = 0;

    for session in sessions {
        for tag in &session.tags {
            *tag_totals.entry(tag.clone()).or_insert(0) += session.duration;
            total_all += session.duration;
        }
    }

    tag_totals.into_iter()
        .map(|(tag, total)| TagStats {
            tag,
            total_minutes: total,
            percentage: if total_all > 0 {
                (total as f64 / total_all as f64) * 100.0
            } else {
                0.0
            },
        })
        .collect()
}
