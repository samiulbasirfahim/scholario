use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};

use super::get_db;
use super::student::Student;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Guardian {
    id: i32,
    name: String,
    phone: String,
    address: String,
    photo: Option<String>,
}

impl Guardian {
    fn new(id: i32, name: &str, phone: &str, address: &str, photo: Option<String>) -> Self {
        Self {
            id,
            name: name.to_string(),
            phone: phone.to_string(),
            address: address.to_string(),
            photo,
        }
    }

    pub fn init() -> Result<()> {
        let conn = get_db()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS guardians (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                phone TEXT NOT NULL,
                address TEXT NOT NULL,
                photo TEXT
            )",
            [],
        )?;
        Ok(())
    }

    pub fn push(name: &str, phone: &str, address: &str, photo: &str) -> Result<Self> {
        let conn = super::get_db()?;
        conn.execute(
            "INSERT INTO guardians (name, phone, address, photo) VALUES (?1, ?2, ?3, ?4)",
            params![name, phone, address, photo],
        )?;

        let id = conn.last_insert_rowid() as i32;
        Ok(Guardian::new(
            id,
            name,
            phone,
            address,
            Some(photo.to_string()),
        ))
    }

    pub fn get() -> Result<Vec<Self>> {
        let conn = get_db()?;
        let mut stmt = conn.prepare("SELECT id, name, phone, address, photo FROM guardians")?;
        let guardian_iter = stmt.query_map([], |row| {
            Ok(Guardian {
                id: row.get(0)?,
                name: row.get(1)?,
                phone: row.get(2)?,
                address: row.get(3)?,
                photo: row.get(4)?, // Base64 string
            })
        })?;
        guardian_iter.collect()
    }

    pub fn search(search_term: &str) -> Result<Vec<Self>> {
        if search_term.is_empty() {
            return Ok(vec![]);
        }
        let conn = get_db()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, phone, address, photo 
             FROM guardians 
             WHERE name LIKE ?1 OR phone LIKE ?2 
             LIMIT 15",
        )?;
        let like_pattern = format!("%{}%", search_term);
        let guardian_iter = stmt.query_map(params![&like_pattern, &like_pattern], |row| {
            Ok(Guardian {
                id: row.get(0)?,
                name: row.get(1)?,
                phone: row.get(2)?,
                address: row.get(3)?,
                photo: row.get(4)?, // Base64 string
            })
        })?;
        guardian_iter.collect()
    }

    pub fn get_by_id(id: i32) -> Result<Option<Self>> {
        let conn = get_db()?;
        let mut stmt =
            conn.prepare("SELECT id, name, phone, address, photo FROM guardians WHERE id = ?1")?;
        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(Guardian {
                id: row.get(0)?,
                name: row.get(1)?,
                phone: row.get(2)?,
                address: row.get(3)?,
                photo: row.get(4)?, // Base64 string
            }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StudentGuardian {
    id: i32,
    student_id: i32,
    guardian_id: i32,
    relation: String,
}

impl StudentGuardian {
    fn new(id: i32, student_id: i32, guardian_id: i32, relation: &str) -> Self {
        Self {
            id,
            student_id,
            guardian_id,
            relation: relation.to_string(),
        }
    }

    pub fn init() -> Result<()> {
        let conn = get_db()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS student_guardian (
                student_id INTEGER NOT NULL,
                guardian_id INTEGER NOT NULL,
                relation TEXT NOT NULL,
                PRIMARY KEY (student_id, guardian_id),
                FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE,
                FOREIGN KEY (guardian_id) REFERENCES guardians(id) ON DELETE CASCADE
            )",
            [],
        )?;
        Ok(())
    }

    pub fn push(student_id: i32, guardian_id: i32, relation: &str) -> Result<Self> {
        let conn = get_db()?;

        conn.execute(
            "INSERT INTO student_guardian (student_id, guardian_id, relation) VALUES (?1, ?2, ?3)",
            params![student_id, guardian_id, relation],
        )?;

        let id = conn.last_insert_rowid() as i32;
        Ok(StudentGuardian::new(id, student_id, guardian_id, relation))
    }

    pub fn get_guardians_for_student(student_id: i32) -> Result<Vec<(Guardian, String)>> {
        let conn = get_db()?;
        let mut stmt = conn.prepare(
            "SELECT g.id, g.name, g.phone, g.address, g.photo, sg.relation
             FROM guardians g
             JOIN student_guardian sg ON g.id = sg.guardian_id
             WHERE sg.student_id = ?1",
        )?;
        let rows = stmt.query_map(params![student_id], |row| {
            Ok((
                Guardian {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    phone: row.get(2)?,
                    address: row.get(3)?,
                    photo: row.get(4)?,
                },
                row.get(5)?, // relation
            ))
        })?;
        rows.collect()
    }

    pub fn get_students_for_guardian(guardian_id: i32) -> Result<Vec<(Student, String)>> {
        let conn = get_db()?;
        let mut stmt = conn.prepare(
            "SELECT s.id, s.name, sg.relation
             FROM students s
             JOIN student_guardian sg ON s.id = sg.student_id
             WHERE sg.guardian_id = ?1",
        )?;
        let rows = stmt.query_map(params![guardian_id], |row| {
            Ok((
                Student::new(row.get(0)?, row.get::<_, String>(1)?.as_str()),
                row.get(3)?,
            ))
        })?;
        rows.collect()
    }
}
