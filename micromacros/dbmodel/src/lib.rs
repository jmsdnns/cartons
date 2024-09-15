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

    let field_idents = fields.iter().map(|f| &f.ident);

    let result = quote! {
        impl #ident {
            pub fn fields() -> ::std::vec::Vec<&'static str> {
                vec![#(#db_fields,)*]
            }

            pub async fn select(pool: &::sqlx::sqlite::SqlitePool, id: i32) -> #ident {
                let select_string = format!(
                    r#"select {} from {} where id = $1;"#,
                    #db_columns, #db_name
                );
                ::sqlx::query_as(select_string.as_str())
                    .bind(id)
                    .fetch_one(pool)
                    .await
                    .unwrap();
            }

            pub async fn insert(&self, pool: &::sqlx::sqlite::SqlitePool) -> ::std::vec::Vec<::sqlx::sqlite::SqliteRow> {
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
                    #(.bind(&self.#field_idents))*
                    .fetch_all(pool)
                    .await
                    .unwrap()
            }
        }
    };

    result.into()
}
