use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<error::Error>>;

pub fn from_str<T>(message: &str) -> Result<T> {
    Err(Error::new(message).into())
}

#[derive(Debug, Clone)]
pub struct Error {
    message: String
}

impl Error {
    pub fn new(message: &str) -> Error {
        Error { message: String::from(message) }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
