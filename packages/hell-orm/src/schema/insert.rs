use crate::error::Error;

use rusqlite::{Connection, Params};


pub trait Insert<'a, T> {
    type Builder;

    fn builder(connection: &'a mut Connection) -> Self::Builder;
}

pub struct InsertBuilder<'a, T> {
    pub connection: &'a mut Connection,
    pub table_name: &'a str,
    _token: T,
}

impl<'a, T> InsertBuilder<'a, T> {
    pub fn new(connection: &'a mut Connection, table_name: &'a str, _token: T) -> InsertBuilder<'a, T> {
        InsertBuilder {
            connection,
            table_name,
            _token,
        }
    }

    pub fn finish(self, columns: &[&str], params: impl Params) -> Result<usize, Error> {
        let placeholders = (1..=columns.len())
            .map(|index| format!("?{}", index))
            .collect::<Vec<String>>()
            .join(", ");

        let sql = format!("INSERT INTO {} ({}) VALUES ({})", self.table_name, columns.join(","), placeholders);

        let mut stmt = self.connection.prepare(sql.as_str())
            .map_err(|err| Error::StatementError(Box::new(err)))?;

        stmt.execute(params)
            .map_err(|err| Error::InsertError(Box::new(err)))
    }
}





