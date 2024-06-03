use crate::parser::ast::*;

mod display_traits;
pub mod token;

#[derive(Clone)]
pub enum LiteralType {
    Number(f64),
    Boolean(bool),
    String(String),
    Nil, //This will be wrapped in an option,
}

#[derive(Clone)]
pub enum TokenType {
    //Single Character Tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Question,
    Colon,

    //One Or Two Character Tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    //Literals
    Identifier,
    String,
    Number,

    //Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl PartialEq for TokenType {
    fn eq(&self, rhs: &Self) -> bool {
        self.to_string() == rhs.to_string()
    }

    fn ne(&self, other: &Self) -> bool {
        self.to_string() != other.to_string()
    }
}

#[derive(Clone)]
pub enum Expression {
    Grouping(Box<Grouping>),
    Unary(Box<Unary>),
    Binary(Box<Binary>),
    Ternary(Box<Ternary>),
    Literal(Box<Literal>),
}

pub struct ParserError {
    pub source: token::Token,
}
