use crate::database::class::Subject;
use crate::database::teacher::{Teacher, TeacherSubject};

#[tauri::command]
pub fn teacher_save(
    name: &str,
    phone: &str,
    address: &str,
    salary: i32,
    photo: &str,
) -> Result<Teacher, String> {
    match Teacher::push(name, phone, address, salary, photo) {
        Ok(teacher) => Ok(teacher),
        Err(e) => Err(format!("Failed to save teacher: {}", e.to_string())),
    }
}

#[tauri::command]
pub fn teacher_get_all() -> Result<Vec<Teacher>, String> {
    match Teacher::get() {
        Ok(teachers) => Ok(teachers),
        Err(e) => Err(format!("Failed to get teachers: {}", e.to_string())),
    }
}

#[tauri::command]
pub fn teacher_delete(id: i32) -> Result<(), String> {
    match Teacher::delete_by_id(id) {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("Failed to delete teacher: {}", e.to_string())),
    }
}

#[tauri::command]
pub fn teacher_get_by_id(id: i32) -> Result<Option<Teacher>, String> {
    match Teacher::get_by_id(id) {
        Ok(teacher) => Ok(teacher),
        Err(e) => Err(format!("Failed to get teacher by ID: {}", e.to_string())),
    }
}

#[tauri::command]
pub fn teacher_subject_delete(id: i32) -> Result<(), String> {
    match TeacherSubject::delete_by_id(id) {
        Ok(()) => Ok(()),
        Err(e) => Err(format!(
            "Failed to delete teacher-subject link: {}",
            e.to_string()
        )),
    }
}

#[tauri::command]
pub fn teacher_subject_get_subjects(teacher_id: i32) -> Result<Vec<Subject>, String> {
    match TeacherSubject::get_subjects_for_teacher(teacher_id) {
        Ok(subjects) => Ok(subjects),
        Err(e) => Err(format!(
            "Failed to get subjects for teacher: {}",
            e.to_string()
        )),
    }
}
