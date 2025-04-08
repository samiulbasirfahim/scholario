mod commands;
mod database;

use commands::class::*;

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
            create_subject,
            get_subjects,
            delete_subject,
            edit_subject,
            create_class,
            get_classes,
            delete_class,
            edit_class,
            create_section,
            get_sections,
            delete_section,
            edit_section,
            create_class_subject,
            get_class_subjects,
            delete_class_subject,
            edit_class_subject,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
