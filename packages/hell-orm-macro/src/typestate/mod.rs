use syn::punctuated::Punctuated;
use syn::{Token, Ident, Field};
use quote::{quote, format_ident, ToTokens};


pub struct Typestate<'a> {
    fields: &'a Punctuated<Field, Token![,]>,
    model: &'a Ident,
}

impl<'a> Typestate<'a> {
    pub fn new(fields: &'a Punctuated<Field, Token![,]>, model: &'a Ident) -> Typestate<'a> {
        Typestate {
            fields,
            model,
        }
    }
}

impl<'a> ToTokens for Typestate<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for field in self.fields.iter().filter(|field| field.attrs.iter().all(|attr| !attr.path().is_ident("primary_key"))) {
            let ident = format_ident!("__{}Has{}", self.model, field.ident.as_ref().expect("expected a named field"));

            tokens.extend(quote! {
                pub struct #ident<T>(::std::marker::PhantomData<T>);
            });
        }
    }
}


