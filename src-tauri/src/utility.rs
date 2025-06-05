use std::path::{Path, PathBuf};
use tauri::path::PathResolver;
use tauri::Runtime;

pub fn get_app_data_dir<R: Runtime>(resolver: &PathResolver<R>) -> PathBuf {
    let base_dir = resolver
        .local_data_dir()
        .expect("Failed to get local data directory");

    let app_data_dir = Path::new(&base_dir).join("scholario");

    if !app_data_dir.exists() {
        std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");
    }

    app_data_dir
}

pub fn get_db_path<R: Runtime>(resolver: &PathResolver<R>) -> PathBuf {
    let mut db_path = get_app_data_dir(resolver);
    db_path.push("db.sqlite");
    db_path
}
