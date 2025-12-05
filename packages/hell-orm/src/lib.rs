pub mod model;
pub mod error;

use crate::model::{Model, Schema, SchemaHas};
use crate::error::Error;

use rusqlite::Connection;

use std::path::Path;
use std::marker::PhantomData;


pub struct Database<T: Schema> {
    connection: Connection,
    _marker: PhantomData<T>,
}

impl<T: Schema> Database<T> {
    /// Opens a database and creates any schemas that dont exist in it.
    pub fn open(path: impl AsRef<Path>) -> Result<Database<T>, Error> {
        let mut connection = Connection::open(path).map_err(|err| Error::OpenError(Box::new(err)))?;

        T::create(&mut connection)?;

        Ok(Database {
            connection,
            _marker: PhantomData,
        })
    }

    /// Insert a row into the database.
    pub fn insert<Row: Model>(&mut self, row: Row)
    where
        T: SchemaHas<Row>
    {
    }
}

/// Commonly used types.
pub mod prelude {
    pub use crate::Database;

    pub use hell_orm_macro::{Schema, Model};
}

/// Exports for derive macro.
pub mod __macro_export {
    pub use rusqlite;
}


