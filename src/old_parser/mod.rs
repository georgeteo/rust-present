// Publically exposed functions
pub use self::token::Slides;
pub use self::token::Builder;

// AST top level material.
mod builder;
mod token;
mod error;

// AST parts: header, (individual) slide.
mod header;
mod slide;
