use super::expression::{
    self, Binary, ExpressionVisitor, Grouping, Literal, Ternary, Unary, Visitable,
};
use super::interpreter::Interpreter;
use crate::types::Expression;

use crate::types::{ExpressionStatement, PrintStatement, Statement};

trait StatementVisitor {
    fn visit_print(&mut self, print: Box<&mut PrintStatement>) -> Statement;
    fn visit_expression(&mut self, expression: Box<&mut ExpressionStatement>) -> Statement;
}

impl StatementVisitor for Interpreter {
    fn visit_expression(&mut self, expression: Box<&mut ExpressionStatement>) -> Statement {
        self.evaluate(&mut expression.expression);
        Statement::PrintStatement(PrintStatement {
            expression: expression.expression.clone(),
        })
    }
    fn visit_print(&mut self, print: Box<&mut PrintStatement>) -> Statement {
        let expression = self.evaluate(&mut print.expression);
        println!("{}", expression);
        Statement::ExpressionStatement(ExpressionStatement {
            expression: print.expression.clone(),
        })
    }
}

impl Visitable<Statement, Interpreter> for PrintStatement {
    fn accept(&mut self, visitor: &mut Interpreter) -> Statement {
        visitor.visit_print(Box::new(self))
    }
}

impl Visitable<Statement, Interpreter> for ExpressionStatement {
    fn accept(&mut self, visitor: &mut Interpreter) -> Statement {
        visitor.visit_expression(Box::new(self))
    }
}

impl Visitable<Statement, Interpreter> for Statement {
    fn accept(&mut self, visitor: &mut Interpreter) -> Statement {
        match self {
            Statement::PrintStatement(statement) => statement.accept(visitor),
            Statement::ExpressionStatement(statement) => statement.accept(visitor),
        }
    }
}
