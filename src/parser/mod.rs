mod header;
mod slide;
mod error;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use parser::error::ParseError;

// AST is the internal representation for a slide.
// It consists of a header (front matter) and a AST for the slide content.
#[derive(Debug, Clone)]
pub struct AST {
    header: header::Header,
    content: Vec<slide::Token>,
}

// Builder for the AST
#[derive(Debug)]
pub struct Builder {
    ast: AST,
    errors: Vec<ParseError>,
}

// parse takes an open file descriptor f to a .slide file and parses it
// into an AST
pub fn parse(f: File) -> Result<AST, Vec<ParseError>> {
    let mut builder = Builder::new();
    let mut reader = BufReader::new(f);
    builder.parse_header(reader.by_ref());
    builder.parse_content(reader);
    builder.build()
}

impl AST {
    // AST constructor
    fn new() -> AST {
        AST {
            header: header::Header::new(),
            content: Vec::new(),
        }
    }
}

impl Builder {
    // AST Builder constructor
    fn new() -> Builder {
        Builder {
            ast: AST::new(),
            errors: Vec::new(),
        }
    }

    // parse_header borrows a BufReader and consumes the header portion.
    fn parse_header<R: Read>(& mut self, reader: & mut BufReader<R>) {
        let mut builder = header::Builder::new();
        for (line_num, line) in reader.lines().enumerate() {
            match line {
                Ok(line) => {
                    if builder.end(&line) {
                        break;
                    }
                    if let Err(e) = builder.parse(line, line_num) {
                        self.errors.push(From::from(e));
                    }
                },
                Err(e) => self.errors.push(From::from(e)),
            }
        }
        self.ast.header = builder.build();
    }

    // parse_content moves a BufReader (previously consumed by parse_header
    // and parses the remaining lines into a slide AST.
    fn parse_content<R: Read>(& mut self, reader: BufReader<R>) {
        let mut builder = slide::Builder::new(slide::Token::new_slide());
        for (line_num, line) in reader.lines().enumerate() {
            match line {
                Ok(line) => {
                    for letter in line.chars() {
                        match builder.parse(false, letter, line_num) {
                            Ok(Some(slide)) => {
                                self.ast.content.push(slide);
                                builder = slide::Builder::new(slide::Token::new_slide());
                            },
                            Ok(None) => {},
                            Err(err) => self.errors.push(From::from(err)),
                        }
                    }
                },
                Err(err) => self.errors.push(From::from(err)),
            }
        }
    }

    fn build(self) -> Result<AST, Vec<ParseError>> {
        if self.errors.len() == 0 {
            return Ok(self.ast.clone());
        }
        Err(self.errors)
    }
}

