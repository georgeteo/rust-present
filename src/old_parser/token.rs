use ast::header::Header;
use ast::slide::Token;

// AST is the internal representation for a slide.
// It consists of a header (front matter) and a AST for the slide content.
#[derive(Debug, Clone)]
pub struct Slides {
    header: Header,
    content: Vec<Token>,
}

impl Slides {
    pub fn new() -> Slides {
        Slides {
            header: Header::new(),
            content: Vec::new(),
        }
    }
}
