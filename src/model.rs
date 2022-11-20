use serde::{de::DeserializeOwned, Serialize};

/// A document or any struct / enum within a document must implement [`Model`]
pub trait Model: Serialize + DeserializeOwned {
    /// Returns the name of the implementing struct or enum
    fn get_name() -> &'static str;
    /// Returns the [`RcmsInfo`] used to display the struct / enum in the web interface
    fn get_rcms_info() -> RcmsInfo;
}

/// Stores infomation about a struct or enum used to display it on the web interface
#[derive(Debug, PartialEq)]
pub enum RcmsInfo {
    Struct {
        name: &'static str,
        fields: Vec<Field>,
    },
    Enum {
        name: &'static str,
        varients: Vec<Varient>,
    },
}

/// Infomation about a struct field
#[derive(Debug, PartialEq)]
pub struct Field {
    pub name: &'static str,
    pub value: FieldType,
}

#[derive(Debug, PartialEq)]
pub enum FieldType {
    String,
    Char,
    Boolean,
    I32,
    U32,
    F32,
    Custom(RcmsInfo),
}

/// Infomation about an enums varient
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, PartialEq)]
pub struct Varient {
    pub name: &'static str,
    // add ability for enums to hold data
}
