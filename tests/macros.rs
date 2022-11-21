#[macro_use]
extern crate rust_cms;
use rust_cms::model::{FieldType, Model, RcmsInfo};
use serde::{Deserialize, Serialize};

models! {
    struct Person {
        name: String,
        age: u32,
        gender: Gender,
    },
    enum Gender {
        Male,
        Female,
        Other,
    },
    struct Restaurant {
        name: String,
        rating: u32,
    },
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

            assert_eq!("gender", fields[2].name);
            if let FieldType::Custom(info) = &fields[2].value {
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
                    "Field 'gender' should be type FieldType::Custom but is '{:?}'",
                    fields[2].value
                );
            }
        }
    };
}
