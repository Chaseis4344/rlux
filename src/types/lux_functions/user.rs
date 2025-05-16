use super::Callable;
use crate::enviroment::Enviroment;
use crate::types::{Expression, lux_functions::Interpreter, statement::FunctionStatement};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct UserFunction {
    pub(crate) declaration: Box<FunctionStatement>,
}

impl Callable for UserFunction {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        mut arguments: Vec<Expression>,
    ) -> Option<Expression> {
        let mut function_enviroment: Enviroment = Enviroment {
            enclosing: Some(Box::new(interpreter.globals.clone())),
            variable_map: HashMap::new(),
        };
        let (params, body) = (&self.declaration.parameters, &self.declaration.body);
        for i in 0..params.len() {
            function_enviroment.define(
                params[i].lexeme.clone(),
                interpreter.evaluate(&mut arguments[i]),
            );
        }

        interpreter.execute_block_in_env(body.clone(), function_enviroment);
        None
    }

    fn arity(&self) -> u64 {
        self.declaration.parameters.len().try_into().unwrap()
    }
}
