mod model;

use model::Model;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};


#[proc_macro_derive(Model, attributes(table_name, primary_key, unique, auto_increment))]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            let model = Model::new(fields.named.iter(), input.attrs, input.ident);

            let ident = model.ident();
            let table_name = model.table_name();

            return TokenStream::from(quote! {
                impl ::hell_orm::model::Model for #ident {
                    const NAME: &'static str = #table_name;

                    const COLUMNS: &'static [(&'static str, &'static str)] = &[
                    ];
                }
            });
        }
    }

    proc_macro::TokenStream::from(syn::Error::new(input.ident.span(), "Model can only be derived for structs with named fields").to_compile_error())
}


