use super::token::Token;
use super::*;

#[derive(Clone, Debug)]
pub(crate) struct Ternary {
    pub(crate) evaluator: Expression,
    pub(crate) left: Expression,
    pub(crate) right: Expression,
}

#[derive(Clone, Debug)]
pub(crate) struct Literal {
    pub(crate) value: LiteralType,
}

#[derive(Clone, Debug)]
pub(crate) struct Unary {
    pub(crate) operator: Token,
    pub(crate) operand: Expression,
}

#[derive(Clone, Debug)]
pub struct Binary {
    pub(crate) operator: Token,
    pub(crate) left: Expression,
    pub(crate) right: Expression,
}

#[derive(Clone, Debug)]
pub struct Grouping {
    pub(crate) expression: Expression,
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub(crate) name: Token,
}

#[derive(Clone, Debug)]
pub struct Assignment {
    pub(crate) name: Token,
    pub(crate) value: Expression,
}

#[derive(Clone, Debug)]
pub struct Logical {
    pub(crate) left: Expression,
    pub(crate) right: Expression,
    pub(crate) operator: Token,
}
