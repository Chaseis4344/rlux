use crate::interpreter::Interpreter;
use crate::parser::{LiteralType, ParserError, TokenType};
use crate::types::statement::*;
use crate::types::Expression;

mod parser_impl;
mod interpreter_impl;

///Internal shorthand to generate Visitor expressions for generating statements
macro_rules! visitable_trait {
    ($trait_type1:ty,  $enum_variant:ty, $enum_parent:ty) => {
        impl Visitable<$trait_type1, $enum_parent> for $enum_variant {
            paste::paste! {
                #[doc = "Redirect Visitors to `" $enum_variant "` version."]
                fn accept(&mut self, visitor: &mut $enum_parent) -> $trait_type1 {
                    paste::item! {visitor.[<visit_ $enum_variant:snake:lower>](self)}
                }
            }
        }
    };
}


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
    fn visit_function_statement(&mut self, function_statement: &mut FunctionStatement) -> Statement;
    fn visit_return_statement(&mut self, return_statement: &mut ReturnStatement) -> Statement;
}

visitable_trait! {Statement, IfStatement, Interpreter}
visitable_trait! {Statement, VariableStatement, Interpreter}
visitable_trait! {Statement, ExpressionStatement, Interpreter}
visitable_trait! {Statement, WhileStatement, Interpreter}
visitable_trait! {Statement, BlockStatement, Interpreter}
visitable_trait! {Statement, FunctionStatement, Interpreter}


impl Visitable<Statement, Interpreter> for Statement {
    fn accept(&mut self, visitor: &mut Interpreter) -> Statement {
        use crate::parser::statement::Visitable; 
        match self {
            // Statement::Print(statement) => statement.accept(visitor),
            Statement::Expression(statement) => statement.accept(visitor),
            Statement::Variable(statement) => statement.accept(visitor),
            Statement::If(statement) => statement.accept(visitor),
            Statement::While(statement) => statement.accept(visitor),
            Statement::Block(statement) => statement.accept(visitor),
            Statement::Function(statement) => statement.accept(visitor),
        }
    }
}

