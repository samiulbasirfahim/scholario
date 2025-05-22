mod commands;
mod database;

use commands::class::*;
use commands::guardian::*;
use commands::session::*;
use commands::subjects::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                database::init().expect("Failed to create neccesary tables.");
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // session commands
            create_session,
            get_sessions,
            get_session_by_id,
            delete_session,
            edit_session,
            // class commands
            create_class,
            get_classes,
            // create_subject,
            // get_subjects,
            // delete_subject,
            // edit_subject,
            // create_class,
            // get_classes,
            // delete_class,
            // edit_class,
            // create_section,
            // get_sections,
            // delete_section,
            // edit_section,
            // create_class_subject,
            // get_class_subjects_by_class,
            // get_class_subjects,
            // delete_class_subject,
            // edit_class_subject,
            // // New commands for Student
            // create_student,
            // get_students,
            // delete_student,
            // edit_student,
            // // New commands for Guardian
            // create_guardian,
            // get_guardians,
            // delete_guardian,
            // edit_guardian,
            // // New commands for Teacher
            // create_teacher,
            // get_teachers,
            // delete_teacher,
            // edit_teacher,
            // // New commands for Staff
            // create_staff,
            // get_staff,
            // delete_staff,
            // edit_staff,
            // // New commands for StudentRelationship
            // create_student_relationship,
            // get_student_relationships,
            // delete_student_relationship,
            // edit_student_relationship,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
