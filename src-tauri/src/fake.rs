use base64::engine::general_purpose;
use base64::Engine;
use chrono::{Local, NaiveDate};
use fake::Fake;
use rand::seq::{IndexedRandom, SliceRandom};
use rand::thread_rng;
use reqwest::blocking::get;

use crate::database::class::{Class, Section};
use crate::database::session::Session;
use crate::database::student::Student;

#[tauri::command()]
pub fn generate_fake_data() -> Result<(), String> {
    generate_fake_data_handler().map_err(|_| "Something went wrong".to_string())
}

pub fn generate_fake_data_handler() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = Session::get_all()?;
    let sections = Section::get()?;
    if sessions.is_empty() {
        return Err("No sessions found".into());
    }

    let mut rng = thread_rng();

    for i in 0..100 {
        // Pick a random session
        let session = sessions.choose(&mut rng).unwrap();

        // Fetch classes for this session
        let classes = Class::get(session.id)?;
        if classes.is_empty() {
            continue; // or handle error
        }

        let class = classes.choose(&mut rng).unwrap();

        // Filter sections for the class
        let class_sections: Vec<_> = sections.iter().filter(|s| s.class_id == class.id).collect();

        let section = if !class_sections.is_empty() {
            Some(class_sections.choose(&mut rng).unwrap().id)
        } else {
            None
        };

        let name: String = fake::faker::name::en::Name().fake();
        let dob: String = fake::faker::chrono::en::Date().fake();
        let dobnd = NaiveDate::parse_from_str(&dob, "%Y-%m-%d").unwrap();
        let address: String = fake::faker::address::en::CityName().fake();
        let phone = fake::faker::phone_number::en::PhoneNumber().fake();

        let healtnote: String = fake::faker::lorem::en::Paragraph(0..3).fake();
        let gnote: String = fake::faker::lorem::en::Paragraph(0..3).fake();

        let url = format!("https://robohash.org/{}.png?size=200x200", name);
        let res = get(url)?;
        let bytes = res.bytes()?;

        let base64_string = general_purpose::STANDARD.encode(&bytes);
        let photo = format!("data:image/png;base64,{}", base64_string);

        Student::create(
            &name,
            class.id,
            section,
            session.id,
            dobnd,
            "MALE",
            "ISLAM",
            &address,
            phone,
            Local::now().naive_local().date(),
            i % 3 == 0,
            -1,
            Some(photo),
            Some(healtnote),
            Some(gnote),
        )?;
    }

    Ok(())
}

pub fn generate_fake_attendacne() {}
