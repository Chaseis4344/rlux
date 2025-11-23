mod display_traits;
pub mod expression;
pub mod lux_functions;
pub mod math_traits;
pub mod statement;
pub mod token;

//TODO: Find a replacement for Strings that allows for Copy to be implemented
//TODO: Replace "String" with a Box<str> or Cow<str> which will reduce velocity but give memory compaction
#[derive(Clone, Debug)]
pub(crate) enum LiteralType {
    Number(f64),
    Boolean(bool),
    String(String),
    Callable(lux_functions::Functions),
    Nil, //This will be wrapped in an option,
}

#[derive(Clone, Debug, Copy)]
pub enum TokenType {
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
    // Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

///Rlux's basic expression type, uses `Box<T>` so that recursive types can be captured
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
    Call(Box<expression::Call>),
    Lambda(Box<expression::Lambda>),
}

#[derive(Clone)]
pub struct ParserError {
    pub source: token::Token,
    pub cause: String,
}

#[derive(Clone, Debug)]
pub struct RuntimeError {
    pub source: token::Token,
}

#[derive(Clone, Debug)]
pub enum LuxErrors {
    ParserError(ParserError),
    RuntimeError(RuntimeError),
}
