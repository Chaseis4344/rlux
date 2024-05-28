use crate::token::Token;
use crate::types::LiteralType;

pub enum Expression {
    Grouping(Box<Grouping>),
    Unary(Box<Unary>),
    Binary(Box<Binary>),
    Literal(Box<Literal>),
}

pub struct Literal {
    pub value: LiteralType,
}

pub struct Unary {
    pub operator: Token,
    pub operand: Expression,
}

pub struct Binary {
    pub operator: Token,
    pub left: Expression,
    pub right: Expression,
}

pub struct Grouping {
    pub expression: Expression,
}

pub trait Visitor<T> {
    fn visit_grouping(&mut self, group: Box<&mut Grouping>) -> T;
    fn visit_binary(&mut self, bin: Box<&mut Binary>) -> T;
    fn visit_unary(&mut self, unary: Box<&mut Unary>) -> T;
    fn visit_literal(&mut self, lit: Box<&mut Literal>) -> T;
}

pub trait Visitable<T> {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> String;
}

impl Visitable<String> for Grouping {
    fn accept(&mut self, visitor: &mut dyn Visitor<String>) -> String {
        visitor.visit_grouping(Box::new(self))
    }
}

impl Visitable<String> for Binary {
    fn accept(&mut self, visitor: &mut dyn Visitor<String>) -> String {
        visitor.visit_binary(Box::new(self))
    }
}

impl Visitable<String> for Unary {
    fn accept(&mut self, visitor: &mut dyn Visitor<String>) -> String {
        visitor.visit_unary(Box::new(self))
    }
}

impl Visitable<String> for Literal {
    fn accept(&mut self, visitor: &mut dyn Visitor<String>) -> String {
        visitor.visit_literal(Box::new(self))
    }
}

impl Visitor<String> for Expression {
    //Visiting is really just a fancy version of self-selection with a level of indirection layered on top
    fn visit_binary(&mut self, bin: Box<&mut Binary>) -> String {
        self.parenthesize(
            bin.operator.lexeme.clone(),
            vec![&mut bin.left, &mut bin.right],
        )
    }

    fn visit_grouping(&mut self, group: Box<&mut Grouping>) -> String {
        self.parenthesize(String::from("group"), vec![&mut group.expression])
    }

    fn visit_literal(&mut self, lit: Box<&mut Literal>) -> String {
        lit.value.to_string()
    }

    fn visit_unary(&mut self, unary: Box<&mut Unary>) -> String {
        self.parenthesize(unary.operator.lexeme.clone(), vec![&mut unary.operand])
    }
}
impl Expression {
    pub fn print(&mut self, expression: &mut dyn Visitable<String>) -> String {
        expression.accept(self)
    }

    fn parenthesize(&mut self, name: String, expressions: Vec<&mut Expression>) -> String {
        let mut result = String::from("");

        result.push('(');
        result.push_str(&name);
        for expression in expressions {
            result.push(' ');
            result.push_str(&mut expression.accept(self));
        }
        result.push(')');

        result
    }
}

impl Visitable<String> for Expression {
    fn accept(&mut self, visitor: &mut dyn Visitor<String>) -> String {
        match self {
            Expression::Binary(bin) => bin.accept(visitor),
            Expression::Literal(lit) => lit.accept(visitor),
            Expression::Grouping(group) => group.accept(visitor),
            Expression::Unary(unary) => unary.accept(visitor),
        }
    }
}
