mod typestate;
mod builder;
mod field;

use typestate::TypestateStructs;
use builder::{BuilderStructInit, BuilderStructFields, BuilderStructFunctions, BuilderStructFinish};

use syn::{DeriveInput, FieldsNamed};
use quote::{quote, format_ident, ToTokens};


pub struct Insert<'a> {
    input: &'a DeriveInput,
    fields: &'a FieldsNamed,
    table_name: &'a str,
}

impl<'a> Insert<'a> {
    pub fn new(input: &'a DeriveInput, fields: &'a FieldsNamed, table_name: &'a str) -> Insert<'a> {
        Insert {
            input,
            fields,
            table_name,
        }
    }
}

impl<'a> ToTokens for Insert<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let builder_ident = format_ident!("__{}Builder", self.input.ident);
        let ident = &self.input.ident;
        let table_name = &self.table_name;

        let typestate_structs = TypestateStructs::new(&self.fields.named, &self.input.ident);
        let builder_struct_init = BuilderStructInit::new(&self.fields.named);
        let builder_struct_fields = BuilderStructFields::new(&self.fields.named);
        let builder_struct_functions = BuilderStructFunctions::new(&self.fields.named, &builder_ident, &self.input.ident);
        let builder_struct_finish = BuilderStructFinish::new(&self.fields.named, &builder_ident, &self.input.ident);

        tokens.extend(quote! {
            #typestate_structs

            pub struct #builder_ident<'a, T> {
                builder: ::hell_orm::schema::insert::InsertBuilder<'a, T>,

                #builder_struct_fields
            }

            impl<'a, T> #builder_ident<'a, T> {
                #builder_struct_functions
            }

            #builder_struct_finish

            impl ::hell_orm::schema::insert::Insert for #ident {
                type Builder<'a> = #builder_ident<'a, ()>;

                fn builder<'a>(connection: &'a mut ::hell_orm::__macro_export::rusqlite::Connection) -> <Self as ::hell_orm::schema::insert::Insert>::Builder<'a> {
                    #builder_ident {
                        builder: ::hell_orm::schema::insert::InsertBuilder::new(connection, #table_name, ()),

                        #builder_struct_init
                    }
                }
            }
        });
    }
}


