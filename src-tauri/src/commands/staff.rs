use chrono::NaiveDate;
use tauri::command;

use crate::database::staff::{Attendance, Complaint, Staff, TeacherClassSubject};

#[command(rename_all = "snake_case")]
pub fn create_staff(
    name: String,
    phone: String,
    address: String,
    salary: i32,
    hire_date: String,
    photo: Option<String>,
    is_teacher: bool,
    role: String,
    qualification: String,
) -> Result<Staff, String> {
    let hire_date = NaiveDate::parse_from_str(&hire_date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date: {}", e))?;

    Staff::create(
        &name,
        &phone,
        &address,
        salary,
        hire_date,
        photo,
        is_teacher,
        &role,
        &qualification,
    )
    .map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn get_staff() -> Result<Vec<Staff>, String> {
    Staff::get_all().map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn update_staff(
    id: i32,
    name: String,
    phone: String,
    address: String,
    salary: i32,
    hire_date: String,
    photo: Option<String>,
    is_teacher: bool,
    role: String,
    qualification: String,
) -> Result<(), String> {
    let hire_date = NaiveDate::parse_from_str(&hire_date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date: {}", e))?;

    Staff::update(
        id,
        &name,
        &phone,
        &address,
        salary,
        hire_date,
        photo,
        is_teacher,
        &role,
        &qualification,
    )
    .map_err(|e| e.to_string())
}

#[command]
pub fn delete_staff(id: i32) -> Result<(), String> {
    Staff::delete(id).map_err(|e| e.to_string())
}

//
// ─── COMPLAINT COMMANDS ─────────────────────────────────────────────────────────
//

#[command(rename_all = "snake_case")]
pub fn create_complaint(
    staff_id: i32,
    date: String,
    text: String,
    level: String,
) -> Result<Complaint, String> {
    let date =
        NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|e| format!("Invalid date: {}", e))?;
    Complaint::create(staff_id, date, &text, &level).map_err(|e| e.to_string())
}

#[command]
pub fn get_complaints_by_staff(staff_id: i32) -> Result<Vec<Complaint>, String> {
    Complaint::get_by_staff(staff_id).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn update_complaint(
    id: i32,
    staff_id: i32,
    date: String,
    text: String,
    level: String,
) -> Result<(), String> {
    let date =
        NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|e| format!("Invalid date: {}", e))?;
    Complaint::update(id, staff_id, date, &text, &level).map_err(|e| e.to_string())
}

#[command]
pub fn delete_complaint(id: i32) -> Result<(), String> {
    Complaint::delete(id).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn create_attendance_staff(
    staff_id: i32,
    date: String,
    status: String,
) -> Result<Attendance, String> {
    let date =
        NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|e| format!("Invalid date: {}", e))?;
    Attendance::create(staff_id, date, &status).map_err(|e| e.to_string())
}

#[command]
pub fn get_attendance_by_staff(staff_id: i32) -> Result<Vec<Attendance>, String> {
    Attendance::get_by_staff(staff_id).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn update_attendance(
    id: i32,
    staff_id: i32,
    date: String,
    status: String,
) -> Result<(), String> {
    let date =
        NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|e| format!("Invalid date: {}", e))?;
    Attendance::update(id, staff_id, date, &status).map_err(|e| e.to_string())
}

#[command]
pub fn delete_attendance_staff(id: i32) -> Result<(), String> {
    Attendance::delete(id).map_err(|e| e.to_string())
}

//
// ─── TEACHER-CLASS-SUBJECT COMMANDS ─────────────────────────────────────────────
//

#[command(rename_all = "snake_case")]
pub fn create_teacher_subject_link(
    teacher_id: i32,
    session_id: i32,
    class_id: i32,
    section_id: Option<i32>,
    subject_id: i32,
) -> Result<TeacherClassSubject, String> {
    TeacherClassSubject::create(teacher_id, session_id, class_id, section_id, subject_id)
        .map_err(|e| e.to_string())
}

#[command]
pub fn get_teacher_subject_links(teacher_id: i32) -> Result<Vec<TeacherClassSubject>, String> {
    TeacherClassSubject::get_by_teacher(teacher_id).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn update_teacher_subject_link(
    id: i32,
    teacher_id: i32,
    session_id: i32,
    class_id: i32,
    section_id: Option<i32>,
    subject_id: i32,
) -> Result<(), String> {
    TeacherClassSubject::update(id, teacher_id, session_id, class_id, section_id, subject_id)
        .map_err(|e| e.to_string())
}

#[command]
pub fn delete_teacher_subject_link(id: i32) -> Result<(), String> {
    TeacherClassSubject::delete(id).map_err(|e| e.to_string())
}
