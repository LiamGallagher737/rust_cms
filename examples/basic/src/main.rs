#[macro_use]
extern crate rocket;
use rust_cms::prelude::*;
use serde::{Deserialize, Serialize};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![get_document, post_document])
}

documents!(Person, Date, Gender);

#[derive(Model, Serialize, Deserialize, Debug)]
pub struct Person {
    name: String,
    age: u32,
    dob: Date,
    gender: Gender,
}

#[derive(Model, Serialize, Deserialize, Debug)]
pub struct Date {
    pub day: u32,
    pub month: u32,
    pub year: i32,
}

#[derive(Model, Serialize, Deserialize, Debug)]
pub enum Gender {
    Male,
    Female,
    Other,
}
