use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};

use super::get_db;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Class {
    id: i32,
    name: String,
    monthly_fee: i32,
    teacher: i32,
}

impl Class {
    pub fn new(id: i32, name: &str, monthly_fee: i32, teacher: i32) -> Self {
        Self {
            id,
            name: name.to_string(),
            monthly_fee,
            teacher,
        }
    }

    pub fn init() -> Result<()> {
        let conn = get_db()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS classes(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                monthly_fee INT NOT NULL,
                teacher INTEGER,
                FOREIGN KEY (teacher) REFERENCES teachers(id)
            )",
            params![],
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Subject {
    id: i32,
    name: String,
}

impl Subject {
    pub fn new(id: i32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }

    pub fn init() -> Result<()> {
        let conn = super::get_db()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS subjects (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn push(name: &str) -> Result<Self> {
        let conn = super::get_db()?;
        conn.execute("INSERT INTO subjects (name) VALUES (?1)", params![name])?;
        let id = conn.last_insert_rowid() as i32;
        Ok(Subject::new(id, name))
    }
}
