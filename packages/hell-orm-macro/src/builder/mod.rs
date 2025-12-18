use syn::punctuated::Punctuated;
use syn::{Token, Ident, Field, Type};
use quote::{quote, format_ident, ToTokens};


pub struct BuilderStructFields<'a> {
    fields: &'a Punctuated<Field, Token![,]>,
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


