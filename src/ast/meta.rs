use std::fmt;

#[derive(Clone, Debug)]
pub struct Meta {
    pub title: Option<Title>,
    pub subtitle: Option<Subtitle>,
    pub date_time: Option<DateTime>,
    pub tags: Tags,
    pub authors: Authors,
}

impl Meta {
    pub fn new() -> Meta {
        Meta{
            title: None,
            subtitle: None,
            date_time: None,
            tags: Tags::new(),
            authors: Authors::new(),
        }
    }

    pub fn push_author(&mut self, author: Author) {
        self.authors.push(author);
    }
}

#[derive(Clone, Debug)]
pub struct Title(pub String);

#[derive(Clone, Debug)]
pub struct Subtitle(pub String);

#[derive(Clone, Debug)]
pub struct DateTime(pub String);

#[derive(Clone, Debug)]
pub struct Tags(pub Vec<String>);

impl Tags {
    fn new() -> Tags {
        Tags(Vec::new())
    }
}

#[derive(Clone, Debug)]
pub struct Authors(pub Vec<Author>);

impl Authors {
    fn new() -> Authors {
        Authors(Vec::new())
    }

    fn push(&mut self, a: Author) {
        self.0.push(a);
    }
}

#[derive(Debug)]
pub enum MetaToken {
    Title(Title),
    Subtitle(Subtitle),
    DateTime(DateTime),
    Tags(Tags),
    Authors(Authors),
    NewLine,
}

impl fmt::Display for MetaToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &MetaToken::Title(_) => write!(f, "title"),
            &MetaToken::Subtitle(_) => write!(f, "subtitle"),
            &MetaToken::DateTime(_) => write!(f, "datetime"),
            &MetaToken::Tags(_) => write!(f, "tag1, tag2, tag3"),
            &MetaToken::Authors(_) => write!(f, "author block"),
            &MetaToken::NewLine => write!(f, "new line"),
        }
    }
}

// Author struct holds information about an individual author
#[derive(Clone, Debug)]
pub struct Author {
    pub name: Option<String>,
    pub job: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
}

impl Author {
    pub fn new() -> Author {
        Author {
            name: None,
            job: None,
            email: None,
            website: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AuthorToken {
    Name,
    Job,
    Email,
    Website,
    Blank,
}

impl fmt::Display for AuthorToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &AuthorToken::Name => write!(f, "author name"),
            &AuthorToken::Job => write!(f, "author job"),
            &AuthorToken::Email => write!(f, "author email"),
            &AuthorToken::Website => write!(f, "author website"),
            &AuthorToken::Blank => write!(f, "blank line"),
        }
    }
}


