use parser::front_matter::author::author::Author;

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
