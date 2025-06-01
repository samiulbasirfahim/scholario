use chrono::NaiveDate;
use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};

use super::conn;

// Teacher struct
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Teacher {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub address: String,
    pub salary: i32,
    pub hire_date: NaiveDate,
    pub photo: Option<String>, // Base64 image, optional
}

// Teacher struct implementation
impl Teacher {
    pub fn new(
        id: i32,
        name: &str,
        phone: &str,
        address: &str,
        salary: i32,
        hire_date: NaiveDate,
        photo: Option<String>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            phone: phone.to_string(),
            address: address.to_string(),
            salary,
            hire_date,
            photo,
        }
    }

    pub fn init() -> Result<()> {
        let db = conn()?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS teachers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                phone TEXT NOT NULL,
                address TEXT NOT NULL,
                salary INTEGER NOT NULL,
                hire_date DATE NOT NULL,
                photo TEXT
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create(
        name: &str,
        phone: &str,
        address: &str,
        salary: i32,
        hire_date: NaiveDate,
        photo: Option<String>,
    ) -> Result<Self> {
        let db = conn()?;
        db.execute(
            "INSERT INTO teachers (name, phone, address, salary, hire_date, photo) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![name, phone, address, salary, hire_date, photo],
        )?;

        let id = db.last_insert_rowid() as i32;
        Ok(Self::new(
            id, name, phone, address, salary, hire_date, photo,
        ))
    }

    pub fn get() -> Result<Vec<Self>> {
        let db = conn()?;
        let mut stmt =
            db.prepare("SELECT id, name, phone, address, salary, hire_date, photo FROM teachers")?;
        let iter = stmt.query_map([], |row| {
            Ok(Teacher {
                id: row.get(0)?,
                name: row.get(1)?,
                phone: row.get(2)?,
                address: row.get(3)?,
                salary: row.get(4)?,
                hire_date: row.get(5)?,
                photo: row.get(6)?,
            })
        })?;
        iter.collect()
    }

    pub fn delete(id: i32) -> Result<()> {
        let db = conn()?;
        let affected = db.execute("DELETE FROM teachers WHERE id = ?1", params![id])?;
        if affected == 0 {
            Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error {
                    code: rusqlite::ffi::ErrorCode::Unknown,
                    extended_code: 0,
                },
                Some("No teacher deleted. Invalid ID.".to_string()),
            ))
        } else {
            Ok(())
        }
    }

    pub fn edit(
        id: i32,
        name: &str,
        phone: &str,
        address: &str,
        salary: i32,
        hire_date: NaiveDate,
        photo: Option<String>,
    ) -> Result<Self> {
        let db = conn()?;
        let affected = db.execute(
            "UPDATE teachers SET name = ?1, phone = ?2, address = ?3, salary = ?4, hire_date = ?5, photo = ?6 WHERE id = ?7",
            params![name, phone, address, salary, hire_date, photo, id],
        )?;

        if affected == 0 {
            Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error {
                    code: rusqlite::ffi::ErrorCode::OperationAborted,
                    extended_code: 0,
                },
                Some("No teacher found to update.".to_string()),
            ))
        } else {
            Ok(Self::new(
                id, name, phone, address, salary, hire_date, photo,
            ))
        }
    }
}

// Staff struct
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Staff {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub address: String,
    pub salary: i32,
    pub occupation: String,
    pub photo: Option<String>, // Base64 image, optional
}

// Staff struct implementation
impl Staff {
    pub fn new(
        id: i32,
        name: &str,
        phone: &str,
        address: &str,
        salary: i32,
        occupation: &str,
        photo: Option<String>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            phone: phone.to_string(),
            address: address.to_string(),
            salary,
            occupation: occupation.to_string(),
            photo,
        }
    }

    pub fn init() -> Result<()> {
        let db = conn()?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS staff (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                phone TEXT NOT NULL,
                address TEXT NOT NULL,
                salary INTEGER NOT NULL,
                occupation TEXT NOT NULL,
                photo TEXT
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create(
        name: &str,
        phone: &str,
        address: &str,
        salary: i32,
        occupation: &str,
        photo: Option<String>,
    ) -> Result<Self> {
        let db = conn()?;
        db.execute(
            "INSERT INTO staff (name, phone, address, salary, occupation, photo) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![name, phone, address, salary, occupation, photo],
        )?;

        let id = db.last_insert_rowid() as i32;
        Ok(Self::new(
            id, name, phone, address, salary, occupation, photo,
        ))
    }

    pub fn get() -> Result<Vec<Self>> {
        let db = conn()?;
        let mut stmt =
            db.prepare("SELECT id, name, phone, address, salary, occupation, photo FROM staff")?;
        let iter = stmt.query_map([], |row| {
            Ok(Staff {
                id: row.get(0)?,
                name: row.get(1)?,
                phone: row.get(2)?,
                address: row.get(3)?,
                salary: row.get(4)?,
                occupation: row.get(5)?,
                photo: row.get(6)?,
            })
        })?;
        iter.collect()
    }

    pub fn delete(id: i32) -> Result<()> {
        let db = conn()?;
        let affected = db.execute("DELETE FROM staff WHERE id = ?1", params![id])?;
        if affected == 0 {
            Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error {
                    code: rusqlite::ffi::ErrorCode::Unknown,
                    extended_code: 0,
                },
                Some("No staff deleted. Invalid ID.".to_string()),
            ))
        } else {
            Ok(())
        }
    }

    pub fn edit(
        id: i32,
        name: &str,
        phone: &str,
        address: &str,
        salary: i32,
        occupation: &str,
        photo: Option<String>,
    ) -> Result<Self> {
        let db = conn()?;
        let affected = db.execute(
            "UPDATE staff SET name = ?1, phone = ?2, address = ?3, salary = ?4, occupation = ?5, photo = ?6 WHERE id = ?7",
            params![name, phone, address, salary, occupation, photo, id],
        )?;

        if affected == 0 {
            Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error {
                    code: rusqlite::ffi::ErrorCode::OperationAborted,
                    extended_code: 0,
                },
                Some("No staff found to update.".to_string()),
            ))
        } else {
            Ok(Self::new(
                id, name, phone, address, salary, occupation, photo,
            ))
        }
    }
}
