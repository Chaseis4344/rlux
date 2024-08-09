use std::collections::HashMap;

use crate::enviroment::Enviroment;
use crate::types::{Expression, LiteralType, TokenType};

use super::expression::{Assignment, ExpressionVisitor};
use super::expression::{Binary, Grouping, Literal, Ternary, Unary, Variable};
use super::Statement;

pub(crate) struct Interpreter {
    pub(crate) enviroment: Box<Enviroment>,
}

pub(crate) trait Visitable<T> {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor<T>) -> T;
}

impl Visitable<LiteralType> for Expression {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor<LiteralType>) -> LiteralType {
        match self {
            Expression::Binary(bin) => bin.accept(visitor),
            Expression::Literal(lit) => lit.accept(visitor),
            Expression::Grouping(group) => group.accept(visitor),
            Expression::Unary(unary) => unary.accept(visitor),
            Expression::Ternary(tern) => tern.accept(visitor),
            Expression::Variable(var) => var.accept(visitor),
            Expression::Assignment(assign) => assign.accept(visitor),
            Expression::Logical(logic) => logic.accept(visitor),
        }
    }
}


impl Visitable<LiteralType> for Logical {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor<LiteralType>) -> LiteralType {
        visitor.visit_logical(Box::new(self))
    }
}

impl Visitable<LiteralType> for Assignment {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor<LiteralType>) -> LiteralType {
        visitor.visit_assignment(Box::new(self))
    }
}

impl Visitable<LiteralType> for Variable {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor<LiteralType>) -> LiteralType {
        visitor.visit_variable(Box::new(self))
    }
}

impl Visitable<LiteralType> for Literal {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor<LiteralType>) -> LiteralType {
        visitor.visit_literal(Box::new(self))
    }
}

impl Visitable<LiteralType> for Grouping {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor<LiteralType>) -> LiteralType {
        visitor.visit_grouping(Box::new(self))
    }
}

impl Visitable<LiteralType> for Binary {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor<LiteralType>) -> LiteralType {
        visitor.visit_binary(Box::new(self))
    }
}

impl Visitable<LiteralType> for Ternary {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor<LiteralType>) -> LiteralType {
        visitor.visit_ternary(Box::new(self))
    }
}

impl Visitable<LiteralType> for Unary {
    fn accept(&mut self, visitor: &mut dyn ExpressionVisitor<LiteralType>) -> LiteralType {
        visitor.visit_unary(Box::new(self))
    }
}

impl Interpreter {
    pub(crate) fn evaluate(&mut self, expr: &mut Expression) -> LiteralType {
        expr.accept(self)
    }
    pub(crate) fn new() -> Interpreter {
        let map = HashMap::new();
        let enviroment = Box::new(Enviroment { variable_map: map });
        Interpreter { enviroment }
    }
    pub(crate) fn interpret(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            self.execute(statement);
        }
    }

    pub(crate) fn execute(&mut self, mut statement: Statement) {
        use super::expression::Visitable as InterpreterVisitable;
        //Hand over between the two architectures
        statement.accept(self);
    }
}

///Logic for how the Interpreter acts with each Data Type
impl ExpressionVisitor<LiteralType> for Interpreter {
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

    fn visit_variable(&mut self, var: Box<&mut Variable>) -> LiteralType {
        let result = self.enviroment.to_owned().get(var.to_owned().name);
        if result.is_ok() {
            return result.unwrap();
        } else {
            return LiteralType::Nil;
        }
    }

    fn visit_assignment(&mut self, assign: Box<&mut Assignment>) -> LiteralType {
        //Decompose assignment to avoid excess cloning
        let (name, value) = (assign.name.to_owned(), &mut assign.value);
        //Evaluate expression inside
        let value = self.evaluate(value);
        //Copy the value then echo out for the rest of the syntax tress
        self.enviroment.assign(name, value.clone());
        value
    }

    fn visit_logical(&mut self, logical: Box<&mut Logical>) -> LiteralType {
        let left = self.evaluate(logical.left);
        
        //Short Cirucuit if we can
        if logical.operator.type == TokenType::Or {
            // True or X will alway be True, so if True, then return True
            if left {
                return left;
            }
        } else {
            // False AND X will always be False, so return False if is_and && is_false
            if !left { return left; }
        }

        //traverse it otherwise
        self.evaluate(logical.right)
    }
}
