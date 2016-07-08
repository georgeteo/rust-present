// Author struct holds information about an individual author
#[derive(Clone, Debug)]
pub struct Author {
    pub name: Option<String>,
    pub job: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
}

