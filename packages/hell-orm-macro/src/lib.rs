mod model;

use model::{Column, ColumnFields};

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Token, DeriveInput, Data, Fields, Type};
use syn::punctuated::Punctuated;


#[proc_macro_derive(Model, attributes(table_name, primary_key, unique, auto_increment))]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            let column = Column::new(input.attrs, input.ident);
            let column_fields = ColumnFields::new(fields.named.iter());

            let ident = column.ident();
            let table_name = column.table_name();

            return TokenStream::from(quote! {
                impl ::hell_orm::model::Model for #ident {
                    const NAME: &'static str = #table_name;

                    const COLUMNS: &'static [(&'static str, &'static str)] = &[
                        #column_fields
                    ];
                }
            });
        }
    }

    TokenStream::from(syn::Error::new(input.ident.span(), "Model can only be derived for structs with named fields").to_compile_error())
}

#[proc_macro_derive(Schema, attributes(models))]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;

    if let Data::Struct(data) = input.data {
        if data.fields.is_empty() {
            let Some(attribute) = input.attrs.iter().find(|attribute| attribute.path().is_ident("models")) else {
                return TokenStream::from(syn::Error::new(input.ident.span(), "Schema derive requires #[models(...)] attribute").to_compile_error());
            };

            let Ok(models): Result<Punctuated<Type, Token![,]>, _> = attribute.parse_args_with(Punctuated::parse_terminated) else {
                return TokenStream::from(syn::Error::new(input.ident.span(), "Failed to parse models from attribute").to_compile_error());
            };

            let schema_has = models.iter().map(|model| quote! {
                impl ::hell_orm::model::SchemaHas<#model> for #ident {}
            });

            let schema_tuple = models.iter().rev().fold(quote! {()}, |acc, model| quote! { (#model, #acc) });

            return TokenStream::from(quote! {
                impl ::hell_orm::model::Schema for #ident {
                    fn create(connection: &mut ::hell_orm::__macro_export::rusqlite::Connection) -> Result<(), ::hell_orm::error::Error> {
                        <#schema_tuple as ::hell_orm::model::Schema>::create(connection)
                    }
                }

                #(#schema_has)*
            });
        }
    }

    TokenStream::from(syn::Error::new(input.ident.span(), "Model can only be derived for structs with zero fields").to_compile_error())
}


