mod commands;
mod database;
mod fake;
mod utility;

use commands::class::*;
use commands::guardian::*;
use commands::session::*;
use commands::student::*;
use commands::subjects::*;
use fake::generate_fake_data;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            database::setup(app);
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
            generate_fake_data,
            // Student commands
            create_student,
            get_students,
            delete_student,
            edit_student,
            // Attendance commands
            create_attendance,
            get_attendance_by_date,
            get_attendance_by_student,
            delete_attendance,
            //
            //
            // Staff
            commands::staff::create_staff,
            commands::staff::get_staff,
            commands::staff::update_staff,
            commands::staff::delete_staff,
            //
            // Complaint
            commands::staff::create_complaint,
            commands::staff::get_complaints_by_staff,
            commands::staff::update_complaint,
            commands::staff::delete_complaint,
            //
            // Attendance
            commands::staff::create_attendance_staff,
            commands::staff::get_attendance_by_staff,
            commands::staff::update_attendance,
            commands::staff::get_staff_attendance_by_date,
            commands::staff::delete_attendance_staff,
            //
            // Teacher-Class-Subject Link
            commands::staff::create_teacher_subject_link,
            commands::staff::get_teacher_subject_links,
            commands::staff::update_teacher_subject_link,
            commands::staff::delete_teacher_subject_link,
            //
            // guardian commands
            create_guardian,
            get_guardians,
            search_guardians,
            delete_guardian,
            edit_guardian,
            //student guardian relationship
            create_student_relationship,
            get_student_relationships,
            delete_student_relationship,
            edit_student_relationship,
            // session commands
            create_session,
            get_sessions,
            get_session_by_id,
            delete_session,
            edit_session,
            // class commands
            create_class,
            get_classes,
            delete_class,
            edit_class,
            // subject commands
            create_subject,
            get_subjects,
            delete_subject,
            edit_subject,
            // section commands
            create_section,
            get_sections,
            delete_section,
            edit_section,
            // class_subject link commands
            create_class_subject,
            get_class_subjects_by_class,
            get_class_subjects,
            delete_class_subject,
            edit_class_subject,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
