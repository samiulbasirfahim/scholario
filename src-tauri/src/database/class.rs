use super::conn;
use rusqlite::{params, Result, Row};
use serde::{Deserialize, Serialize};

// Class struct
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Class {
    pub id: i32,
    pub name: String,
    pub level: i32,
    pub admission_fee: i32,
    pub monthly_fee: i32,
    pub readmission_fee: i32,
    pub session_id: i32,
}

impl Class {
    pub fn new(
        id: i32,
        name: &str,
        level: i32,
        admission_fee: i32,
        monthly_fee: i32,
        readmission_fee: i32,
        session_id: i32,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            level,
            admission_fee,
            monthly_fee,
            readmission_fee,
            session_id,
        }
    }

    pub fn init() -> Result<()> {
        let db = conn()?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS classes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            level INTEGER NOT NULL,
            admission_fee INTEGER NOT NULL,
            monthly_fee INTEGER NOT NULL,
            readmission_fee INTEGER NOT NULL,
            session_id INTEGER NOT NULL,
            FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
        )",
            [],
        )?;
        Ok(())
    }

    pub fn create(
        name: &str,
        level: i32,
        admission_fee: i32,
        monthly_fee: i32,
        readmission_fee: i32,
        session_id: i32,
    ) -> Result<Self> {
        let db = conn()?;
        db.execute(
            "INSERT INTO classes (name, level, admission_fee, monthly_fee, readmission_fee, session_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![name, level, admission_fee, monthly_fee, readmission_fee, session_id],
        )?;

        let id = db.last_insert_rowid() as i32;

        Ok(Class {
            id,
            name: name.to_string(),
            level,
            admission_fee,
            monthly_fee,
            readmission_fee,
            session_id,
        })
    }

    pub fn get(session_id: i32) -> Result<Vec<Self>> {
        let db = conn()?;
        let mut stmt = db.prepare(
            "SELECT id, name, level, admission_fee, monthly_fee, readmission_fee, session_id
         FROM classes
         WHERE session_id = ?1",
        )?;

        let class_iter = stmt.query_map([session_id], |row: &Row| {
            Ok(Class {
                id: row.get(0)?,
                name: row.get(1)?,
                level: row.get(2)?,
                admission_fee: row.get(3)?,
                monthly_fee: row.get(4)?,
                readmission_fee: row.get(5)?,
                session_id: row.get(6)?,
            })
        })?;

        class_iter.collect()
    }

    pub fn delete(id: i32) -> Result<()> {
        let db = conn()?;
        let rows_affected = db.execute("DELETE FROM classes WHERE id = ?1", params![id])?;

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

    pub fn edit(
        id: i32,
        name: &str,
        level: i32,
        admission_fee: i32,
        monthly_fee: i32,
        readmission_fee: i32,
        session_id: i32,
    ) -> Result<Self> {
        let db = conn()?;

        let rows_affected = db.execute(
            "UPDATE classes SET name = ?1, level = ?2, admission_fee = ?3, monthly_fee = ?4, readmission_fee = ?5, session_id = ?6 WHERE id = ?7",
            params![name, level, admission_fee, monthly_fee, readmission_fee, session_id, id],
        )?;

        if rows_affected == 0 {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error {
                    code: rusqlite::ffi::ErrorCode::OperationAborted,
                    extended_code: 0,
                },
                Some("No class found with the given ID".to_string()),
            ));
        }

        Ok(Class {
            id,
            name: name.to_string(),
            level,
            admission_fee,
            monthly_fee,
            readmission_fee,
            session_id,
        })
    }
}

// Section struct
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Section {
    pub id: i32,
    pub class_id: i32,
    pub name: String,
}

impl Section {
    pub fn new(id: i32, class_id: i32, name: &str) -> Self {
        Self {
            id,
            class_id,
            name: name.to_string(),
        }
    }

    pub fn init() -> Result<()> {
        let db = conn()?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS sections (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            class_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            UNIQUE (class_id, name),
            FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE
           )",
            params![],
        )?;
        Ok(())
    }

    pub fn create(class_id: i32, name: &str) -> Result<Self> {
        let db = conn()?;
        db.execute(
            "INSERT INTO sections (class_id, name) VALUES (?1, ?2)",
            params![class_id, name],
        )?;

        let id = db.last_insert_rowid() as i32;

        let mut stmt = db.prepare("SELECT COUNT(*) FROM sections WHERE class_id = ?1")?;
        let count: i32 = stmt.query_row(params![class_id], |row| row.get(0))?;

        if count == 1 {
            db.execute(
                "UPDATE students SET section_id = ?1 WHERE class_id = ?2 AND section_id IS NULL",
                params![id, class_id],
            )?;
        }

        Ok(Section {
            id,
            class_id,
            name: name.to_string(),
        })
    }

    pub fn get() -> Result<Vec<Self>> {
        let db = conn()?;
        let mut stmt = db.prepare("SELECT id, class_id, name FROM sections")?;

        let section_iter = stmt.query_map([], |row: &Row| {
            Ok(Section {
                id: row.get(0)?,
                class_id: row.get(1)?,
                name: row.get(2)?,
            })
        })?;

        let sections: Result<Vec<Self>> = section_iter.collect();

        sections
    }

    pub fn delete(id: i32) -> Result<()> {
        let db = conn()?;
        let rows_affected = db.execute("DELETE FROM sections WHERE id = ?1", params![id])?;

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

    pub fn edit(id: i32, class_id: i32, name: &str) -> Result<Self> {
        let db = conn()?;

        let rows_affected = db.execute(
            "UPDATE sections SET class_id = ?1, name = ?2 WHERE id = ?3",
            params![class_id, name, id],
        )?;

        if rows_affected == 0 {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error {
                    code: rusqlite::ffi::ErrorCode::OperationAborted,
                    extended_code: 0,
                },
                Some("No section found with the given ID".to_string()),
            ));
        }

        Ok(Section {
            id,
            class_id,
            name: name.to_string(),
        })
    }
}
