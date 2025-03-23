use crate::types::Expression;
use crate::types::lux_functions::Interpreter;
use crate::types::lux_functions::Callable;
use crate::types::LiteralType;
use crate::types::Expression::Literal;


#[derive(Clone, Debug)]
pub(crate) struct Print {}

impl Callable for Print {
    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Expression>) -> Expression {
        
        let printable: String = arguments[0].to_string();
        println!("{printable}");
        Literal(Box::new(crate::types::expression::Literal{value:LiteralType::Nil}))
    }
    fn arity(&self) -> u64 {
        1
    }
}
