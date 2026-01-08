mod builder;
mod field;

use syn::{DeriveInput, FieldsNamed};
use quote::{quote, ToTokens};


pub struct Query<'a> {
    input: &'a DeriveInput,
    fields: &'a FieldsNamed,
}

impl<'a> Query<'a> {
    pub fn new(input: &'a DeriveInput, fields: &'a FieldsNamed) -> Query<'a> {
        Query {
            input,
            fields,
        }
    }
}

impl<'a> ToTokens for Query<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.input.ident;

        tokens.extend(quote! {
            //impl ::hell_orm::schema::query::Query for #ident {
            //}
        });
    }
}


