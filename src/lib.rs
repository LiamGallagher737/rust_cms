//! # Example
//!
//! ```rust
//! #[macro_use] extern crate rocket;
//! #[macro_use] extern crate rust_cms;
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
//! - `/api/document/restaurant/<id>`
//!
//! ### Post
//! - `/api/document/person`
//! - `/api/document/restaurant`

// use model::Model;
// struct Docs {
//     v: &'static dyn Model,
// }

pub mod model;

#[doc(hidden)]
pub mod macros;

#[doc(hidden)]
pub mod __reexports {
    pub use bincode;
    pub use casey;
    pub use rocket::{data::Data, http::Status};
    pub use serde;
}
