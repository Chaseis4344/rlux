use super::Expression;
use crate::parser::interpreter::Interpreter;
use super::expression::Callable as CallableStruct;
use crate::types::Expression::Literal;

pub(crate) trait Callable {
    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Expression>) -> Expression;
    fn arity( self) -> u64;
}

impl Callable for CallableStruct {
    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Expression>) -> Expression{
       todo!() 
    }
    fn arity(self) -> u64{
        todo!()
    }
}

pub(crate) enum Functions {
    Clock(Clock),
}

pub(crate) struct Clock {}

impl Callable for Clock {
    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Expression>) -> Expression {
        Literal(value: Box::new(Literal{ value: Number(std::time::SystemTime::now() as f64)}))
    }
    fn arity(self) -> u64 {
        0
    }
}
