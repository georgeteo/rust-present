use super::{Parser, ParseResult, ParseReader};
use super::error::{ContentError};
use ast::ContentToken;

use std::io::{BufRead};

#[derive(Debug)]
pub struct ContentBuilder{
}

impl Parser for ContentBuilder {
    type P = ContentBuilder;
    type E = ContentError;
    type O = Vec<ContentToken>;

    fn new() -> Self::P {
        unimplemented!();
    }

    fn parse<B: BufRead>(mut self, mut reader: ParseReader<B>) -> ParseResult<B, Self::O, Self::E> {
        unimplemented!();
    }
}
