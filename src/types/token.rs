use super::{
    LiteralType,
    TokenType,
};
use std::fmt::Formatter;

#[derive(Clone, Debug, PartialEq)]
pub struct Token <'token>{
    pub token_type: TokenType,
    pub lexeme: &'token str,
    pub literal: Option<LiteralType<'token>>,
    pub line: u32,
}

impl std::fmt::Display for Token<'_> {
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

impl<'token> Token<'_> {
    pub fn new(
        token_type: TokenType,
        lexeme: &'token str,
        literal: Option<LiteralType<'token>>,
        line: u32,
    ) -> Token<'token> {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
