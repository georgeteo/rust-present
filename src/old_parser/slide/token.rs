use std::fmt;
use parser::slide::error::TokenError;

// Token for AST
#[derive(Debug, Clone)]
pub enum Token {
    Slide(Tokens),
    H1(Tokens),
    H2(Tokens),
    H3(Tokens),
    H4(Tokens),
    P(Tokens),
    OL(Tokens),
    OLEntry(Tokens),
    UL(Tokens),
    ULEntry(Tokens),
    Link(Pair),
    Image(Pair),
    PathEntry(String), // Value
    DescriptionEntry(String), // Value
    Text(String), // Value
    Bold(String), // Value
    Italics(String), // Value
}

#[derive(Debug, Clone)]
pub struct Pair(Option<Box<Token>>, Option<Box<Token>>);

impl Pair {
    fn new() -> Pair {
        Pair(None, None)
    }

    fn push(& mut self, t: Token) {
        match self {
            & mut Pair(None, _) => { self.0 = Some(Box::new(t)); },
            & mut Pair(_, None) => { self.1 = Some(Box::new(t)); },
            _ => {/* TODO: Error */ },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tokens(Vec<Token>);

impl Tokens {
    pub fn new() -> Tokens {
        Tokens(Vec::new())
    }

    fn push(& mut self, t: Token) {
        self.0.push(t);
    }
}

pub enum MaybeToken {
    Recursive(Token),
    NotRecursive(Token),
    NoToken
}

impl Token {
    pub fn new(parent: &Token, lookback: &str, input: char, line_num: usize) -> Result<MaybeToken, TokenError> {
        match (parent, lookback, input) {
            (&Token::Slide(_), "#", '#') | (&Token::Slide(_), "##", '#') | (&Token::Slide(_), "###", '#') =>
                Ok(MaybeToken::NoToken),
            (&Token::Slide(_), "#", ' ') =>
                Ok(MaybeToken::NotRecursive(Token::H1(Tokens::new()))),
            (&Token::Slide(_), "##", ' ') =>
                Ok(MaybeToken::NotRecursive(Token::H2(Tokens::new()))),
            (&Token::Slide(_), "###", ' ') =>
                Ok(MaybeToken::NotRecursive(Token::H3(Tokens::new()))),
            (&Token::Slide(_), "####", ' ') =>
                Ok(MaybeToken::NotRecursive(Token::H4(Tokens::new()))),
            // TODO: ("#", _) | ("##", _) | ("###", _) | ("####", _) => Err("Expected ' ' or '#'"),
            (&Token::Slide(_), "", _) =>
                Ok(MaybeToken::Recursive(Token::P(Tokens::new()))),
            (&Token::Slide(_), "-", ' ') =>
                Ok(MaybeToken::Recursive(Token::UL(Tokens::new()))),
            (&Token::UL(_), "-", ' ') =>
                Ok(MaybeToken::NotRecursive(Token::ULEntry(Tokens::new()))),
            // TODO: OL and OLEntry
            (&Token::Slide(_), "[", _) =>
                Ok(MaybeToken::Recursive(Token::Link(Pair::new()))),
            (&Token::Link(_), "[", _) =>
                Ok(MaybeToken::Recursive(Token::DescriptionEntry(String::new()))),
            (&Token::Slide(_), "![", _) =>
                Ok(MaybeToken::Recursive(Token::Image(Pair::new()))),
            (&Token::Image(_), "![", _) =>
                Ok(MaybeToken::Recursive(Token::DescriptionEntry(String::new()))),
            (&Token::Link(_), "(", _) | (&Token::Image(_), "(", _) =>
                Ok(MaybeToken::Recursive(Token::PathEntry(String::new()))),
            // TODO: Fix (()) bug.
            (&Token::Text(_), _, _) | (&Token::Bold(_), _, _) | (&Token::Italics(_), _, _) |
                (&Token::DescriptionEntry(_), _, _) | (&Token::PathEntry(_), _, _) =>
                Ok(MaybeToken::NoToken),
            (_, "*", '*') =>
                Ok(MaybeToken::NotRecursive(Token::Bold(String::new()))),
            (_, "*", _) =>
                Ok(MaybeToken::Recursive(Token::Italics(String::new()))),
            (_, _, _) =>
                Ok(MaybeToken::Recursive(Token::Text(String::new()))),
        }
    }

    pub fn new_slide() -> Token {
        Token::Slide(Tokens::new())
    }

    pub fn push(&mut self, token: Token) {
        match self {
            & mut Token::Link(ref mut p) | & mut Token::Image(ref mut p) => p.push(token),
            & mut Token::Slide(ref mut v) | & mut Token::H1(ref mut v) | & mut Token::H2(ref mut v) | & mut Token::H3(ref mut v)
            | & mut Token::H4(ref mut v) | & mut Token::P(ref mut v) | & mut Token::OL(ref mut v) | & mut Token::OLEntry(ref mut v)
            | & mut Token::UL(ref mut v) | & mut Token::ULEntry(ref mut v)  => v.push(token),
            _ => panic!("Error: Should not be calling Token::push on {:?}", self),
        }
    }

    pub fn end(& self, lookback: &str, c: char) -> bool {
        match (self, lookback, c) {
            (&Token::OLEntry(_), _, '\n') | (&Token::ULEntry(_), _, '\n') => true,
            (&Token::PathEntry(_), _, ']') => true,
            (&Token::DescriptionEntry(_), _, ')') => true,
            (&Token::Bold(_), "*", '*') => true,
            (&Token::Italics(_), _, '_') => true,
            (_, "\n", '\n') => true,
            (_, _, _) => false,
        }
    }


}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Slide(_)  => write!(f, "new slide token"),
            Token::H1(_) => write!(f, "#"),
            Token::H2(_) => write!(f, "##"),
            Token::H3(_) => write!(f, "###"),
            Token::H4(_) => write!(f, "####"),
            Token::P(_) => write!(f, "new paragraph"),
            Token::OL(_) => write!(f, "1."),
            Token::OLEntry(_) => write!(f, "2."),
            Token::UL(_) => write!(f, "-"),
            Token::ULEntry(_) => write!(f, "-"),
            Token::Link(_) => write!(f, "[description](url) e.g., [Google](www.google.com)"),
            Token::Image(_) => write!(f, "![description](path)"),
            Token::PathEntry(_) => write!(f, "[Text]"),
            Token::DescriptionEntry(_) => write!(f, "(Text)"),
            Token::Text(_) => write!(f, "Text"),
            Token::Bold(_) => write!(f, "**Text**"),
            Token::Italics(_) => write!(f, "_Text_"),

        }
    }
}
