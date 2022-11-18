use bincode::ErrorKind;
use rocket::serde::DeserializeOwned;
use serde::Serialize;

// pub(crate) struct ModelSerdeInfo {
//     pub(crate) serialize: Box<dyn Fn(Box<dyn Model>) -> Vec<u8>>,
//     pub(crate) deserialize: Box<dyn Fn(Vec<u8>) -> Box<dyn Model>>,
// }

pub trait Model: Serialize + DeserializeOwned {
    fn get_serialize_fn() -> Box<dyn Fn(Self) -> Vec<u8>> {
        Box::new(|item| bincode::serialize(&item).unwrap())
    }
    fn get_deserialize_fn() -> Box<dyn Fn(Vec<u8>) -> Self> {
        Box::new(|bin| bincode::deserialize(&bin[..]).unwrap())
    }
    fn get_name() -> &'static str;
    fn get_rcms_info() -> RcmsInfo;
}

#[derive(Debug)]
pub enum RcmsInfo {
    Struct {
        name: &'static str,
        fields: Vec<Field>,
    },
    Enum {
        name: &'static str,
        varients: Vec<Varient>,
    },
    // Union {
    //     name: &'static str,
    // },
}

#[derive(Debug)]
pub struct Field {
    pub name: &'static str,
    pub value: FieldType,
}

#[derive(Debug)]
pub enum FieldType {
    String,
    Char,
    Boolean,
    I32,
    U32,
    F32,
    Custom(RcmsInfo),
}

#[derive(Debug)]
pub struct Varient {
    pub name: &'static str,
    // add ability for enums to hold data
}
