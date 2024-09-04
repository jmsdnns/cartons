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

    let db_name = db_model.name.clone();
    let db_fields: Vec<String> = db_model.fields.iter().map(|f| f.name.to_string()).collect();
    let db_columns = db_fields.join(",");

    let result = quote! {
        impl #ident {
            pub fn fields() -> ::std::vec::Vec<&'static str> {
                vec![#(#db_fields,)*]
            }

            pub fn select() -> ::std::string::String {
                let select_string = format!("select {} from {};", #db_columns, #db_name);
                ::std::string::String::from(select_string)
                //::std::string::String::from(#select_string)
            }

            pub fn insert(&self) -> ::std::string::String {
                let insert_fields = #ident::fields()
                    .iter()
                    .map(|_| "?".to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                let insert_string = format!(
                    "insert into {} ({}) values({});",
                    #db_name, #db_columns, insert_fields
                );
                ::std::string::String::from(insert_string)
            }
        }
    };

    result.into()
}
