pub mod class;
pub mod guardian;
pub mod session;
pub mod student;
pub mod subject;

use chrono::{NaiveDate, NaiveTime};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, MutexGuard, OnceLock};

use self::class::{Class, Section};
use self::guardian::Guardian;
use self::subject::{ClassSubject, Subject};

static DB: OnceLock<Mutex<Connection>> = OnceLock::new();

pub fn conn() -> Result<MutexGuard<'static, Connection>> {
    let conn = DB
        .get_or_init(|| Mutex::new(Connection::open_in_memory().expect("Failed to open database")))
        .lock()
        .expect("Failed to lock mutex");
    Ok(conn)
}

pub fn init() -> Result<()> {
    Session::init()?;
    Subject::init()?;
    ClassSubject::init()?;
    Class::init()?;
    Section::init()?;
    Guardian::init()?;
    Ok(())
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
pub enum RelatedType {
    Guardian,
    Teacher,
    Staff,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StudentRelationship {
    pub id: i32,
    pub student_id: i32,
    pub related_id: i32,
    pub related_type: RelatedType,
    pub relationship: Option<String>,
}

// Struct for student-specific fee overrides
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StudentFeeOverride {
    pub id: i32,
    pub student_id: i32,
    pub admission_fee: Option<i32>,
    pub monthly_fee: Option<i32>,
    pub readmission_fee: Option<i32>,
}

// Enum for fee types
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum FeeType {
    Admission,
    Monthly,
    Readmission,
}

// Struct for payments
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Payment {
    pub id: i32,
    pub student_id: i32,
    pub class_id: i32,
    pub amount: i32,
    pub payment_date: NaiveDate,
    pub fee_type: FeeType,
    pub payer_id: i32,
    pub payer_type: RelatedType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Exam {
    pub id: i32,
    pub name: String,            // e.g., "Midterm", "Final"
    pub class_id: i32,           // Which class this exam belongs to
    pub section_id: Option<i32>, // Optional: null means it's for all sections
    pub exam_fee: i32,           // In cents/taka*100
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExamSubject {
    pub id: i32,
    pub exam_id: i32,
    pub subject_id: i32,
    pub exam_date: NaiveDate, // ISO 8601 format (e.g., "2025-06-12")
    pub total_marks: i32,     // Full marks for this subject in this exam
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StudentResult {
    pub id: i32,
    pub student_id: i32,
    pub exam_subject_id: i32, // Link to subject+exam combo
    pub obtained_marks: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SalaryPayment {
    pub id: i32,
    pub employee_id: i32,            // Refers to teacher or staff
    pub employee_type: RelatedType,  // Teacher or Staff
    pub amount: i32,                 // In cents
    pub payment_date: NaiveDate,     // ISO 8601 (e.g., "2025-04-06")
    pub month: String,               // e.g., "April 2025"
    pub description: Option<String>, // Optional remarks
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum FinanceType {
    Income,
    Expense,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FinanceEntry {
    pub id: i32,
    pub entry_type: FinanceType,
    pub title: String,               // e.g., "Water Bill", "New Chairs", "Donation"
    pub amount: i32,                 // In cents
    pub date: NaiveDate,             // ISO 8601
    pub description: Option<String>, // Optional remarks
}

pub struct ResidentFee {
    pub id: i32,
    pub student_id: i32,
    pub month: String,
    pub fee: i32,
    pub payment_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AcademyConfig {
    pub id: i32,                   // Always 1 (singleton)
    pub name: String,              // Academy name
    pub default_resident_fee: i32, // In cents
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attendance {
    pub id: i32,
    pub student_id: i32,
    pub date: NaiveDate,          // Date of the class or exam (ISO 8601)
    pub status: AttendanceStatus, // Present, Absent, Late
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AttendanceStatus {
    Present,
    Absent,
    Late,
}

// ClassRoutine struct
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClassRoutine {
    pub id: i32,
    pub day_of_week: String,   // e.g., "Monday"
    pub class_id: i32,         // Which class this routine belongs to
    pub section_id: i32,       // Which section of the class
    pub subject_id: i32,       // Which subject is being taught
    pub teacher_id: i32,       // Which teacher is assigned
    pub start_time: NaiveTime, // Start time of the class
    pub end_time: NaiveTime,   // End time of the class
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    pub id: i32,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl Session {
    pub fn new(id: i32, name: &str, start_date: NaiveDate, end_date: NaiveDate) -> Self {
        Self {
            id,
            name: name.to_string(),
            start_date,
            end_date,
        }
    }

    pub fn init() -> Result<()> {
        let db = conn()?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                start_date DATE NOT NULL,
                end_date DATE NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create(name: &str, start_date: NaiveDate, end_date: NaiveDate) -> Result<Self> {
        let db = conn()?;
        db.execute(
            "INSERT INTO sessions (name, start_date, end_date) VALUES (?1, ?2, ?3)",
            params![name, start_date, end_date],
        )?;

        let id = db.last_insert_rowid() as i32;
        Ok(Self::new(id, name, start_date, end_date))
    }

    pub fn get_all() -> Result<Vec<Self>> {
        let db = conn()?;
        let mut stmt = db.prepare(
            "SELECT id, name, start_date, end_date FROM sessions ORDER BY start_date DESC",
        )?;
        let iter = stmt.query_map([], |row| {
            Ok(Session {
                id: row.get(0)?,
                name: row.get(1)?,
                start_date: row.get(2)?,
                end_date: row.get(3)?,
            })
        })?;
        iter.collect()
    }

    pub fn get_current(today: NaiveDate) -> Result<Option<Self>> {
        let db = conn()?;
        let mut stmt = db.prepare(
            "SELECT id, name, start_date, end_date FROM sessions WHERE start_date <= ?1 AND end_date >= ?1 LIMIT 1",
        )?;

        let mut rows = stmt.query(params![today])?;

        if let Some(row) = rows.next()? {
            Ok(Some(Session {
                id: row.get(0)?,
                name: row.get(1)?,
                start_date: row.get(2)?,
                end_date: row.get(3)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn delete(id: i32) -> Result<()> {
        let db = conn()?;
        let affected = db.execute("DELETE FROM sessions WHERE id = ?1", params![id])?;
        if affected == 0 {
            Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error {
                    code: rusqlite::ffi::ErrorCode::Unknown,
                    extended_code: 0,
                },
                Some("No session deleted. Invalid ID.".to_string()),
            ))
        } else {
            Ok(())
        }
    }

    pub fn edit(id: i32, name: &str, start_date: NaiveDate, end_date: NaiveDate) -> Result<Self> {
        let db = conn()?;
        let affected = db.execute(
            "UPDATE sessions SET name = ?1, start_date = ?2, end_date = ?3 WHERE id = ?4",
            params![name, start_date, end_date, id],
        )?;

        if affected == 0 {
            Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error {
                    code: rusqlite::ffi::ErrorCode::OperationAborted,
                    extended_code: 0,
                },
                Some("No session found to update.".to_string()),
            ))
        } else {
            Ok(Self::new(id, name, start_date, end_date))
        }
    }
}
