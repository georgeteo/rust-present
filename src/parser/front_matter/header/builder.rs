use parser::front_matter::header::header;
use parser::front_matter::header::Header;
use parser::front_matter::author;

// Header Builder
pub struct Builder {
    header: Header,
    author_builder: author::Builder,
}

impl Builder {
    // parse takes input and line_num and builds header struct
    // TODO: Return Result<(), Error> to inform caller of possibility of error.
    pub fn parse(& mut self, input: String, line_num: usize) {
        let mut maybe_author = Ok(None);
        match line_num {
            0 => self.header.title = Some(header::Title(input)),
            1 => self.header.subtitle = Some(header::Subtitle(input)),
            2 => self.header.date_time = Some(header::DateTime(input)),
            3 => self.header.tags = header::Tags(input.split(",").map(|x| x.to_string()).collect()),
            4 => {if input == "" {
                      self.author_builder.reset();
                  } else {
                      print!("Line {}: expected new line, got {}\n", line_num, input);
                  }},
            _ => {maybe_author = self.author_builder.parse(input, line_num);},
        }

        match maybe_author {
            Ok(Some(author)) => self.header.authors.push(author),
            Err(s) => { print!("Line {}: {}\n", line_num, s); },
            _ => {},
        }
    }

    pub fn end(&self, input: &String) -> bool {
        for (i, letter) in input.chars().enumerate() {
            if i == 3 {
                return true;
            }


            if letter != '+' {
                break;
            }

        }
        false
    }

    pub fn build(&self) -> Header {
        self.header.clone()
    }
}

impl Builder {
    pub fn new() -> Builder {
        Builder{
            header: Header::new(),
            author_builder: author::Builder::new(),
        }
    }
}

