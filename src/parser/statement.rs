use crate::interpreter::Interpreter;
use crate::parser::{LiteralType, ParserError, TokenType};
use crate::types::statement::*;
use crate::types::Expression;
use crate::macros::visitable_trait_two_elements as visitable_trait;

mod interpreter_impl;
mod parser_impl;
pub(crate) trait Visitable<T, U> {
    fn accept(&mut self, visitor: &mut U) -> T;
}

pub(crate) trait StatementVisitor {
    //Turning print into a native function
    //fn visit_print_statement(&mut self, print: &mut PrintStatement) -> Statement;
    fn visit_expression_statement(&mut self, expression: &mut ExpressionStatement) -> Statement;
    fn visit_variable_statement(&mut self, var: &mut VariableStatement) -> Statement;
    fn visit_if_statement(&mut self, if_statement: &mut IfStatement) -> Statement;
    fn visit_while_statement(&mut self, while_statement: &mut WhileStatement) -> Statement;
    fn visit_block_statement(&mut self, block_statement: &mut BlockStatement) -> Statement;
    fn visit_function_statement(&mut self, function_statement: &mut FunctionStatement)
        -> Statement;
    fn visit_return_statement(&mut self, return_statement: &mut ReturnStatement) -> Statement;
}

visitable_trait! {Statement, IfStatement, Interpreter}
visitable_trait! {Statement, VariableStatement, Interpreter}
visitable_trait! {Statement, ExpressionStatement, Interpreter}
visitable_trait! {Statement, WhileStatement, Interpreter}
visitable_trait! {Statement, BlockStatement, Interpreter}
visitable_trait! {Statement, FunctionStatement, Interpreter}
visitable_trait! {Statement, ReturnStatement, Interpreter}
impl StatementVisitor for Interpreter {
    fn visit_expression_statement(&mut self, expression: &mut ExpressionStatement) -> Statement {
        self.evaluate(&mut expression.expression);
        /* Statement::Print(PrintStatement {
            expression: expression.expression.clone(),
        })*/
        Statement::Expression(ExpressionStatement {
            expression: expression.expression.clone(),
        })
    }
    /*fn visit_print_statement(&mut self, print: &mut PrintStatement) -> Statement {
        let expression = self.evaluate(&mut print.expression);

        println!("{}", expression);
        Statement::Expression(ExpressionStatement {
            expression: print.expression.clone(),
        })
    }*/

    fn visit_return_statement(&mut self, return_statement: &mut ReturnStatement) -> Statement{
        todo!();
    }

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
        self.execute_block(block_statement.statements.to_owned());

        Statement::Block(block_statement.to_owned())
    }

    fn visit_function_statement(
        &mut self,
        function_statement: &mut FunctionStatement,
    ) -> Statement {
        use crate::types::lux_functions::user::UserFunction;
        use crate::types::lux_functions::Functions;

        let function_name = &function_statement.name.lexeme;
        let function = Functions::User(UserFunction {
            declaration: Box::new(function_statement.to_owned()),
        });

        // println!("Defined Function: {}", function_name);
        self.enviroment
            .define(function_name.to_string(), LiteralType::Callable(function));

        Statement::Function(function_statement.to_owned())
    }
}
>>>>>>> 24f5bd8 (feat: Return Statements)

impl Visitable<Statement, Interpreter> for Statement {
    fn accept(&mut self, visitor: &mut Interpreter) -> Statement {
        use crate::parser::statement::Visitable;
        match self {
            // Statement::Print(statement) => statement.accept(visitor),
            Statement::Return(statement) => statement.accept(visitor),
            Statement::Expression(statement) => statement.accept(visitor),
            Statement::Variable(statement) => statement.accept(visitor),
            Statement::If(statement) => statement.accept(visitor),
            Statement::While(statement) => statement.accept(visitor),
            Statement::Block(statement) => statement.accept(visitor),
            Statement::Function(statement) => statement.accept(visitor),
            Statement::Return(statement) => statement.accept(visitor),
        }
    }
}
<<<<<<< HEAD
=======

visitable_trait! {Statement, IfStatement, Interpreter}
// visitable_trait! {Statement, PrintStatement, Interpreter}
visitable_trait! {Statement, ReturnStatement, Interpreter}
visitable_trait! {Statement, VariableStatement, Interpreter}
visitable_trait! {Statement, ExpressionStatement, Interpreter}
visitable_trait! {Statement, WhileStatement, Interpreter}
visitable_trait! {Statement, BlockStatement, Interpreter}
visitable_trait! {Statement, FunctionStatement, Interpreter}
>>>>>>> 24f5bd8 (feat: Return Statements)
