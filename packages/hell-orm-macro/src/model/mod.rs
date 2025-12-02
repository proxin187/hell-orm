use syn::{Ident, Type, Field, PathArguments, GenericArgument, Attribute, Meta, Expr, Lit};
use quote::{quote, ToTokens};


pub struct ColumnField {
    ident: Option<Ident>,
    ty: Type,
}

// TODO: we have to learn how to use parse_macro_input instead, this is not a good way, we are
// essentially parsing it while generating, stupid
impl ToTokens for ColumnField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = self.ident.expect("expected ident");

        tokens.extend(quote! {
            (#ident, ),
        });
    }
}

impl ColumnField {
    pub fn new(field: Field) -> ColumnField {
        ColumnField {
            ident: field.ident,
            ty: field.ty,
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

    fn raw_type_name(&self) -> &str {
        match self.inner_option_type().into_token_stream().to_string().as_str() {
            "String" | "&str" => "TEXT",
            _ => "TEXT",
        }
    }
}

pub struct ColumnFields<'a> {
    fields: &'a [ColumnField],
}

impl<'a> ToTokens for ColumnFields<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for field in self.fields.iter() {
            let ident = field.ident.as_ref().map(|ident| ident.to_string());

            tokens.extend(quote! {
                (#ident, ),
            });
        }
    }
}

impl<'a> ColumnFields<'a> {
    pub fn new(fields: &'a [ColumnField]) -> ColumnFields<'a> {
        ColumnFields {
            fields,
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


