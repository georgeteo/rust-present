use std::fs::File;
use std::io::BufReader;

extern crate present;
use present::parser::{Parser, Builder, ParseResult};

fn main() {
    // Filename
    // TODO: hardcoded for now.
    // Change to parse from CLI.
    let path = "foo.txt";

    // Open file
    let reader;
    match File::open(path) {
        Ok(f) => {reader = BufReader::new(f);},
        Err(e) => { print!("Error: {}", e);
                    return () },
    }

    // Parse
    let mut parser = Builder::new();
    let ast;
    match parser.parse(reader) {
        ParseResult(_, Ok(a)) => {ast = a;},
        ParseResult(_, Err(errs)) => {
            for e in errs {
                print!("{}", e);
            }
            return ();
        }
    }
}
