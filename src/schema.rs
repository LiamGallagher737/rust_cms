#[derive(Debug, PartialEq)]
pub struct EnumData {
    pub name: &'static str,
    pub varients: Vec<&'static str>,
}

#[derive(Debug, PartialEq)]
pub struct StructData {
    pub name: &'static str,
    pub fields: Vec<Field>,
}

#[derive(Debug, PartialEq)]
pub struct Field {
    pub name: &'static str,
    pub data: FieldData,
}

#[derive(Debug, PartialEq)]
pub enum FieldData {
    String,
    Char,
    Boolean,
    I32,
    U32,
    F32,
    List(Box<FieldData>),
    Struct(StructData),
    Enum(EnumData),
}

pub trait Schema: Clone {
    fn get_schema_data() -> StructData;
    fn create(input: Vec<Box<dyn std::any::Any>>) -> Self;
}
