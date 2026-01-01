//! Core traits for defining database models and schemas.

pub mod insert;
pub mod query;

use crate::error::Error;

use insert::Insert;

use rusqlite::{Connection, Params};


pub trait Model<'a, T>: Insert<'a, T> {}

impl<'a, T: Insert<'a, T>> Model<'a, T> for T {}

pub trait SchemaHas<'a, Row: Model<'a, Row>> {}

pub trait Schema {
    fn create(connection: &mut Connection) -> Result<(), Error>;
}

impl Schema for () {
    fn create(_connection: &mut Connection) -> Result<(), Error> {
        Ok(())
    }
}

impl<Head: for <'a> Model<'a, Head>, Tail: Schema> Schema for (Head, Tail) {
    fn create(connection: &mut Connection) -> Result<(), Error> {
        /*
        let columns = Head::COLUMNS.iter()
            .map(|(name, type_)| format!("{} {}", name, type_))
            .collect::<Vec<_>>()
            .join(", ");

        connection
            .execute(&format!("CREATE TABLE IF NOT EXISTS {}({})", Head::NAME, columns), [])
            .map_err(|err| Error::SchemaError(Box::new(err)))?;

        Tail::create(connection)
        */

        Ok(())
    }
}


