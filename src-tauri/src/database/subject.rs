use super::conn;
use rusqlite::{params, Result, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Subject {
    pub id: i32,
    pub name: String,
    pub code: i32,
}

impl Subject {
    pub fn new(id: i32, name: &str, code: i32) -> Self {
        Self {
            id,
            name: name.to_string(),
            code,
        }
    }

    pub fn init() -> Result<()> {
        let db = conn()?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS subjects (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                code INTEGER NOT NULL UNIQUE
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create(name: &str, code: i32) -> Result<Self> {
        let db = conn()?;
        db.execute(
            "INSERT INTO subjects (name, code) VALUES (?1, ?2)",
            params![name, code],
        )?;

        let id = db.last_insert_rowid() as i32;
        Ok(Self::new(id, name, code))
    }

    pub fn get() -> Result<Vec<Self>> {
        let db = conn()?;
        let mut stmt = db.prepare("SELECT id, name, code FROM subjects")?;
        let iter = stmt.query_map([], |row| {
            Ok(Subject {
                id: row.get(0)?,
                name: row.get(1)?,
                code: row.get(2)?,
            })
        })?;

        iter.collect()
    }

    pub fn delete(id: i32) -> Result<()> {
        let db = conn()?;
        let affected = db.execute("DELETE FROM subjects WHERE id = ?", params![id])?;

        if affected == 0 {
            Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error {
                    code: rusqlite::ffi::ErrorCode::Unknown,
                    extended_code: 0,
                },
                Some("No subject deleted. Invalid ID.".to_string()),
            ))
        } else {
            Ok(())
        }
    }

    pub fn edit(id: i32, name: &str, code: i32) -> Result<Self> {
        let db = conn()?;
        let affected = db.execute(
            "UPDATE subjects SET name = ?1, code = ?2 WHERE id = ?3",
            params![name, code, id],
        )?;

        if affected == 0 {
            Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error {
                    code: rusqlite::ffi::ErrorCode::OperationAborted,
                    extended_code: 0,
                },
                Some("No subject found to update.".to_string()),
            ))
        } else {
            Ok(Self::new(id, name, code))
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClassSubject {
    pub id: i32,
    pub class_id: i32,
    pub subject_id: i32,
    pub is_mandatory: bool,
}

impl ClassSubject {
    pub fn new(id: i32, class_id: i32, subject_id: i32, is_mandatory: bool) -> Self {
        Self {
            id,
            class_id,
            subject_id,
            is_mandatory,
        }
    }

    pub fn init() -> Result<()> {
        let db = conn()?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS class_subjects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            class_id INTEGER NOT NULL,
            subject_id INTEGER NOT NULL,
            is_mandatory BOOLEAN NOT NULL,
            FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE,
            FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE,
            UNIQUE (class_id, subject_id)
        )",
            params![],
        )?;
        Ok(())
    }

    pub fn create(class_id: i32, subject_id: i32, is_mandatory: bool) -> Result<Self> {
        let db = conn()?;
        db.execute(
            "INSERT INTO class_subjects (class_id, subject_id, is_mandatory) VALUES (?1, ?2, ?3)",
            params![class_id, subject_id, is_mandatory],
        )?;

        let id = db.last_insert_rowid() as i32;

        Ok(ClassSubject {
            id,
            class_id,
            subject_id,
            is_mandatory,
        })
    }

    pub fn get_by_class(class_id: i32) -> Result<Vec<Self>> {
        let db = conn()?;
        let mut stmt = db.prepare(
            "SELECT id, class_id, subject_id, is_mandatory FROM class_subjects WHERE class_id = ?1",
        )?;

        let class_subject_iter = stmt.query_map([class_id], |row| {
            Ok(ClassSubject {
                id: row.get(0)?,
                class_id: row.get(1)?,
                subject_id: row.get(2)?,
                is_mandatory: row.get(3)?,
            })
        })?;

        class_subject_iter.collect()
    }

    pub fn get() -> Result<Vec<Self>> {
        let db = conn()?;
        let mut stmt =
            db.prepare("SELECT id, class_id, subject_id, is_mandatory FROM class_subjects")?;

        let class_subject_iter = stmt.query_map([], |row: &Row| {
            Ok(ClassSubject {
                id: row.get(0)?,
                class_id: row.get(1)?,
                subject_id: row.get(2)?,
                is_mandatory: row.get(3)?,
            })
        })?;

        let class_subjects: Result<Vec<Self>> = class_subject_iter.collect();

        class_subjects
    }

    pub fn delete(id: i32) -> Result<()> {
        let db = conn()?;
        let rows_affected = db.execute("DELETE FROM class_subjects WHERE id = ?1", params![id])?;

        if rows_affected == 0 {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error {
                    code: rusqlite::ffi::ErrorCode::PermissionDenied,
                    extended_code: 0,
                },
                Some("No rows deleted".to_string()),
            ));
        }

        Ok(())
    }

    pub fn edit(id: i32, class_id: i32, subject_id: i32, is_mandatory: bool) -> Result<Self> {
        let db = conn()?;

        let rows_affected = db.execute(
            "UPDATE class_subjects SET class_id = ?1, subject_id = ?2, is_mandatory = ?3 WHERE id = ?4",
            params![class_id, subject_id, is_mandatory, id],
        )?;

        if rows_affected == 0 {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error {
                    code: rusqlite::ffi::ErrorCode::OperationAborted,
                    extended_code: 0,
                },
                Some("No class_subject found with the given ID".to_string()),
            ));
        }

        Ok(ClassSubject {
            id,
            class_id,
            subject_id,
            is_mandatory,
        })
    }
}
