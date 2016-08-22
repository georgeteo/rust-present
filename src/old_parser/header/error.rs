use super::header::HeaderType;
use super::header::author::AuthorFields;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum HeaderError {
    LineError(LineError),
    AuthorError(AuthorError),
}

impl fmt::Display for HeaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self{
            HeaderError::LineError(ref err) => write!(f, "LineError: {}", err),
            HeaderError::AuthorError(ref err) => write!(f, "AuthorError: {}", err),
        }
    }
}

impl Error for HeaderError {
    fn description(&self) -> &str {
        match *self{
            HeaderError::LineError(ref err) => err.description(),
            HeaderError::AuthorError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self{
            HeaderError::LineError(ref err) => Some(err),
            HeaderError::AuthorError(ref err) => Some(err),
        }
    }
}

impl From<LineError> for HeaderError {
    fn from(err: LineError) -> HeaderError {
        HeaderError::LineError(err)
    }
}

impl From<AuthorError> for HeaderError {
    fn from(err: AuthorError) -> Error {
        HeaderError::AuthorError(err)
    }
}

#[derive(Debug)]
pub struct LineError {
    line_num: usize,
    input: String,
    expected: HeaderType,
}

impl fmt::Display for LineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line {}: Expected {} - Got {}\n", self.line_num, self.expected, self.input)
    }
}

impl Error for LineError {
    fn description(&self) -> &str {
        "Token Error"
    }
}

impl LineError {
    pub fn new(line_num: usize, input: String, expected: HeaderType) -> LineError {
        LineError {
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

impl Error for AuthorError {
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
