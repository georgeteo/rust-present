use parser::slide::token::{Token, MaybeToken};

// Token Builder
pub struct Builder<Token> {
    pub stack: String,
    token: Token,
    pub parser: Option<Box<Builder<Token>>>,
}

impl Builder<Token> {
    pub fn new(token: Token) -> Builder<Token> {
        Builder{
            stack: String::new(),
            token: token,
            parser: None,
        }
    }

    // parse takes a Char input and stores it in on the stack.
    // If a new token indicator is observed, recursively call new and parse.
    // Return None if no new token has been generated yet.
    pub fn parse(& mut self, parent_end: bool, input: char, line_num: usize) -> Option<Token> {
        let mut new_token = None;
        let end = parent_end |  self.token.end(self.stack.as_str(), input);

        if let Some(ref mut p) = self.parser {
            new_token = p.as_mut().parse(end, input, line_num);
        }

        if let Some(t)= new_token {
            self.token.push(t);
            self.parser = None;
        }

        if let None = self.parser.as_ref() {
            if end {
                return Some(self.token.clone()) ;
            } else {
                match Token::new(&self.token, self.stack.as_str(), input, line_num) {
                    MaybeToken::Recursive(t) => {
                        let mut builder = Builder::new(t);
                        // TODO: Find a better way of handling this error
                        if let None = builder.parse(end, input, line_num) {
                            panic!("Error, second parse in slide should never be None.")
                        }
                        self.parser = Some(Box::new(builder));
                    }
                    MaybeToken::NotRecursive(t) => {
                        let builder = Builder::new(t);
                        self.parser = Some(Box::new(builder));
                    }
                    MaybeToken::NoToken => self.stack.push(input),
                }
            }
        }
        None
    }
}
