#[macro_use]
extern crate rocket;

use model::RcmsInfo;
use rocket::{Build, Rocket};
pub use rust_cms_derive_macros;
use std::collections::HashMap;

mod api;
mod db;
pub mod model;

type Doctypes = HashMap<&'static str, model::RcmsInfo>;
pub struct RustCMS {
    doctypes: Doctypes,
}

impl Default for RustCMS {
    fn default() -> Self {
        Self {
            doctypes: HashMap::new(),
        }
    }
}

impl RustCMS {
    pub fn new<T: RustCmsDocumentsCollection>(docs: T) -> Self {
        Self {
            doctypes: HashMap::new(),
        }
    }
    pub fn build(self) -> Rocket<Build> {
        println!("Document Types:\n{:#?}", self.doctypes);

        rocket::build()
            .manage(self.doctypes)
            .mount("/api", routes![api::get_document, api::post_document])
    }
}

struct Document {
    info: RcmsInfo,
    serialize: Box<dyn Fn(Vec<u8>) -> Self>,
}

pub mod prelude {
    pub use crate::{model::Model, RustCMS};
    pub use rust_cms_derive_macros::Model;
}

#[macro_export]
macro_rules! documents {
    ($($t:ident),+) => {
        pub enum RustCmsDocuments {
            $($t{ serialize: Box<dyn Fn($t) -> Vec<u8>>, deserialize: Box<dyn Fn(Vec<u8>) -> $t> }),+
        }
        impl rust_cms::RustCmsDocumentsCollection for RustCmsDocuments {
            fn create() -> std::collections::HashMap<String, Box<Self>> {
                std::collections::HashMap::from([
                    $((stringify!($t).to_lowercase(), Box::new(RustCmsDocuments::$t { serialize: $t::get_serialize_fn(), deserialize: $t::get_deserialize_fn() }))),+
                ])
            }
        }
    };
}

pub trait RustCmsDocumentsCollection {
    fn create() -> HashMap<String, Box<Self>>;
}
