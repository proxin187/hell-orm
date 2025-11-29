

#[derive(Debug)]
pub enum Error {
    OpenError(Box<dyn std::error::Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Error::OpenError(error) => f.write_fmt(format_args!("failed to open: {}", error)),
        }
    }
}

impl std::error::Error for Error {}


