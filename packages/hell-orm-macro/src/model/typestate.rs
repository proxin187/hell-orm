use syn::{Ident, Type, Field, PathArguments, GenericArgument, Attribute, Meta, Expr, Lit};
use quote::{quote, format_ident, ToTokens};



// TODO: finish typestate, then do the builder
pub struct Typestate<'a> {
    fields: &'a [Field],
}

impl<'a> Typestate<'a> {
    pub fn new(fields: &'a [Field]) -> Typestate<'a> {
        Typestate {
            fields,
        }
    }
}

impl<'a> ToTokens for Typestate<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    }
}


