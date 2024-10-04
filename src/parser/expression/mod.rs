use crate::types::{token::Token, Expression, LiteralType};
mod parser_impl;

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

/*
    fn visit_X(&mut self, Y: Box<&mut X>) -> T;
*/
pub(crate) trait ExpressionVisitor<T> {
    fn visit_grouping(&mut self, group: Box<&mut Grouping>) -> T;
    fn visit_binary(&mut self, bin: Box<&mut Binary>) -> T;
    fn visit_unary(&mut self, unary: Box<&mut Unary>) -> T;
    fn visit_literal(&mut self, lit: Box<&mut Literal>) -> T;
    fn visit_ternary(&mut self, tern: Box<&mut Ternary>) -> T;
    fn visit_variable(&mut self, var: Box<&mut Variable>) -> T;
    fn visit_assignment(&mut self, assign: Box<&mut Assignment>) -> T;
    fn visit_logical(&mut self, logical: Box<&mut Logical>) -> T;
}

pub(crate) trait Visitable<T, U> {
    fn accept(&mut self, visitor: &mut U) -> T;
}
