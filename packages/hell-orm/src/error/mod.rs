//! Error types for database operations.

/// Errors that can occur during database operations.
#[derive(Debug)]
pub enum Error {
    /// An error occurred while opening the database file.
    OpenError(Box<dyn std::error::Error>),

    /// An error occurred while creating or modifying the database schema.
    SchemaError(Box<dyn std::error::Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Error::OpenError(error) => f.write_fmt(format_args!("failed to open: {}", error)),
            Error::SchemaError(error) => f.write_fmt(format_args!("failed to create schema: {}", error)),
        }
    }
}

impl std::error::Error for Error {}


