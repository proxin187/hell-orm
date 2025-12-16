use crate::error::Error;

use rusqlite::{Connection, Params};


pub trait Insert {
    type Builder<'a>;

    fn builder(connection: &mut Connection) -> Self::Builder<'_>;
}

pub struct InsertBuilder<'a, T> {
    pub connection: &'a mut Connection,
    _token: T,
}

impl<'a, T> InsertBuilder<'a, T> {
    pub fn new(connection: &'a mut Connection, _token: T) -> InsertBuilder<'a, T> {
        InsertBuilder {
            connection,
            _token,
        }
    }

    pub fn finish(self, table_name: &str, column_names: &[&str], params: impl Params) -> Result<usize, Error> {
        let placeholders = (1..=column_names.len())
            .map(|index| format!("?{}", index))
            .collect::<Vec<String>>()
            .join(", ");

        let sql = format!("INSERT INTO {} ({}) VALUES ({})", table_name, column_names.join(","), placeholders);

        let mut stmt = self.connection.prepare(sql.as_str())
            .map_err(|err| Error::StatementError(Box::new(err)))?;

        stmt.execute(params)
            .map_err(|err| Error::InsertError(Box::new(err)))
    }
}





