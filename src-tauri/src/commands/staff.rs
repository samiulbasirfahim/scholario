#[command(rename_all = "snake_case")]
pub fn create_teacher(
    name: String,
    phone: String,
    address: String,
    salary: i32,
    hire_date: String, // Assuming date comes as string from frontend, parse it
    photo: Option<String>,
) -> Result<Teacher, String> {
    let hire_date = NaiveDate::parse_from_str(&hire_date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid hire date: {}", e))?;
    Teacher::create(&name, &phone, &address, salary, hire_date, photo).map_err(|e| e.to_string())
}

#[command]
pub fn get_teachers() -> Result<Vec<Teacher>, String> {
    Teacher::get().map_err(|e| e.to_string())
}

#[command]
pub fn delete_teacher(id: i32) -> Result<(), String> {
    Teacher::delete(id).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn edit_teacher(
    id: i32,
    name: String,
    phone: String,
    address: String,
    salary: i32,
    hire_date: String,
    photo: Option<String>,
) -> Result<Teacher, String> {
    let hire_date = NaiveDate::parse_from_str(&hire_date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid hire date: {}", e))?;
    Teacher::edit(id, &name, &phone, &address, salary, hire_date, photo).map_err(|e| e.to_string())
}

// --- STAFF ---

#[command(rename_all = "snake_case")]
pub fn create_staff(
    name: String,
    phone: String,
    address: String,
    salary: i32,
    occupation: String,
    photo: Option<String>,
) -> Result<Staff, String> {
    Staff::create(&name, &phone, &address, salary, &occupation, photo).map_err(|e| e.to_string())
}

#[command]
pub fn get_staff() -> Result<Vec<Staff>, String> {
    Staff::get().map_err(|e| e.to_string())
}

#[command]
pub fn delete_staff(id: i32) -> Result<(), String> {
    Staff::delete(id).map_err(|e| e.to_string())
}

#[command(rename_all = "snake_case")]
pub fn edit_staff(
    id: i32,
    name: String,
    phone: String,
    address: String,
    salary: i32,
    occupation: String,
    photo: Option<String>,
) -> Result<Staff, String> {
    Staff::edit(id, &name, &phone, &address, salary, &occupation, photo).map_err(|e| e.to_string())
}
