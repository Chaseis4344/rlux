use crate::types::lux_functions::Callable;
use crate::types::lux_functions::Interpreter;
use crate::types::Expression;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Print {}

impl Callable for Print {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Expression>,
    ) -> Option<Expression> {
        //We are garunteeing that only 1 argument will ever be passed
        let mut arg = arguments[0].to_owned();
        let printable: String = interpreter.evaluate(&mut arg).to_string();
        println!("{printable}");
        None
    }
    fn arity(&self) -> u64 {
        1
    }
}
