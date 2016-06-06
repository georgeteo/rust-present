use std::fs::File;

extern crate present;

fn main() {
    let maybe_f = File::open("foo.txt");
    let mut ast = None;
    match maybe_f {
        Ok(f) => ast = Some(present::parser::parse(f)),
        Err(e) => print!("Error opening file {}", e),
    }
    print!("{:?}", ast);
}
