use super::expression::Visitable;
use super::interpreter::Interpreter;
use super::token::Token;
use super::LiteralType;
use crate::types::Expression;

#[derive(Clone, Debug)]
pub(crate) struct PrintStatement {
    pub(crate) expression: Expression,
}

#[derive(Clone, Debug)]
pub(crate) struct ExpressionStatement {
    pub(crate) expression: Expression,
}

#[derive(Clone, Debug)]
pub(crate) enum Statement {
    Print(PrintStatement),
    Expression(ExpressionStatement),
    Variable(VariableStatement),
}
#[derive(Clone, Debug)]
pub(crate) struct VariableStatement {
    pub(crate) name: Token,
    pub(crate) initalizer: Option<Expression>,
}

trait StatementVisitor {
    fn visit_print(&mut self, print: Box<&mut PrintStatement>) -> Statement;
    fn visit_expression(&mut self, expression: Box<&mut ExpressionStatement>) -> Statement;
    fn visit_variable(&mut self, var: Box<&mut VariableStatement>) -> Statement;
}

impl StatementVisitor for Interpreter {
    fn visit_expression(&mut self, expression: Box<&mut ExpressionStatement>) -> Statement {
        self.evaluate(&mut expression.expression);
        Statement::Print(PrintStatement {
            expression: expression.expression.clone(),
        })
    }
    fn visit_print(&mut self, print: Box<&mut PrintStatement>) -> Statement {
        let expression = self.evaluate(&mut print.expression);
        println!("{}", expression);
        Statement::Expression(ExpressionStatement {
            expression: print.expression.clone(),
        })
    }

    fn visit_variable(&mut self, var: Box<&mut VariableStatement>) -> Statement {
        // self.evaluate(&mut var.initalizer);
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

impl Visitable<Statement, Interpreter> for VariableStatement {
    fn accept(&mut self, visitor: &mut Interpreter) -> Statement {
        visitor.visit_variable(Box::new(self))
    }
}

impl Visitable<Statement, Interpreter> for Statement {
    fn accept(&mut self, visitor: &mut Interpreter) -> Statement {
        match self {
            Statement::Print(statement) => statement.accept(visitor),
            Statement::Expression(statement) => statement.accept(visitor),
            Statement::Variable(statement) => statement.accept(visitor),
        }
    }
}
