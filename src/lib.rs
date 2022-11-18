#[macro_use]
extern crate rocket;

use model::RcmsInfo;
use rocket::{Build, Rocket};
pub use rust_cms_derive_macros;
use std::collections::HashMap;

mod api;
mod db;
pub mod model;

pub struct RustCMS {
    // doctypes: HashMap<&'static str, Box<dyn model::Model>>,
}

impl Default for RustCMS {
    fn default() -> Self {
        Self {
            // doctypes: HashMap::new(),
        }
    }
}

impl RustCMS {
    pub fn new(docs: HashMap<String, Box<impl model::Model>>) -> Self {
        Self {
            // doctypes: HashMap::from([
            //     ("key", "value")
            // ]),
        }
    }
    pub fn build(self) -> Rocket<Build> {
        rocket::build()
            // .manage(self.doctypes)
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
            fn from_str(&self, input: &str) -> Self {
                match input.to_lowercase() {
                    $(stringify!($t).to_lowercase() => Self::$t { serialize: Box::new(|item| { bincode::serialize::<$t>(&item) }), deserialize: |bin| { bincode::deserialize(bin) as $t } }),+
                }
            }
            // fn serialize(self, item: impl model::Model, ) -> Vec<u8> {
            //     self::
            // }
        }
    };
}

pub trait RustCmsDocumentsCollection {
    fn create() -> HashMap<String, Box<Self>>;
    fn from_str(&self, input: &str) -> Self;
    // fn serialize(self, item: impl model::Model) -> Vec<u8>;
    // fn deserialize(bin: Vec<u8>) -> Box<dyn model::Model>;
}
