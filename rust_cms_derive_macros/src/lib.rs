extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DataStruct};

#[proc_macro_derive(Schema)]
pub fn schema_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_schema_macro(&ast)
}

fn impl_schema_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let get_schema_data = match &ast.data {
        syn::Data::Struct(struct_info) => {
            let info = gen_struct_info(struct_info, name.to_string());
            quote! {
                #info
            }
        }
        syn::Data::Enum(enum_info) => {
            let _varients = enum_info
                .variants
                .iter()
                .map(|variant| variant.ident.to_string());
            todo!()
        }
        syn::Data::Union(_) => panic!("Union not surported as schema"),
    };

    let create = match &ast.data {
        syn::Data::Struct(struct_info) => {
            let fields = struct_info
                .fields
                .iter()
                .map(|field| field.ident.clone().unwrap());
            let values = struct_info
                .fields
                .iter()
                .enumerate()
                .map(|(i, field)| match &field.ty {
                    syn::Type::Path(e) => {
                        let t = e.path.get_ident().unwrap();
                        quote! {
                            input[#i].downcast_ref::<#t>().unwrap().to_owned()
                        }
                    }
                    _ => panic!(),
                });
            quote! {
                Self {
                    #(#fields: #values,)*
                }
            }
        }
        syn::Data::Enum(_) => {
            todo!()
        }
        syn::Data::Union(_) => panic!("Union not surported as schema"),
    };

    let gen = quote! {
        impl rust_cms::schema::Schema for #name {
            fn get_schema_data() -> rust_cms::schema::StructData {
                #get_schema_data
            }
            fn create(input: Vec<std::boxed::Box<dyn std::any::Any>>) -> Self {
                #create
            }
        }
        // impl rust_cms::Serialize for #name {}
        // impl rust_cms::Deserialize for #name {}
    };
    gen.into()
}

fn gen_struct_info(data: &DataStruct, name: String) -> quote::__private::TokenStream {
    let fields = data.fields.iter().map(|field| {
        let type_info = match &field.ty {
            syn::Type::Path(e) => e,
            _ => panic!("Field not surported"),
        };

        let field_name = field.ident.clone().unwrap().to_string();
        let field_type = match type_info
            .path
            .get_ident()
            .expect("Failed to get type name")
            .to_string()
            .as_str()
        {
            "String" => quote! { rust_cms::schema::FieldData::String },
            "char" => quote! { rust_cms::schema::FieldData::Char },
            "bool" => quote! { rust_cms::schema::FieldData::Boolean },
            "i32" => quote! { rust_cms::schema::FieldData::I32 },
            "u32" => quote! { rust_cms::schema::FieldData::U32 },
            "f32" => quote! { rust_cms::schema::FieldData::F32 },
            _ => {
                let n = type_info.path.get_ident().unwrap();
                quote! {
                    rust_cms::schema::FieldData::Struct(#n::get_schema_data())
                }
            }
        };

        quote! {
            rust_cms::schema::Field {
                name: #field_name,
                data: #field_type,
            }
        }
    });

    quote! {
        rust_cms::schema::StructData {
            name: #name,
            fields: std::vec![
                #(#fields,)*
            ],
        }
    }
}
