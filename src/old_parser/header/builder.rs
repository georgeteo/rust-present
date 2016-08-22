use super::header;
use super::header::{Header, HeaderType};
use super::header::author;
use super::error::{HeaderError, TokenError};

// Header Builder
pub struct Builder {
    header: Header,
    author_builder: author::Builder,
}

impl Builder {
    // parse takes input and line_num and builds header struct
    // TODO: Return Result<(), Error> to inform caller of possibility of error.
    pub fn parse(& mut self, input: String, line_num: usize) -> Result<(), HeaderError> {
        let mut maybe_author = Ok(None);
        match line_num {
            0 => self.header.title = Some(header::Title(input)),
            1 => self.header.subtitle = Some(header::Subtitle(input)),
            2 => self.header.date_time = Some(header::DateTime(input)),
            3 => self.header.tags = header::Tags(input.split(",").map(|x| x.to_string()).collect()),
            4 => {if input == "" {
                      self.author_builder.reset();
                  } else {
                      return Err(From::from(TokenError::new(line_num, input, HeaderType::NewLine)));
                  }},
            _ => {maybe_author = self.author_builder.parse(input, line_num);},
        }

        match maybe_author {
            Ok(Some(author)) => self.header.authors.push(author),
            Err(err) => {return Err(From::from(err));},
            _ => {},
        }
        Ok(())
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

