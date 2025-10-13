use crate::interpreter::Interpreter;
use crate::parser::LiteralType;
use crate::parser::statement::*;

macro_rules! new_literal {
    ($value:expr) => {
        Expression::Literal(Box::new(crate::types::expression::Literal { value: $value }))
    };
}

impl StatementVisitor for Interpreter {
    fn visit_return_statement(&mut self, ret: &mut ReturnStatement) ->Statement {
        if ret.value.is_some(){
          let mut ret = ret.to_owned();
           ret.value = Some(new_literal!(self.evaluate(&mut ret.value.unwrap())));
          Statement::Return(ret)
        }else {
            Statement::Return(ret.to_owned())
        }
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
            self.evaluate(var.initalizer.as_mut().unwrap())
        } else {
            LiteralType::Nil
        };

        self.enviroment
            .define(var.name.lexeme.clone(), init.to_owned());

        let clone = var.clone();
        Statement::Variable(VariableStatement {
            name: clone.name,
            initalizer: clone.initalizer,
        })
    }
    fn visit_if_statement(&mut self, if_statement: &mut IfStatement) -> Statement {
        let unboxed = if_statement.to_owned();
        let return_thing = unboxed.clone();
        let mut condition = unboxed.condition;
        let then_branch = *(unboxed.then_branch);
        let else_branch = unboxed.else_branch;

        if self.evaluate(&mut condition).into() {
            self.execute(then_branch.clone());

            return then_branch;
        } else if else_branch.is_some() {
            let else_branch = else_branch.unwrap();
            self.execute(else_branch.clone());

            return else_branch;
        }

        Statement::If(return_thing)
    }

    fn visit_while_statement(&mut self, while_statement: &mut WhileStatement) -> Statement {
        let unboxed = while_statement.to_owned();
        let return_thing = unboxed.clone();
        let mut condition = unboxed.condition;
        let body = *(unboxed.body);

        while self.evaluate(&mut condition).into() {
            self.execute(body.clone());
        }

        Statement::While(return_thing)
    }

    fn visit_block_statement(&mut self, block_statement: &mut BlockStatement) -> Statement {
        // println!("Before Block Execution");
        if let Some(return_val) = self.execute_block(block_statement.statements.to_owned()){
            return Statement::Return(return_val);    
        }else{
        // println!("After Block Execution");
        Statement::Block(block_statement.to_owned())
        }
    }

    fn visit_function_statement(
        &mut self,
        function_statement: &mut FunctionStatement,
    ) -> Statement {
        use crate::types::lux_functions::Functions;
        use crate::types::lux_functions::user::UserFunction;

        let function_name = &function_statement.name.lexeme;
        let function = Functions::User(UserFunction {
            declaration: Box::new(function_statement.to_owned()),
        });
        self.enviroment
            .define(function_name.to_string(), LiteralType::Callable(function));

        Statement::Function(function_statement.to_owned())
    }
}
