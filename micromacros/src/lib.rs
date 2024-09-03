#![allow(dead_code)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data::Struct, DataStruct, DeriveInput, Field, Fields::Named, FieldsNamed,
    Path, Type, TypePath,
};

#[derive(Debug)]
struct DBModel {
    name: String,
    fields: Vec<DBField>,
}

#[derive(Debug)]
struct DBField {
    name: String,
    ty: String,
}

fn get_db_field(field: &Field) -> Option<DBField> {
    let ident = match &field.ident {
        Some(id) => Some(format!("{}", id)),
        None => {
            return None;
        }
    };

    let type_ident = match &field.ty {
        Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) => segments.first().map(|s| format!("{}", s.ident)),
        _ => {
            return None;
        }
    };

    let db_field = DBField {
        name: ident.unwrap(),
        ty: type_ident.unwrap(),
    };

    Some(db_field)
}

#[proc_macro_derive(DBModel)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

    let fields = match data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => panic!("Uhhh wat"),
    };

    let db_model = DBModel {
        name: ident.to_string().to_lowercase(),
        fields: fields.iter().filter_map(get_db_field).collect(),
    };

    let fields: Vec<String> = db_model.fields.iter().map(|f| f.name.to_string()).collect();
    let columns = fields.join(",");
    let input_fields = fields
        .iter()
        .map(|_| "?".to_string())
        .collect::<Vec<String>>()
        .join(",");

    let select_string = format!("select {} from {};", &columns, &db_model.name);
    let insert_string = format!(
        "insert into {} ({}) values({});",
        &db_model.name, &columns, &input_fields
    );

    let result = quote! {
        impl #ident {
            pub fn fields() -> ::std::vec::Vec<&'static str> {
                vec![#(#fields,)*]
            }
            pub fn select() -> ::std::string::String {
                ::std::string::String::from(#select_string)
            }
            pub fn insert(&self) -> ::std::string::String {
                ::std::string::String::from(#insert_string)
            }
        }
    };

    result.into()
}
