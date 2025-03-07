use super::Expression;
use crate::parser::interpreter::Interpreter;
use super::expression::Callable as CallableStruct;
pub(crate) trait Callable {
    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Expression>) -> Expression;
    fn arity(&mut self) -> u64;
}

impl Callable for CallableStruct {
    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Expression>) -> Expression {
       todo!() 
    }
    fn arity(&mut self) -> u64{
        todo!()
    }
}

