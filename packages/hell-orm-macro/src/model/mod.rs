use syn::{Ident, Field, Attribute, Meta, Expr, Lit};


pub struct Model<'a, T: Iterator<Item = &'a Field>> {
    fields: T,
    attributes: Vec<Attribute>,
    ident: Ident,
}

impl<'a, T: Iterator<Item = &'a Field>> Model<'a, T> {
    pub fn new(fields: T, attributes: Vec<Attribute>, ident: Ident) -> Model<'a, T> {
        Model {
            fields,
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


