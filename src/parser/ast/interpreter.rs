use crate::types::{Expression, LiteralType, TokenType};

use super::super::Statement;
use super::expression::{Binary, Grouping, Literal, Ternary, Unary};
use super::{statement, Visitable, Visitor};

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

impl Visitable<LiteralType> for Literal {
    fn accept(&mut self, visitor: &mut dyn Visitor<LiteralType>) -> LiteralType {
        visitor.visit_literal(Box::new(self))
    }
}

impl Visitable<LiteralType> for Grouping {
    fn accept(&mut self, visitor: &mut dyn Visitor<LiteralType>) -> LiteralType {
        visitor.visit_grouping(Box::new(self))
    }
}

impl Visitable<LiteralType> for Binary {
    fn accept(&mut self, visitor: &mut dyn Visitor<LiteralType>) -> LiteralType {
        visitor.visit_binary(Box::new(self))
    }
}

impl Visitable<LiteralType> for Ternary {
    fn accept(&mut self, visitor: &mut dyn Visitor<LiteralType>) -> LiteralType {
        visitor.visit_ternary(Box::new(self))
    }
}

impl Visitable<LiteralType> for Unary {
    fn accept(&mut self, visitor: &mut dyn Visitor<LiteralType>) -> LiteralType {
        visitor.visit_unary(Box::new(self))
    }
}

pub struct Interpreter {}

impl Interpreter {
    pub(crate) fn evaluate(&mut self, expr: &mut Expression) -> LiteralType {
        expr.accept(self)
    }
    pub(crate) fn new() -> Interpreter {
        Interpreter {}
    }
    pub fn interpret(&mut self, statements: Vec<&mut Statement>) {
        for statement in statements {
            self.execute(statement);
        }
    }

    pub fn execute(&mut self, statement: &mut Statement) {
        use super::expression::Visitable;

        statement.accept(self);
    }
}

impl Visitor<LiteralType> for Interpreter {
    fn visit_binary(&mut self, bin: Box<&mut Binary>) -> LiteralType {
        let left = self.evaluate(&mut bin.left);
        let right = self.evaluate(&mut bin.right);
        let operator = &bin.operator;

        //We can abstract all this logic away to rust's traits
        /*TODO: ARCHITECT WAY FOR TYPE ERRORS TO BE PASSED UP FROM HERE TO USER */
        match operator.token_type {
            TokenType::Plus => left + right,
            TokenType::Star => left * right,
            TokenType::Slash => left / right,
            TokenType::Minus => left - right,
            TokenType::Greater => LiteralType::Boolean(left > right),
            TokenType::GreaterEqual => LiteralType::Boolean(left >= right),
            TokenType::Less => LiteralType::Boolean(left < right),
            TokenType::LessEqual => LiteralType::Boolean(left <= right),
            TokenType::EqualEqual => LiteralType::Boolean(left == right),
            TokenType::BangEqual => LiteralType::Boolean(left != right),
            _ => left,
        }
    }
    fn visit_grouping(&mut self, group: Box<&mut Grouping>) -> LiteralType {
        self.evaluate(&mut group.expression)
    }
    fn visit_literal(&mut self, lit: Box<&mut Literal>) -> LiteralType {
        lit.value.clone()
    }
    fn visit_ternary(&mut self, tern: Box<&mut Ternary>) -> LiteralType {
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
    fn visit_unary(&mut self, unary: Box<&mut Unary>) -> LiteralType {
        //let right =
        let right = self.evaluate(&mut unary.operand);

        match unary.operator.token_type {
            TokenType::Minus => match right {
                LiteralType::Number(num) => LiteralType::Number(-num),
                _ => right,
            },
            TokenType::Bang => match right {
                LiteralType::Boolean(boolean) => LiteralType::Boolean(!boolean),
                _ => right,
            },
            _ => right,
        }
    }
}
