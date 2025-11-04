use super::RuntimeError;
use crate::types;
use crate::types::lux_functions::user::UserFunction;
use std::error::Error;
use std::fmt::Display as DisplayTrait;

//Token Display implementation moved to token.rs because of private field implementation
impl DisplayTrait for super::expression::Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Call: (callee:{}, paren:{}, arguments:{})",
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
            Self::Callable(function) => write!(f, "{}", function),
        }
    }
}

impl DisplayTrait for types::lux_functions::Functions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::User(u) => write!(f, "<fn {}>", u),
            Self::Clock(_) => write!(f, "<fn Clock>"),
            Self::Print(_) => write!(f, "<fn Print>"),
        }
    }
}

impl DisplayTrait for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error on: {}", self.source)
    }
}
impl DisplayTrait for UserFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.declaration.name.lexeme)
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
            // Self::Print => write!(f, "Print"),
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
        write!(
            f,
            "ParserError: \n\t Source Token: \n\t\t{:?}\n\tCause Message:\n\t\t{:?}",
            self.source, self.cause
        )
    }
}

impl DisplayTrait for crate::types::Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (*self).clone() {
            Self::Grouping(group) => {
                write!(f, "group:{}", group.expression)
            }
            Self::Binary(bin) => {
                write!(
                    f,
                    "(Binary: left:{}, right{}, operator:{})",
                    bin.left, bin.right, bin.operator
                )
            }
            Self::Literal(lit) => {
                //This is the only path used since everything gets evaluated to a literal before it
                //gets displayed internally in print
                write!(f, "literal:{}", lit.value)
            }
            Self::Unary(unary) => {
                write!(
                    f,
                    "(unary:  Operator:{} Operand:{})",
                    unary.operator, unary.operand
                )
            }
            Self::Ternary(tern) => {
                write!(
                    f,
                    "(ternary: Evaluator:{} , leftHand side:{},  rightHand side:{})",
                    tern.evaluator, tern.left, tern.right
                )
            }
            Self::Variable(var) => {
                write!(f, "(variable:{})", var.name)
            }
            //TODO: IMPLEMENT BELOW
            Self::Assignment(assign) => {
                write!(
                    f,
                    "(Assignment: value:{}, name:{})",
                    assign.value, assign.name
                )
            }
            Self::Logical(logic) => {
                write!(
                    f,
                    "(Logical: left:{}, right:{}, operator:{})",
                    logic.left, logic.right, logic.operator
                )
            }
            Self::Call(call) => {
                write!(f, "(Call: callee:{})", call.callee)
            }
        }
    }
}

impl DisplayTrait for crate::types::statement::Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (*self).clone() {
            Self::Return(ret) => {
                if let Some(value) = ret.value {
                    write!(
                        f,
                        "(Return Statement: value:{}, token:{}",
                        value, ret.keyword
                    )
                } else {
                    write!(
                        f,
                        "(Return Statement: value:{}, token:{}",
                        "None", ret.keyword
                    )
                }
            }
            Self::Block(block) => {
                for statement in block.statements {
                    if let Err(e) = write!(f, "(Block Statement: inner:{})", statement) {
                        return Err(e);
                    }
                }
                Ok(())
            }
            Self::If(iffy) => {
                if let Some(else_branch) = *iffy.else_branch {
                    write!(
                        f,
                        "(If Statement: condition: {}, body:{}, else:{})",
                        iffy.condition, *iffy.then_branch, else_branch
                    )
                } else {
                    write!(
                        f,
                        "(If Statement: condition: {}, body:{}, else: None)",
                        iffy.condition, *iffy.then_branch
                    )
                }
            }
            Self::While(whilly) => {
                write!(
                    f,
                    "(Whille Statement: condition: {}, body: {})",
                    whilly.condition, whilly.body
                )
            }
            Self::Expression(expr) => {
                write!(f, "(Expression: {})", expr.expression)
            }
            Self::Function(func) => {
                write!(f, "(Func declaration: {})", func.name)
            }
            _ => todo!("Unimplemented Display on Statement"),
        }
    }
}

impl DisplayTrait for crate::types::statement::ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if let Some(val) = &self.value {
            write!(f, "Ret:{}", val)
        } else {
            write!(f, "None")
        }
    }
}
