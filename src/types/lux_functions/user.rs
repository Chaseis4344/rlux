use super::{Callable, Interpreter};
use crate::enviroment::Enviroment;
use crate::types::{statement::*, *};
use std::fmt::{Debug, Formatter};

#[derive(Clone, PartialEq)]
pub(crate) struct UserFunction {
    pub(crate) declaration: Box<FunctionStatement>,
}

impl Debug for UserFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "UserFunction: {}", self.declaration.name.lexeme)
    }
}

impl Callable for UserFunction {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        mut arguments: Vec<Expression>,
    ) -> Option<Expression> {
        let mut function_enviroment: Enviroment = Enviroment {
            enclosing: Some(Box::new(interpreter.globals.clone())),
            variable_map: interpreter.enviroment.variable_map.clone(),
        };
        let (params, body, function_name) = (
            &self.declaration.parameters,
            &self.declaration.body,
            &self.declaration.name.lexeme,
        );
        for i in 0..params.len() {
            function_enviroment.define(
                params[i].lexeme.clone(),
                interpreter.evaluate(&mut arguments[i]),
            );
        }

        //Enabling Recursion
        let function = crate::types::lux_functions::Functions::User(UserFunction {
            declaration: Box::new(*self.declaration.clone()),
        });
        function_enviroment.define(function_name.to_string(), LiteralType::Callable(function));

        interpreter.execute_block_in_env(body.clone(), function_enviroment);
        None
    }

    fn arity(&self) -> u64 {
        self.declaration.parameters.len().try_into().unwrap()
    }
}
