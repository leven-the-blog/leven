use handlebars;
use glob;
use std::{fmt, io, result};

pub type Result = result::Result<(), Error>;

pub enum Error {
    Render(handlebars::RenderError),
    Template(handlebars::TemplateFileError),
    Glob(glob::GlobError),
    Pattern(glob::PatternError),
    Io(io::Error),
    Other(&'static str),
}

//TODO: Better error messages.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::Render(ref e) => write!(f, "Render: {}.", e),
            &Error::Template(ref e) => write!(f, "Template: {}.", e),
            &Error::Glob(ref e) => write!(f, "Glob: {}.", e),
            &Error::Pattern(ref e) => write!(f, "Pattern: {}.", e),
            &Error::Io(ref e) => write!(f, "IO: {}.", e),
            &Error::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<handlebars::RenderError> for Error {
    fn from(e: handlebars::RenderError) -> Error {
        Error::Render(e)
    }
}

impl From<handlebars::TemplateFileError> for Error {
    fn from(e: handlebars::TemplateFileError) -> Error {
        Error::Template(e)
    }
}

impl From<glob::GlobError> for Error {
    fn from(e: glob::GlobError) -> Error {
        Error::Glob(e)
    }
}

impl From<glob::PatternError> for Error {
    fn from(e: glob::PatternError) -> Error {
        Error::Pattern(e)
    }
}
