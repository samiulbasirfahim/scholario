pub mod class;
pub mod guardian;
pub mod student;
pub mod teacher;

use lazy_static::lazy_static;
use rusqlite::{Connection, Result};
use std::sync::{Mutex, MutexGuard};

lazy_static! {
    static ref DB_CONN: Mutex<Connection> =
        Mutex::new(Connection::open("/home/rxen/pb_data/scholario.db").expect("Failed to open database"));
}

pub fn get_db() -> Result<MutexGuard<'static, Connection>> {
    DB_CONN
        .lock()
        .map_err(|e| return rusqlite::Error::UnwindingPanic)
}
