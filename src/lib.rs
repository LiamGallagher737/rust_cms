#[macro_use]
extern crate rocket;

use bevy_reflect::{
    ArrayInfo, EnumInfo, GetTypeRegistration, ListInfo, MapInfo, StructInfo, TypeInfo,
    TypeRegistration, TypeRegistry, ValueInfo,
};
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;

pub use bevy_reflect;

pub mod models;
pub mod pages;
pub mod prelude;

pub struct RustCMS {
    registry: TypeRegistry,
    documents: Vec<TypeRegistration>,
    schemas: Vec<Schema>,
}

impl Default for RustCMS {
    fn default() -> Self {
        Self::new()
    }
}

impl RustCMS {
    pub fn new() -> Self {
        Self {
            registry: TypeRegistry::new(),
            documents: vec![],
            schemas: vec![],
        }
    }
    pub fn register<T: GetTypeRegistration>(mut self, document: bool) -> Self {
        self.registry.register::<T>();
        if document {
            self.documents.push(T::get_type_registration());
        }
        self
    }
    pub fn build(mut self) -> Rocket<Build> {
        self = self
            .register::<models::Time>(false)
            .register::<models::Date>(false)
            .register::<models::Geolocation>(false)
            .register::<String>(false);

        for document in &self.documents {
            if let TypeInfo::Struct(info) = document.type_info() {
                self.schemas.push(create_schema(info, &self.registry));
            }
        }

        println!("Schemas\n{:#?}", self.schemas);

        rocket::build()
            .mount("/rust_cms", routes![pages::login::login_page])
            .attach(Template::fairing())
    }
}

fn create_schema(struct_info: &StructInfo, registry: &TypeRegistry) -> Schema {
    let mut schema = Schema {
        name: struct_info.name().to_string(),
        fields: vec![],
    };
    for field in struct_info.iter() {
        let field_info = registry
            .get(field.type_id())
            .expect(&format!(
                "Unable to get type info for field {} of type {}! Have you registered it?",
                field.name(),
                field.type_name()
            ))
            .type_info();
        schema.fields.push(Field {
            name: field.name().to_string(),
            value: match field_info {
                TypeInfo::Value(e) => FieldValue::Value(e.to_owned()),
                TypeInfo::Enum(e) => FieldValue::Enum(e.to_owned()),
                TypeInfo::Struct(e) => FieldValue::Schema(create_schema(e, registry)),
                TypeInfo::List(e) => FieldValue::List(e.to_owned()),
                TypeInfo::Array(e) => FieldValue::Array(e.to_owned()),
                TypeInfo::Map(e) => FieldValue::Map(e.to_owned()),
                // TypeInfo::TupleStruct(_) => todo!(),
                // TypeInfo::Tuple(_) => todo!(),
                _ => panic!(),
            },
        })
    }
    schema
}

#[derive(Debug)]
struct Schema {
    name: String,
    fields: Vec<Field>,
}

#[derive(Debug)]
struct Field {
    name: String,
    value: FieldValue,
}

#[derive(Debug)]
enum FieldValue {
    Value(ValueInfo),
    Enum(EnumInfo),
    List(ListInfo),
    Array(ArrayInfo),
    Map(MapInfo),
    Schema(Schema),
}
