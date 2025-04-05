use serde::{Deserialize, Serialize};

// Student struct
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub class_id: i32,
    pub section_id: i32,
    pub dob: String,
    pub gender: String,
    pub religion: String,
    pub address: String,
    pub phone: String,
    pub admission_date: String,
    pub is_resident: bool,
    pub photo: Option<String>, // Base64 image, optional
}

// Guardian struct
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Guardian {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub relationship: Option<String>,
    pub address: Option<String>,
    pub photo: Option<String>, // Base64 image, optional
}

// Teacher struct
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Teacher {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub address: String,
    pub salary: i32,
    pub hire_date: String,
    pub photo: Option<String>, // Base64 image, optional
}

// Staff struct
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Staff {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub address: String,
    pub salary: i32,
    pub occupation: String,
    pub photo: Option<String>, // Base64 image, optional
}
