use syn::{Field, Type, Ident};


pub struct ModelField<'a> {
    field: &'a Field,
}

impl<'a> ModelField<'a> {
    pub fn new(field: &'a Field) -> ModelField<'a> {
        ModelField {
            field,
        }
    }

    pub fn is_optional(&self) -> bool {
        if let Type::Path(path) = &self.field.ty && path.path.segments.last().map(|last| last.ident == "Option").unwrap_or(false) {
            true
        } else {
            self.field.attrs.iter().any(|attr| attr.path().is_ident("auto_increment"))
        }
    }

    pub fn ident(&self) -> &'a Option<Ident> { &self.field.ident }

    pub fn type_(&self) -> &'a Type { &self.field.ty }
}


