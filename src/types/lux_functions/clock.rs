use super::{Callable, Expression};
use crate::parser::interpreter::Interpreter;
use crate::types::expression::Literal as LiteralStruct;
use crate::types::Expression::Literal;
use crate::types::LiteralType;
use std::time::SystemTime;

#[derive(Clone, Debug)]
pub(crate) struct Clock {}

impl Callable for Clock {
    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Expression>) -> Expression {
        Literal(Box::new(LiteralStruct {
            value: LiteralType::Number(
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("Died getting Unix Time")
                    .as_secs() as f64,
            ),
        }))
    }
    fn arity(&self) -> u64 {
        0
    }
}
