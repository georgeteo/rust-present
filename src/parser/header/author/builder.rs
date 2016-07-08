use parser::header::author::author::Author;
use std;

// Builder is a builder for an author
#[derive(Clone)]
pub struct Builder {
    author: Author,
    next_field: AuthorFields,
}

#[derive(Clone)]
enum AuthorFields {
    Name,
    Job,
    Email,
    Website,
    Blank,
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

    pub fn parse(& mut self, input: String, line_num: usize) -> Result<Option<Author>, String> {
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
                          return Err(std::fmt::format(format_args!(
                                  "Line {}: Expected new line, got {}",
                                                   line_num, input)));
                      }},
        }
        Ok(None)
    }

}


