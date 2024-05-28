use crate::types::LiteralType;
use crate::types::TokenType;

//Token Display implementation moved to token.rs because of private field implementation

impl std::fmt::Display for LiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{}", num),
            Self::Boolean(val) => write!(f, "{}", val),
            Self::String(string) => write!(f, "{}", string),
            Self::Nil => write!(f, "NIL"),
        }
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::LeftBrace => write!(f, "Left Brace"),
            Self::RightBrace => write!(f, "Right Brace"),

            Self::LeftParen => write!(f, "Left Parentheses"),
            Self::RightParen => write!(f, "Right Parentheses"),

            Self::Comma => write!(f, "Comma"),
            Self::Dot => write!(f, "Dot"),
            Self::Minus => write!(f, "Minus"),
            Self::Plus => write!(f, "Plus"),
            Self::Semicolon => write!(f, "Semicolon"),
            Self::Slash => write!(f, "Slash"),
            Self::Star => write!(f, "Star"),
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
