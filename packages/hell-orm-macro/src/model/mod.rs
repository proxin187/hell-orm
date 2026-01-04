use syn::punctuated::Punctuated;
use syn::{Token, Field, Attribute, Type, Meta, Expr, Lit, PathArguments, GenericArgument};
use quote::{quote, ToTokens};


pub struct Model<'a> {
    fields: &'a Punctuated<Field, Token![,]>,
    attributes: Vec<Attribute>,
}

impl<'a> Model<'a> {
    pub fn new(fields: &'a Punctuated<Field, Token![,]>, attributes: Vec<Attribute>) -> Model<'a> {
        Model {
            fields,
            attributes,
        }
    }

    pub fn table_name(&self) -> Option<String> {
        self.attributes.iter()
            .filter_map(|attribute| {
                if let Meta::NameValue(value) = &attribute.meta && let Expr::Lit(literal) = &value.value && let Lit::Str(string) = &literal.lit && attribute.path().is_ident("table_name") {
                    Some(string.value())
                } else {
                    None
                }
            })
            .next()
    }

    pub fn columns(&self) -> impl Iterator<Item = proc_macro2::TokenStream> {
        self.fields.iter()
            .map(|field| {
                let name = field.ident.as_ref().map(|ident| ident.to_string());
                let sqlite_type = FieldType::new(&field).sqlite_type();

                quote! {
                    (#name, #sqlite_type)
                }
            })
    }
}

impl<'a> ToTokens for Model<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let table_name = self.table_name();
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

    fn raw_type(&self) -> &'a str {
        match self.inner_option_type().to_token_stream().to_string().as_str() {
            "String" => "TEXT",
            "u8" | "u16" | "u32" | "u64"| "u128" | "usize" | "i8" | "i16" | "i32" | "i64"| "i128" | "isize" => "INTEGER",
            // TODO: make it error here
            _ => "TEXT",
        }
    }

    fn sqlite_type(&self) -> String {
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

        format!("{}{}", self.raw_type(), attributes)
    }
}


