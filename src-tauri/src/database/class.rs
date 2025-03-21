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

    pub fn push(name: &str) -> Result<Self> {
        let conn = get_db()?;
        conn.execute("INSERT INTO classes (name) VALUES (?1)", params![name])?;
        Ok(Self::new(conn.last_insert_rowid() as i32, name))
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

    pub fn push(name: &str) -> Result<Self> {
        let conn = get_db()?;
        conn.execute("INSERT INTO subjects (name) VALUES (?1)", params![name])?;
        Ok(Self::new(conn.last_insert_rowid() as i32, name))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubjectClass {
    id: i32,
    subject_id: i32,
    class_id: i32,
}

impl SubjectClass {
    fn new(id: i32, subject_id: i32, class_id: i32) -> Self {
        Self {
            id,
            subject_id,
            class_id,
        }
    }

    pub fn init() -> Result<()> {
        let conn = get_db()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS subject_class (
                subject_id INTEGER NOT NULL,
                guardian_id INTEGER NOT NULL,
                PRIMARY KEY (subject_id, class_id),
                FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE,
                FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE
            )",
            [],
        )?;
        Ok(())
    }

    pub fn push(subject_id: i32, class_id: i32) -> Result<Self> {
        let conn = get_db()?;
        conn.execute(
            "INSERT INTO subject_class (subject_id, class_id) VALUES (?1, ?2)",
            params![subject_id, class_id],
        )?;
        Ok(Self::new(
            conn.last_insert_rowid() as i32,
            subject_id,
            class_id,
        ))
    }
}
