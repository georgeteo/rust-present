use parser::header;
use std::error::Error;
use std::fmt;
use parser::slide::SlideError;
use std::io;

#[derive(Debug)]
pub enum ParseError {
    HeaderError(header::Error),
    SlideError(SlideError),
    IOError(io::Error),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::HeaderError(ref err) => write!(f, "ParseError: {}", err),
            ParseError::SlideError(ref err) => write!(f, "ParseError: {}", err),
            ParseError::IOError(ref err) => write!(f, "ParseError: {}", err),
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::HeaderError(ref err) => err.description(),
            ParseError::SlideError(ref err) => err.description(),
            ParseError::IOError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ParseError::HeaderError(ref err) => Some(err),
            ParseError::SlideError(ref err) => Some(err),
            ParseError::IOError(ref err) => Some(err),
        }
    }
}

impl From<header::Error> for ParseError {
    fn from(err: header::Error) -> ParseError {
        ParseError::HeaderError(err)
    }
}

impl From<SlideError> for ParseError {
    fn from(err: SlideError) -> ParseError {
        ParseError::SlideError(err)
    }
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> ParseError {
        ParseError::IOError(err)

    }

}
