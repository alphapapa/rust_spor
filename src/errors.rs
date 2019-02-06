use std::fmt;
use std::error::Error as StdErr;
use std::path::PathBuf;

use failure::{Backtrace, Context, Fail};

// Common result type
pub type Result<T> = std::result::Result<T, Error>;

// Common error type
#[derive(Debug)]
pub struct Error {
    ctx: Context<ErrorKind>,
}

impl Error {
    // Create a RepositoryNotFound error
    pub(crate) fn repository_not_found(p: PathBuf) -> Error {
        Error::from(ErrorKind::RepositoryNotFound(p))
    }

    // Create a RepositoryExists error
    pub(crate) fn repository_exists(p: PathBuf) -> Error {
        Error::from(ErrorKind::RepositoryExists(p))
    }

    // Create error from IOError
    pub(crate) fn io(err: &std::io::Error) -> Error {
        Error::from(ErrorKind::Io(err.kind(), 
                    err.description().to_string()))
    }
    // Create an "other" error
    pub fn other<T: AsRef<str>>(msg: T) -> Error {
        Error::from(ErrorKind::Other(msg.as_ref().to_owned()))
    }

    // Get the ErrorKind for an Error
    pub fn find(&self) -> &ErrorKind {
        self.ctx.get_context()
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.ctx.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.ctx.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.ctx.fmt(f)
    }
}

// The various kinds of errors in the system
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    // No alignments could be found
    NoAlignments,

    // An alignment doesn't match the text
    InvalidAlignment,

    // A repository can't be found
    RepositoryNotFound(PathBuf),

    // A repository already exists
    RepositoryExists(PathBuf),

    // IO error has occurred
    Io(std::io::ErrorKind, String),

    // Some other kind of error has occurred
    Other(String),

    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::NoAlignments => {
                write!(f, "No alignment found")
            }
            ErrorKind::InvalidAlignment => {
                write!(f, "Alignment does not map to source")
            }
            ErrorKind::RepositoryNotFound(ref p) => {
                write!(f, "repository not found for {:?}", p)
            }
            ErrorKind::RepositoryExists(ref p) => {
                write!(f, "repository already exists: {:?}", p)
            }
            ErrorKind::Other(ref msg) => {
                write!(f, "{}", msg)
            }
            ErrorKind::Io(ref kind, ref description) => {
                write!(f, "I/O error: {:?} {}", kind, description)
            }
            ErrorKind::__Nonexhaustive => panic!("invalid error"),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error::from(Context::new(kind))
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(ctx: Context<ErrorKind>) ->Error {
        Error { ctx }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::io(&err)
    }
}