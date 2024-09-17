#![allow(dead_code)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data::Struct, DataStruct, DeriveInput, Field, Fields::Named, FieldsNamed,
    Ident, Path, Type, TypePath,
};

#[derive(Debug)]
struct ORMModel {
    name: String,
    fields: Vec<ORMField>,
}

#[derive(Debug)]
struct ORMField {
    name: String,
    ty: String,
    ident: Ident,
}

fn get_db_field(field: &Field) -> Option<ORMField> {
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

    let db_field = ORMField {
        name: ident.unwrap(),
        ty: type_ident.unwrap(),
        ident: field.ident.clone().unwrap(),
    };

    Some(db_field)
}

#[proc_macro_derive(MiniORM)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

    let fields = match data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => panic!("Uhhh wat"),
    };

    let db_model = ORMModel {
        name: ident.to_string().to_lowercase(),
        fields: fields.iter().filter_map(get_db_field).collect(),
    };

    let db_name = db_model.name.clone();
    let db_fields: Vec<String> = db_model.fields.iter().map(|f| f.name.to_string()).collect();
    let db_columns = db_fields.join(",");

    let column_idents: Vec<&Ident> = db_model.fields.iter().map(|f| &f.ident).collect();

    let result = quote! {
        impl #ident {
            pub fn fields() -> ::std::vec::Vec<&'static str> {
                vec![#(#db_fields,)*]
            }

            pub async fn select(pool: &::sqlx::sqlite::SqlitePool, id: i32) -> ::core::result::Result<#ident, ::sqlx::error::Error> {
                let select_string = format!(
                    r#"SELECT {} FROM {} WHERE ID = $1;"#,
                    #db_columns, #db_name
                );

                ::sqlx::query_as(select_string.as_str())
                    .bind(id)
                    .fetch_one(pool)
                    .await
            }

            pub async fn insert(&self, pool: &::sqlx::sqlite::SqlitePool) -> ::core::result::Result<::sqlx::sqlite::SqliteQueryResult, ::sqlx::error::Error> {
                let insert_fields = #ident::fields()
                    .iter()
                    .enumerate()
                    .map(|(idx, _)| format!("${}", idx+1))
                    .collect::<Vec<String>>()
                    .join(",");

                let insert_string = format!(
                    r#"INSERT INTO {} ({}) VALUES ({});"#,
                    #db_name, #db_columns, insert_fields
                );

                ::sqlx::query(insert_string.as_str())
                    #(.bind(&self.#column_idents))*
                    .execute(pool)
                    .await
            }

            pub async fn update(&self, pool: &::sqlx::sqlite::SqlitePool) -> ::core::result::Result<::sqlx::sqlite::SqliteQueryResult, ::sqlx::error::Error> {
                let update_fields = #ident::fields()
                    .iter()
                    .filter(|f| **f != "id")
                    .enumerate()
                    .map(|(idx, f)| format!("{} = ${}", f, idx+2))
                    .collect::<Vec<String>>()
                    .join(", ");

                let update_string = format!(
                    r#"UPDATE {} SET {} WHERE id = $1;"#,
                    #db_name, update_fields
                );

                ::sqlx::query(update_string.as_str())
                    #(.bind(&self.#column_idents))*
                    .execute(pool)
                    .await
            }

            pub async fn delete(pool: &::sqlx::sqlite::SqlitePool, id: i32) -> ::core::result::Result<sqlx::sqlite::SqliteQueryResult, ::sqlx::error::Error> {
                let query = format!(
                    r#"DELETE FROM {} WHERE ID = $1;"#,
                    #db_name
                );

                ::sqlx::query(query.as_str())
                    .bind(id)
                    .execute(pool)
                    .await
            }
        }
    };

    result.into()
}
