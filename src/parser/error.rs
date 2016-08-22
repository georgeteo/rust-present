use std::error::Error;
use std::fmt;
use std::io;
use std::iter::{FromIterator, IntoIterator};

use ast::{ContentToken, MetaToken, AuthorToken}; // TODO (George): Rename stuff, make consistent

#[derive(Debug)]
pub enum ParseError {
    MetaError(MetaError),
    ContentError(ContentError),
    IOError(io::Error),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::MetaError(ref err) => write!(f, "ParseError: {}", err),
            ParseError::ContentError(ref err) => write!(f, "ParseError: {}", err),
            ParseError::IOError(ref err) => write!(f, "ParseError: {}", err),
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::MetaError(ref err) => err.description(),
            ParseError::ContentError(ref err) => err.description(),
            ParseError::IOError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ParseError::MetaError(ref err) => Some(err),
            ParseError::ContentError(ref err) => Some(err),
            ParseError::IOError(ref err) => Some(err),
        }
    }
}

impl From<MetaError> for ParseError {
    fn from(err: MetaError) -> ParseError {
        ParseError::MetaError(err)
    }
}

impl From<ContentError> for ParseError {
    fn from(err: ContentError) -> ParseError {
        ParseError::ContentError(err)
    }
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> ParseError {
        ParseError::IOError(err)
    }
}

#[derive(Debug)]
pub struct ContentError {
    expected: ContentToken,
    got: ContentToken,
    line_num: usize,
}

impl fmt::Display for ContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: expected {} - got {}", self.line_num, self.expected, self.got)
    }
}

impl Error for ContentError {
    fn description(&self) -> &str {
        "Slide Error"
    }
}

#[derive(Debug)]
pub enum MetaError {
    LineError(LineError),
    AuthorError(AuthorError),
    IOError(io::Error),
}

impl fmt::Display for MetaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self{
            MetaError::LineError(ref err) => write!(f, "LineError: {}", err),
            MetaError::AuthorError(ref err) => write!(f, "AuthorError: {}", err),
            MetaError::IOError(ref err) => write!(f, "IOError: {}", err),
        }
    }
}

impl Error for MetaError {
    fn description(&self) -> &str {
        match *self{
            MetaError::LineError(ref err) => err.description(),
            MetaError::AuthorError(ref err) => err.description(),
            MetaError::IOError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self{
            MetaError::LineError(ref err) => Some(err),
            MetaError::AuthorError(ref err) => Some(err),
            MetaError::IOError(ref err) => Some(err),
        }
    }
}

impl From<LineError> for MetaError {
    fn from(err: LineError) -> MetaError {
        MetaError::LineError(err)
    }
}

impl From<AuthorError> for MetaError {
    fn from(err: AuthorError) -> MetaError {
        MetaError::AuthorError(err)
    }
}

impl From<io::Error> for MetaError {
    fn from(err: io::Error) -> MetaError {
        MetaError::IOError(err)
    }
}

#[derive(Debug)]
pub struct LineError {
    line_num: usize,
    input: String,
    expected: MetaToken,
}

impl fmt::Display for LineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line {}: Expected {} - Got {}\n", self.line_num, self.expected, self.input)
    }
}

impl Error for LineError {
    fn description(&self) -> &str {
        "Meta Line Error"
    }
}

impl LineError {
    pub fn new(line_num: usize, input: String, expected: MetaToken) -> LineError {
        LineError {
            line_num: line_num,
            input: input,
            expected: expected,
        }
    }
}

#[derive(Debug)]
pub enum AuthorError {
    AuthorContentError(AuthorContentError),
    IOError(io::Error)
}

impl fmt::Display for AuthorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self{
            AuthorError::AuthorContentError(ref err) => write!(f, "AuthorError: {}", err),
            AuthorError::IOError(ref err) => write!(f, "AuthorError: {}", err),
        }
    }
}

impl Error for AuthorError {
    fn description(&self) -> &str {
        match *self{
            AuthorError::AuthorContentError(ref err) => err.description(),
            AuthorError::IOError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self{
            AuthorError::AuthorContentError(ref err) => Some(err),
            AuthorError::IOError(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for AuthorError {
    fn from(err: io::Error) -> AuthorError {
        AuthorError::IOError(err)
    }
}

impl From<AuthorContentError> for AuthorError {
    fn from(err: AuthorContentError) -> AuthorError {
        AuthorError::AuthorContentError(err)
    }
}

#[derive(Debug)]
pub struct AuthorContentError {
    line_num: usize,
    input: String,
    expected: AuthorToken,
}

impl fmt::Display for AuthorContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line {}: Expected {} - Got {}\n", self.line_num, self.expected, self.input)
    }
}

impl Error for AuthorContentError {
    fn description(&self) -> &str {
        "Author Error"
    }
}

impl AuthorContentError {
    pub fn new(line_num: usize, input: String, expected: AuthorToken) -> AuthorContentError {
        AuthorContentError {
            line_num: line_num,
            input: input,
            expected: expected,
        }
    }
}
