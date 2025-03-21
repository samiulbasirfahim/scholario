use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};

use super::get_db;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Student {
    id: i32,
    name: String,
}

impl Student {
    pub fn new(id: i32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }
    pub fn init() -> Result<()> {
        let conn = get_db()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS students (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn push(name: &str) -> Result<Self> {
        let conn = get_db()?;
        conn.execute("INSERT INTO students (name) VALUES (?1)", params![name])?;
        let id = conn.last_insert_rowid() as i32;
        Ok(Student {
            id,
            name: name.to_string(),
        })
    }
}
