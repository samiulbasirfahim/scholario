use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
struct Guardian {
    id: i32,
    name: String,
    phone: String,
    address: String,
    photo: String,
}
