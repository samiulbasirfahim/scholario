pub mod class;
pub mod guardian;
pub mod session;
pub mod staff;
pub mod student;
pub mod subject;

use rusqlite::{Connection, Result};
use std::sync::{Mutex, MutexGuard, OnceLock};

use self::class::{Class, Section};
use self::guardian::{Guardian, StudentRelationship};
use self::session::Session;
use self::student::Student;
use self::subject::{ClassSubject, Subject};

static DB: OnceLock<Mutex<Connection>> = OnceLock::new();

pub fn conn() -> Result<MutexGuard<'static, Connection>> {
    let conn = DB
        .get_or_init(|| Mutex::new(Connection::open_in_memory().expect("Failed to open database")))
        // .get_or_init(|| Mutex::new(Connection::open_in_memory().expect("Failed to open database")))
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
    Student::init()?;
    StudentRelationship::init()?;
    Ok(())
}
