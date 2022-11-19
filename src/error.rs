use std::{fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

pub type Error = Box<ErrorKind>;

#[derive(Debug)]
pub enum ErrorKind {
    Io(io::Error),
    SerToml(toml::ser::Error),
    DeToml(toml::de::Error),
    NoConfigPath,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(err) => writeln!(f, "io error: {}", err),
            Self::SerToml(err) => writeln!(f, "toml serialize error: {}", err),
            Self::DeToml(err) => writeln!(f, "toml deserialize error: {}", err),
            Self::NoConfigPath => writeln!(f, "Unable to determine config path!"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: io::Error) -> Self {
        ErrorKind::Io(err).into()
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        ErrorKind::SerToml(err).into()
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        ErrorKind::DeToml(err).into()
    }
}
