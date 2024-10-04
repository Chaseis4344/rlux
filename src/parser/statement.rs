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
pub(crate) struct IfStatement {
    pub(crate) condition: Expression,
    pub(crate) then_branch: Box<Statement>,
    pub(crate) else_branch: Box<Option<Statement>>,
}

#[derive(Clone, Debug)]
pub(crate) enum Statement {
    Print(PrintStatement),
    Expression(ExpressionStatement),
    Variable(VariableStatement),
    If(IfStatement),
}
#[derive(Clone, Debug)]
pub(crate) struct VariableStatement {
    pub(crate) name: Token,
    pub(crate) initalizer: Option<Expression>,
}

trait StatementVisitor {
    fn visit_print_statement(&mut self, print: Box<&mut PrintStatement>) -> Statement;
    fn visit_expression_statement(
        &mut self,
        expression: Box<&mut ExpressionStatement>,
    ) -> Statement;
    fn visit_variable_statement(&mut self, var: Box<&mut VariableStatement>) -> Statement;
    fn visit_if_statement(&mut self, if_statement: Box<&mut IfStatement>) -> Statement;
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
        if self.evaluate(&mut if_statement.condition) == LiteralType::Boolean(true) {
            self.execute(*if_statement.then_branch.clone());
        } else {
            match (*(if_statement.else_branch)).clone() {
                Some(statement) => {
                    self.execute(statement);
                    return (*(if_statement.else_branch)).to_owned().unwrap();
                }
                None => {}
            }
        }
        *if_statement.then_branch.to_owned()
    }
}

impl Visitable<Statement, Interpreter> for Statement {
    fn accept(&mut self, visitor: &mut Interpreter) -> Statement {
        match self {
            Statement::Print(statement) => statement.accept(visitor),
            Statement::Expression(statement) => statement.accept(visitor),
            Statement::Variable(statement) => statement.accept(visitor),
            Statement::If(statement) => statement.accept(visitor),
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

