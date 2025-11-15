use super::{
    Callable,
    Interpreter,
};
use crate::{
    enviroment::Enviroment,
    types::{
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
pub(crate) struct UserFunction <'function>{
    pub(crate) declaration: Box<FunctionStatement>,
    pub(crate) closure: Box<Enviroment<'function>>,
}

impl PartialEq for UserFunction<'_> {
        
    fn eq(&self, other: &Self) -> bool {
        //What is in the closure is entirely dependent on what is in the declaration, so we can
        //ignore the closure when comparing them, since identical declarations will produce
        //identical closures 
        self.declaration == other.declaration
    }
}

impl Debug for UserFunction<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "UserFunction: {}", self.declaration.name.lexeme)
    }
}

impl Callable for UserFunction<'_> {
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
            self.declaration.body.clone(),
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
            closure: Box::new(*self.closure.clone())
        });
        //Define this function in it's own enviroment
        function_enviroment.define(function_name.to_string(), LiteralType::Callable(function));

        // let ret = Interpreter::execute_block_in_env( body.clone(), function_enviroment);
        let ret = catch_unwind(|| Interpreter::execute_block_in_env(body, function_enviroment));

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
