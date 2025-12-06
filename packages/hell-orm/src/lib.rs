//! A lightweight, type-safe SQLite ORM for Rust.
//!
//! This library provides a simple way to define database schemas and models
//! using Rust structs with derive macros. The ORM automatically generates
//! table creation SQL and provides compile-time verification that your
//! database operations match your schema definitions.
//!
//! # Example
//!
//! ```rust
//! use hell_orm::prelude::*;
//!
//! #[derive(Model)]
//! #[table_name = "users"]
//! struct User {
//!     #[primary_key]
//!     id: i32,
//!
//!     #[unique]
//!     email: String,
//!
//!     name: String,
//!     bio: Option<String>,  // Optional fields become nullable columns
//! }
//!
//! #[derive(Schema)]
//! #[models(User)]
//! struct MySchema;
//!
//! let mut db = Database::<MySchema>::open("my_database.db")?;
//! db.insert(User {
//!     id: 1,
//!     email: "alice@example.com".to_string(),
//!     name: "Alice".to_string(),
//!     bio: None,
//! });
//! ```

pub mod model;
pub mod error;

use crate::model::{Model, Schema, SchemaHas};
use crate::error::Error;

use rusqlite::Connection;

use std::path::Path;
use std::marker::PhantomData;

/// A type-safe database connection parameterized by a schema type.
///
/// The `Database` type ensures at compile time that all operations performed
/// on the database are valid for the given schema `T`. This prevents runtime
/// errors from attempting to insert rows into tables that don't exist in your schema.
pub struct Database<T: Schema> {
    connection: Connection,
    _marker: PhantomData<T>,
}

impl<T: Schema> Database<T> {
    /// Opens a database at the specified path and creates any missing tables.
    ///
    /// If the database file doesn't exist, it will be created. All tables defined
    /// in the schema will be created if they don't already exist in the database.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hell_orm::prelude::*;
    /// let db = Database::<MySchema>::open("my_database.db")?;
    /// ```
    pub fn open(path: impl AsRef<Path>) -> Result<Database<T>, Error> {
        let mut connection = Connection::open(path).map_err(|err| Error::OpenError(Box::new(err)))?;

        T::create(&mut connection)?;

        Ok(Database {
            connection,
            _marker: PhantomData,
        })
    }

    /// Inserts a row into the database.
    ///
    /// This method is only available when the schema contains the table
    /// corresponding to the `Row` type, which is enforced at compile time
    /// through the [`SchemaHas`] trait bound. This bound is automatically
    /// implemented by the `#[derive(Schema)]` macro for all models listed
    /// in the `#[models(...)]` attribute.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hell_orm::prelude::*;
    /// let mut db = Database::<MySchema>::open("app.db")?;
    ///
    /// db.insert(User {
    ///     id: 1,
    ///     name: "Alice".to_string()
    /// });
    /// ```
    pub fn insert<Row: Model>(&mut self, row: Row)
    where
        T: SchemaHas<Row>
    {
    }
}

/// Commonly used types for convenient importing.
pub mod prelude {
    pub use crate::Database;

    pub use hell_orm_macro::{Schema, Model};
}

/// Internal exports used by derive macros.
///
/// This module re-exports dependencies that the derive macros need to reference.
/// You should not need to use this module directly in your code.
#[doc(hidden)]
pub mod __macro_export {
    pub use rusqlite;
}


