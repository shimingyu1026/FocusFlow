use rusqlite::{Connection, Result as SqliteResult};
use std::path::{PathBuf, Path};
use crate::models::FocusSession;
use chrono::Utc;

const DB_NAME: &str = "focusflow.db";

pub fn get_db_path(app_data_dir: PathBuf) -> PathBuf {
    app_data_dir.join(DB_NAME)
}

pub fn init_db(app_data_dir: PathBuf) -> SqliteResult<()> {
    let db_path = get_db_path(app_data_dir);

    // Ensure directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            task TEXT NOT NULL,
            duration INTEGER NOT NULL,
            start_time TEXT NOT NULL,
            end_time TEXT NOT NULL,
            completed BOOLEAN NOT NULL,
            tags TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}

pub fn save_session(session: &FocusSession, db_path: &Path) -> SqliteResult<()> {
    let conn = Connection::open(db_path)?;

    let tags_json = serde_json::to_string(&session.tags).unwrap();

    conn.execute(
        "INSERT INTO sessions (id, task, duration, start_time, end_time, completed, tags)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        [
            &session.id,
            &session.task,
            &session.duration.to_string(),
            &session.start_time,
            &session.end_time,
            &session.completed.to_string(),
            &tags_json,
        ],
    )?;

    Ok(())
}

pub fn get_sessions(db_path: &Path, limit: Option<i32>) -> SqliteResult<Vec<FocusSession>> {
    let conn = Connection::open(db_path)?;

    let query = if let Some(limit) = limit {
        format!("SELECT * FROM sessions ORDER BY start_time DESC LIMIT {}", limit)
    } else {
        "SELECT * FROM sessions ORDER BY start_time DESC".to_string()
    };

    let mut stmt = conn.prepare(&query)?;
    let rows = stmt.query_map([], |row| {
        let tags_str: String = row.get(6)?;
        let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();

        Ok(FocusSession {
            id: row.get(0)?,
            task: row.get(1)?,
            duration: row.get(2)?,
            start_time: row.get(3)?,
            end_time: row.get(4)?,
            completed: row.get(5)?,
            tags,
        })
    })?;

    let mut sessions = Vec::new();
    for row in rows {
        sessions.push(row?);
    }

    Ok(sessions)
}

pub fn delete_session(db_path: &Path, id: &str) -> SqliteResult<()> {
    let conn = Connection::open(db_path)?;
    conn.execute("DELETE FROM sessions WHERE id = ?1", [id])?;
    Ok(())
}

pub fn export_data(db_path: &Path) -> SqliteResult<String> {
    let sessions = get_sessions(db_path, None)?;
    let json = serde_json::to_string(&sessions).map_err(|e| {
        rusqlite::Error::ToSqlConversionFailure(Box::new(e))
    })?;
    Ok(json)
}

pub fn import_data(db_path: &Path, json_data: &str) -> SqliteResult<usize> {
    let sessions: Vec<FocusSession> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    let conn = Connection::open(db_path)?;
    let mut count = 0;

    for session in sessions {
        let tags_json = serde_json::to_string(&session.tags).unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO sessions (id, task, duration, start_time, end_time, completed, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            [
                &session.id,
                &session.task,
                &session.duration.to_string(),
                &session.start_time,
                &session.end_time,
                &session.completed.to_string(),
                &tags_json,
            ],
        )?;
        count += 1;
    }

    Ok(count)
}
