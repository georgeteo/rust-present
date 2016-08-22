use ast::error::ParseError;
use ast::Slides;
use ast::header::Builder as HeaderBuilder;
use ast::slide::Builder as SlideBuilder;

use std::fs::File;
use std::io::{BufReader, Read};

// Builder for the AST
#[derive(Debug)]
pub struct Builder {
    ast: Slides,
    errors: Vec<ParseError>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            ast: Slides::new(),
            content: Vec::new(),
        }
    }

    pub fn parse(f: File) -> Result<Slides, Vec<ParseError>> {
        let mut builder = Builder::new();
        let mut reader = BufReader::new(f);
        builder.parse_header(reader.by_ref());
        builder.parse_content(reader);
        builder.build()
    }

    // parse_header borrows a BufReader and consumes the header portion.
    fn parse_header<R: Read>(& mut self, reader: & mut BufReader<R>) {
        let mut builder = HeaderBuilder::new();
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
        let mut builder = SlideBuilder::new();
        for (line_num, line) in reader.lines().enumerate() {
            match line {
                Ok(line) => {
                    for letter in line.chars() {
                        match builder.parse(false, letter, line_num) {
                            Ok(Some(slide)) => {
                                self.ast.content.push(slide);
                                builder = SlideBuilder::new();
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

    fn build(self) -> Result<Slides, Vec<ParseError>> {
        if self.errors.len() == 0 {
            return Ok(self.ast.clone());
        }
        Err(self.errors)
    }
}

