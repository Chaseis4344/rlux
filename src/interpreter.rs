use crate::{
    enviroment::Enviroment,
    types::{
        Expression,
        LiteralType,
    },
};
use interpreter_traits::InterpreterVisitor;

pub(crate) struct Interpreter {
    pub(crate) enviroment: Box<Enviroment>,
    pub(crate) globals: Enviroment,
}

mod interpret_ir;
mod interpreter_impl;
mod interpreter_traits;

impl interpreter_traits::Visitable<LiteralType> for Expression {
    fn accept(&mut self, visitor: &mut dyn InterpreterVisitor<LiteralType>) -> LiteralType {
        match self {
            Expression::Binary(bin) => bin.accept(visitor),
            Expression::Literal(lit) => lit.accept(visitor),
            Expression::Grouping(group) => group.accept(visitor),
            Expression::Unary(unary) => unary.accept(visitor),
            Expression::Ternary(tern) => tern.accept(visitor),
            Expression::Variable(var) => var.accept(visitor),
            Expression::Assignment(assign) => assign.accept(visitor),
            Expression::Logical(logic) => logic.accept(visitor),
            Expression::Call(call) => call.accept(visitor),
            Expression::Lambda(lambda) => lambda.accept(visitor),
        }
    }
}
