use bincode::ErrorKind;

use rocket::serde::DeserializeOwned;

use serde::Serialize;

pub trait Model: Serialize + DeserializeOwned {
    // fn serialize_bin(&self) -> Result<Vec<u8>, Box<ErrorKind>> {
    //     bincode::serialize(self)
    // }
    // fn deserialize_bin(bin: Vec<u8>) -> Result<Self, Box<ErrorKind>> {
    //     bincode::deserialize(&bin[..])
    // }
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
