use chrono::NaiveDate;
use fake::faker::chrono::ar_sa::Date;
use fake::Fake;
use rand::seq::{IndexedRandom, SliceRandom};
use rand::thread_rng;

use crate::database::class::{Class, Section};
use crate::database::session::Session;
use crate::database::student::Student;

pub fn generate_fake_data() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = Session::get_all()?;
    let sections = Section::get()?;
    if sessions.is_empty() {
        return Err("No sessions found".into());
    }

    let mut rng = thread_rng();

    for _ in 0..100 {
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

        // // Generate fake data fields
        // // let name: String = fake::faker::name::raw::Name();
        // let dob = /* generate dob */;
        // let gender = if fake::Fake::fake::<f32>() > 0.5 { "Male" } else { "Female" };
        // let religion = "None";
        // let address: String = fake::faker::address::raw::StreetAddress().fake();
        // let phone = None;
        // let admission_date = /* generate admission date */;
        // let is_resident = fake::Fake::fake::<f32>() > 0.7;
        // let roll = -1;
        // let photo = None;
        // let health_notes = None;
        // let general_notes = None;

        let name: String = fake::faker::name::en::Name().fake();
        // let dob: NaiveDate = 

        // Student::create(
        //     name,
        //     class.id,
        //     section,
        //     session.id,
        //     dob,
        //     gender,
        //     religion,
        //     &address,
        //     phone,
        //     admission_date,
        //     is_resident,
        //     roll,
        //     photo,
        //     health_notes,
        //     general_notes,
        // )?;
    }

    // Student::
    Ok(())
}
