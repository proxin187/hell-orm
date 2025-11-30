use crate::error::Error;

use rusqlite::Connection;


pub trait Model {
    const NAME: &'static str;
    const COLUMNS: &'static [(&'static str, &'static str)];
}

pub trait Schema {
    fn create(connection: &mut Connection) -> Result<(), Error>;
}

impl Schema for () {
    fn create(_connection: &mut Connection) -> Result<(), Error> {
        Ok(())
    }
}

impl<Head: Model, Tail: Schema> Schema for (Head, Tail) {
    fn create(connection: &mut Connection) -> Result<(), Error> {
        let columns = Head::COLUMNS.iter()
            .map(|(name, type_)| format!("{} {}", name, type_))
            .collect::<Vec<_>>()
            .join(", ");

        connection
            .execute(&format!("CREATE TABLE IF NOT EXISTS {}({})", Head::NAME, columns), [])
            .map_err(|err| Error::SchemaError(Box::new(err)))?;

        Tail::create(connection)
    }
}

#[macro_export]
macro_rules! schema {
    [] => { () };
    [$head:ty $(, $tail:ty)* $(,)?] => {
        ($head, schema![$($tail),*])
    };
}


