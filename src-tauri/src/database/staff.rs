use super::conn;
use chrono::NaiveDate;
use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Staff {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub address: String,
    pub salary: i32,
    pub hire_date: NaiveDate,
    pub photo: Option<String>,
    pub is_teacher: bool,
    pub role: String,
    pub qualification: String,
    pub general_note: Option<String>,
    pub health_note: Option<String>,
}

impl Staff {
    pub fn init() -> Result<()> {
        let db = conn()?;
        db.execute("PRAGMA foreign_keys = ON", [])?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS staffs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                phone TEXT NOT NULL,
                address TEXT NOT NULL,
                salary INTEGER NOT NULL,
                hire_date DATE NOT NULL,
                photo TEXT,
                is_teacher BOOLEAN NOT NULL,
                role TEXT NOT NULL,
                qualification TEXT NOT NULL,
                general_note TEXT,
                health_note TEXT
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
        is_teacher: bool,
        role: &str,
        qualification: &str,
        general_note: Option<String>,
        health_note: Option<String>,
    ) -> Result<Self> {
        let db = conn()?;
        db.execute(
            "INSERT INTO staffs (
                name, phone, address, salary, hire_date, photo,
                is_teacher, role, qualification, general_note, health_note
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                name,
                phone,
                address,
                salary,
                hire_date,
                photo,
                is_teacher,
                role,
                qualification,
                general_note,
                health_note
            ],
        )?;
        let id = db.last_insert_rowid() as i32;
        Ok(Self {
            id,
            name: name.to_string(),
            phone: phone.to_string(),
            address: address.to_string(),
            salary,
            hire_date,
            photo,
            is_teacher,
            role: role.to_string(),
            qualification: qualification.to_string(),
            general_note,
            health_note,
        })
    }

    pub fn get_all() -> Result<Vec<Self>> {
        let db = conn()?;
        let mut stmt = db.prepare(
            "SELECT id, name, phone, address, salary, hire_date, photo, is_teacher, role, qualification, general_note, health_note FROM staffs",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Self {
                id: row.get(0)?,
                name: row.get(1)?,
                phone: row.get(2)?,
                address: row.get(3)?,
                salary: row.get(4)?,
                hire_date: row.get(5)?,
                photo: row.get(6)?,
                is_teacher: row.get(7)?,
                role: row.get(8)?,
                qualification: row.get(9)?,
                general_note: row.get(10)?,
                health_note: row.get(11)?,
            })
        })?;
        rows.collect()
    }

    pub fn update(
        id: i32,
        name: &str,
        phone: &str,
        address: &str,
        salary: i32,
        hire_date: NaiveDate,
        photo: Option<String>,
        is_teacher: bool,
        role: &str,
        qualification: &str,
        general_note: Option<String>,
        health_note: Option<String>,
    ) -> Result<()> {
        let db = conn()?;
        let affected = db.execute(
            "UPDATE staffs SET
                name = ?1,
                phone = ?2,
                address = ?3,
                salary = ?4,
                hire_date = ?5,
                photo = ?6,
                is_teacher = ?7,
                role = ?8,
                qualification = ?9,
                general_note = ?10,
                health_note = ?11
             WHERE id = ?12",
            params![
                name,
                phone,
                address,
                salary,
                hire_date,
                photo,
                is_teacher,
                role,
                qualification,
                general_note,
                health_note,
                id
            ],
        )?;
        if affected == 0 {
            Err(rusqlite::Error::QueryReturnedNoRows)
        } else {
            Ok(())
        }
    }

    pub fn delete(id: i32) -> Result<()> {
        let db = conn()?;
        let affected = db.execute("DELETE FROM staffs WHERE id = ?1", params![id])?;
        if affected == 0 {
            Err(rusqlite::Error::QueryReturnedNoRows)
        } else {
            Ok(())
        }
    }
}

//
// ─── COMPLAINT ────────────────────────────────────────────────────────────────
//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Complaint {
    pub id: i32,
    pub staff_id: i32,
    pub date: NaiveDate,
    pub text: String,
    pub level: String,
}

