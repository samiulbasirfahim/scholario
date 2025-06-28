use std::result;

use super::{class, conn};
use chrono::{Datelike, Local, NaiveDate};
use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};
use tauri::http::uri::PathAndQuery;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub class_id: i32,
    pub section_id: Option<i32>,
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
        section_id: Option<i32>,
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
                section_id INTEGER,
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
                FOREIGN KEY (section_id) REFERENCES sections(id) ON DELETE SET NULL,
                UNIQUE (class_id, section_id, roll)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create(
        name: &str,
        class_id: i32,
        section_id: Option<i32>,
        session_id: i32,
        dob: NaiveDate,
        gender: &str,
        religion: &str,
        address: &str,
        phone: Option<String>,
        admission_date: NaiveDate,
        is_resident: bool,
        mut roll: i32,
        photo: Option<String>,
        health_notes: Option<String>,
        general_notes: Option<String>,
    ) -> Result<Self> {
        let db = conn()?;
        db.execute("PRAGMA foreign_keys = ON", [])?;

        if roll == -1 {
            let mut query =
                String::from("SELECT COUNT(*) FROM students WHERE class_id = ? AND session_id = ?");
            let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> =
                vec![Box::new(class_id), Box::new(session_id)];

            if let Some(sec_id) = section_id {
                query.push_str(" AND section_id = ?");
                params_vec.push(Box::new(sec_id));
            } else {
                query.push_str(" AND section_id IS NULL");
            }

            let mut stmt = db.prepare(&query)?;

            let params_refs: Vec<&dyn rusqlite::ToSql> =
                params_vec.iter().map(|p| p.as_ref()).collect();
            roll = stmt.query_row(&params_refs[..], |row| {
                let count: i32 = row.get(0)?;
                Ok(count + 1)
            })?;
        }

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

    pub fn get_by_id(id: i32) -> Result<Self> {
        let db = conn()?;

        let mut stmt = db.prepare(
            "SELECT id, name, class_id, section_id, session_id, dob, gender, religion, address,
                phone, admission_date, is_resident, roll, photo, health_notes, general_notes
         FROM students WHERE id = ?",
        )?;

        let student = stmt.query_row([id], |row| {
            Ok(Student {
                id: row.get(0)?,
                name: row.get(1)?,
                class_id: row.get(2)?,
                section_id: row.get::<_, Option<i32>>(3)?,
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

        Ok(student)
    }

    pub fn get(
        session_id: i32,
        class_id: Option<i32>,
        section_id: Option<i32>,
    ) -> Result<Vec<Self>> {
        let db = conn()?;

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

        query.push_str(" ORDER BY roll ASC");

        let param_refs: Vec<&dyn rusqlite::ToSql> = params_c.iter().map(|p| p.as_ref()).collect();
        let mut stmt = db.prepare(&query)?;
        let students = stmt.query_map(param_refs.as_slice(), |row| {
            Ok(Student {
                id: row.get(0)?,
                name: row.get(1)?,
                class_id: row.get(2)?,
                section_id: row.get::<_, Option<i32>>(3)?,
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
        section_id: Option<i32>,
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

        let mut stmt =
            db.prepare("SELECT session_id, class_id, section_id, roll FROM students WHERE id = ?")?;

        let student = stmt.query_row(params![id], |row| {
            Ok((
                row.get::<_, i32>(0)?,
                row.get::<_, i32>(1)?,
                row.get::<_, Option<i32>>(2)?,
                row.get::<_, i32>(3)?,
            ))
        })?;

        let (session_id, class_id, section_id, roll) = student;

        let affected = db.execute("DELETE FROM students WHERE id = ?", params![id])?;
        if affected == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        if let Some(section_id) = section_id {
            db.execute(
                "UPDATE students
             SET roll = roll - 1
             WHERE session_id = ?
             AND class_id = ?
             AND section_id = ?
             AND roll > ?",
                params![session_id, class_id, section_id, roll],
            )?;
        } else {
            db.execute(
                "UPDATE students
             SET roll = roll - 1
             WHERE session_id = ?
             AND class_id = ?
             AND section_id IS NULL
             AND roll > ?",
                params![session_id, class_id, roll],
            )?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attendance {
    pub id: i32,
    pub student_id: i32,
    pub date: NaiveDate,
    pub status: String,
}

impl Attendance {
    pub fn new(id: i32, student_id: i32, date: NaiveDate, status: String) -> Self {
        Self {
            id,
            student_id,
            date,
            status,
        }
    }

    pub fn init() -> Result<()> {
        let db = conn()?;
        db.execute("PRAGMA foreign_keys = ON", [])?;

        db.execute(
            "CREATE TABLE IF NOT EXISTS attendance (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                student_id INTEGER NOT NULL,
                date DATE NOT NULL,
                status TEXT NOT NULL,
                FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
                UNIQUE (student_id, date)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create(student_id: i32, date: NaiveDate, status: &str) -> Result<Self> {
        let db = conn()?;
        db.execute("PRAGMA foreign_keys = ON", [])?;

        db.execute(
            "INSERT OR REPLACE INTO attendance (student_id, date, status) VALUES (?1, ?2, ?3)",
            params![student_id, date, status],
        )?;

        let id = db.last_insert_rowid() as i32;

        Ok(Self::new(id, student_id, date, status.to_string()))
    }

    pub fn get_by_date(
        date: NaiveDate,
        session_id: i32,
        class_id: Option<i32>,
        section_id: Option<i32>,
    ) -> Result<Vec<Self>, rusqlite::Error> {
        let db = conn()?;

        let mut query = String::from(
            "SELECT a.id, a.student_id, a.date, a.status
         FROM attendance a
         JOIN students s ON a.student_id = s.id
         WHERE a.date = ? AND s.session_id = ?",
        );

        let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(date), Box::new(session_id)];

        if let Some(cid) = class_id {
            query.push_str(" AND s.class_id = ?");
            params.push(Box::new(cid));
        }

        if let Some(sid) = section_id {
            query.push_str(" AND s.section_id = ?");
            params.push(Box::new(sid));
        }

        query.push_str(" ORDER BY a.student_id ASC");

        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        let mut stmt = db.prepare(&query)?;

        let rows = stmt.query_map(param_refs.as_slice(), |row| {
            Ok(Self {
                id: row.get(0)?,
                student_id: row.get(1)?,
                date: row.get(2)?,
                status: row.get(3)?,
            })
        })?;

        let result: Result<Vec<Self>, rusqlite::Error> = rows.collect();
        result
    }

    pub fn get_by_student(
        student_id: i32,
        year: Option<i32>,
        month: Option<u32>,
    ) -> Result<Vec<Self>> {
        let today = Local::now().naive_local().date();

        let year = year.unwrap_or(today.year());
        let month = month.unwrap_or(today.month());

        let start_date =
            NaiveDate::from_ymd_opt(year, month, 1).ok_or_else(|| rusqlite::Error::InvalidQuery)?;

        let end_date = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1).ok_or_else(|| rusqlite::Error::InvalidQuery)?
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1)
                .ok_or_else(|| rusqlite::Error::InvalidQuery)?
        };

        let db = conn()?;
        let mut stmt = db.prepare(
            "SELECT id, student_id, date, status
             FROM attendance
             WHERE student_id = ?1 AND date >= ?2 AND date < ?3
             ORDER BY date ASC",
        )?;

        let rows = stmt.query_map(params![student_id, start_date, end_date], |row| {
            Ok(Self {
                id: row.get(0)?,
                student_id: row.get(1)?,
                date: row.get(2)?,
                status: row.get(3)?,
            })
        })?;

        rows.collect()
    }

    pub fn delete_by_student_and_date(student_id_val: i32, date_val: NaiveDate) -> Result<()> {
        let conn = conn()?;

        conn.execute(
            "DELETE FROM attendances WHERE student_id = ?1 AND date = ?2",
            params![student_id_val, date_val],
        )?;

        Ok(())
    }

    pub fn delete(id: i32) -> Result<()> {
        let db = conn()?;
        let affected = db.execute("DELETE FROM attendance WHERE id = ?", params![id])?;

        if affected == 0 {
            Err(rusqlite::Error::QueryReturnedNoRows)
        } else {
            Ok(())
        }
    }
}
