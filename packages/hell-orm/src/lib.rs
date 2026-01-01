pub mod schema;
pub mod error;

use crate::schema::insert::Insert;
use crate::schema::{Model, Schema, SchemaHas};
use crate::error::Error;

use rusqlite::Connection;

use std::path::Path;
use std::marker::PhantomData;

pub struct Database<T: Schema> {
    connection: Connection,
    _marker: PhantomData<T>,
}

impl<T: Schema> Database<T> {
    pub fn open(path: impl AsRef<Path>) -> Result<Database<T>, Error> {
        let mut connection = Connection::open(path).map_err(|err| Error::OpenError(Box::new(err)))?;

        T::create(&mut connection)?;

        Ok(Database {
            connection,
            _marker: PhantomData,
        })
    }

    pub fn insert<'a, Row: Model<'a, Row>>(&'a mut self) -> <Row as Insert<'a, Row>>::Builder
    where
        T: SchemaHas<'a, Row>
    {
        <Row as Insert<'a, Row>>::builder(&mut self.connection)
    }

    /*
    pub fn query<Row: Model>(&self)
    where
        T: SchemaHas<Row>
    {
    }
    */
}

/// Commonly used types for convenient importing.
pub mod prelude {
    pub use crate::Database;

    pub use hell_orm_macro::{Schema, Model};
}

#[doc(hidden)]
pub mod __macro_export {
    pub use rusqlite;
}


