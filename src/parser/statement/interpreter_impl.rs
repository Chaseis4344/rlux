use crate::{
    interpreter::Interpreter,
    parser::{
        LiteralType,
        statement::*,
    },
};

macro_rules! new_literal {
    ($value:expr) => {
        Expression::Literal(Box::new(crate::types::expression::Literal {
            value: $value,
        }))
    };
}

impl StatementVisitor for Interpreter {
    fn visit_return_statement(&mut self, ret: &mut ReturnStatement) -> Statement {
        use std::panic;
        // let original_hook =
        //Prevent Display of this particluar panic
        panic::set_hook(Box::new(|_info| {
            // do nothing
        }));
        let ret_value: Statement = if ret.value.is_some() {
            ret.value = Some(new_literal!(self.evaluate(&mut ret.value.clone().unwrap())));
            Statement::Return(ret.clone())
        } else {
            Statement::Return(ret.clone())
        };

        panic::panic_any(ret_value);
    }
    fn visit_expression_statement(&mut self, expression: &mut ExpressionStatement) -> Statement {
        let result = self.evaluate(&mut expression.expression);
        Statement::Expression(ExpressionStatement {
            expression: new_literal!(result),
        })
    }
    /*fn visit_print_statement(&mut self, print: &mut PrintStatement) -> Statement {
        let expression = self.evaluate(&mut print.expression);

        println!("{}", expression);
        Statement::Expression(ExpressionStatement {
            expression: print.expression.clone(),
        })
    }*/

    fn visit_variable_statement(&mut self, var: &mut VariableStatement) -> Statement {
        let init: LiteralType = if var.initalizer.is_some() {
            //Should be fine because we are directly checking there is something before unwrapping
            self.evaluate(&mut var.initalizer.clone().unwrap())
        } else {
            LiteralType::Nil
        };

        self.enviroment
            .define(&var.name.lexeme.clone(), init);

        let var = var.clone();
        Statement::Variable(VariableStatement {
            name: var.name,
            initalizer: var.initalizer,
        })
    }
    fn visit_if_statement(&mut self, if_statement: &mut IfStatement) -> Statement {
        let unboxed = if_statement.clone();
        let return_thing = unboxed.clone();
        let mut condition = unboxed.condition;
        let then_branch = *(unboxed.then_branch);
        let else_branch = unboxed.else_branch;

        if self.evaluate(&mut condition).into() {
            self.execute(then_branch.clone());

            then_branch
        } else if else_branch.is_some() {
            let else_branch = else_branch.unwrap();
            self.execute(else_branch.clone());

            else_branch
        } else {
            Statement::If(return_thing)
        }
    }

    fn visit_while_statement(&mut self, while_statement: &mut WhileStatement) -> Statement {
        let return_thing = while_statement.clone();
        let unboxed = while_statement.clone();
        let (body, mut condition) = (*(unboxed.body), unboxed.condition);

        while Into::<bool>::into(self.evaluate(&mut condition)) {
                self.execute(body.clone());
        }

        Statement::While(return_thing)
    }

    fn visit_block_statement(&mut self, block_statement: &mut BlockStatement) -> Statement {
        // println!("Before Block Execution");
        if let Some(return_val) = self.execute_block(block_statement.statements.clone()) {
            Statement::Return(return_val)
        } else {
            // println!("After Block Execution");
            Statement::Block(block_statement.clone())
        }
    }

    fn visit_function_statement(
        &mut self,
        function_statement: &mut FunctionStatement,
    ) -> Statement {
        //! Define user function declarations
        use crate::types::lux_functions::{
            Functions,
            user::UserFunction,
        };

        let function_name = &function_statement.name.lexeme;
        let function = Functions::User(UserFunction {
            closure: *self.enviroment.clone(),
            declaration: Box::new(function_statement.clone()),
        });
        self.enviroment
            .define(function_name, LiteralType::Callable(function));

        Statement::Function(function_statement.clone())
    }
}
