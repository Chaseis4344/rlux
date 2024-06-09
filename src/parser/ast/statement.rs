use crate::{parser::interpreter::Interpreter, types::Expression};

use super::expression::{
    self, Binary, ExpressionVisitor, Grouping, Literal, Ternary, Unary, Visitable,
};

use crate::types::{ExpressionStatement, PrintStatement, Statement};

trait StatementVisitor {
    fn visit_print(&mut self, print: Box<&mut PrintStatement>);
    fn visit_expression(&mut self, expression: Box<&mut ExpressionStatement>);
}

impl StatementVisitor for Interpreter {
    fn visit_expression(&mut self, expression: Box<&mut ExpressionStatement>) {
        self.evaluate(&mut expression.expression);
    }
    fn visit_print(&mut self, print: Box<&mut PrintStatement>) {
        let expression = self.evaluate(&mut print.expression);
        println!("{}", expression);
    }
}

impl Visitable<Statement, Interpreter> for PrintStatement {
    fn accept(&mut self, visitor: &mut Interpreter) -> Statement {}
}

impl Visitable<Statement, Interpreter> for ExpressionStatement {
    fn accept(&mut self, visitor: &mut Interpreter) -> Statement {
        visitor.visit_expression(self);
    }
}
