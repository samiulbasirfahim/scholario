use super::conn;
use chrono::NaiveDate;
use rusqlite::{params, Result, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    pub id: i32,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl Session {
    pub fn new(id: i32, name: &str, start_date: NaiveDate, end_date: NaiveDate) -> Self {
        Self {
            id,
            name: name.to_string(),
            start_date,
            end_date,
        }
    }

    pub fn init() -> Result<()> {
        let db = conn()?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                start_date TEXT NOT NULL,
                end_date TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create(name: &str, start_date: NaiveDate, end_date: NaiveDate) -> Result<Self> {
        let db = conn()?;
        db.execute(
            "INSERT INTO sessions (name, start_date, end_date) VALUES (?1, ?2, ?3)",
            params![name, start_date, end_date],
        )?;

        let id = db.last_insert_rowid() as i32;

        Ok(Self {
            id,
            name: name.to_string(),
            start_date,
            end_date,
        })
    }

    pub fn get_all() -> Result<Vec<Self>> {
        let db = conn()?;
        let mut stmt = db.prepare("SELECT id, name, start_date, end_date FROM sessions")?;

        let session_iter = stmt.query_map([], |row: &Row| {
            Ok(Session {
                id: row.get(0)?,
                name: row.get(1)?,
                start_date: row.get::<_, String>(2)?.parse().unwrap(),
                end_date: row.get::<_, String>(3)?.parse().unwrap(),
            })
        })?;

        session_iter.collect()
    }

    pub fn get_by_id(id: i32) -> Result<Option<Self>> {
        let db = conn()?;
        let mut stmt =
            db.prepare("SELECT id, name, start_date, end_date FROM sessions WHERE id = ?1")?;

        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(Session {
                id: row.get(0)?,
                name: row.get(1)?,
                start_date: row.get::<_, String>(2)?.parse().unwrap(),
                end_date: row.get::<_, String>(3)?.parse().unwrap(),
            }))
        } else {
            Ok(None)
        }
    }

    pub fn delete(id: i32) -> Result<()> {
        let db = conn()?;
        let rows_affected = db.execute("DELETE FROM sessions WHERE id = ?1", params![id])?;

        if rows_affected == 0 {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error {
                    code: rusqlite::ffi::ErrorCode::PermissionDenied,
                    extended_code: 0,
                },
                Some("No session deleted".to_string()),
            ));
        }

        Ok(())
    }

    pub fn edit(id: i32, name: &str, start_date: NaiveDate, end_date: NaiveDate) -> Result<Self> {
        let db = conn()?;

        let rows_affected = db.execute(
            "UPDATE sessions SET name = ?1, start_date = ?2, end_date = ?3 WHERE id = ?4",
            params![name, start_date, end_date, id],
        )?;

        if rows_affected == 0 {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error {
                    code: rusqlite::ffi::ErrorCode::OperationAborted,
                    extended_code: 0,
                },
                Some("No session found with the given ID".to_string()),
            ));
        }

        Ok(Self {
            id,
            name: name.to_string(),
            start_date,
            end_date,
        })
    }
}
