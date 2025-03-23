use crate::types;
use std::error::Error;
use std::fmt::Display as DisplayTrait;

use super::RuntimeError;

//Token Display implementation moved to token.rs because of private field implementation

impl DisplayTrait for super::expression::Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Call: (callee: {}, paren: {}, arguments: {})",
            self.callee,
            self.paren,
            self.arguments.len()
        )
    }
}
impl DisplayTrait for types::LiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{}", num),
            Self::Boolean(val) => write!(f, "{}", val),
            Self::String(string) => write!(f, "{}", string),
            Self::Nil => write!(f, "NIL"),
            Self::Callable(function) => write!(f, "{:?}", function),
        }
    }
}

impl DisplayTrait for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error on: {}", self.source)
    }
}

impl DisplayTrait for types::TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::LeftBrace => write!(f, "Left Brace"),
            Self::RightBrace => write!(f, "Right Brace"),

            Self::LeftParen => write!(f, "Left Parentheses"),
            Self::RightParen => write!(f, "Right Parentheses"),

            Self::Dot => write!(f, "Dot"),
            Self::Minus => write!(f, "Minus"),
            Self::Plus => write!(f, "Plus"),
            Self::Semicolon => write!(f, "Semicolon"),
            Self::Slash => write!(f, "Slash"),
            Self::Star => write!(f, "Star"),
            Self::Question => write!(f, "Question"),
            Self::Colon => write!(f, "Colon"),
            Self::Comma => write!(f, "Comma"),

            Self::Bang => write!(f, "Bang"),
            Self::BangEqual => write!(f, "Bang Equal"),
            Self::Equal => write!(f, "Equal"),
            Self::EqualEqual => write!(f, "Double Equal"),
            Self::GreaterEqual => write!(f, "Greater Equal"),
            Self::Greater => write!(f, "Greater"),
            Self::Less => write!(f, "Less"),
            Self::LessEqual => write!(f, "Less Equal"),

            Self::Identifier => write!(f, "Identifier"),
            Self::String => write!(f, "String"),
            Self::Number => write!(f, "Number"),

            Self::And => write!(f, "And"),
            Self::Class => write!(f, "Class"),
            Self::Else => write!(f, "Else"),
            Self::False => write!(f, "False"),
            Self::Fun => write!(f, "Fun"),
            Self::For => write!(f, "For"),
            Self::If => write!(f, "If"),
            Self::Nil => write!(f, "Nil"),
            Self::Or => write!(f, "Or"),
            Self::Print => write!(f, "Print"),
            Self::Return => write!(f, "Return"),
            Self::Super => write!(f, "Super"),
            Self::This => write!(f, "This"),
            Self::True => write!(f, "True"),
            Self::Var => write!(f, "Var"),
            Self::While => write!(f, "While"),
            Self::Eof => write!(f, "Eof"),
        }
    }
}

impl Error for types::ParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
impl DisplayTrait for types::ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parser Error occured on Token: \n\t {:?} at line {} because {}",
            self.source, self.source.line, self.cause
        )
    }
}

impl std::fmt::Debug for types::ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParserError: \n\t self.source: \n\t{:?};", self.source)
    }
}

impl DisplayTrait for crate::types::Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (*self).clone() {
            Self::Grouping(group) => {
                write!(f, "(group {})", group.expression)
            }
            Self::Binary(bin) => {
                write!(
                    f,
                    "(binary: l: {} op: ({}) r:{}) ",
                    bin.left, bin.operator, bin.right
                )
            }
            Self::Literal(lit) => {
                write!(f, "(literal: {})", lit.value)
            }
            Self::Unary(unary) => {
                write!(
                    f,
                    "(unary:  Operator:({}) Operand:{})",
                    unary.operator, unary.operand
                )
            }
            Self::Ternary(tern) => {
                write!(
                    f,
                    "(ternary: Evaluator:({}) , leftHand side:({}),  rightHand side:({}) )",
                    tern.evaluator, tern.left, tern.right
                )
            }
            Self::Variable(var) => {
                write!(f, "(variable:{})", var.name)
            }
            //TODO: IMPLEMENT BELOW
            Self::Assignment(_) => todo!(),
            Self::Logical(_) => todo!(),
            Self::Call(_) => todo!(),
        }
    }
}
