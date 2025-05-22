use crate::database::session::Session;
use chrono::NaiveDate;
use tauri::command;

#[command(rename_all = "snake_case")]
pub fn create_session(
    name: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Session, String> {
    Session::create(&name, start_date, end_date).map_err(|e| e.to_string())
}

#[command]
pub fn get_sessions() -> Result<Vec<Session>, String> {
    Session::get_all().map_err(|e| e.to_string())
}

#[command]
pub fn get_session_by_id(id: i32) -> Result<Option<Session>, String> {
    Session::get_by_id(id).map_err(|e| e.to_string())
}

#[command]
pub fn delete_session(id: i32) -> Result<(), String> {
    Session::delete(id).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn edit_session(
    id: i32,
    name: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Session, String> {
    Session::edit(id, &name, start_date, end_date).map_err(|e| e.to_string())
}
