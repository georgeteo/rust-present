use super::{Parser, ParseResult, ParseReader};
use super::error::{MetaError, AuthorError, AuthorContentError, LineError};
use ast::{Meta, MetaToken, Author, AuthorToken};
use ast::meta::{Title, Subtitle, DateTime, Tags};

use std::io;
use std::io::{BufRead, BufReader, Read};

// Meta Builder
#[derive(Debug)]
pub struct MetaBuilder {
    token: Meta,
    author_builder: AuthorBuilder,
    errors: Vec<MetaError>,
}

impl Parser for MetaBuilder {
    type P = MetaBuilder;
    type E = MetaError;
    type O = Meta;

    fn new() -> Self::P {
        MetaBuilder {
            token: Meta::new(),
            author_builder: AuthorBuilder::new(),
            errors: Vec::new(),
        }
    }

    fn parse<B: BufRead>(mut self, mut reader: ParseReader<B>) -> ParseResult<B, Self::O, Self::E> {
        // Parse first meta lines (e.g., Title, Subtitle, Tags, etc)
        for (line_num, input) in reader.by_ref() {
            // If end of meta information, break.
            match input {
                Err(err) => self.errors.push(From::from(err)),
                Ok(input) => {
                    match MetaBuilder::end_lines(&input, line_num) {
                        Ok(true) => break,
                        Err(err) => {
                            self.errors.push(From::from(err));
                            break;
                        },
                        _ => {},
                    }

                    // Else, parse line as header
                    if let Err(e) = self.parse_line(input, line_num) {
                        // If errors, add to errors vec.
                        self.errors.push(e);
                    }
                }
            }
        }

        // Parse input as authors until +++ or reader is entirely consumed
        loop {
            self.author_builder = AuthorBuilder::new();
            // Check to see if next line is "+++..." or no more reader to consume.
            // Then end of meta.
            if MetaBuilder::end_meta(reader.peek()) {
                let _ = reader.next();
                break;
            }

            // Else, parse as Author
            let ParseResult(new_reader, maybe_author) = self.author_builder.parse(reader);
            reader = new_reader;
            match maybe_author {
                Err(errs) => {
                    for e in errs {
                        self.errors.push(From::from(e));
                    }
                },
                Ok(a) => {
                    self.token.push_author(a);
                },
            }
        }

        // Return Ok(token) if errors is empty,
        // else return errors.
        if self.errors.len() == 0 {
            // If no errors, return Meta.
            ParseResult(reader, Ok(self.token.clone()))
        } else {
            // Else, there are errors.
            ParseResult(reader, Err(self.errors))
        }
    }
}

impl MetaBuilder {
    // Returns true if it recognizes the expected blank line in line 4.
    // Format:
    // Title
    // Subtitle
    // DateTime
    // Tags
    // Blank <---- here
    fn end_lines(input: &String, line_num: usize) -> Result<bool, LineError> {
        if *input == "" && line_num == 4 {
            return Ok(true);
        }
        if line_num == 4 {
            return Err(LineError::new(line_num, *input, MetaToken::NewLine));
        }
       Ok(false)
    }

    fn end_meta(input: Option<&(usize, io::Result<String>)>) -> bool {
        unimplemented!();
        //match input {
        //    Some(line) => {
        //        // Loop over letters and check that the first 3 is +++
        //        for (i, letter) in input.chars().enumerate() {
        //            if i == 3 {
        //                return true;
        //            }
        //            if letter != '+' {
        //                break;
        //            }
        //        }
        //        return false;
        //    },
        //    None => true,
        //}
    }

    fn parse_line(&mut self, input: String, line_num: usize) -> Result<(), MetaError> {
        match line_num {
            0 => self.token.title = Some(Title(input)),
            1 => self.token.subtitle = Some(Subtitle(input)),
            2 => self.token.date_time = Some(DateTime(input)),
            3 => self.token.tags = Tags(input.split(",").map(|x| x.to_string()).collect()),
            _ => return Err(From::from(LineError::new(4, input, MetaToken::NewLine))),
        }
        Ok(())
    }
}

// AuthorBuilder is a builder for an author
#[derive(Debug)]
pub struct AuthorBuilder {
    author: Author,
    next_field: AuthorToken,
    errors: Vec<AuthorError>,
}

impl Parser for AuthorBuilder {
    type P = AuthorBuilder;
    type E = AuthorError;
    type O = Author;

    fn new() -> Self::P {
        AuthorBuilder{
            author: Author::new(),
            next_field: AuthorToken::Name,
            errors: Vec::new(),
        }
    }

    fn parse<B: BufRead>(mut self, mut reader: ParseReader<B>) ->  ParseResult<B, Self::O, Self::E> {
        for (line_num, input) in reader.by_ref() {
            match input {
                Err(e) => self.errors.push(From::from(e)),
                Ok(input) => {
                    match self.next_field {
                        AuthorToken::Name => {
                            self.author.name = Some(input);
                            self.next_field = AuthorToken::Job;
                        },
                        AuthorToken::Job => {
                            self.author.job  = Some(input);
                            self.next_field = AuthorToken::Email;
                        },
                        AuthorToken::Email => {
                            self.author.email = Some(input);
                            self.next_field = AuthorToken::Website;
                        },
                        AuthorToken::Website => {
                            self.author.website = Some(input);
                            self.next_field = AuthorToken::Blank;
                        },
                        AuthorToken::Blank => {
                            if input == "" {
                                break;
                            } else {
                                self.errors.push(From::from(AuthorContentError::new(line_num, input, AuthorToken::Blank)));
                                break;
                            }
                        }
                    }
                },
            }
        }

        // If there are errors, return errors.
        if self.errors.len() != 0 {
            return ParseResult(reader, Err(self.errors));
        }
        // Else, return authors
        let new_author = self.author.clone();
        self.reset();
        ParseResult(reader, Ok(new_author))
    }
}

impl AuthorBuilder {
    fn reset(&mut self) {
        self.author.name = None;
        self.author.job = None;
        self.author.email = None;
        self.author.website = None;
        self.next_field = AuthorToken::Name;
    }
}
