pub mod model;
pub mod error;

use crate::model::Schema;
use crate::error::Error;

use rusqlite::Connection;

use std::path::Path;


pub struct Database {
    connection: Connection,
}

impl Database {
    /// Opens a database and creates any schemas that dont exist in it.
    pub fn open<T: Schema>(path: impl AsRef<Path>) -> Result<Database, Error> {
        let mut connection = Connection::open(path).map_err(|err| Error::OpenError(Box::new(err)))?;

        T::create(&mut connection)?;

        Ok(Database {
            connection,
        })
    }
}

pub mod prelude {
    pub use crate::{schema, Database};

    pub use hell_orm_macro::Model;
}


