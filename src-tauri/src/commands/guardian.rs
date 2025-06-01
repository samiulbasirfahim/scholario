use crate::database::guardian::{Guardian, StudentRelationship};
use tauri::command;

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
pub fn search_guardians(query: String) -> Result<Vec<Guardian>, String> {
    Guardian::search(&query).map_err(|e| e.to_string())
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
