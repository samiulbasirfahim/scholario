use crate::database::class::{Class, Section};
use tauri::command;

#[command(rename_all = "snake_case")]
pub fn create_class(
    name: String,
    level: i32,
    admission_fee: i32,
    monthly_fee: i32,
    readmission_fee: i32,
    session_id: i32,
) -> Result<Class, String> {
    Class::create(
        &name,
        level,
        admission_fee,
        monthly_fee,
        readmission_fee,
        session_id,
    )
    .map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn get_classes(session_id: i32) -> Result<Vec<Class>, String> {
    Class::get(session_id).map_err(|e| e.to_string())
}

#[command]
pub fn delete_class(id: i32) -> Result<(), String> {
    Class::delete(id).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn edit_class(
    id: i32,
    name: String,
    level: i32,
    admission_fee: i32,
    monthly_fee: i32,
    readmission_fee: i32,
    session_id: i32,
) -> Result<Class, String> {
    Class::edit(
        id,
        &name,
        level,
        admission_fee,
        monthly_fee,
        readmission_fee,
        session_id,
    )
    .map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn create_section(class_id: i32, name: String) -> Result<Section, String> {
    Section::create(class_id, &name).map_err(|e| e.to_string())
}

#[command]
pub fn get_sections() -> Result<Vec<Section>, String> {
    Section::get().map_err(|e| e.to_string())
}

#[command]
pub fn delete_section(id: i32) -> Result<(), String> {
    Section::delete(id).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn edit_section(id: i32, class_id: i32, name: String) -> Result<Section, String> {
    Section::edit(id, class_id, &name).map_err(|e| e.to_string())
}
