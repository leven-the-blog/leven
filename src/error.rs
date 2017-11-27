use git2;
use std::{fmt, io, result};
use tenjin;
use toml;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ConfigNotFound,
    TemplateNotFound(String),

    Io(io::Error),
    Git2(git2::Error),
    Toml(toml::de::Error),
    Tenjin(tenjin::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<git2::Error> for Error {
    fn from(e: git2::Error) -> Error {
        Error::Git2(e)
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Error {
        Error::Toml(e)
    }
}

impl From<tenjin::Error> for Error {
    fn from(e: tenjin::Error) -> Error {
        Error::Tenjin(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match *self {
            ConfigNotFound => write!(f, "no config found"),
            TemplateNotFound(ref s) => write!(f, "template `{}` not found", s),

            Io(ref e) => e.fmt(f),
            Git2(ref e) => e.fmt(f),
            Toml(ref e) => e.fmt(f),
            Tenjin(ref e) => e.fmt(f),
        }
    }
}
