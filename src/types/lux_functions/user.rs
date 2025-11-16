use super::{
    Callable,
    Interpreter,
};
use crate::{
    enviroment::Enviroment,
    types::{
        lux_functions::Functions,
        statement::*,
        *,
    },
};
use std::{
    fmt::{
        Debug,
        Formatter,
    },
    panic::catch_unwind,
};

#[derive(Clone)]
/// This is the User-Defined Function Capability and encapsulates all non-native functions
pub(crate) struct UserFunction {
    pub(crate) closure: Enviroment,
    pub(crate) declaration: Box<FunctionStatement>,
}

impl PartialEq for UserFunction {
    fn eq(&self, other: &Self) -> bool {
        self.declaration == other.declaration
    }
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
        //Enabling Recursion
        let function = UserFunction {
            closure: *interpreter.enviroment.clone(),
            declaration: Box::new(*self.declaration.clone()),
        };

        let (params, body, function_name) = (
            &self.declaration.parameters,
            self.declaration.body.clone(),
            &self.declaration.name.lexeme,
        );

        let mut enviroment: Enviroment = Enviroment {
            enclosing: Some(Box::new(function.closure.clone())),
            variable_map: function.closure.variable_map.clone(),
        };

        for i in 0..params.len() {
            enviroment.define(&params[i].lexeme, interpreter.evaluate(&mut arguments[i]));
        }

        let function = Functions::User(function);
        //Define this function in it's own enviroment
        enviroment.define(function_name, LiteralType::Callable(function));

        // let ret = Interpreter::execute_block_in_env( body.clone(), function_enviroment);
        let ret = catch_unwind(|| {
            Interpreter::execute_block_in_env(body, *interpreter.enviroment.clone())
        });

        match ret {
            Err(thing) => {
                if thing.is::<crate::types::statement::Statement>() {
                    if let Ok(return_statement) =
                        thing.downcast::<crate::types::statement::Statement>()
                    {
                        match *return_statement {
                            Statement::Return(ret_val) => ret_val.value,
                            _ => None,
                        }
                    } else {
                        panic!("Not a Return Statement")
                    }
                } else {
                    panic!("Uncaught")
                }
            }
            Ok(value) => {
                // println!("Returned nothing");
                if value.is_some() {
                    panic!("Returned Nothing from function that has a value");
                }
                value
            }
        }
    }

    fn arity(&self) -> u64 {
        self.declaration.parameters.len().try_into().unwrap()
    }
}
