use crate::types::{
    LiteralType,
    expression::{
        Call,
        *,
    },
};

///Shorthand to internally generate accept() functions for the Enum Variant and type passed in,
///internally will result in performing the corresponding instruction
macro_rules! visitable_trait {
    ($trait_type:ty,$enum_variant:ty, $enum_parent:ty) => {
        impl Visitable<$trait_type> for $enum_variant {
            fn accept(&mut self, visitor: &mut dyn InterpreterVisitor<$trait_type>) -> $trait_type {
                paste::item! {visitor.[<visit_ $enum_variant:lower>](self)}
            }
        }
    };
}

///Required Trait for Interpreter to garuntee we can accomadate all expressions
pub(crate) trait InterpreterVisitor<T> {
    fn visit_grouping(&mut self, group: &mut Grouping) -> T;
    fn visit_binary(&mut self, bin: &mut Binary) -> T;
    fn visit_unary(&mut self, unary: &mut Unary) -> T;
    fn visit_literal(&mut self, lit: &mut Literal) -> T;
    fn visit_ternary(&mut self, tern: &mut Ternary) -> T;
    fn visit_variable(&mut self, var: &mut Variable) -> T;
    fn visit_assignment(&mut self, assign: &mut Assignment) -> T;
    fn visit_logical(&mut self, logical: &mut Logical) -> T;
    fn visit_call(&mut self, call: &mut Call) -> T;
    fn visit_lambda(&mut self, lambda: &mut Lambda) -> T;
}

pub(crate) trait Visitable<T> {
    fn accept(&mut self, visitor: &mut dyn InterpreterVisitor<T>) -> T;
}

visitable_trait! {LiteralType,Binary,Expression}
visitable_trait! {LiteralType,Literal,Expression}
visitable_trait! {LiteralType,Grouping,Expression}
visitable_trait! {LiteralType,Unary,Expression}
visitable_trait! {LiteralType,Ternary,Expression}
visitable_trait! {LiteralType,Variable,Expression}
visitable_trait! {LiteralType,Assignment,Expression}
visitable_trait! {LiteralType,Logical,Expression}
visitable_trait! {LiteralType,Call,Expression}
visitable_trait! {LiteralType,Lambda,Expression}
