//! Error types for database operations.

/// Errors that can occur during database operations.
#[derive(Debug)]
pub enum Error {
    /// An error occurred while opening the database file.
    OpenError(Box<dyn std::error::Error>),

    /// An error occurred while creating or modifying the database schema.
    SchemaError(Box<dyn std::error::Error>),

    /// An error occurred while preparing a statement.
    StatementError(Box<dyn std::error::Error>),

    /// An error occurred while executing an insert.
    InsertError(Box<dyn std::error::Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Error::OpenError(error) => f.write_fmt(format_args!("failed to open: {}", error)),
            Error::SchemaError(error) => f.write_fmt(format_args!("failed to create schema: {}", error)),
            Error::StatementError(error) => f.write_fmt(format_args!("failed to prepare statement: {}", error)),
            Error::InsertError(error) => f.write_fmt(format_args!("failed to insert: {}", error)),
        }
    }
}

impl std::error::Error for Error {}


