use super::conn;
use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};

// Guardian struct
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Guardian {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub address: Option<String>,
    pub photo: Option<String>, // Base64 image, optional
}

// Guardian struct implementation
impl Guardian {
    pub fn new(
        id: i32,
        name: &str,
        phone: &str,
        address: Option<String>,
        photo: Option<String>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            phone: phone.to_string(),
            address,
            photo,
        }
    }

    pub fn init() -> Result<()> {
        let db = conn()?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS guardians (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                phone TEXT NOT NULL UNIQUE,
                address TEXT,
                photo TEXT
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create(
        name: &str,
        phone: &str,
        address: Option<String>,
        photo: Option<String>,
    ) -> Result<Self> {
        let db = conn()?;
        db.execute(
            "INSERT INTO guardians (name, phone, address, photo) VALUES (?1, ?2, ?3, ?4)",
            params![name, phone, address, photo],
        )?;

        let id = db.last_insert_rowid() as i32;
        Ok(Self::new(id, name, phone, address, photo))
    }

    pub fn get() -> Result<Vec<Self>> {
        let db = conn()?;
        let mut stmt = db.prepare("SELECT id, name, phone, address, photo FROM guardians")?;
        let iter = stmt.query_map([], |row| {
            Ok(Guardian {
                id: row.get(0)?,
                name: row.get(1)?,
                phone: row.get(2)?,
                address: row.get(3)?,
                photo: row.get(4)?,
            })
        })?;
        iter.collect()
    }

    pub fn search(query: &str) -> Result<Vec<Self>> {
        let db = conn()?;
        let like_query = format!("%{}%", query);
        let mut stmt = db.prepare(
            "SELECT id, name, phone, address, photo 
             FROM guardians 
             WHERE name LIKE ?1 OR phone LIKE ?1
             LIMIT 15",
        )?;
        let iter = stmt.query_map(params![like_query], |row| {
            Ok(Guardian {
                id: row.get(0)?,
                name: row.get(1)?,
                phone: row.get(2)?,
                address: row.get(3)?,
                photo: row.get(4)?,
            })
        })?;
        iter.collect()
    }

    pub fn delete(id: i32) -> Result<()> {
        let db = conn()?;
        let affected = db.execute("DELETE FROM guardians WHERE id = ?1", params![id])?;
        if affected == 0 {
            Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error {
                    code: rusqlite::ffi::ErrorCode::Unknown,
                    extended_code: 0,
                },
                Some("No guardian deleted. Invalid ID.".to_string()),
            ))
        } else {
            Ok(())
        }
    }

    pub fn edit(
        id: i32,
        name: &str,
        phone: &str,
        address: Option<String>,
        photo: Option<String>,
    ) -> Result<Self> {
        let db = conn()?;
        let affected = db.execute(
            "UPDATE guardians SET name = ?1, phone = ?2, address = ?3, photo = ?4 WHERE id = ?5",
            params![name, phone, address, photo, id],
        )?;

        if affected == 0 {
            Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error {
                    code: rusqlite::ffi::ErrorCode::OperationAborted,
                    extended_code: 0,
                },
                Some("No guardian found to update.".to_string()),
            ))
        } else {
            Ok(Self::new(id, name, phone, address, photo))
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StudentRelationship {
    pub id: i32,
    pub student_id: i32,
    pub related_id: i32,
    pub relationship: Option<String>,
}

// StudentRelationship struct implementation
impl StudentRelationship {
    pub fn new(id: i32, student_id: i32, related_id: i32, relationship: Option<String>) -> Self {
        Self {
            id,
            student_id,
            related_id,
            relationship,
        }
    }

    pub fn init() -> Result<()> {
        let db = conn()?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS student_relationships (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                student_id INTEGER NOT NULL,
                related_id INTEGER NOT NULL,
                relationship TEXT,
                FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create(student_id: i32, related_id: i32, relationship: Option<String>) -> Result<Self> {
        let db = conn()?;
        db.execute(
            "INSERT INTO student_relationships (student_id, related_id, relationship) VALUES (?1, ?2, ?3)",
            params![student_id, related_id, relationship],
        )?;

        let id = db.last_insert_rowid() as i32;
        Ok(Self::new(id, student_id, related_id, relationship))
    }

    pub fn get() -> Result<Vec<Self>> {
        let db = conn()?;
        let mut stmt = db.prepare(
            "SELECT id, student_id, related_id, relationship FROM student_relationships",
        )?;
        let iter = stmt.query_map([], |row| {
            Ok(StudentRelationship {
                id: row.get(0)?,
                student_id: row.get(1)?,
                related_id: row.get(2)?,
                relationship: row.get(3)?,
            })
        })?;
        iter.collect()
    }

    pub fn delete(id: i32) -> Result<()> {
        let db = conn()?;
        let affected = db.execute(
            "DELETE FROM student_relationships WHERE id = ?1",
            params![id],
        )?;
        if affected == 0 {
            Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error {
                    code: rusqlite::ffi::ErrorCode::Unknown,
                    extended_code: 0,
                },
                Some("No relationship deleted. Invalid ID.".to_string()),
            ))
        } else {
            Ok(())
        }
    }

    pub fn edit(
        id: i32,
        student_id: i32,
        related_id: i32,
        relationship: Option<String>,
    ) -> Result<Self> {
        let db = conn()?;
        let affected = db.execute(
            "UPDATE student_relationships SET student_id = ?1, related_id = ?2, relationship = ?3 WHERE id = ?4",
            params![student_id, related_id, relationship, id],
        )?;

        if affected == 0 {
            Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error {
                    code: rusqlite::ffi::ErrorCode::OperationAborted,
                    extended_code: 0,
                },
                Some("No relationship found to update.".to_string()),
            ))
        } else {
            Ok(Self::new(id, student_id, related_id, relationship))
        }
    }
}
