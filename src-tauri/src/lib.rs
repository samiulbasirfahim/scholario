mod database;
mod utility;

use database::{
    guardian::{Guardian, StudentGuardian},
    student::Student,
    teacher::Teacher,
};

use utility::{
    guardian::{guardian_save, guardian_search},
    teacher::{
        teacher_delete, teacher_get_all, teacher_get_by_id, teacher_save, teacher_subject_delete,
        teacher_subject_get_subjects,
    },
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            Teacher::init().expect("Failed to initialize teacher table");
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
        .invoke_handler(tauri::generate_handler![
            guardian_search,
            guardian_save,
            teacher_save,
            teacher_get_by_id,
            teacher_delete,
            teacher_subject_delete,
            teacher_subject_get_subjects,
            teacher_get_all
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
