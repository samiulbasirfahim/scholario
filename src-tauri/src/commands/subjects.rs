use crate::database::subject::{ClassSubject, Subject};
use tauri::command;

#[command(rename_all = "snake_case")]
pub fn create_subject(name: String, code: i32) -> Result<Subject, String> {
    Subject::create(&name, code).map_err(|e| e.to_string())
}

#[command]
pub fn get_subjects() -> Result<Vec<Subject>, String> {
    Subject::get().map_err(|e| e.to_string())
}

#[command]
pub fn delete_subject(id: i32) -> Result<(), String> {
    Subject::delete(id).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn edit_subject(id: i32, name: String, code: i32) -> Result<Subject, String> {
    Subject::edit(id, &name, code).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn create_class_subject(
    class_id: i32,
    subject_id: i32,
    is_mandatory: bool,
) -> Result<ClassSubject, String> {
    ClassSubject::create(class_id, subject_id, is_mandatory).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn get_class_subjects_by_class(class_id: i32) -> Result<Vec<ClassSubject>, String> {
    ClassSubject::get_by_class(class_id).map_err(|e| e.to_string())
}

#[command]
pub fn get_class_subjects() -> Result<Vec<ClassSubject>, String> {
    ClassSubject::get().map_err(|e| e.to_string())
}

#[command]
pub fn delete_class_subject(id: i32) -> Result<(), String> {
    ClassSubject::delete(id).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn edit_class_subject(
    id: i32,
    class_id: i32,
    subject_id: i32,
    is_mandatory: bool,
) -> Result<ClassSubject, String> {
    ClassSubject::edit(id, class_id, subject_id, is_mandatory).map_err(|e| e.to_string())
}
