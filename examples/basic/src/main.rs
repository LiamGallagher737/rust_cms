#[macro_use]
extern crate rocket;
use rust_cms::prelude::*;
use serde::{Deserialize, Serialize};
use rust_cms::documents;
use rust_cms::RustCmsDocumentsCollection;

#[launch]
fn rocket() -> _ {
    match "Test".to_lowercase() {
        "Test".to_lowercase() => {

        },
    }
    let input = RustCmsDocuments::create();
    RustCMS::new(input).build()
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
