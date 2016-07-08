mod header;
mod slide;

use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader, Read};


// AST is the internal representation for a slide.
// It consists of a header (front matter) and a AST for the slide content.
#[derive(Debug)]
pub struct AST {
    header: header::Header,
    content: Vec<slide::Token>,
}

// parse takes an open file descriptor f to a .slide file and parses it
// into an AST
pub fn parse(f: File) -> Result<AST, Box<Error>> {
    let mut ast = AST::new();
    let mut reader = BufReader::new(f);
    try!(ast.parse_header(reader.by_ref()));
    try!(ast.parse_content(reader));
    Ok(ast)
}

impl AST {
    // AST constructor
    fn new() -> AST {
        AST{
            header: header::Header::new(),
            content: Vec::new(),
        }
    }
    // parse_header takes an open file descriptor f and parses it line by line,
    // generating the header (front matter) and returning the partially
    // consumed file descriptor.
    fn parse_header<R: Read>(& mut self, reader: & mut BufReader<R>) -> Result<(), Box<Error>> {
        let mut builder = header::Builder::new();
        for (line_num, line) in reader.lines().enumerate() {
            let line = try!(line);
            if builder.end(&line) {
                break;
            }
            builder.parse(line, line_num);
        }
        self.header = builder.build();
        Ok(())
    }

    // parse_content takes an open file descriptor and parses it characterwise into
    // an AST for the slides.
    // TODO: add support for line numbers when parsing.
    fn parse_content<R: Read>(& mut self, reader: BufReader<R>) -> Result<(), Box<Error>>{
        let mut builder = slide::Builder::new(slide::Token::new_slide());
        for (line_num, line) in reader.lines().enumerate() {
            let line = try!(line);
            for letter in line.chars() {
                if let Some(slide) = builder.parse(false, letter, line_num) {
                    self.content.push(slide);
                    builder = slide::Builder::new(slide::Token::new_slide());
                }
            }

        }
        Ok(())
    }
}

