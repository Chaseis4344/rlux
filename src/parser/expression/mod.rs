use crate::types::{token::Token, Expression, LiteralType};
mod parser_impl;

#[derive(Clone, Debug)]
pub(crate) struct Ternary {
    pub(crate) evaluator: Expression,
    pub(crate) left: Expression,
    pub(crate) right: Expression,
}

#[derive(Clone, Debug)]
pub(crate) struct Literal {
    pub(crate) value: LiteralType,
}

#[derive(Clone, Debug)]
pub(crate) struct Unary {
    pub(crate) operator: Token,
    pub(crate) operand: Expression,
}

#[derive(Clone, Debug)]
pub struct Binary {
    pub(crate) operator: Token,
    pub(crate) left: Expression,
    pub(crate) right: Expression,
}

#[derive(Clone, Debug)]
pub struct Grouping {
    pub(crate) expression: Expression,
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub(crate) name: Token,
}

#[derive(Clone, Debug)]
pub struct Assignment {
    pub(crate) name: Token,
    pub(crate) value: Expression,
}

#[derive(Clone, Debug)]
pub struct Logical {
    pub(crate) left: Expression,
    pub(crate) right: Expression,
    pub(crate) operator: Token,
}

/*
    fn visit_X(&mut self, Y: Box<&mut X>) -> T;
*/

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

pub(crate) trait ExpressionVisitor<T> {
    fn visit_grouping(&mut self, group: Box<&mut Grouping>) -> T;
    fn visit_binary(&mut self, bin: Box<&mut Binary>) -> T;
    fn visit_unary(&mut self, unary: Box<&mut Unary>) -> T;
    fn visit_literal(&mut self, lit: Box<&mut Literal>) -> T;
    fn visit_ternary(&mut self, tern: Box<&mut Ternary>) -> T;
    fn visit_variable(&mut self, var: Box<&mut Variable>) -> T;
    fn visit_assignment(&mut self, assign: Box<&mut Assignment>) -> T;
    fn visit_logical(&mut self, logical: Box<&mut Logical>) -> T;
}

pub(crate) trait Visitable<T, U> {
    fn accept(&mut self, visitor: &mut U) -> T;
}

visitable_trait! {String,Binary,Expression}
visitable_trait! {String,Literal,Expression}
visitable_trait! {String,Grouping,Expression}
visitable_trait! {String,Unary,Expression}
visitable_trait! {String,Ternary,Expression}
visitable_trait! {String,Variable,Expression}
visitable_trait! {String,Assignment,Expression}
visitable_trait! {String,Logical,Expression}

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
    fn visit_variable(&mut self, _var: Box<&mut Variable>) -> String {
        String::from("")
    }
    fn visit_assignment(&mut self, _assign: Box<&mut Assignment>) -> String {
        String::from("")
    }
    fn visit_logical(&mut self, _assign: Box<&mut Logical>) -> String {
        String::from("")
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
            Expression::Variable(var) => var.accept(visitor),
            Expression::Assignment(assign) => assign.accept(visitor),
            Expression::Logical(logic) => logic.accept(visitor),
        }
    }
}

impl Expression {
    fn parenthesize(&mut self, _name: String, _terms: Vec<&mut Expression>) -> String {
        todo!("Implement Parenthesization for grouping vision");
    }
}
