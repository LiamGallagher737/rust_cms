/// Generates a `get` and `post` endpoint for each of the inputted document types
///
/// ## Arugments
///
/// Takes atleast one struct or enum identifiers separated by commas
///
/// ## Example
///
/// Use the `documents!()` macro to create document endpoints for the Person and House models
///
/// ```rust
/// # #[macro_use] extern crate rocket;
/// # #[macro_use] extern crate rust_cms;
/// # use serde::{Deserialize, Serialize};
/// documents!(Person, House);
/// # models! {
/// #     #[derive(Debug)]
/// #     struct Person {
/// #         name: String,
/// #         age: u32,
/// #     },
/// #     #[derive(Debug)]
/// #     struct House {
/// #         number: u32,
/// #         street: String,
/// #         city: String,
/// #         country: String,
/// #     },
/// # }
/// ```
#[macro_export]
macro_rules! documents {
    ($($t:ident),+) => {

        #[get("/document/<doctype>/<id>", format = "application/octet-stream")]
        pub fn get_document(doctype: String, id: usize) -> Result<Vec<u8>, rust_cms::__reexports::Status> {
            let path = format!("{}/db/document/{doctype}/{id}.bin", std::env::current_dir().unwrap().display());
            match std::fs::read(path) {
                Ok(e) => Ok(e),
                Err(_) => Err(rust_cms::__reexports::Status::InternalServerError),
            }
        }

        #[post("/document/<doctype>", format = "application/octet-stream", data = "<doc>")]
        pub async fn post_document(doctype: String, mut doc: rust_cms::__reexports::Data<'_>) -> Result<String, rust_cms::__reexports::Status> {
            let bytes = doc.peek(512).await;

            // Check if successfully deserialize to document before adding it to database
            match doctype.to_lowercase().as_str() {
                $( rust_cms::__reexports::casey::lower!(stringify!($t))  => {
                    let document = match rust_cms::__reexports::bincode::deserialize::<$t>(bytes) {
                        Ok(_) => {},
                        Err(_) => return Err(rust_cms::__reexports::Status::BadRequest),
                    };
                } ),+,
                _ => return Err(rust_cms::__reexports::Status::NotFound)
            };

            let id = 0;

            let path = format!("{}/db/document/{doctype}/", std::env::current_dir().unwrap().display());
            std::fs::create_dir_all(path.clone());
            match std::fs::write(format!("{path}/{id}.bin"), bytes) {
                Ok(_) => Ok(id.to_string()),
                Err(_) => Err(rust_cms::__reexports::Status::InternalServerError),
            }
        }

        // fn get_current_working_dir() -> String {
        //     let res = std::env::current_dir();
        //     match res {
        //         Ok(path) => path.into_os_string().into_string().unwrap(),
        //         Err(_) => "FAILED".to_string()
        //     }
        // }

    };
}

/// Any struct or enum you want to use as a document or in
///
/// ## Arguments
///
/// Takes at least one unique struct or enum definition or multiple seperated by commas
///
/// ## Example
///
/// Use the `models! { }` macro to define a Person, Gender and Restaurant models
///
/// ```rust
/// # #[macro_use] extern crate rust_cms;
/// # use serde::{Deserialize, Serialize};
/// models! {
///     struct Person {
///         name: String,
///         age: u32,
///         gender: Gender,
///     },
///     enum Gender {
///         Male,
///         Female,
///         Other,
///     },
///     struct Restaurant {
///         name: String,
///         rating: u32,
///     },
/// }
/// ```
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
