use crate::types::{token::Token, Expression, LiteralType};

#[derive(Clone)]
pub struct Ternary {
    pub evaluator: Expression,
    pub left: Expression,
    pub right: Expression,
}

#[derive(Clone)]
pub struct Literal {
    pub value: LiteralType,
}

#[derive(Clone)]
pub struct Unary {
    pub operator: Token,
    pub operand: Expression,
}

#[derive(Clone)]
pub struct Binary {
    pub operator: Token,
    pub left: Expression,
    pub right: Expression,
}

#[derive(Clone)]
pub struct Grouping {
    pub expression: Expression,
}

pub trait ExpressionVisitor<T> {
    fn visit_grouping(&mut self, group: Box<&mut Grouping>) -> T;
    fn visit_binary(&mut self, bin: Box<&mut Binary>) -> T;
    fn visit_unary(&mut self, unary: Box<&mut Unary>) -> T;
    fn visit_literal(&mut self, lit: Box<&mut Literal>) -> T;
    fn visit_ternary(&mut self, tern: Box<&mut Ternary>) -> T;
}

pub trait Visitable<T, U> {
    fn accept(&mut self, visitor: &mut U) -> T;
}

impl Visitable<String, Expression> for Ternary {
    fn accept(&mut self, visitor: &mut Expression) -> String {
        visitor.visit_ternary(Box::new(self))
    }
}

impl Visitable<String, Expression> for Grouping {
    fn accept(&mut self, visitor: &mut Expression) -> String {
        visitor.visit_grouping(Box::new(self))
    }
}

impl Visitable<String, Expression> for Binary {
    fn accept(&mut self, visitor: &mut Expression) -> String {
        visitor.visit_binary(Box::new(self))
    }
}

impl Visitable<String, Expression> for Unary {
    fn accept(&mut self, visitor: &mut Expression) -> String {
        visitor.visit_unary(Box::new(self))
    }
}

impl Visitable<String, Expression> for Literal {
    fn accept(&mut self, visitor: &mut Expression) -> String {
        visitor.visit_literal(Box::new(self))
    }
}

impl ExpressionVisitor<String> for Expression {
    //Visiting is really just a fancy version of self-selection with a level of indirection layered on top
    fn visit_binary(&mut self, bin: Box<&mut Binary>) -> String {
        self.parenthesize(
            bin.operator.lexeme.clone(),
            vec![&mut bin.left, &mut bin.right],
        )
    }

    fn visit_grouping(&mut self, group: Box<&mut Grouping>) -> String {
        self.parenthesize("group".to_string(), vec![&mut group.expression])
    }

    fn visit_literal(&mut self, lit: Box<&mut Literal>) -> String {
        lit.value.to_string()
    }

    fn visit_unary(&mut self, unary: Box<&mut Unary>) -> String {
        self.parenthesize(unary.operator.lexeme.clone(), vec![&mut unary.operand])
    }

    fn visit_ternary(&mut self, tern: Box<&mut Ternary>) -> String {
        self.parenthesize(
            "ternary".to_string(),
            vec![&mut tern.evaluator, &mut tern.left, &mut tern.right],
        )
    }
}

impl Visitable<String, Expression> for Expression {
    fn accept(&mut self, visitor: &mut Expression) -> String {
        match self {
            Expression::Binary(bin) => bin.accept(visitor),
            Expression::Literal(lit) => lit.accept(visitor),
            Expression::Grouping(group) => group.accept(visitor),
            Expression::Unary(unary) => unary.accept(visitor),
            Expression::Ternary(tern) => tern.accept(visitor),
        }
    }
}
