//! Core traits for defining database models and schemas.

use crate::error::Error;

use rusqlite::{Connection, Params};

/// A trait representing a database table model.
///
/// Types implementing this trait correspond to database tables and define
/// the table name and column structure. This should be derived using
/// the `#[derive(Model)]` macro.
///
/// # Example
///
/// ```rust
/// use hell_orm::prelude::*;
///
/// #[derive(Model)]
/// #[table_name = "users"]
/// struct User {
///     #[primary_key]
///     id: usize,
///
///     #[unique]
///     email: String,
///
///     name: String,
///     bio: Option<String>,
/// }
/// ```
///
/// This generates SQL like:
/// ```sql
/// CREATE TABLE IF NOT EXISTS users(
///     id INTEGER NOT NULL PRIMARY KEY,
///     email TEXT NOT NULL UNIQUE,
///     name TEXT NOT NULL,
///     bio TEXT
/// )
/// ```
pub trait Model {
    /// The name of the database table.
    const NAME: &'static str;

    /// The columns of the table as (name, type) pairs.
    ///
    /// Each tuple contains the column name and a SQLite type string with
    /// constraints (e.g., `"INTEGER NOT NULL PRIMARY KEY"`, `"TEXT UNIQUE"`).
    const COLUMNS: &'static [(&'static str, &'static str)];

    fn params(&self) -> impl Params;
}

/// A marker trait indicating that a schema contains a specific model type.
///
/// This trait is used to enforce at compile time that operations
/// are only performed on tables that exist in the schema. It is automatically
/// implemented by the `#[derive(Schema)]` macro for each model type listed
/// in the `#[models(...)]` attribute.
///
/// You should never need to implement this trait manually.
pub trait SchemaHas<Row: Model> {}

/// A trait representing a complete database schema.
///
/// A schema defines the set of tables in a database. This should be derived
/// using the `#[derive(Schema)]` macro on an empty struct with a `#[models(...)]`
/// attribute listing all the model types.
///
/// # Example
///
/// ```rust
/// use hell_orm::prelude::*;
///
/// #[derive(Model)]
/// struct User {
///     id: usize,
///     name: String,
/// }
///
/// #[derive(Model)]
/// struct Post {
///     id: usize,
///     user_id: usize,
///     content: String,
/// }
///
/// #[derive(Schema)]
/// #[models(User, Post)]
/// struct MySchema;
/// ```
pub trait Schema {
    /// Creates all tables in the schema if they don't already exist.
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


