use crate::{
    parser::ast::Visitable,
    types::{token::Token, Expression, LiteralType},
};

use super::ast::{self, Visitor};

impl Visitable<LiteralType> for Expression {
    fn accept(&mut self, visitor: &mut dyn Visitor<LiteralType>) -> LiteralType {
        match self {
            Expression::Binary(bin) => bin.accept(visitor),
            Expression::Literal(lit) => lit.accept(visitor),
            Expression::Grouping(group) => group.accept(visitor),
            Expression::Unary(unary) => unary.accept(visitor),
            Expression::Ternary(tern) => tern.accept(visitor),
        }
    }
}

impl Visitable<LiteralType> for ast::Literal {
    fn accept(&mut self, visitor: &mut dyn ast::Visitor<LiteralType>) -> LiteralType {
        visitor.visit_literal(Box::new(self))
    }
}

impl Visitable<LiteralType> for ast::Grouping {
    fn accept(&mut self, visitor: &mut dyn Visitor<LiteralType>) -> LiteralType {
        visitor.visit_grouping(Box::new(self))
    }
}

impl Visitable<LiteralType> for ast::Binary {
    fn accept(&mut self, visitor: &mut dyn Visitor<LiteralType>) -> LiteralType {
        visitor.visit_binary(Box::new(self))
    }
}

impl Visitable<LiteralType> for ast::Ternary {
    fn accept(&mut self, visitor: &mut dyn Visitor<LiteralType>) -> LiteralType {
        visitor.visit_ternary(Box::new(self))
    }
}

impl Visitable<LiteralType> for ast::Unary {
    fn accept(&mut self, visitor: &mut dyn Visitor<LiteralType>) -> LiteralType {
        visitor.visit_unary(Box::new(self))
    }
}

pub struct Interpreter {}

impl Interpreter {
    fn evaluate(&mut self, expr: &mut Expression) -> LiteralType {
        expr.accept(self)
    }
}

impl Visitor<LiteralType> for Interpreter {
    fn visit_binary(&mut self, bin: Box<&mut ast::Binary>) -> LiteralType {
        let left = self.evaluate(&mut bin.left);
        let right = self.evaluate(&mut bin.right);
        let operator = &bin.operator;

        match operator.token_type {
            super::TokenType::Plus => left + right,
            super::TokenType::Star => left * right,
            super::TokenType::Slash => left / right,
            _ => left,
        }
    }
    fn visit_grouping(&mut self, group: Box<&mut ast::Grouping>) -> LiteralType {
        self.evaluate(&mut group.expression)
    }
    fn visit_literal(&mut self, lit: Box<&mut ast::Literal>) -> LiteralType {
        lit.value.clone()
    }
    fn visit_ternary(&mut self, tern: Box<&mut ast::Ternary>) -> LiteralType {
        let evaluator = self.evaluate(&mut tern.evaluator);
        let mut left = &mut tern.left;
        let mut right = &mut tern.right;

        match evaluator {
            LiteralType::Boolean(truthy) => {
                if truthy {
                    self.evaluate(&mut left)
                } else {
                    self.evaluate(&mut right)
                }
            }
            _ => evaluator,
        }
    }
    fn visit_unary(&mut self, unary: Box<&mut ast::Unary>) -> LiteralType {
        //let right =
        let right = self.evaluate(&mut unary.operand);

        match unary.operator.token_type {
            super::TokenType::Minus => match right {
                LiteralType::Number(num) => LiteralType::Number(-num),
                _ => right,
            },
            super::TokenType::Bang => match right {
                LiteralType::Boolean(boolean) => LiteralType::Boolean(!boolean),
                _ => right,
            },
            _ => right,
        }
    }
}
