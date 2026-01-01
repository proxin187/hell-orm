mod typestate;
mod builder;
mod field;

use typestate::TypestateStructs;
use builder::{BuilderStructInit, BuilderStructFields, BuilderStructFunctions, BuilderStructFinish};

use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, Token, DeriveInput, Data, Fields, Type, Meta, Expr, Lit};
use syn::punctuated::Punctuated;


#[proc_macro_derive(Model, attributes(table_name, primary_key, unique, auto_increment))]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            let builder_ident = format_ident!("__{}Builder", input.ident);
            let ident = &input.ident;

            let table_name = input.attrs.iter()
                .filter_map(|attribute| {
                    if let Meta::NameValue(value) = &attribute.meta && let Expr::Lit(literal) = &value.value && let Lit::Str(string) = &literal.lit && attribute.path().is_ident("table_name") {
                        Some(string.value())
                    } else {
                        None
                    }
                })
                .next();

            let typestate_structs = TypestateStructs::new(&fields.named, &input.ident);
            let builder_struct_init = BuilderStructInit::new(&fields.named);
            let builder_struct_fields = BuilderStructFields::new(&fields.named);
            let builder_struct_functions = BuilderStructFunctions::new(&fields.named, &builder_ident, &input.ident);
            let builder_struct_finish = BuilderStructFinish::new(&fields.named, &builder_ident, &input.ident);

            return TokenStream::from(quote! {
                #typestate_structs

                pub struct #builder_ident<'a, T> {
                    builder: ::hell_orm::schema::insert::InsertBuilder<'a, T>,

                    #builder_struct_fields
                }

                impl<'a, T> #builder_ident<'a, T> {
                    #builder_struct_functions
                }

                #builder_struct_finish

                impl<'a, T> ::hell_orm::schema::insert::Insert<'a, T> for #ident {
                    type Builder = #builder_ident<'a, ()>;

                    fn builder(connection: &'a mut ::hell_orm::__macro_export::rusqlite::Connection) -> <Self as ::hell_orm::schema::insert::Insert<'a, T>>::Builder {
                        #builder_ident {
                            builder: ::hell_orm::schema::insert::InsertBuilder::new(connection, #table_name, ()),

                            #builder_struct_init
                        }
                    }
                }

                /*
                impl ::hell_orm::model::Model for #ident {
                    const NAME: &'static str = #table_name;

                    const COLUMNS: &'static [(&'static str, &'static str)] = &[
                        #column_fields
                    ];

                    fn params(&self) -> impl ::hell_orm::__macro_export::rusqlite::Params {
                        (#(#params)*)
                    }
                }
                */
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
                impl<'a> ::hell_orm::schema::SchemaHas<'a, #model> for #ident {}
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


