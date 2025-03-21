use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};

use super::get_db;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Class {
    id: i32,
    name: String,
}

impl Class {
    fn new(id: i32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }

    pub fn init() -> Result<()> {
        let conn = get_db()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS classes (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                UNIQUE(name)
            )",
            [],
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
    fn new(id: i32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }

    pub fn init() -> Result<()> {
        let conn = get_db()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS subjects (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                UNIQUE(name)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn push(name: &str) -> Result<Self> {}

}
