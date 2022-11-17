#[macro_use]
extern crate rocket;
use rust_cms::prelude::*;
use serde::{Deserialize, Serialize};

#[launch]
fn rocket() -> _ {

    let person = Person {
        name: "Jack".into(),
        age: 17,
        dob: Date { day: 4, month: 3, year: 1998 },
        house: House { floors: 1 },
    };
    let bin: Vec<u8> = bincode::serialize(&person).unwrap();
    dbg!(bin.clone());

    let p: Person = bincode::deserialize(&bin[..]).unwrap();

    dbg!(p);

    RustCMS::new().register_document::<Person>().build()
}

#[derive(Schema, Clone, Serialize, Deserialize, Debug)]
pub struct Person {
    name: String,
    age: u32,
    dob: Date,
    // gender: Gender,
    house: House,
}

#[derive(Schema, Clone, Serialize, Deserialize, Debug)]
pub struct Date {
    pub day: u32,
    pub month: u32,
    pub year: i32,
}

#[derive(Schema, Clone, Serialize, Deserialize, Debug)]
pub struct House {
    floors: u32,
}

// #[derive()]
pub enum Gender {
    Male,
    Female,
    Other,
}
