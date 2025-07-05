pub mod class;
pub mod guardian;
pub mod session;
pub mod staff;
pub mod student;
pub mod subject;

use rusqlite::{Connection, Result};
use std::sync::{Mutex, MutexGuard, OnceLock};
use tauri::{App, Manager};

use self::class::{Class, Section};
use self::guardian::{Guardian, StudentRelationship};
use self::session::Session;
use self::student::{Attendance, Student};
use self::subject::{ClassSubject, Subject};

use super::utility::get_db_path;

static DB: OnceLock<Mutex<Connection>> = OnceLock::new();

pub fn conn() -> Result<MutexGuard<'static, Connection>, rusqlite::Error> {
    let conn = DB
        .get()
        .expect("DB not initialized")
        .lock()
        .expect("Failed to lock mutex");
    Ok(conn)
}

pub fn setup(app: &App) {
    let path = get_db_path(app.path());
    let connection = Connection::open(path).expect("Failed to open database");
    DB.set(Mutex::new(connection))
        .expect("Failed to store the database connection in a static variable");
    self::init().expect("Failed to create neccesary tables.");
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
    Attendance::init()?;
    staff::init_all()?;
    Ok(())
}
