use std::fmt;

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

pub type Tokens = Vec<Token>;

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
