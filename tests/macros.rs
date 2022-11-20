use rust_cms::{
    model::{FieldType, RcmsInfo},
    prelude::*,
};
use serde::{Deserialize, Serialize};

#[derive(Model, Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    dob: Date,
    gender: Gender,
}

#[derive(Model, Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Date {
    pub day: u32,
    pub month: u32,
    pub year: i32,
}

#[derive(Model, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[test]
fn get_name() {
    let name = Person::get_name();
    assert_eq!("Person", name);
}

#[test]
fn get_rcms_info() {
    let info = Person::get_rcms_info();
    match info {
        RcmsInfo::Enum {
            name: _,
            varients: _,
        } => panic!("Person rcms info varient is Enum, should be Struct"),
        RcmsInfo::Struct { name, fields } => {
            assert_eq!("Person", name);

            assert_eq!("name", fields[0].name);
            assert_eq!(FieldType::String, fields[0].value);

            assert_eq!("age", fields[1].name);
            assert_eq!(FieldType::U32, fields[1].value);

            assert_eq!("dob", fields[2].name);
            if let FieldType::Custom(info) = &fields[2].value {
                match info {
                    RcmsInfo::Enum {
                        name: _,
                        varients: _,
                    } => panic!("Date rcms info varient is Enum, should be Struct"),
                    RcmsInfo::Struct { name, fields } => {
                        assert_eq!("Date", *name);

                        assert_eq!("day", fields[0].name);
                        assert_eq!(FieldType::U32, fields[0].value);

                        assert_eq!("month", fields[1].name);
                        assert_eq!(FieldType::U32, fields[1].value);

                        assert_eq!("year", fields[2].name);
                        assert_eq!(FieldType::I32, fields[2].value);
                    }
                }
            }

            assert_eq!("gender", fields[3].name);
            if let FieldType::Custom(info) = &fields[3].value {
                match info {
                    RcmsInfo::Struct { name: _, fields: _ } => {
                        panic!("Gender rcms varient is Struct, should be Enum")
                    }
                    RcmsInfo::Enum { name, varients } => {
                        assert_eq!("Gender", *name);
                        assert_eq!("Male", varients[0].name);
                        assert_eq!("Female", varients[1].name);
                        assert_eq!("Other", varients[2].name);
                    }
                }
            } else {
                panic!(
                    "Field 'dob' should be type FieldType::Custom but is '{:?}'",
                    fields[2].value
                );
            }
        }
    };
}
