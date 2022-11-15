#[macro_use] extern crate rocket;
use rust_cms::*;
use rust_cms::bevy_reflect::prelude::*;

#[launch]
fn rocket() -> _ {
    rust_cms::RustCMS::new()
        .register::<Person>(true)
        .register::<House>(true)
        .register::<Gender>(false)
        .build()
}

#[derive(Reflect)]
pub struct Person {
    name: String,
    age: usize,
    house: House,
    gender: Gender,
}

#[derive(Reflect)]
pub struct House {
    floors: u8,
}

#[derive(Reflect)]
pub enum Gender {
    Male,
    Female,
}
