use crate::types::Expression;
use super::Callable;
#[derive(Debug, Clone)]
pub(crate) struct UserFunction {
    name: String,
    definition: Vec<Expression>
}
