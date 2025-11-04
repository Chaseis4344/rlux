use super::{
    token::Token,
    *,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Ternary {
    pub(crate) evaluator: Expression,
    pub(crate) left: Expression,
    pub(crate) right: Expression,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Literal {
    pub(crate) value: LiteralType,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Unary {
    pub(crate) operator: Token,
    pub(crate) operand: Expression,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Binary {
    pub(crate) operator: Token,
    pub(crate) left: Expression,
    pub(crate) right: Expression,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Grouping {
    pub(crate) expression: Expression,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    pub(crate) name: Token,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assignment {
    pub(crate) name: Token,
    pub(crate) value: Expression,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Logical {
    pub(crate) left: Expression,
    pub(crate) right: Expression,
    pub(crate) operator: Token,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Call {
    pub(crate) callee: Expression,
    pub(crate) paren: Token,
    pub(crate) arguments: Vec<Expression>,
}
