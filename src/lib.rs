//! # Example
//!
//! ```rust
//! #[macro_use] extern crate rocket;
//! use rust_cms::prelude::*;
//! use serde::{Deserialize, Serialize};
//!
//! #[launch]
//! fn rocket() -> _ {
//!     rocket::build().mount("/api", routes![get_document, post_document])
//! }
//!
//! documents!(Person, Restaurant);
//!
//! models! {
//!     #[derive(Debug)]
//!     struct Person {
//!         name: String,
//!         age: u32,
//!         gender: Gender,
//!     },
//!     #[derive(Debug)]
//!     enum Gender {
//!         Male,
//!         Female,
//!         Other,
//!     },
//!     #[derive(Debug)]
//!     struct Restaurant {
//!         name: String,
//!         rating: u32,
//!     },
//! }
//! ```
//! This will result in the following endpoints
//!
//! ### Get
//! - `/api/document/person/<id>`
//! - `/api/restaurant/person/<id>`
//!
//! ### Post
//! - `/api/document/person`
//! - `/api/restaurant/person`

mod db;
pub mod model;

pub mod prelude {
    //! Exports the required items to create a basic Rust CMS project
    pub use crate::{documents, model::Model, models, rcms_value_type};
}

#[doc(hidden)]
pub mod __reexports {
    pub use bincode;
    pub use casey;
    pub use rocket::{data::Data, http::Status};
    pub use serde;
}

/// Generates a `get` and `post` endpoint for each of the inputted document types
///
/// ## Arugments
///
/// Takes atleast one struct or enum identifiers separated by commas
///
/// eg. `documents!(Person, Address)`
///
/// Where both `Person` and `Address` are names of structs or enums
///
/// ## Example
///
/// Use the `documents!()` macro to create endpoints for a Person document and a House document
///
/// ```rust
/// # #[macro_use] extern crate rocket;
/// # use rust_cms::prelude::*;
/// # use serde::{Deserialize, Serialize};
/// documents!(Person, House);
/// models! {
///     #[derive(Debug)]
///     struct Person {
///         name: String,
///         age: u32,
///     },
///     #[derive(Debug)]
///     struct House {
///         number: u32,
///         street: String,
///         city: String,
///         country: String,
///     },
/// }
/// ```
#[macro_export]
macro_rules! documents {
    ($($t:ident),+) => {

        #[get("/document/<doctype>/<id>", format = "application/octet-stream")]
        pub fn get_document(doctype: String, id: usize) -> Result<Vec<u8>, rust_cms::__reexports::Status> {
            let item: std::boxed::Box<dyn std::any::Any> = std::boxed::Box::new(12_u32);
            let bin : Vec<u8> = match doctype.to_lowercase().as_str() {
                $( rust_cms::__reexports::casey::lower!(stringify!($t)) => {
                    match rust_cms::__reexports::bincode::serialize(item.downcast_ref::<$t>().unwrap()) {
                        Ok(e) => e,
                        Err(_) => return Err(rust_cms::__reexports::Status::BadRequest),
                    }
                } ),+,
                _ => return Err(rust_cms::__reexports::Status::NotFound)
            };
            Ok(bin)
        }

        #[post("/document/<doctype>", format = "application/octet-stream", data = "<doc>")]
        pub async fn post_document(doctype: String, mut doc: rust_cms::__reexports::Data<'_>) -> Result<String, rust_cms::__reexports::Status> {
            let bytes = doc.peek(512).await;
            println!("{:#?}", bytes);
            match doctype.to_lowercase().as_str() {
                $( rust_cms::__reexports::casey::lower!(stringify!($t))  => {
                    let document = match rust_cms::__reexports::bincode::deserialize::<$t>(bytes) {
                        Ok(e) => e,
                        Err(_) => return Err(rust_cms::__reexports::Status::BadRequest),
                    };
                    return Ok(format!("{:#?}", document));
                } ),+,
                _ => return Err(rust_cms::__reexports::Status::NotFound)
            };
        }

    };
}

// trailing comma: https://discord.com/channels/442252698964721669/443150878111694848/1043840349879144609
#[macro_export]
macro_rules! models {

    ($(#[$attr:meta])* enum $name:ident {
        $($varient:ident),+$(,)?
    }) => {

        $(#[$attr])*
        #[derive(Serialize, Deserialize)]
        pub enum $name {
            $($varient),+
        }

        impl rust_cms::model::Model for $name {
            fn get_name() -> &'static str {
                stringify!($name)
            }
            fn get_rcms_info() -> rust_cms::model::RcmsInfo {
                rust_cms::model::RcmsInfo::Enum {
                    name: stringify!($name),
                    varients: std::vec![
                        $(
                            rust_cms::model::Varient {
                                name: stringify!($varient),
                            }
                        ),+
                    ],
                }
            }
        }

    };

    ($(#[$attr:meta])* struct $name:ident {
        $($field_name:ident : $field_type:ident),+$(,)?
    }) => {

        $(#[$attr])*
        #[derive(Serialize, Deserialize)]
        // #[serde(crate = "rust_cms::__reexports::serde")]
        pub struct $name {
            $(pub $field_name: $field_type),+
        }

        impl rust_cms::model::Model for $name {
            fn get_name() -> &'static str {
                stringify!($name)
            }
            fn get_rcms_info() -> rust_cms::model::RcmsInfo {
                rust_cms::model::RcmsInfo::Struct {
                    name: stringify!($name),
                    fields: std::vec![
                        $(
                            rust_cms::model::Field {
                                name: stringify!($field_name),
                                value: rcms_value_type!($field_type)
                            }
                        ),+
                    ],
                }
            }
        }

    };

    ($($(#[$attr:meta])* $t:ident $n:ident { $($tt:tt)* }),+$(,)?) => {
        $(models! {
            $(#[$attr])*
            $t $n { $($tt)* }
        })+
    };

}

#[doc(hidden)]
#[macro_export]
macro_rules! rcms_value_type {
    (String) => {
        rust_cms::model::FieldType::String
    };
    (char) => {
        rust_cms::model::FieldType::Char
    };
    (bool) => {
        rust_cms::model::FieldType::Boolean
    };
    (i32) => {
        rust_cms::model::FieldType::I32
    };
    (u32) => {
        rust_cms::model::FieldType::U32
    };
    ($t:ident) => {
        rust_cms::model::FieldType::Custom($t::get_rcms_info())
    };
}
