extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DataEnum, DataStruct};

#[proc_macro_derive(Model)]
pub fn model_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_model_macro(&ast)
}

fn impl_model_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let type_name = match &ast.data {
        syn::Data::Struct(_) => quote! { Struct },
        syn::Data::Enum(_) => quote! { Enum },
        syn::Data::Union(_) => panic!("Unions not surported"),
    };

    let rcms_info = match &ast.data {
        syn::Data::Struct(e) => struct_rcms_info(e, &name.to_string()),
        syn::Data::Enum(e) => enum_rcms_info(e, &name.to_string()),
        syn::Data::Union(_) => panic!("Unions not surported"),
    };

    let gen = quote! {
        impl rust_cms::model::Model for #name {
            fn get_name() -> &'static str {
                stringify!(#name)
            }
            fn get_rcms_info() -> rust_cms::model::RcmsInfo {
                rust_cms::model::RcmsInfo::#type_name {
                    #rcms_info
                }
            }
        }
    };
    gen.into()
}

fn struct_rcms_info(data: &DataStruct, name: &str) -> quote::__private::TokenStream {
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
            "String" => quote! { rust_cms::model::FieldType::String },
            "char" => quote! { rust_cms::model::FieldType::Char },
            "bool" => quote! { rust_cms::model::FieldType::Boolean },
            "i32" => quote! { rust_cms::model::FieldType::I32 },
            "u32" => quote! { rust_cms::model::FieldType::U32 },
            "f32" => quote! { rust_cms::model::FieldType::F32 },
            _ => {
                let n = type_info.path.get_ident().unwrap();
                quote! {
                    rust_cms::model::FieldType::Custom(#n::get_rcms_info())
                }
            }
        };

        quote! {
            rust_cms::model::Field {
                name: #field_name,
                value: #field_type,
            }
        }
    });
    let gen = quote! {
        name: #name,
        fields: std::vec![
            #(#fields,)*
        ],
    };
    gen
}

fn enum_rcms_info(data: &DataEnum, name: &str) -> quote::__private::TokenStream {
    let varients = data.variants.iter().map(|varient| {
        let vname = &varient.ident.to_string();
        quote! {
            rust_cms::model::Varient {
                name: #vname,
            }
        }
    });
    let gen = quote! {
        name: #name,
        varients: std::vec![
            #(#varients,)*
        ],
    };
    gen
}
