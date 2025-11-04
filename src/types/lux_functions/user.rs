use super::{Callable, Interpreter};
use crate::enviroment::Enviroment;
use crate::types::{statement::*, *};
use std::fmt::{Debug, Formatter};
use std::panic::catch_unwind;

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
        });
        //Define this function in it's own enviroment
        function_enviroment.define(function_name.to_string(), LiteralType::Callable(function));

       // let ret = Interpreter::execute_block_in_env( body.clone(), function_enviroment);
        let ret = catch_unwind(|| {
            Interpreter::execute_block_in_env(body, function_enviroment)
        });

       match ret {
           Err(thing) => {
               if thing.is::<crate::types::statement::Statement>()
               {
                  if let Ok(return_statement) = thing.downcast::<crate::types::statement::Statement>() {
                      match *return_statement  {
                          Statement::Return(ret_val) => {
                              match ret_val.value {
                                 Some(val) => {
                                     Some(val)
                                 }
                                 None => None, 
                              }
                              
                          },
                          _ => {return None;},
                      }
                  }else {
                      panic!("Not a Return Statement")
                  } 
               }else {
                   panic!("Uncaught")
               }

                   
           }
           Ok(value) => {
               println!("Returned nothing");
               if value.is_some() {
                   panic!("Returned Nothing from function that has a value");
               }
               return value;
           }
       }
    }

    fn arity(&self) -> u64 {
        self.declaration.parameters.len().try_into().unwrap()
    }
}