impl Complaint {
    pub fn init() -> Result<()> {
        let db = conn()?;
        db.execute("PRAGMA foreign_keys = ON", [])?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS complaints (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                staff_id INTEGER NOT NULL,
                date DATE NOT NULL,
                text TEXT NOT NULL,
                level TEXT NOT NULL,
                FOREIGN KEY (staff_id) REFERENCES staffs(id) ON DELETE CASCADE
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create(staff_id: i32, date: NaiveDate, text: &str, level: &str) -> Result<Self> {
        let db = conn()?;
        db.execute(
            "INSERT INTO complaints (staff_id, date, text, level)
             VALUES (?1, ?2, ?3, ?4)",
            params![staff_id, date, text, level],
        )?;
        let id = db.last_insert_rowid() as i32;
        Ok(Self {
            id,
            staff_id,
            date,
            text: text.to_string(),
            level: level.to_string(),
        })
    }

    pub fn get_by_staff(staff_id: i32) -> Result<Vec<Self>> {
        let db = conn()?;
        let mut stmt = db.prepare(
            "SELECT id, staff_id, date, text, level FROM complaints WHERE staff_id = ?1 ORDER BY date DESC",
        )?;
        let rows = stmt.query_map(params![staff_id], |row| {
            Ok(Self {
                id: row.get(0)?,
                staff_id: row.get(1)?,
                date: row.get(2)?,
                text: row.get(3)?,
                level: row.get(4)?,
            })
        })?;
        rows.collect()
    }

    pub fn update(id: i32, staff_id: i32, date: NaiveDate, text: &str, level: &str) -> Result<()> {
        let db = conn()?;
        let affected = db.execute(
            "UPDATE complaints SET staff_id = ?1, date = ?2, text = ?3, level = ?4 WHERE id = ?5",
            params![staff_id, date, text, level, id],
        )?;
        if affected == 0 {
            Err(rusqlite::Error::QueryReturnedNoRows)
        } else {
            Ok(())
        }
    }

    pub fn delete(id: i32) -> Result<()> {
        let db = conn()?;
        let affected = db.execute("DELETE FROM complaints WHERE id = ?1", params![id])?;
        if affected == 0 {
            Err(rusqlite::Error::QueryReturnedNoRows)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attendance {
    pub id: i32,
    pub staff_id: i32,
    pub date: NaiveDate,
    pub status: String,
}

impl Attendance {
    pub fn init() -> Result<()> {
        let db = conn()?;
        db.execute("PRAGMA foreign_keys = ON", [])?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS attendance_staff (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                staff_id INTEGER NOT NULL,
                date DATE NOT NULL,
                status TEXT NOT NULL,
                FOREIGN KEY (staff_id) REFERENCES staffs(id) ON DELETE CASCADE,
                UNIQUE (staff_id, date)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create(staff_id: i32, date: NaiveDate, status: &str) -> Result<Self> {
        let db = conn()?;
        db.execute(
            "INSERT OR REPLACE INTO attendance_staff (staff_id, date, status) VALUES (?1, ?2, ?3)",
            params![staff_id, date, status],
        )?;
        let id = db.last_insert_rowid() as i32;
        Ok(Self {
            id,
            staff_id,
            date,
            status: status.to_string(),
        })
    }

    pub fn get_by_date(date: NaiveDate) -> Result<Vec<Self>> {
        let db = conn()?;
        let mut stmt = db.prepare(
            "SELECT id, staff_id, date, status FROM attendance_staff WHERE date = ?1 ORDER BY staff_id ASC",
        )?;
        let rows = stmt.query_map(params![date], |row| {
            Ok(Self {
                id: row.get(0)?,
                staff_id: row.get(1)?,
                date: row.get(2)?,
                status: row.get(3)?,
            })
        })?;
        rows.collect()
    }

    pub fn get_by_staff(staff_id: i32) -> Result<Vec<Self>> {
        let db = conn()?;
        let mut stmt = db.prepare(
            "SELECT id, staff_id, date, status FROM attendance_staff WHERE staff_id = ?1 ORDER BY date ASC",
        )?;
        let rows = stmt.query_map(params![staff_id], |row| {
            Ok(Self {
                id: row.get(0)?,
                staff_id: row.get(1)?,
                date: row.get(2)?,
                status: row.get(3)?,
            })
        })?;
        rows.collect()
    }

    pub fn update(id: i32, staff_id: i32, date: NaiveDate, status: &str) -> Result<()> {
        let db = conn()?;
        let affected = db.execute(
            "UPDATE attendance_staff SET staff_id = ?1, date = ?2, status = ?3 WHERE id = ?4",
            params![staff_id, date, status, id],
        )?;
        if affected == 0 {
            Err(rusqlite::Error::QueryReturnedNoRows)
        } else {
            Ok(())
        }
    }

    pub fn delete(id: i32) -> Result<()> {
        let db = conn()?;
        let affected = db.execute("DELETE FROM attendance_staff WHERE id = ?1", params![id])?;
        if affected == 0 {
            Err(rusqlite::Error::QueryReturnedNoRows)
        } else {
            Ok(())
        }
    }
}

//
// ─── TEACHER CLASS SUBJECT ───────────────────────────────────────────────────────────
//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeacherClassSubject {
    pub id: i32,
    pub teacher_id: i32,
    pub session_id: i32,
    pub class_id: i32,
    pub section_id: Option<i32>,
    pub subject_id: i32,
}

impl TeacherClassSubject {
    pub fn init() -> Result<()> {
        let db = conn()?;
        db.execute("PRAGMA foreign_keys = ON", [])?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS teacher_class_subjects (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                teacher_id INTEGER NOT NULL,
                session_id INTEGER NOT NULL,
                class_id INTEGER NOT NULL,
                section_id INTEGER,
                subject_id INTEGER NOT NULL,
                FOREIGN KEY (teacher_id) REFERENCES staffs(id) ON DELETE CASCADE,
                FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE,
                FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE,
                FOREIGN KEY (section_id) REFERENCES sections(id) ON DELETE CASCADE,
                FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create(
        teacher_id: i32,
        session_id: i32,
        class_id: i32,
        section_id: Option<i32>,
        subject_id: i32,
    ) -> Result<Self> {
        let db = conn()?;
        db.execute(
            "INSERT INTO teacher_class_subjects (teacher_id, session_id, class_id, section_id, subject_id)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![teacher_id, session_id, class_id, section_id, subject_id],
        )?;
        let id = db.last_insert_rowid() as i32;
        Ok(Self {
            id,
            teacher_id,
            session_id,
            class_id,
            section_id,
            subject_id,
        })
    }

    pub fn get_by_teacher(teacher_id: i32) -> Result<Vec<Self>> {
        let db = conn()?;
        let mut stmt = db.prepare(
            "SELECT id, teacher_id, session_id, class_id, section_id, subject_id
             FROM teacher_class_subjects WHERE teacher_id = ?1",
        )?;
        let rows = stmt.query_map(params![teacher_id], |row| {
            Ok(Self {
                id: row.get(0)?,
                teacher_id: row.get(1)?,
                session_id: row.get(2)?,
                class_id: row.get(3)?,
                section_id: row.get(4)?,
                subject_id: row.get(5)?,
            })
        })?;
        rows.collect()
    }

    pub fn update(
        id: i32,
        teacher_id: i32,
        session_id: i32,
        class_id: i32,
        section_id: Option<i32>,
        subject_id: i32,
    ) -> Result<()> {
        let db = conn()?;
        let affected = db.execute(
            "UPDATE teacher_class_subjects SET teacher_id = ?1, session_id = ?2, class_id = ?3, 
             section_id = ?4, subject_id = ?5 WHERE id = ?6",
            params![teacher_id, session_id, class_id, section_id, subject_id, id],
        )?;
        if affected == 0 {
            Err(rusqlite::Error::QueryReturnedNoRows)
        } else {
            Ok(())
        }
    }

    pub fn delete(id: i32) -> Result<()> {
        let db = conn()?;
        let affected = db.execute(
            "DELETE FROM teacher_class_subjects WHERE id = ?1",
            params![id],
        )?;
        if affected == 0 {
            Err(rusqlite::Error::QueryReturnedNoRows)
        } else {
            Ok(())
        }
    }
}

pub fn init_all() -> Result<()> {
    Staff::init()?;
    Complaint::init()?;
    Attendance::init()?;
    TeacherClassSubject::init()?;
    Ok(())
}
