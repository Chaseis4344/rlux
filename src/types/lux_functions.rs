
pub(crate) trait Callable {
    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Expression>) -> Expression;
    fn arity(&mut self) -> u64;
}

impl crate::types::functional_traits::Callable for super::expression::Callable {
    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Expression>) -> Expression {
       todo!() 
    }
    fn arity(&mut self) -> u64{
        todo!()
    }
}

