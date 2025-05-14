use crate::database::guardian::{Guardian, Staff, Student, StudentRelationship, Teacher};
use chrono::NaiveDate;
use tauri::command;

#[command(rename_all = "snake_case")]
pub fn create_student(
    name: String,
    class_id: i32,
    section_id: i32,
    dob: String, // Assuming date comes as string from frontend, parse it
    gender: String,
    religion: String,
    address: String,
    phone: String,
    admission_date: String, // Assuming date comes as string from frontend, parse it
    is_resident: bool,
    photo: Option<String>,
) -> Result<Student, String> {
    let dob = NaiveDate::parse_from_str(&dob, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date of birth: {}", e))?;
    let admission_date = NaiveDate::parse_from_str(&admission_date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid admission date: {}", e))?;
    Student::create(
        &name,
        class_id,
        section_id,
        dob,
        &gender,
        &religion,
        &address,
        &phone,
        admission_date,
        is_resident,
        photo,
    )
    .map_err(|e| e.to_string())
}

#[command]
pub fn get_students() -> Result<Vec<Student>, String> {
    Student::get().map_err(|e| e.to_string())
}

#[command]
pub fn delete_student(id: i32) -> Result<(), String> {
    Student::delete(id).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn edit_student(
    id: i32,
    name: String,
    class_id: i32,
    section_id: i32,
    dob: String,
    gender: String,
    religion: String,
    address: String,
    phone: String,
    admission_date: String,
    is_resident: bool,
    photo: Option<String>,
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
        dob,
        &gender,
        &religion,
        &address,
        &phone,
        admission_date,
        is_resident,
        photo,
    )
    .map_err(|e| e.to_string())
}

// --- GUARDIAN ---

#[command(rename_all = "snake_case")]
pub fn create_guardian(
    name: String,
    phone: String,
    address: Option<String>,
    photo: Option<String>,
) -> Result<Guardian, String> {
    Guardian::create(&name, &phone, address, photo).map_err(|e| e.to_string())
}

#[command]
pub fn get_guardians() -> Result<Vec<Guardian>, String> {
    Guardian::get().map_err(|e| e.to_string())
}

#[command]
pub fn delete_guardian(id: i32) -> Result<(), String> {
    Guardian::delete(id).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn edit_guardian(
    id: i32,
    name: String,
    phone: String,
    address: Option<String>,
    photo: Option<String>,
) -> Result<Guardian, String> {
    Guardian::edit(id, &name, &phone, address, photo).map_err(|e| e.to_string())
}

// --- TEACHER ---

#[command(rename_all = "snake_case")]
pub fn create_teacher(
    name: String,
    phone: String,
    address: String,
    salary: i32,
    hire_date: String, // Assuming date comes as string from frontend, parse it
    photo: Option<String>,
) -> Result<Teacher, String> {
    let hire_date = NaiveDate::parse_from_str(&hire_date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid hire date: {}", e))?;
    Teacher::create(&name, &phone, &address, salary, hire_date, photo).map_err(|e| e.to_string())
}

#[command]
pub fn get_teachers() -> Result<Vec<Teacher>, String> {
    Teacher::get().map_err(|e| e.to_string())
}

#[command]
pub fn delete_teacher(id: i32) -> Result<(), String> {
    Teacher::delete(id).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn edit_teacher(
    id: i32,
    name: String,
    phone: String,
    address: String,
    salary: i32,
    hire_date: String,
    photo: Option<String>,
) -> Result<Teacher, String> {
    let hire_date = NaiveDate::parse_from_str(&hire_date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid hire date: {}", e))?;
    Teacher::edit(id, &name, &phone, &address, salary, hire_date, photo).map_err(|e| e.to_string())
}

// --- STAFF ---

#[command(rename_all = "snake_case")]
pub fn create_staff(
    name: String,
    phone: String,
    address: String,
    salary: i32,
    occupation: String,
    photo: Option<String>,
) -> Result<Staff, String> {
    Staff::create(&name, &phone, &address, salary, &occupation, photo).map_err(|e| e.to_string())
}

#[command]
pub fn get_staff() -> Result<Vec<Staff>, String> {
    Staff::get().map_err(|e| e.to_string())
}

#[command]
pub fn delete_staff(id: i32) -> Result<(), String> {
    Staff::delete(id).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn edit_staff(
    id: i32,
    name: String,
    phone: String,
    address: String,
    salary: i32,
    occupation: String,
    photo: Option<String>,
) -> Result<Staff, String> {
    Staff::edit(id, &name, &phone, &address, salary, &occupation, photo).map_err(|e| e.to_string())
}

// --- STUDENT RELATIONSHIP ---

#[command(rename_all = "snake_case")]
pub fn create_student_relationship(
    student_id: i32,
    related_id: i32,
    relationship: Option<String>,
) -> Result<StudentRelationship, String> {
    StudentRelationship::create(student_id, related_id, relationship).map_err(|e| e.to_string())
}

#[command]
pub fn get_student_relationships() -> Result<Vec<StudentRelationship>, String> {
    StudentRelationship::get().map_err(|e| e.to_string())
}

#[command]
pub fn delete_student_relationship(id: i32) -> Result<(), String> {
    StudentRelationship::delete(id).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn edit_student_relationship(
    id: i32,
    student_id: i32,
    related_id: i32,
    relationship: Option<String>,
) -> Result<StudentRelationship, String> {
    StudentRelationship::edit(id, student_id, related_id, relationship).map_err(|e| e.to_string())
}
