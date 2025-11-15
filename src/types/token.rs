use super::{
    LiteralType,
    TokenType,
};
use std::fmt::Formatter;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralType>,
    pub line: u32,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Type: {}, Lexeme: \'{}\', Literal Value: {}",
            self.token_type,
            self.lexeme,
            self.literal.as_ref().unwrap_or(&LiteralType::Nil)
        )
    }
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: &str,
        literal: Option<LiteralType>,
        line: u32,
    ) -> Token {
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }
}
