mod display_traits;
pub(crate) mod expression;
pub(crate) mod functional_traits;
pub(crate) mod statement;
pub(crate) mod token;

#[derive(Clone, Debug)]
pub(crate) enum LiteralType {
    Number(f64),
    Boolean(bool),
    String(String),
    Callable(&mut expression::Callable),
    Nil, //This will be wrapped in an option,
}

#[derive(Clone, Debug, Copy)]
pub(crate) enum TokenType {
    //Single Character Tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Question,
    Colon,
    Comma,

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

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Grouping(Box<expression::Grouping>),
    Logical(Box<expression::Logical>),
    Unary(Box<expression::Unary>),
    Binary(Box<expression::Binary>),
    Ternary(Box<expression::Ternary>),
    Literal(Box<expression::Literal>),
    Variable(Box<expression::Variable>),
    Assignment(Box<expression::Assignment>),
    Callable(Box<expression::Callable>),
}

#[derive(Clone)]

pub struct ParserError {
    pub(crate) source: token::Token,
    pub(crate) cause: String,
}

#[derive(Clone, Debug)]
pub(crate) struct RuntimeError {
    pub(crate) source: token::Token,
}

#[derive(Clone, Debug)]

pub(crate) enum LuxErrors {
    ParserError(ParserError),
    RuntimeError(RuntimeError),
}
