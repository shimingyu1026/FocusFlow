use rusqlite::{params, Connection, Result as SqliteResult};
use std::path::{PathBuf, Path};
use crate::models::FocusSession;

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

    let tags_json = serde_json::to_string(&session.tags)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    conn.execute(
        "INSERT INTO sessions (id, task, duration, start_time, end_time, completed, tags)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            &session.id,
            &session.task,
            session.duration,
            &session.start_time,
            &session.end_time,
            session.completed,
            &tags_json,
        ],
    )?;

    Ok(())
}

pub fn get_sessions(db_path: &Path, limit: Option<i32>) -> SqliteResult<Vec<FocusSession>> {
    let conn = Connection::open(db_path)?;

    let safe_limit = limit.filter(|value| *value > 0);
    let query = if safe_limit.is_some() {
        "SELECT * FROM sessions ORDER BY start_time DESC LIMIT ?1"
    } else {
        "SELECT * FROM sessions ORDER BY start_time DESC"
    };

    let mut stmt = conn.prepare(&query)?;
    let map_row = |row: &rusqlite::Row<'_>| {
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
    };

    let rows = if let Some(limit) = safe_limit {
        stmt.query_map([limit], map_row)?
    } else {
        stmt.query_map([], map_row)?
    };

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

pub fn clear_all_sessions(db_path: &Path) -> SqliteResult<usize> {
    let conn = Connection::open(db_path)?;
    let deleted = conn.execute("DELETE FROM sessions", [])?;
    Ok(deleted)
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

    let mut conn = Connection::open(db_path)?;
    let tx = conn.transaction()?;
    let mut count = 0;

    // Replace existing data with imported snapshot
    tx.execute("DELETE FROM sessions", [])?;

    for session in sessions {
        let tags_json = serde_json::to_string(&session.tags)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        tx.execute(
            "INSERT OR REPLACE INTO sessions (id, task, duration, start_time, end_time, completed, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                &session.id,
                &session.task,
                session.duration,
                &session.start_time,
                &session.end_time,
                session.completed,
                &tags_json,
            ],
        )?;
        count += 1;
    }

    tx.commit()?;

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_app_dir(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("focusflow-{test_name}-{unique}"))
    }

    fn sample_session(id: &str, completed: bool, duration: i32) -> FocusSession {
        FocusSession {
            id: id.to_string(),
            task: "Write tests".to_string(),
            duration,
            start_time: "2026-05-21T01:00:00Z".to_string(),
            end_time: "2026-05-21T01:25:00Z".to_string(),
            completed,
            tags: vec!["work".to_string(), "deep".to_string()],
        }
    }

    #[test]
    fn save_and_load_preserves_typed_fields() {
        let app_dir = temp_app_dir("typed-fields");
        init_db(app_dir.clone()).expect("database should initialize");
        let db_path = get_db_path(app_dir.clone());
        let session = sample_session("session-1", true, 25);

        save_session(&session, &db_path).expect("session should save");
        let loaded = get_sessions(&db_path, None).expect("sessions should load");

        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].duration, 25);
        assert!(loaded[0].completed);
        assert_eq!(loaded[0].tags, vec!["work", "deep"]);

        let conn = Connection::open(&db_path).expect("database should open");
        let completed_type: String = conn
            .query_row("SELECT typeof(completed) FROM sessions", [], |row| row.get(0))
            .expect("completed type should exist");
        assert_eq!(completed_type, "integer");

        fs::remove_dir_all(app_dir).ok();
    }

    #[test]
    fn import_replaces_existing_sessions() {
        let app_dir = temp_app_dir("import-replace");
        init_db(app_dir.clone()).expect("database should initialize");
        let db_path = get_db_path(app_dir.clone());

        save_session(&sample_session("old", false, 5), &db_path).expect("old session should save");
        let imported = vec![
            sample_session("new-1", true, 15),
            sample_session("new-2", false, 3),
        ];
        let json = serde_json::to_string(&imported).expect("sessions should serialize");

        let count = import_data(&db_path, &json).expect("import should succeed");
        let loaded = get_sessions(&db_path, None).expect("sessions should load");

        assert_eq!(count, 2);
        assert_eq!(loaded.len(), 2);
        assert!(loaded.iter().all(|session| session.id != "old"));

        fs::remove_dir_all(app_dir).ok();
    }

    #[test]
    fn positive_limit_is_applied() {
        let app_dir = temp_app_dir("limit");
        init_db(app_dir.clone()).expect("database should initialize");
        let db_path = get_db_path(app_dir.clone());

        save_session(&sample_session("one", true, 25), &db_path).expect("first session should save");
        save_session(&sample_session("two", true, 25), &db_path).expect("second session should save");

        assert_eq!(get_sessions(&db_path, Some(1)).expect("limited sessions").len(), 1);
        assert_eq!(get_sessions(&db_path, Some(0)).expect("unlimited sessions").len(), 2);

        fs::remove_dir_all(app_dir).ok();
    }

    #[test]
    fn session_json_uses_frontend_field_names() {
        let session = sample_session("json-contract", true, 25);
        let json = serde_json::to_value(&session).expect("session should serialize");

        assert!(json.get("startTime").is_some());
        assert!(json.get("endTime").is_some());
        assert!(json.get("start_time").is_none());
        assert!(json.get("end_time").is_none());

        let legacy_json = serde_json::json!({
            "id": "legacy",
            "task": "Legacy export",
            "duration": 25,
            "start_time": "2026-05-21T01:00:00Z",
            "end_time": "2026-05-21T01:25:00Z",
            "completed": true,
            "tags": ["work"]
        });
        let legacy_session: FocusSession =
            serde_json::from_value(legacy_json).expect("legacy snake_case export should import");
        assert_eq!(legacy_session.start_time, "2026-05-21T01:00:00Z");
    }
}
