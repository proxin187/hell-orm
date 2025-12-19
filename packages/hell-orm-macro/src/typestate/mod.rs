use syn::punctuated::{Iter, Punctuated};
use syn::{Token, Ident, Field};
use quote::{quote, format_ident, ToTokens};


pub struct TypestateStructs<'a> {
    fields: &'a Punctuated<Field, Token![,]>,
    model: &'a Ident,
}

impl<'a> TypestateStructs<'a> {
    pub fn new(fields: &'a Punctuated<Field, Token![,]>, model: &'a Ident) -> TypestateStructs<'a> {
        TypestateStructs {
            fields,
            model,
        }
    }
}

impl<'a> ToTokens for TypestateStructs<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // TODO: here we are only filtering the primary keys, we should also filter optional fields
        for field in self.fields.iter().filter(|field| field.attrs.iter().all(|attr| !attr.path().is_ident("primary_key"))) {
            let ident = format_ident!("__{}Has{}", self.model, field.ident.as_ref().expect("expected a named field"));

            tokens.extend(quote! {
                pub struct #ident<T>(::std::marker::PhantomData<T>);
            });
        }
    }
}

pub struct TypestateCheck<'a> {
    fields: &'a Punctuated<Field, Token![,]>,
    model: &'a Ident,
}

impl<'a> TypestateCheck<'a> {
    pub fn new(fields: &'a Punctuated<Field, Token![,]>, model: &'a Ident) -> TypestateCheck<'a> {
        TypestateCheck {
            fields,
            model,
        }
    }
}

impl<'a> ToTokens for TypestateCheck<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut typestate_check = quote! { () };

        // TODO: here we are only filtering the primary keys, we should also filter optional fields
        for field in self.fields.iter().filter(|field| field.attrs.iter().all(|attr| !attr.path().is_ident("primary_key"))) {
            let ident = format_ident!("__{}Has{}", self.model, field.ident.as_ref().expect("expected a named field"));

            typestate_check = quote! { #ident<#typestate_check> };
        }

        tokens.extend(typestate_check);
    }
}


