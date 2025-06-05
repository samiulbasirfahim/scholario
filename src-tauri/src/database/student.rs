use super::conn;
use chrono::NaiveDate;
use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub class_id: i32,
    pub section_id: i32,
    pub session_id: i32,
    pub dob: NaiveDate,
    pub gender: String,
    pub religion: String,
    pub address: String,
    pub phone: Option<String>,
    pub admission_date: NaiveDate,
    pub is_resident: bool,
    pub roll: i32,
    pub photo: Option<String>,
    pub health_notes: Option<String>,
    pub general_notes: Option<String>,
}

impl Student {
    pub fn new(
        id: i32,
        name: &str,
        class_id: i32,
        section_id: i32,
        session_id: i32,
        dob: NaiveDate,
        gender: &str,
        religion: &str,
        address: &str,
        phone: Option<String>,
        admission_date: NaiveDate,
        is_resident: bool,
        roll: i32,
        photo: Option<String>,
        health_notes: Option<String>,
        general_notes: Option<String>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            class_id,
            section_id,
            session_id,
            dob,
            gender: gender.to_string(),
            religion: religion.to_string(),
            address: address.to_string(),
            phone,
            admission_date,
            is_resident,
            roll,
            photo,
            health_notes,
            general_notes,
        }
    }

    pub fn init() -> Result<()> {
        let db = conn()?;
        db.execute("PRAGMA foreign_keys = ON", [])?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS students (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                class_id INTEGER NOT NULL,
                section_id INTEGER NOT NULL,
                session_id INTEGER NOT NULL,
                dob DATE NOT NULL,
                gender TEXT NOT NULL,
                religion TEXT NOT NULL,
                address TEXT NOT NULL,
                phone TEXT,
                admission_date DATE NOT NULL,
                is_resident BOOLEAN NOT NULL,
                roll INTEGER NOT NULL,
                photo TEXT,
                health_notes TEXT,
                general_notes TEXT,
                FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE RESTRICT,
                FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE,
                FOREIGN KEY (section_id) REFERENCES sections(id) ON DELETE RESTRICT,
                UNIQUE (class_id, section_id, roll)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create(
        name: &str,
        class_id: i32,
        section_id: i32,
        session_id: i32,
        dob: NaiveDate,
        gender: &str,
        religion: &str,
        address: &str,
        phone: Option<String>,
        admission_date: NaiveDate,
        is_resident: bool,
        roll: i32,
        photo: Option<String>,
        health_notes: Option<String>,
        general_notes: Option<String>,
    ) -> Result<Self> {
        let db = conn()?;
        db.execute("PRAGMA foreign_keys = ON", [])?;
        db.execute(
            "INSERT INTO students (
                name, class_id, section_id, session_id, dob, gender, religion,
                address, phone, admission_date, is_resident, roll, photo,
                health_notes, general_notes
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![
                name,
                class_id,
                section_id,
                session_id,
                dob,
                gender,
                religion,
                address,
                phone,
                admission_date,
                is_resident,
                roll,
                photo,
                health_notes,
                general_notes
            ],
        )?;

        let id = db.last_insert_rowid() as i32;
        Ok(Self::new(
            id,
            name,
            class_id,
            section_id,
            session_id,
            dob,
            gender,
            religion,
            address,
            phone,
            admission_date,
            is_resident,
            roll,
            photo,
            health_notes,
            general_notes,
        ))
    }

    pub fn get(
        session_id: i32,
        class_id: Option<i32>,
        section_id: Option<i32>,
    ) -> Result<Vec<Self>> {
        let db = conn()?;
        db.execute("PRAGMA foreign_keys = ON", [])?;

        let mut query = String::from(
            "SELECT id, name, class_id, section_id, session_id, dob, gender, religion, address,
                    phone, admission_date, is_resident, roll, photo, health_notes, general_notes
             FROM students WHERE session_id = ?",
        );

        let mut params_c: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(session_id)];

        if let Some(class) = class_id {
            query.push_str(" AND class_id = ?");
            params_c.push(Box::new(class));
        }

        if let Some(section) = section_id {
            query.push_str(" AND section_id = ?");
            params_c.push(Box::new(section));
        }

        let param_refs: Vec<&dyn rusqlite::ToSql> = params_c.iter().map(|p| p.as_ref()).collect();
        let mut stmt = db.prepare(&query)?;
        let students = stmt.query_map(param_refs.as_slice(), |row| {
            Ok(Student {
                id: row.get(0)?,
                name: row.get(1)?,
                class_id: row.get(2)?,
                section_id: row.get(3)?,
                session_id: row.get(4)?,
                dob: row.get(5)?,
                gender: row.get(6)?,
                religion: row.get(7)?,
                address: row.get(8)?,
                phone: row.get(9)?,
                admission_date: row.get(10)?,
                is_resident: row.get(11)?,
                roll: row.get(12)?,
                photo: row.get(13)?,
                health_notes: row.get(14)?,
                general_notes: row.get(15)?,
            })
        })?;
        students.collect()
    }

    pub fn edit(
        id: i32,
        name: &str,
        class_id: i32,
        section_id: i32,
        session_id: i32,
        dob: NaiveDate,
        gender: &str,
        religion: &str,
        address: &str,
        phone: Option<String>,
        admission_date: NaiveDate,
        is_resident: bool,
        roll: i32,
        photo: Option<String>,
        health_notes: Option<String>,
        general_notes: Option<String>,
    ) -> Result<Self> {
        let db = conn()?;
        db.execute("PRAGMA foreign_keys = ON", [])?;
        let affected = db.execute(
            "UPDATE students SET
                name = ?1,
                class_id = ?2,
                section_id = ?3,
                session_id = ?4,
                dob = ?5,
                gender = ?6,
                religion = ?7,
                address = ?8,
                phone = ?9,
                admission_date = ?10,
                is_resident = ?11,
                roll = ?12,
                photo = ?13,
                health_notes = ?14,
                general_notes = ?15
             WHERE id = ?16",
            params![
                name,
                class_id,
                section_id,
                session_id,
                dob,
                gender,
                religion,
                address,
                phone,
                admission_date,
                is_resident,
                roll,
                photo,
                health_notes,
                general_notes,
                id
            ],
        )?;

        if affected == 0 {
            Err(rusqlite::Error::QueryReturnedNoRows)
        } else {
            Ok(Self::new(
                id,
                name,
                class_id,
                section_id,
                session_id,
                dob,
                gender,
                religion,
                address,
                phone,
                admission_date,
                is_resident,
                roll,
                photo,
                health_notes,
                general_notes,
            ))
        }
    }

    pub fn delete(id: i32) -> Result<()> {
        let db = conn()?;
        db.execute("PRAGMA foreign_keys = ON", [])?;
        let affected = db.execute("DELETE FROM students WHERE id = ?", params![id])?;

        if affected == 0 {
            Err(rusqlite::Error::QueryReturnedNoRows)
        } else {
            Ok(())
        }
    }
}
