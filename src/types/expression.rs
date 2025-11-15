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
pub(crate) struct Literal<'literal> {
    pub(crate) value: LiteralType<'literal>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Unary<'unary> {
    pub(crate) operator: Token<'unary>,
    pub(crate) operand: Expression,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Binary<'unary> {
    pub(crate) operator: Token<'unary>,
    pub(crate) left: Expression,
    pub(crate) right: Expression,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Grouping {
    pub(crate) expression: Expression,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variable<'var> {
    pub(crate) name: Token<'var>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assignment<'assign> {
    pub(crate) name: Token<'assign>,
    pub(crate) value: Expression,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Logical<'logic> {
    pub(crate) left: Expression,
    pub(crate) right: Expression,
    pub(crate) operator: Token<'logic>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Call<'call> {
    pub(crate) callee: Expression,
    pub(crate) paren: Token<'call>,
    pub(crate) arguments: Vec<Expression>,
}
