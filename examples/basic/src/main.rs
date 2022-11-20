#[macro_use]
extern crate rocket;
use rust_cms::prelude::*;
use serde::{Deserialize, Serialize};

#[launch]
fn rocket() -> _ {
    println!("{:#?}", Person::get_rcms_info());
    rocket::build().mount("/api", routes![get_document, post_document])
}

documents!(Person, Restaurant);

models! {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        gender: Gender,
    },
    #[derive(Debug)]
    enum Gender {
        Male,
        Female,
        Other,
    },
    #[derive(Debug)]
    struct Restaurant {
        name: String,
        rating: u32,
    },
}
