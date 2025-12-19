use crate::typestate::TypestateCheck;

use syn::punctuated::Punctuated;
use syn::{Token, Ident, Field, Type};
use quote::{quote, format_ident, ToTokens};


pub struct BuilderStructFields<'a> {
    pub fields: &'a Punctuated<Field, Token![,]>,
}

impl<'a> BuilderStructFields<'a> {
    pub fn new(fields: &'a Punctuated<Field, Token![,]>) -> BuilderStructFields<'a> {
        BuilderStructFields {
            fields,
        }
    }
}

impl<'a> ToTokens for BuilderStructFields<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for field in self.fields.iter() {
            let ident = &field.ident;
            let ty = &field.ty;

            if let Type::Path(path) = &field.ty && path.path.segments.last().map(|last| last.ident == "Option").unwrap_or(false) {
                tokens.extend(quote! { #ident: #ty, });
            } else {
                tokens.extend(quote! { #ident: ::std::option::Option<#ty>, });
            }
        }
    }
}

pub struct BuilderStructFinish<'a> {
    fields: &'a Punctuated<Field, Token![,]>,
    builder_ident: &'a Ident,
    model: &'a Ident,
}

impl<'a> BuilderStructFinish<'a> {
    pub fn new(fields: &'a Punctuated<Field, Token![,]>, builder_ident: &'a Ident, model: &'a Ident) -> BuilderStructFinish<'a> {
        BuilderStructFinish {
            fields,
            builder_ident,
            model,
        }
    }
}

impl<'a> ToTokens for BuilderStructFinish<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let builder_ident = &self.builder_ident;

        let typestate_check = TypestateCheck::new(&self.fields, &self.model);

        tokens.extend(quote! {
            impl<'a> #builder_ident<'a, #typestate_check> {
                fn finish(self) -> Result<(), ::hell_orm::error::Error> {
                    self.builder.finish([], ())?;

                    Ok(())
                }
            }
        });
    }
}

pub struct BuilderFinishParams<'a> {
    fields: &'a Punctuated<Field, Token![,]>,
}

impl<'a> BuilderFinishParams<'a> {
    pub fn new(fields: &'a Punctuated<Field, Token![,]>) -> BuilderFinishParams<'a> {
        BuilderFinishParams {
            fields,
        }
    }
}

// TODO: here we can generate the arguments to the builder.finish function
impl<'a> ToTokens for BuilderFinishParams<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    }
}


