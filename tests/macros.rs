#![allow(dead_code)]

use std::any::Any;

use rust_cms::{FieldData, Schema};
use rust_cms_derive_macros::Schema;

#[derive(Schema, Clone, Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
    dob: Date,
}

#[derive(Schema, Clone, Debug, PartialEq)]
struct Date {
    pub day: u32,
    pub month: u32,
    pub year: i32,
}

#[test]
fn test_get_schema_data() {
    let data = Person::get_schema_data();

    assert_eq!(data.name, "Person");

    assert_eq!(data.fields[0].name, "name");
    assert_eq!(data.fields[0].data, FieldData::String);

    assert_eq!(data.fields[1].name, "age");
    assert_eq!(data.fields[1].data, FieldData::U32);

    assert_eq!(data.fields[2].name, "dob");
    if let FieldData::Struct(dob) = &data.fields[2].data {
        assert_eq!(dob.name, "Date");

        assert_eq!(dob.fields[0].name, "day");
        assert_eq!(dob.fields[0].data, FieldData::U32);

        assert_eq!(dob.fields[1].name, "month");
        assert_eq!(dob.fields[1].data, FieldData::U32);

        assert_eq!(dob.fields[2].name, "year");
        assert_eq!(dob.fields[2].data, FieldData::I32);
    } else {
        panic!(
            "Field 'dob' should be struct but is '{:?}'",
            data.fields[2].data
        );
    }
}

#[test]
fn test_create() {
    let params: Vec<Box<dyn Any>> = vec![Box::new(20_u32), Box::new(2_u32), Box::new(2022_i32)];
    let date = Date::create(params);
    assert_eq!(
        date.clone(),
        Date {
            day: 20,
            month: 2,
            year: 2022
        }
    );

    let params: Vec<Box<dyn Any>> = vec![
        Box::new("Bob".to_string()),
        Box::new(70_u32),
        Box::new(date.clone()),
    ];
    let person = Person::create(params);
    assert_eq!(
        person,
        Person {
            name: "Bob".to_string(),
            age: 70,
            dob: date
        }
    );
}
