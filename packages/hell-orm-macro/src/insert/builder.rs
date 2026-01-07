use super::typestate::TypestateCheck;
use super::field::ModelField;

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

pub struct BuilderStructInit<'a> {
    pub fields: &'a Punctuated<Field, Token![,]>,
}

impl<'a> BuilderStructInit<'a> {
    pub fn new(fields: &'a Punctuated<Field, Token![,]>) -> BuilderStructInit<'a> {
        BuilderStructInit {
            fields,
        }
    }
}

impl<'a> ToTokens for BuilderStructInit<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for field in self.fields.iter() {
            let ident = &field.ident;

            tokens.extend(quote! { #ident: None, });
        }
    }
}

pub struct BuilderStructFunctions<'a> {
    fields: Vec<ModelField<'a>>,
    builder_ident: &'a Ident,
    model_ident: &'a Ident,
}

impl<'a> BuilderStructFunctions<'a> {
    pub fn new(fields: &'a Punctuated<Field, Token![,]>, builder_ident: &'a Ident, model_ident: &'a Ident) -> BuilderStructFunctions<'a> {
        BuilderStructFunctions {
            fields: fields.iter().map(|field| ModelField::new(field)).collect(),
            builder_ident,
            model_ident,
        }
    }
}

impl<'a> ToTokens for BuilderStructFunctions<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for field in self.fields.iter() {
            let ident = field.ident();
            let type_ = field.type_();
            let builder_ident = &self.builder_ident;
            let typestate_ident = format_ident!("__{}Has{}", self.model_ident, field.ident().as_ref().expect("expected a named field"));
            let builder_struct_update_fields = BuilderStructUpdateFields::new(&self.fields, field.ident());

            if field.is_optional() {
                tokens.extend(quote! {
                    pub fn #ident(self, #ident: #type_) -> #builder_ident<'a, T> {
                        #builder_ident {
                            builder: self.builder,
                            #builder_struct_update_fields
                        }
                    }
                });
            } else {
                tokens.extend(quote! {
                    pub fn #ident(self, #ident: #type_) -> #builder_ident<'a, #typestate_ident<T>> {
                        #builder_ident {
                            builder: ::hell_orm::schema::insert::InsertBuilder::new(self.builder.connection, self.builder.table_name, #typestate_ident(::std::marker::PhantomData)),
                            #builder_struct_update_fields
                        }
                    }
                });
            }
        }
    }
}

pub struct BuilderStructUpdateFields<'a> {
    fields: &'a [ModelField<'a>],
    update: &'a Option<Ident>,
}

impl<'a> BuilderStructUpdateFields<'a> {
    pub fn new(fields: &'a [ModelField<'a>], update: &'a Option<Ident>) -> BuilderStructUpdateFields<'a> {
        BuilderStructUpdateFields {
            fields,
            update,
        }
    }
}

impl<'a> ToTokens for BuilderStructUpdateFields<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for field in self.fields.iter() {
            let ident = field.ident();

            if field.ident() == self.update {
                tokens.extend(quote! { #ident: Some(#ident), });
            } else {
                tokens.extend(quote! { #ident: self.#ident, });
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
        let finish_params = BuilderFinishParams::new(&self.fields);

        tokens.extend(quote! {
            impl<'a> #builder_ident<'a, #typestate_check> {
                pub fn finish(self) -> ::std::result::Result<(), ::hell_orm::error::Error> {
                    let mut columns: ::std::vec::Vec<&str> = ::std::vec::Vec::new();
                    let mut params: ::std::vec::Vec<&dyn ::hell_orm::__macro_export::rusqlite::ToSql> = ::std::vec::Vec::new();
                    #finish_params
                    self.builder.finish(columns.as_slice(), params.as_slice())?;
                    Ok(())
                }
            }
        });
    }
}

pub struct BuilderFinishParams<'a> {
    fields: Vec<ModelField<'a>>,
}

impl<'a> BuilderFinishParams<'a> {
    pub fn new(fields: &'a Punctuated<Field, Token![,]>) -> BuilderFinishParams<'a> {
        BuilderFinishParams {
            fields: fields.iter().map(|field| ModelField::new(field)).collect(),
        }
    }
}

impl<'a> ToTokens for BuilderFinishParams<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for field in self.fields.iter() {
            let ident = field.ident();
            let column = field.ident().as_ref().map(|ident| ident.to_string());

            tokens.extend(quote! {
                if let Some(value) = &self.#ident {
                    columns.push(#column);
                    params.push(value as &dyn ::hell_orm::__macro_export::rusqlite::ToSql);
                }
            });
        }
    }
}


