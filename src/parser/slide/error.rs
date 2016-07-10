use parser::slide::token::Token;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct TokenError {
    expected: Token,
    got: Token,
    line_num: usize,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: expected {} - got {}", self.line_num, self.expected, self.got)
    }
}

impl Error for TokenError {
    fn description(&self) -> &str {
        "Token Error"
    }
}

#[derive(Debug)]
pub enum SlideError {
    TokenError(TokenError),
}

impl fmt::Display for SlideError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SlideError::TokenError(ref err) => write!(f, "Slide Error: {}", err),
        }
    }
}

impl Error for SlideError {
    fn description(&self) -> &str {
        match *self {
            SlideError::TokenError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            SlideError::TokenError(ref err) => Some(err),
        }
    }
}

impl From<TokenError> for SlideError {
    fn from(err: TokenError) -> SlideError {
        SlideError::TokenError(err)
    }
}


