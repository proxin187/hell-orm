use syn::{Ident, Type, Field, PathArguments, GenericArgument, Attribute, Meta, Expr, Lit};
use quote::{quote, ToTokens};


pub struct ColumnField<'a> {
    ident: &'a Option<Ident>,
    ty: &'a Type,
    attributes: &'a [Attribute],
}

impl<'a> ToTokens for ColumnField<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = self.ident.as_ref().map(|ident| ident.to_string());
        let ty = self.type_name();

        tokens.extend(quote! {
            (#ident, #ty),
        });
    }
}

impl<'a> ColumnField<'a> {
    pub fn new(field: &'a Field) -> ColumnField<'a> {
        ColumnField {
            ident: &field.ident,
            ty: &field.ty,
            attributes: &field.attrs,
        }
    }

    fn inner_option_type(&self) -> &Type {
        if let Type::Path(path) = &self.ty {
            if let Some(segment) = path.path.segments.last().and_then(|last| (last.ident == "Option").then_some(last)) {
                if let PathArguments::AngleBracketed(arguments) = &segment.arguments {
                    if let Some(GenericArgument::Type(ty)) = arguments.args.first() {
                        return ty;
                    }
                }
            }
        }

        &self.ty
    }

    fn is_option(&self) -> bool {
        match self.ty {
            Type::Path(path) => path.path.segments.last().map(|last| last.ident == "Option").unwrap_or_default(),
            _ => false,
        }
    }

    fn raw_type(&self) -> &str {
        match self.inner_option_type().into_token_stream().to_string().as_str() {
            "String" | "&str" => "TEXT",
            "usize" => "INTEGER",
            _ => "TEXT",
        }
    }

    fn type_name(&self) -> String {
        let mut ty = String::new();

        if !self.is_option() {
            ty.push_str("NOT NULL ");
        }

        for (ident, attribute_type) in [("primary_key", "PRIMARY KEY "), ("unique", "UNIQUE ")] {
            if self.attributes.iter().any(|attribute| attribute.path().is_ident(ident)) {
                ty.push_str(attribute_type);
            }
        }

        format!("{} {}", self.raw_type(), ty)
    }
}

pub struct ColumnFields<'a> {
    fields: Vec<ColumnField<'a>>,
}

impl<'a> ToTokens for ColumnFields<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for field in self.fields.iter() {
            field.to_tokens(tokens);
        }
    }
}

impl<'a> ColumnFields<'a> {
    pub fn new(fields: impl Iterator<Item = &'a Field>) -> ColumnFields<'a> {
        ColumnFields {
            fields: fields.map(|field| ColumnField::new(field)).collect(),
        }
    }
}

pub struct Column {
    attributes: Vec<Attribute>,
    ident: Ident,
}

impl Column {
    pub fn new(attributes: Vec<Attribute>, ident: Ident) -> Column {
        Column {
            attributes,
            ident,
        }
    }

    #[inline(always)]
    pub fn ident(&self) -> Ident {
        self.ident.clone()
    }

    pub fn table_name(&self) -> String {
        for attribute in self.attributes.iter() {
            if attribute.path().is_ident("table_name") {
                if let Meta::NameValue(value) = &attribute.meta {
                    if let Expr::Lit(literal) = &value.value {
                        if let Lit::Str(string) = &literal.lit {
                            return string.value();
                        }
                    }
                }
            }
        }

        self.ident.to_string().to_lowercase()
    }
}


