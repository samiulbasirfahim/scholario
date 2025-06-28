use chrono::NaiveDate;
use tauri::command;

use crate::database::student::{Attendance, Student};

#[command(rename_all = "snake_case")]
pub fn create_student(
    name: String,
    class_id: i32,
    section_id: Option<i32>,
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

#[command(rename_all = "snake_case")]
pub fn get_students(
    session_id: i32,
    class_id: Option<i32>,
    section_id: Option<i32>,
) -> Result<Vec<Student>, String> {
    Student::get(session_id, class_id, section_id).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn edit_student(
    id: i32,
    name: String,
    class_id: i32,
    section_id: Option<i32>,
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

#[command(rename_all = "snake_case")]
pub fn delete_student(id: i32) -> Result<(), String> {
    Student::delete(id).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn create_attendance(
    student_id: i32,
    date: String,
    status: String,
) -> Result<Attendance, String> {
    let date =
        NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|e| format!("Invalid date: {}", e))?;

    match Attendance::create(student_id, date, &status) {
        Ok(attendance) => Ok(attendance),

        Err(e) => {
            if e.to_string().contains("UNIQUE constraint failed") {
                Attendance::delete_by_student_and_date(student_id, date)
                    .map_err(|err| format!("Failed to delete existing record: {}", err))?;

                Attendance::create(student_id, date, &status)
                    .map_err(|err| format!("Retry failed: {}", err))
            } else {
                Err(format!("Insert failed: {}", e))
            }
        }
    }
}

#[command(rename_all = "snake_case")]
pub fn get_attendance_by_date(
    date: String,
    session_id: i32,
    class_id: Option<i32>,
    section_id: Option<i32>,
) -> Result<Vec<Attendance>, String> {
    let date =
        NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|e| format!("Invalid date: {}", e))?;

    Attendance::get_by_date(date, session_id, class_id, section_id).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn get_attendance_by_student(
    student_id: i32,
    year: Option<i32>,
    month: Option<u32>,
) -> Result<Vec<Attendance>, String> {
    Attendance::get_by_student(student_id, year, month).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn delete_attendance(id: i32) -> Result<(), String> {
    Attendance::delete(id).map_err(|e| e.to_string())
}
