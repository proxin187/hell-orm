mod insert;
mod query;
mod model;

use insert::Insert;
use query::Query;
use model::Model;

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Token, DeriveInput, Data, Fields, Type, Meta, Expr, Lit, Attribute};


#[inline]
fn table_name(attrs: &[Attribute]) -> Option<String> {
    attrs.iter()
        .filter_map(|attribute| {
            if let Meta::NameValue(value) = &attribute.meta && let Expr::Lit(literal) = &value.value && let Lit::Str(string) = &literal.lit && attribute.path().is_ident("table_name") {
                Some(string.value())
            } else {
                None
            }
        })
        .next()
}

#[proc_macro_derive(Model, attributes(table_name, primary_key, unique, auto_increment))]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let Some(table_name) = table_name(&input.attrs) else {
        return TokenStream::from(syn::Error::new(input.ident.span(), "You must provide a table_name attribute").to_compile_error());
    };

    if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            let ident = &input.ident;

            let insert = Insert::new(&input, &fields, &table_name);
            let query = Query::new(&input, &fields);
            let model = Model::new(&fields.named, &input.attrs, &table_name);

            return TokenStream::from(quote! {
                #insert

                #query

                impl ::hell_orm::schema::Model for #ident {
                    #model
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
                impl ::hell_orm::schema::SchemaHas<#model> for #ident {}
            });

            let schema_tuple = models.iter().rev().fold(quote! {()}, |acc, model| quote! { (#model, #acc) });

            return TokenStream::from(quote! {
                impl ::hell_orm::schema::Schema for #ident {
                    fn create(connection: &mut ::hell_orm::__macro_export::rusqlite::Connection) -> Result<(), ::hell_orm::error::Error> {
                        <#schema_tuple as ::hell_orm::schema::Schema>::create(connection)
                    }
                }

                #(#schema_has)*
            });
        }
    }

    TokenStream::from(syn::Error::new(input.ident.span(), "Model can only be derived for structs with zero fields").to_compile_error())
}


