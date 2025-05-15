use super::conn;
use chrono::NaiveDate;
use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};

// Student struct
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub class_id: i32,
    pub section_id: i32,
    pub dob: NaiveDate,
    pub gender: String,
    pub religion: String,
    pub address: String,
    pub phone: String,
    pub admission_date: NaiveDate,
    pub is_resident: bool,
    pub photo: Option<String>,
}

// Guardian struct
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Guardian {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub address: Option<String>,
    pub photo: Option<String>, // Base64 image, optional
}

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StudentRelationship {
    pub id: i32,
    pub student_id: i32,
    pub related_id: i32,
    pub relationship: Option<String>,
}

// Student struct implementation
impl Student {
    pub fn new(
        id: i32,
        name: &str,
        class_id: i32,
        section_id: i32,
        dob: NaiveDate,
        gender: &str,
        religion: &str,
        address: &str,
        phone: &str,
        admission_date: NaiveDate,
        is_resident: bool,
        photo: Option<String>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            class_id,
            section_id,
            dob,
            gender: gender.to_string(),
            religion: religion.to_string(),
            address: address.to_string(),
            phone: phone.to_string(),
            admission_date,
            is_resident,
            photo,
        }
    }

    pub fn init() -> Result<()> {
        let db = conn()?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS students (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                class_id INTEGER NOT NULL,
                section_id INTEGER NOT NULL,
                dob DATE NOT NULL,
                gender TEXT NOT NULL,
                religion TEXT NOT NULL,
                address TEXT NOT NULL,
                phone TEXT NOT NULL,
                admission_date DATE NOT NULL,
                is_resident BOOLEAN NOT NULL,
                photo TEXT,
                FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE,
                FOREIGN KEY (section_id) REFERENCES sections(id) ON DELETE CASCADE
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create(
        name: &str,
        class_id: i32,
        section_id: i32,
        dob: NaiveDate,
        gender: &str,
        religion: &str,
        address: &str,
        phone: &str,
        admission_date: NaiveDate,
        is_resident: bool,
        photo: Option<String>,
    ) -> Result<Self> {
        let db = conn()?;
        db.execute(
            "INSERT INTO students (name, class_id, section_id, dob, gender, religion, address, phone, admission_date, is_resident, photo) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![name, class_id, section_id, dob, gender, religion, address, phone, admission_date, is_resident, photo],
        )?;

        let id = db.last_insert_rowid() as i32;
        Ok(Self::new(
            id,
            name,
            class_id,
            section_id,
            dob,
            gender,
            religion,
            address,
            phone,
            admission_date,
            is_resident,
            photo,
        ))
    }

    pub fn get() -> Result<Vec<Self>> {
        let db = conn()?;
        let mut stmt = db.prepare("SELECT id, name, class_id, section_id, dob, gender, religion, address, phone, admission_date, is_resident, photo FROM students")?;
        let iter = stmt.query_map([], |row| {
            Ok(Student {
                id: row.get(0)?,
                name: row.get(1)?,
                class_id: row.get(2)?,
                section_id: row.get(3)?,
                dob: row.get(4)?,
                gender: row.get(5)?,
                religion: row.get(6)?,
                address: row.get(7)?,
                phone: row.get(8)?,
                admission_date: row.get(9)?,
                is_resident: row.get(10)?,
                photo: row.get(11)?,
            })
        })?;
        iter.collect()
    }

    pub fn delete(id: i32) -> Result<()> {
        let db = conn()?;
        let affected = db.execute("DELETE FROM students WHERE id = ?1", params![id])?;
        if affected == 0 {
            Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error {
                    code: rusqlite::ffi::ErrorCode::Unknown,
                    extended_code: 0,
                },
                Some("No student deleted. Invalid ID.".to_string()),
            ))
        } else {
            Ok(())
        }
    }

    pub fn edit(
        id: i32,
        name: &str,
        class_id: i32,
        section_id: i32,
        dob: NaiveDate,
        gender: &str,
        religion: &str,
        address: &str,
        phone: &str,
        admission_date: NaiveDate,
        is_resident: bool,
        photo: Option<String>,
    ) -> Result<Self> {
        let db = conn()?;
        let affected = db.execute(
            "UPDATE students SET name = ?1, class_id = ?2, section_id = ?3, dob = ?4, gender = ?5, religion = ?6, 
             address = ?7, phone = ?8, admission_date = ?9, is_resident = ?10, photo = ?11 WHERE id = ?12",
            params![name, class_id, section_id, dob, gender, religion, address, phone, admission_date, is_resident, photo, id],
        )?;

        if affected == 0 {
            Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error {
                    code: rusqlite::ffi::ErrorCode::OperationAborted,
                    extended_code: 0,
                },
                Some("No student found to update.".to_string()),
            ))
        } else {
            Ok(Self::new(
                id,
                name,
                class_id,
                section_id,
                dob,
                gender,
                religion,
                address,
                phone,
                admission_date,
                is_resident,
                photo,
            ))
        }
    }
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
        let mut stmt = db.prepare("SELECT id, student_id, related_id, related_type, relationship FROM student_relationships")?;
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
            Ok(Self::new(
                id,
                student_id,
                related_id,
                relationship,
            ))
        }
    }
}
