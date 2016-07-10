use parser::header::header::HeaderType;
use parser::header::author::builder::AuthorFields;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    HeaderError(HeaderError),
    AuthorError(AuthorError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self{
            Error::HeaderError(ref err) => write!(f, "Header Error: {}", err),
            Error::AuthorError(ref err) => write!(f, "Author Error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self{
            Error::HeaderError(ref err) => err.description(),
            Error::AuthorError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self{
            Error::HeaderError(ref err) => Some(err),
            Error::AuthorError(ref err) => Some(err),
        }
    }
}

impl From<HeaderError> for Error {
    fn from(err: HeaderError) -> Error {
        Error::HeaderError(err)
    }
}

impl From<AuthorError> for Error {
    fn from(err: AuthorError) -> Error {
        Error::AuthorError(err)
    }
}

#[derive(Debug)]
pub struct HeaderError {
    line_num: usize,
    input: String,
    expected: HeaderType,
}

impl fmt::Display for HeaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line {}: Expected {} - Got {}\n", self.line_num, self.expected, self.input)
    }
}

impl error::Error for HeaderError {
    fn description(&self) -> &str {
        "Header Error"
    }
}

impl HeaderError {
    pub fn new(line_num: usize, input: String, expected: HeaderType) -> HeaderError {
        HeaderError {
            line_num: line_num,
            input: input,
            expected: expected,
        }
    }
}

#[derive(Debug)]
pub struct AuthorError {
    line_num: usize,
    input: String,
    expected: AuthorFields,
}

impl fmt::Display for AuthorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line {}: Expected {} - Got {}\n", self.line_num, self.expected, self.input)
    }
}

impl error::Error for AuthorError {
    fn description(&self) -> &str {
        "Author Error"
    }
}

impl AuthorError {
    pub fn new(line_num: usize, input: String, expected: AuthorFields) -> AuthorError {
        AuthorError {
            line_num: line_num,
            input: input,
            expected: expected,
        }
    }
}
