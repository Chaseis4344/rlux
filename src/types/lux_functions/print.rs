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
        print!("{printable}");
        None
    }
    fn arity(&self) -> u64 {
        1
    }
}

#[derive(Clone,Debug,PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Println {}

impl Callable for Println {
    fn call(
            &mut self,
            interpreter: &mut Interpreter,
            arguments: Vec<Expression>,
        ) -> Option<Expression> {
        
        //We are garunteeing that only 1 argument will ever be passed
        let mut arg = arguments[0].clone();
        let printable: String = interpreter.evaluate(&mut arg).to_string();
        print!("{printable}");
        None
    }
    fn arity(&self) -> u64 {
        1
    }
}

