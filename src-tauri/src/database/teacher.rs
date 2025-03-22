use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};

use super::class::Subject;
use super::get_db;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Teacher {
    id: i32,
    name: String,
    phone: String,
    address: String,
    salary: i32,
    photo: Option<String>,
}

impl Teacher {
    fn new(
        id: i32,
        name: &str,
        phone: &str,
        address: &str,
        salary: i32,
        photo: Option<String>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            phone: phone.to_string(),
            address: address.to_string(),
            salary,
            photo,
        }
    }

    pub fn init() -> Result<()> {
        let conn = get_db()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS teachers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                phone TEXT NOT NULL,
                address TEXT NOT NULL,
                salary INTEGER NOT NULL,
                photo TEXT
            )",
            [],
        )?;
        Ok(())
    }

    pub fn push(name: &str, phone: &str, address: &str, salary: i32, photo: &str) -> Result<Self> {
        let conn = super::get_db()?;
        conn.execute(
            "INSERT INTO teachers (name, phone, address, salary, photo) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![name, phone, address, salary, photo],
        )?;

        let id = conn.last_insert_rowid() as i32;
        Ok(Teacher::new(
            id,
            name,
            phone,
            address,
            salary,
            Some(photo.to_string()),
        ))
    }

    pub fn delete_by_id(id: i32) -> Result<()> {
        let conn = super::get_db()?;
        conn.execute("DELETE FROM teachers WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn get() -> Result<Vec<Self>> {
        let conn = get_db()?;
        let mut stmt = conn.prepare("SELECT id, name, phone, address, salary, photo FROM teachers")?;
        let teacher_iter = stmt.query_map([], |row| {
            Ok(Teacher {
                id: row.get(0)?,
                name: row.get(1)?,
                phone: row.get(2)?,
                address: row.get(3)?,
                salary: row.get(4)?,
                photo: row.get(5)?,
            })
        })?;
        teacher_iter.collect()
    }

    pub fn search(search_term: &str) -> Result<Vec<Self>> {
        if search_term.is_empty() {
            return Ok(vec![]);
        }
        let conn = get_db()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, phone, address, photo 
             FROM teachers
             WHERE name LIKE ?1 OR phone LIKE ?2 
             LIMIT 15",
        )?;
        let like_pattern = format!("%{}%", search_term);
        let teacher_iter = stmt.query_map(params![&like_pattern, &like_pattern], |row| {
            Ok(Teacher {
                id: row.get(0)?,
                name: row.get(1)?,
                phone: row.get(2)?,
                address: row.get(3)?,
                salary: row.get(4)?,
                photo: row.get(5)?,
            })
        })?;
        teacher_iter.collect()
    }

    pub fn get_by_id(id: i32) -> Result<Option<Self>> {
        let conn = get_db()?;
        let mut stmt =
            conn.prepare("SELECT id, name, phone, address, photo FROM teachers WHERE id = ?1")?;
        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(Teacher {
                id: row.get(0)?,
                name: row.get(1)?,
                phone: row.get(2)?,
                address: row.get(3)?,
                salary: row.get(4)?,
                photo: row.get(5)?,
            }))
        } else {
            Ok(None)
        }
    }
}

pub struct TeacherSubject {
    id: i32,
    teacher_id: i32,
    subject_id: i32,
}

impl TeacherSubject {
    fn new(id: i32, teacher_id: i32, subject_id: i32) -> Self {
        Self {
            id,
            teacher_id,
            subject_id,
        }
    }

    pub fn init() -> Result<()> {
        let conn = super::get_db()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS teacher_subject (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                teacher_id INTEGER NOT NULL,
                subject_id INTEGER NOT NULL,
                FOREIGN KEY (teacher_id) REFERENCES teachers(id) ON DELETE CASCADE,
                FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
            )",
            [],
        )?;
        Ok(())
    }

    pub fn push(teacher_id: i32, subject_id: i32) -> Result<Self> {
        let conn = super::get_db()?;
        conn.execute(
            "INSERT INTO teacher_subject (teacher_id, subject_id) VALUES (?1, ?2)",
            params![teacher_id, subject_id],
        )?;
        let id = conn.last_insert_rowid() as i32;
        Ok(TeacherSubject::new(id, teacher_id, subject_id))
    }

    pub fn delete_by_id(id: i32) -> Result<()> {
        let conn = super::get_db()?;
        conn.execute("DELETE FROM teacher_subject WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn get_subjects_for_teacher(teacher_id: i32) -> Result<Vec<Subject>> {
        let conn = super::get_db()?;
        let mut stmt = conn.prepare(
            "SELECT s.id, s.name
             FROM subjects s
             JOIN teacher_subject tsl ON s.id = tsl.subject_id
             WHERE tsl.teacher_id = ?1",
        )?;
        let rows = stmt.query_map(params![teacher_id], |row| {
            Ok(Subject::new(row.get(0)?, row.get::<_, String>(1)?.as_str()))
        })?;
        rows.collect()
    }
}
