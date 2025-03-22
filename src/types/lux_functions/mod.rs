use super::expression::Callable as CallableStruct;
use super::Expression;
use crate::parser::interpreter::Interpreter;
pub(crate) mod clock;

pub(crate) trait Callable {
    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Expression>) -> Expression;
    fn arity(self) -> u64;
}

impl Callable for CallableStruct {
    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Expression>) -> Expression {
        todo!()
    }
    fn arity(self) -> u64 {
        todo!()
    }
}

pub(crate) enum Functions {
    Clock(clock::Clock),
}
