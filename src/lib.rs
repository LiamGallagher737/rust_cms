#[macro_use]
extern crate rocket;

use std::collections::HashMap;

use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;

pub use rust_cms_derive_macros;
pub use serde::{Serialize, Deserialize};

mod api;
mod db;
mod pages;
pub mod schema;

type Doctypes = HashMap<&'static str, schema::StructData>;
pub struct RustCMS {
    doctypes: Doctypes,
}

impl Default for RustCMS {
    fn default() -> Self {
        Self::new()
    }
}

impl RustCMS {
    pub fn new() -> Self {
        Self { doctypes: HashMap::new() }
    }
    pub fn register_document<T: schema::Schema>(mut self) -> Self {
        let data = T::get_schema_data();
        self.doctypes.insert(data.name, data);
        self
    }
    pub fn build(self) -> Rocket<Build> {
        println!("Document Types:\n{:#?}", self.doctypes);

        rocket::build()
            .manage(self.doctypes)
            .mount("/api", routes![api::get_document, api::post_document])
            .mount("/rust_cms", routes![pages::login::login_page])
            .attach(Template::fairing())
    }
}

pub mod prelude {
    pub use crate::RustCMS;
    pub use rust_cms_derive_macros::Schema;
}
