mod meta;
mod content;
mod error;

use ast::AST;
use self::error::ParseError;
use self::meta::MetaBuilder;
use self::content::ContentBuilder;

use std::fs::File;
use std::io::{BufReader, BufRead, Read, Lines};
use std::iter::{Enumerate, Peekable};
use std::error::Error;

pub type ParseReader<B: BufRead> = Peekable<Enumerate<Lines<B>>>;

pub trait Parser {
    type P : Parser; // Parser type
    type E : Error; // Error type
    type O; // Completed object type

    fn new() -> Self::P;
    fn parse<B: BufRead>(mut self, mut reader: ParseReader<B>) -> ParseResult<B, Self::O, Self::E>;
}

pub struct ParseResult<B: BufRead, O, E: Error>(pub ParseReader<B>, pub Result<O, Vec<E>>);

// Builder for the AST
#[derive(Debug)]
pub struct Builder {
    ast: AST,
    meta_builder: meta::MetaBuilder,
    content_builder: content::ContentBuilder,
    errors: Vec<ParseError>,
}

impl Parser for Builder {
    type P = Builder;
    type E = ParseError;
    type O = AST;

    fn new() -> Self::P {
        Builder {
            ast: AST::new(),
            meta_builder: MetaBuilder::new(),
            content_builder: ContentBuilder::new(),
            errors: Vec::new(),
        }
    }

    fn parse<B: BufRead>(mut self, reader: ParseReader<B>) -> ParseResult<B, Self::O, Self::E> {
        // Parse first part of reader stream as meta information.
        let ParseResult(reader, maybe_meta) = self.meta_builder.parse(reader);
        match maybe_meta {
            Ok(meta) => self.ast.update_meta(meta),
            Err(err) => { return ParseResult(reader,
                                             Err(err.into_iter().map(|x| From::from(x)).collect())
                                             ); },
        }

        // Parse rest of reader stream as content
        let ParseResult(reader, maybe_content) = self.content_builder.parse(reader);
        match maybe_content {
            Ok(content) => { self.ast.content = content; },
            Err(err) => { return ParseResult(reader,
                                             Err(err.into_iter().map(|x| From::from(x)).collect())
                                             ); },
        }

        ParseResult(reader, Ok(self.ast.clone()))
    }
}
