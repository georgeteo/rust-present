pub use self::meta::Meta;
pub use self::meta::Author;
pub use self::content::Token as ContentToken;
pub use self::meta::MetaToken;
pub use self::meta::AuthorToken;

pub mod meta;
mod content;

#[derive(Debug, Clone)]
pub struct AST {
    pub meta: Option<Meta>,
    pub content: Vec<ContentToken>,
}

impl AST {
    pub fn new() -> AST {
        AST {
            meta: None,
            content: Vec::new(),
        }
    }

    pub fn update_meta(&mut self, meta: Meta) {
        self.meta = Some(meta);
    }

}
