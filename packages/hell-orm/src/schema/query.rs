use crate::schema::Model;

use rusqlite::Connection;


pub enum Where<T> {
    Equal(T),
    NotEqual(T),
    LessThan(T),
    GreaterThan(T),
}

pub trait Query<'a, T> {
    type Builder;

    fn builder(connection: &'a mut Connection) -> Self::Builder;
}

pub struct QueryBuilder<'a> {
    pub connection: &'a mut Connection,
    pub table_name: &'a str,
}

impl<'a> QueryBuilder<'a> {
    pub fn new(connection: &'a mut Connection, table_name: &'a str) -> QueryBuilder<'a> {
        QueryBuilder {
            connection,
            table_name,
        }
    }

    // TODO: we need a get() and all() function for retrieving single values and multiple values
    //fn get<T: Model<'a>>(self) -> &'a T {
    //}
}


