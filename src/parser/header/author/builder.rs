use parser::header::author::author::Author;
use std;
use parser::header::error::AuthorError;

// Builder is a builder for an author
#[derive(Clone)]
pub struct Builder {
    author: Author,
    next_field: AuthorFields,
}

#[derive(Clone, Debug)]
pub enum AuthorFields {
    Name,
    Job,
    Email,
    Website,
    Blank,
}

impl std::fmt::Display for AuthorFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &AuthorFields::Name => write!(f, "author name"),
            &AuthorFields::Job => write!(f, "author job"),
            &AuthorFields::Email => write!(f, "author email"),
            &AuthorFields::Website => write!(f, "author website"),
            &AuthorFields::Blank => write!(f, "blank line"),
        }
    }
}

impl Builder {
    pub fn new() -> Builder {
        Builder{
            author: Author{
                name: None,
                job: None,
                email: None,
                website: None,
            },
            next_field: AuthorFields::Blank,
        }
    }

    pub fn reset(& mut self) {
        self.author.name = None;
        self.author.job = None;
        self.author.email = None;
        self.author.website = None;
        self.next_field = AuthorFields::Name;
    }

    pub fn parse(& mut self, input: String, line_num: usize) -> Result<Option<Author>, AuthorError> {
        match self.next_field {
            AuthorFields::Name => {self.author.name = Some(input);
                     self.next_field = AuthorFields::Job;},
            AuthorFields::Job => {self.author.job  = Some(input);
                    self.next_field = AuthorFields::Email;},
            AuthorFields::Email => {self.author.email = Some(input);
                      self.next_field = AuthorFields::Website;},
            AuthorFields::Website => {self.author.website = Some(input);
                        self.next_field = AuthorFields::Blank;},
            AuthorFields::Blank => {if input == "" {
                          let new_author = self.author.clone();
                          self.reset();
                          return Ok(Some(new_author));
                      } else {
                          return Err(AuthorError::new(line_num, input, AuthorFields::Blank));
                      }},
        }
        Ok(None)
    }

}


