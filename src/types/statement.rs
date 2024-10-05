use super::{token::Token, Expression};

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
pub(crate) struct VariableStatement {
    pub(crate) name: Token,
    pub(crate) initalizer: Option<Expression>,
}

#[derive(Clone, Debug)]
pub(crate) struct WhileStatement {
    pub(crate) condition: Expression,
    pub(crate) body: Box<Statement>,
}

#[derive(Clone, Debug)]
pub(crate) struct BlockStatement {
    pub(crate) statements: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub(crate) enum Statement {
    Print(PrintStatement),
    Expression(ExpressionStatement),
    Variable(VariableStatement),
    While(WhileStatement),
    If(IfStatement),
    Block(BlockStatement),
}
