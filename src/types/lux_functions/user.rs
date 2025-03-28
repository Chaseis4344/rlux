use super::Callable;
use crate::enviroment::Enviroment;
use crate::types::lux_functions::Interpreter;
use crate::types::statement::{FunctionStatement, Statement};
use crate::types::token::Token;
use crate::types::Expression;
use std::{collections::HashMap, env::VarError};

#[derive(Debug, Clone)]
pub(crate) struct UserFunction {
    declaration: Box<FunctionStatement>,
}

impl Callable for UserFunction {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        mut arguments: Vec<Expression>,
    ) -> Option<Expression> {
        let mut function_enviroment: Enviroment = Enviroment {
            enclosing: None,
            variable_map: HashMap::new(),
        };
        let mut decl = &self.declaration;

        for i in 0..decl.parameters.len() {
            function_enviroment.define(
                decl.parameters[i].lexeme.clone(),
                interpreter.evaluate(&mut arguments[i]),
            );
        }

        interpreter.execute_block(decl.body, function_enviroment);
        None
    }

    fn arity(&self) -> u64 {
        self.declaration.parameters.len().try_into().unwrap()
    }
}
