use super::Expression;
use crate::parser::interpreter::Interpreter;
pub(crate) mod clock;
pub(crate) mod print;
pub(crate) mod user;

pub(crate) trait Callable {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Expression>,
    ) -> Option<Expression>;
    fn arity(&self) -> u64;
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Functions {
    Clock(clock::Clock),
    Print(print::Print),
    User(user::UserFunction),
}
