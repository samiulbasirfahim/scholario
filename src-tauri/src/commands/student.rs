use chrono::NaiveDate;
use tauri::command;

use crate::database::student::Student;

#[command(rename_all = "snake_case")]
pub fn create_student(
    name: String,
    class_id: i32,
    section_id: i32,
    session_id: i32,
    dob: String,
    gender: String,
    religion: String,
    address: String,
    phone: Option<String>,
    admission_date: String,
    is_resident: bool,
    photo: Option<String>,
    health_notes: Option<String>,
    general_notes: Option<String>,
) -> Result<Student, String> {
    let dob = NaiveDate::parse_from_str(&dob, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date of birth: {}", e))?;
    let admission_date = NaiveDate::parse_from_str(&admission_date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid admission date: {}", e))?;
    Student::create(
        &name,
        class_id,
        section_id,
        session_id,
        dob,
        &gender,
        &religion,
        &address,
        phone,
        admission_date,
        is_resident,
        photo,
        health_notes,
        general_notes,
    )
    .map_err(|e| e.to_string())
}

#[command]
pub fn get_students() -> Result<Vec<Student>, String> {
    Student::get().map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn get_students_paginated(limit: i32, skip: i32) -> Result<Vec<Student>, String> {
    Student::get_paginated(limit, skip).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn edit_student(
    id: i32,
    name: String,
    class_id: i32,
    section_id: i32,
    session_id: i32,
    dob: String,
    gender: String,
    religion: String,
    address: String,
    phone: Option<String>,
    admission_date: String,
    is_resident: bool,
    photo: Option<String>,
    health_notes: Option<String>,
    general_notes: Option<String>,
) -> Result<Student, String> {
    let dob = NaiveDate::parse_from_str(&dob, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date of birth: {}", e))?;
    let admission_date = NaiveDate::parse_from_str(&admission_date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid admission date: {}", e))?;
    Student::edit(
        id,
        &name,
        class_id,
        section_id,
        session_id,
        dob,
        &gender,
        &religion,
        &address,
        phone,
        admission_date,
        is_resident,
        photo,
        health_notes,
        general_notes,
    )
    .map_err(|e| e.to_string())
}

#[command]
pub fn delete_student(id: i32) -> Result<(), String> {
    Student::delete(id).map_err(|e| e.to_string())
}
