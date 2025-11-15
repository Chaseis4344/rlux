use crate::types::{
    Expression,
    lux_functions::{
        Callable,
        Interpreter,
    },
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Print {}

impl Callable for Print {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Expression>,
    ) -> Option<Expression> {
        //We are garunteeing that only 1 argument will ever be passed
        let mut arg = arguments[0].clone();
        let printable: String = interpreter.evaluate(&mut arg).to_string();
        println!("{printable}");
        None
    }
    fn arity(&self) -> u64 {
        1
    }
}
