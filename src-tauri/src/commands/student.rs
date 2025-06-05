use chrono::NaiveDate;
use tauri::command;

use crate::database::student::Student;

#[command]
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
    roll: i32,
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
        roll,
        photo,
        health_notes,
        general_notes,
    )
    .map_err(|e| e.to_string())
}

#[command]
pub fn get_students(
    session_id: i32,
    class_id: Option<i32>,
    section_id: Option<i32>,
) -> Result<Vec<Student>, String> {
    Student::get(session_id, class_id, section_id).map_err(|e| e.to_string())
}

#[command]
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
    roll: i32,
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
        roll,
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
