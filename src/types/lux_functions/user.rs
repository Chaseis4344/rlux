use super::Callable;
use crate::types::Expression;
#[derive(Debug, Clone)]
pub(crate) struct UserFunction {
    name: String,
    definition: Vec<Expression>,
}
