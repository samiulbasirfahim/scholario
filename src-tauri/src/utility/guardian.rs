use crate::database::guardian::Guardian;

#[tauri::command]
pub fn guardian_search(term: &str) -> Result<Vec<Guardian>, String> {
    match Guardian::search(term) {
        Ok(guardians) => Ok(guardians),
        Err(e) => Err(format!("Failed to search guardian {}", e.to_string())),
    }
}

#[tauri::command]
pub fn guardian_save(name: &str, phone: &str, address: &str, photo: &str) -> Result<Guardian, String> {
    match Guardian::push(name, phone, address, photo) {
        Ok(guardian) => Ok(guardian),
        Err(e) => Err(format!("Failed to Save guardian {}", e.to_string())),
    }
}
