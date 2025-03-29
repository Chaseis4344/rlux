use crate::types::lux_functions::Callable;
use crate::types::lux_functions::Interpreter;
use crate::types::Expression;
use crate::types::Expression::Literal;
use crate::types::LiteralType;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Print {}

impl Callable for Print {
    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Expression>) -> Option<Expression> {
        let printable: String = arguments[0].to_string();
        println!("{printable}");
        None
    }
    fn arity(&self) -> u64 {
        1
    }
}
