use crate::types::statement::Statement;
use inkwell;
#[derive(Debug)]
pub(crate) enum IrBlock {
    Return,
}

pub(crate) fn statements_to_ir(statements: Vec<Statement>) -> IrBlock {
    IrBlock::Return
}
