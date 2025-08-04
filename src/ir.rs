use inkwell;
use crate::types::statement::Statement;
#[derive(Debug)]
pub(crate) enum IrBlock {
    Return,
}

pub(crate) fn statements_to_ir(statements: Vec<Statement>) -> IrBlock {
    IrBlock::Return
}
