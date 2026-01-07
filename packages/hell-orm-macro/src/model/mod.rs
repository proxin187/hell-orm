use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Token, Field, Attribute, Type, PathArguments, GenericArgument};
use quote::{quote, ToTokens};


pub struct Model<'a> {
    fields: &'a Punctuated<Field, Token![,]>,
    attributes: &'a [Attribute],
    table_name: &'a str,
}

impl<'a> Model<'a> {
    pub fn new(fields: &'a Punctuated<Field, Token![,]>, attributes: &'a [Attribute], table_name: &'a str) -> Model<'a> {
        Model {
            fields,
            attributes,
            table_name,
        }
    }

    pub fn columns(&self) -> impl Iterator<Item = proc_macro2::TokenStream> {
        self.fields.iter()
            .map(|field| {
                let name = field.ident.as_ref().map(|ident| ident.to_string());
                let sqlite_type = FieldType::new(&field).sqlite_type();

                match sqlite_type {
                    Ok(ty) => {
                        quote! { (#name, #ty) }
                    },
                    Err(err) => {
                        let error = err.to_compile_error();

                        quote! { #error }
                    },
                }
            })
    }
}

impl<'a> ToTokens for Model<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let table_name = &self.table_name;
        let columns = self.columns();

        tokens.extend(quote! {
            const NAME: &'static str = #table_name;

            const COLUMNS: &'static [(&'static str, &'static str)] = &[#(#columns,)*];
        });
    }
}

pub struct FieldType<'a> {
    field: &'a Field,
}

impl<'a> FieldType<'a> {
    pub fn new(field: &'a Field) -> FieldType<'a> {
        FieldType {
            field,
        }
    }

    fn inner_option_type(&self) -> &Type {
        if let Type::Path(path) = &self.field.ty {
            if let Some(segment) = path.path.segments.last().and_then(|last| (last.ident == "Option").then_some(last)) {
                if let PathArguments::AngleBracketed(arguments) = &segment.arguments {
                    if let Some(GenericArgument::Type(ty)) = arguments.args.first() {
                        return ty;
                    }
                }
            }
        }

        &self.field.ty
    }

    fn raw_type(&self) -> Result<&'a str, syn::Error> {
        let ty = self.inner_option_type().to_token_stream();

        match ty.to_string().as_str() {
            "String" => Ok("TEXT"),
            "u8" | "u16" | "u32" | "u64"| "u128" | "usize" | "i8" | "i16" | "i32" | "i64"| "i128" | "isize" => Ok("INTEGER"),
            _ => Err(syn::Error::new(ty.span(), "Invalid type in field")),
        }
    }

    fn sqlite_type(&self) -> Result<String, syn::Error> {
        let mut attributes = if let Type::Path(path) = &self.field.ty && path.path.segments.last().map(|last| last.ident == "Option").unwrap_or_default() {
            String::from(" NOT NULL")
        } else {
            String::new()
        };

        for (attribute_name, sqlite_type) in [("primary_key", " PRIMARY KEY"), ("auto_increment", " AUTOINCREMENT"), ("unique", " UNIQUE")] {
            if self.field.attrs.iter().any(|attr| attr.path().is_ident(attribute_name)) {
                attributes.extend(sqlite_type.chars());
            }
        }

        self.raw_type().map(|ty| format!("{}{}", ty, attributes))
    }
}


