use chrono::NaiveDate;

use crate::commands::{class, session};

pub fn generate_fake_data() {
    let s = session::create_session(
        "2025".to_string(),
        NaiveDate::parse_from_str("2025-06-01", "%Y-%m-%d").unwrap(),
        NaiveDate::parse_from_str("2026-06-01", "%Y-%m-%d").unwrap(),
    )
    .unwrap();
    let c = class::create_class("Primary".to_string(), 1, 100000, 40000, 80000, s.id).unwrap();
    let sec = class::create_section(c.id, "Basic".to_string()).unwrap();
}
