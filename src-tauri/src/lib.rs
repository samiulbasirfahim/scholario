use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Guardian {
    id: i32,
    name: String,
    phone: String,
    address: String,
    photo: String,
}
#[tauri::command]
fn guardian_search(phone: &str) -> Vec<Guardian> {
    let guardians = vec![
        Guardian {
            id: 1,
            name: String::from("Abdullah Rahman"),
            phone: String::from("+8801712345678"),
            address: String::from("Dhaka, Bangladesh"),
            photo: String::from("https://picsum.photos/seed/1/200/200"),
        },
        Guardian {
            id: 2,
            name: String::from("Karim Hossain"),
            phone: String::from("+8801812345678"),
            address: String::from("Chattogram, Bangladesh"),
            photo: String::from("https://picsum.photos/seed/2/200/200"),
        },
        Guardian {
            id: 3,
            name: String::from("Jamil Uddin"),
            phone: String::from("+8801912345678"),
            address: String::from("Sylhet, Bangladesh"),
            photo: String::from("https://picsum.photos/seed/3/200/200"),
        },
        Guardian {
            id: 4,
            name: String::from("Fahim Ahmed"),
            phone: String::from("+8801712345679"),
            address: String::from("Mymensingh, Bangladesh"),
            photo: String::from("https://picsum.photos/seed/4/200/200"),
        },
        Guardian {
            id: 5,
            name: String::from("Nurul Islam"),
            phone: String::from("+8801623456789"),
            address: String::from("Rajshahi, Bangladesh"),
            photo: String::from("https://picsum.photos/seed/5/200/200"),
        },
        Guardian {
            id: 6,
            name: String::from("Shakib Hasan"),
            phone: String::from("+8801723456789"),
            address: String::from("Barishal, Bangladesh"),
            photo: String::from("https://picsum.photos/seed/6/200/200"),
        },
    ];

    guardians
        .iter()
        .filter(|g| g.phone.contains(phone))
        .cloned()
        .collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![guardian_search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
