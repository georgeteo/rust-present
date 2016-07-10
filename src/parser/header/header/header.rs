use parser::header::author::author::Author;
use std::fmt;

// Header struct contains metadata
#[derive(Clone, Debug)]
pub struct Header {
    pub title: Option<Title>,
    pub subtitle: Option<Subtitle>,
    pub date_time: Option<DateTime>,
    pub tags: Tags,
    pub authors: Authors,
}

#[derive(Clone, Debug)]
pub struct Title(pub String);

#[derive(Clone, Debug)]
pub struct Subtitle(pub String);

#[derive(Clone, Debug)]
pub struct DateTime(pub String);

#[derive(Clone, Debug)]
pub struct Tags(pub Vec<String>);

#[derive(Clone, Debug)]
pub struct Authors(pub Vec<Author>);

#[derive(Debug)]
pub enum HeaderType {
    Title(Title),
    Subtitle(Subtitle),
    DateTime(DateTime),
    Tags(Tags),
    Authors(Authors),
    NewLine,
}

impl fmt::Display for HeaderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &HeaderType::Title(_) => write!(f, "title"),
            &HeaderType::Subtitle(_) => write!(f, "subtitle"),
            &HeaderType::DateTime(_) => write!(f, "datetime"),
            &HeaderType::Tags(_) => write!(f, "tag1, tag2, tag3"),
            &HeaderType::Authors(_) => write!(f, "author block"),
            &HeaderType::NewLine => write!(f, "new line"),
        }
    }

}

impl Header {
    pub fn new() -> Header {
        Header{
            title: None,
            subtitle: None,
            date_time: None,
            tags: Tags::new(),
            authors: Authors::new(),
        }
    }
}

impl Tags {
    fn new() -> Tags {
        Tags(Vec::new())
    }
}

impl Authors {
    fn new() -> Authors {
        Authors(Vec::new())
    }

    pub fn push(& mut self, a: Author){
        let & mut Authors(ref mut authors) = self;
        authors.push(a);
    }
}
