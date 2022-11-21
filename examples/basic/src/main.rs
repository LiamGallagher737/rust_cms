#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rust_cms;
use rust_cms::model::Model;
use serde::{Deserialize, Serialize};

#[cfg(test)] mod tests;

#[launch]
fn rocket() -> _ {
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
