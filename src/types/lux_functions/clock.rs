use super::{Callable, Expression};
use crate::{
    interpreter::Interpreter,
    types::{Expression::Literal, LiteralType, expression::Literal as LiteralStruct},
};
use std::time::SystemTime;

#[derive(Clone, Debug, PartialEq)]
/// Exposes Clock logic from OS to the language
pub(crate) struct Clock {}

impl Callable for Clock {
    fn call(
        &mut self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Expression>,
    ) -> Option<Expression> {
        Some(Literal(Box::new(LiteralStruct {
            value: LiteralType::Number(
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("Died getting Unix Time")
                    .as_secs() as f64,
            ),
        })))
    }
    fn arity(&self) -> u64 {
        0
    }
}
