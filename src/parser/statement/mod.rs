use super::interpreter::Interpreter;
use super::{LiteralType, ParserError, TokenType};
use crate::types::statement::*;
use crate::types::Expression;

mod parser_impl;

pub(crate) trait Visitable<T, U> {
    fn accept(&mut self, visitor: &mut U) -> T;
}

trait StatementVisitor {
    fn visit_print_statement(&mut self, print: Box<&mut PrintStatement>) -> Statement;
    fn visit_expression_statement(
        &mut self,
        expression: Box<&mut ExpressionStatement>,
    ) -> Statement;
    fn visit_variable_statement(&mut self, var: Box<&mut VariableStatement>) -> Statement;
    fn visit_if_statement(&mut self, if_statement: Box<&mut IfStatement>) -> Statement;
    fn visit_while_statement(&mut self, while_statement: Box<&mut WhileStatement>) -> Statement;
    fn visit_block_statement(&mut self, block_statement: Box<&mut BlockStatement>) -> Statement;
}

impl StatementVisitor for Interpreter {
    fn visit_expression_statement(
        &mut self,
        expression: Box<&mut ExpressionStatement>,
    ) -> Statement {
        self.evaluate(&mut expression.expression);
        Statement::Print(PrintStatement {
            expression: expression.expression.clone(),
        })
    }
    fn visit_print_statement(&mut self, print: Box<&mut PrintStatement>) -> Statement {
        let expression = self.evaluate(&mut print.expression);

        println!("{}", expression);
        Statement::Expression(ExpressionStatement {
            expression: print.expression.clone(),
        })
    }

    fn visit_variable_statement(&mut self, var: Box<&mut VariableStatement>) -> Statement {
        let init: LiteralType;

        if var.initalizer.is_some() {
            init = self.evaluate(&mut var.initalizer.as_mut().unwrap());
        } else {
            init = LiteralType::Nil;
        }

        self.enviroment.define(var.name.clone(), init);

        let clone = var.clone();
        Statement::Variable(VariableStatement {
            name: clone.name,
            initalizer: clone.initalizer,
        })
    }
    fn visit_if_statement(&mut self, if_statement: Box<&mut IfStatement>) -> Statement {
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

    fn visit_while_statement(&mut self, while_statement: Box<&mut WhileStatement>) -> Statement {
        let unboxed = while_statement.to_owned();
        let return_thing = unboxed.clone();
        let mut condition = unboxed.condition;
        let body = *(unboxed.body);

        while self.evaluate(&mut condition).into() {
            self.execute(body.clone());
        }

        Statement::While(return_thing)
    }

    fn visit_block_statement(&mut self, block_statement: Box<&mut BlockStatement>) -> Statement {
        self.execute_block(block_statement.statements.to_owned());

        Statement::Block(block_statement.to_owned())
    }
}

impl Visitable<Statement, Interpreter> for Statement {
    fn accept(&mut self, visitor: &mut Interpreter) -> Statement {
        match self {
            Statement::Print(statement) => statement.accept(visitor),
            Statement::Expression(statement) => statement.accept(visitor),
            Statement::Variable(statement) => statement.accept(visitor),
            Statement::If(statement) => statement.accept(visitor),
            Statement::While(statement) => statement.accept(visitor),
            Statement::Block(statement) => statement.accept(visitor),
        }
    }
}

macro_rules! visitable_trait {
    ($trait_type1:ty,  $enum_variant:ty, $enum_parent:ty) => {
        impl Visitable<$trait_type1, $enum_parent> for $enum_variant {
            paste::paste! {
                #[doc = "Redirect Visitors to `" $enum_variant "` version."]
                fn accept(&mut self, visitor: &mut $enum_parent) -> $trait_type1 {
                    paste::item! {visitor.[<visit_ $enum_variant:snake:lower>](Box::new(self))}
                }
            }
        }
    };
}

visitable_trait! {Statement, IfStatement, Interpreter}
visitable_trait! {Statement, PrintStatement, Interpreter}
visitable_trait! {Statement, VariableStatement, Interpreter}
visitable_trait! {Statement, ExpressionStatement, Interpreter}
visitable_trait! {Statement, WhileStatement, Interpreter}
visitable_trait! {Statement, BlockStatement, Interpreter}
