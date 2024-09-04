use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data::Struct, DataStruct, DeriveInput, Fields::Named, FieldsNamed};

#[proc_macro_derive(Iterable)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

    let fields = match data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => panic!("Uhhh wat"),
    };

    let field_pairs = fields.iter().map(|field| {
        let field_ident = &field.ident;
        let field_name = field_ident.as_ref().unwrap().to_string();
        quote! {
            (#field_name, &(self.#field_ident) as &dyn std::any::Any)
        }
    });

    let expanded = quote! {
        impl Iterable for #ident {
            fn iter<'a>(&'a self) -> std::vec::IntoIter<(&'static str, &'a dyn std::any::Any)> {
                vec![#(#field_pairs),*].into_iter()
            }
        }
    };

    TokenStream::from(expanded)
}
