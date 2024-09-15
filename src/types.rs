use crate::parser::expression::*;

mod display_traits;
pub(crate) mod functional_traits;
pub(crate) mod token;

#[derive(Clone, Debug)]
pub(crate) enum LiteralType {
    Number(f64),
    Boolean(bool),
    String(String),
    Nil, //This will be wrapped in an option,
}

#[derive(Clone, Debug)]
pub(crate) enum TokenType {
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

#[derive(Clone, Debug)]
pub enum Expression {
    Grouping(Box<Grouping>),
    Logical(Box<Logical>),
    Unary(Box<Unary>),
    Binary(Box<Binary>),
    Ternary(Box<Ternary>),
    Literal(Box<Literal>),
    Variable(Box<Variable>),
    Assignment(Box<Assignment>),
}

pub struct ParserError {
    pub(crate) source: token::Token,
    pub(crate) cause: String,
}

#[derive(Clone)]
pub(crate) struct RuntimeError {
    pub(crate) source: token::Token,
}

pub(crate) enum LuxErrors {
    ParserError(ParserError),
    RuntimeError(RuntimeError),
}
