mod database;
mod utility;

use database::{
    guardian::{Guardian, StudentGuardian},
    student::Student,
};

#[tauri::command]
fn guardian_search(term: &str) -> Vec<Guardian> {
    Guardian::search(term).expect("Failed to search for guardian")
}

#[tauri::command]
fn guardian_save(name: &str, phone: &str, address: &str, photo: &str) -> Guardian {
    Guardian::push(name, phone, address, photo).expect("Failed to save guardian")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            Guardian::init().expect("Failed to initialize guardian table");
            Student::init().expect("Failed to initialize student table");
            StudentGuardian::init().expect("Failed to initialize student_guardian table");

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![guardian_search, guardian_save])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
