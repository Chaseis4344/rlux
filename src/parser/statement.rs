use crate::interpreter::Interpreter;
use crate::macros::visitable_trait_two_elements as visitable_trait;
use crate::parser::{
    LiteralType,
    ParserError,
    TokenType,
};
use crate::types::{
    Expression,
    statement::*,
};

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

impl Visitable<Statement, Interpreter> for Statement {
    fn accept(&mut self, visitor: &mut Interpreter) -> Statement {
        match self {
            // Statement::Print(statement) => statement.accept(visitor),
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

visitable_trait! {Statement, IfStatement, Interpreter}
// visitable_trait! {Statement, PrintStatement, Interpreter}
visitable_trait! {Statement, ReturnStatement, Interpreter}
visitable_trait! {Statement, VariableStatement, Interpreter}
visitable_trait! {Statement, ExpressionStatement, Interpreter}
visitable_trait! {Statement, WhileStatement, Interpreter}
visitable_trait! {Statement, BlockStatement, Interpreter}
visitable_trait! {Statement, FunctionStatement, Interpreter}
