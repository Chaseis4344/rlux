use super::{
    Callable,
    Expression,
};
use crate::{
    interpreter::Interpreter,
    types::{
        Expression::Literal,
        LiteralType,
        expression::Literal as LiteralStruct,
    },
};
use std::time::SystemTime;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Clock {}

#[allow(unused_variables)]
impl Callable for Clock {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Expression>,
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

    #[inline]
    fn arity(&self) -> u64 {
        0
    }
}
