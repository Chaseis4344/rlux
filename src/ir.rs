use crate::types::statement::Statement;
pub(crate) enum IR_Block {
    OP_RETURN,
}

#[derive(Debug)]
pub struct Chunk {
    pub(crate) code: Vec<u8>,
}

fn write_chunk(mut chunk: Chunk, byte: u8) {
    chunk.code.push(byte);
}

pub(crate) fn statements_to_ir(states: Vec<Statement>) -> Vec<IR_Block> {
    let mut ir = vec![];
    for statement in state {
        ir.push(statement_to_ir(statement));
    }
    ir
}

pub(crate) fn statement_to_ir(statement: Statement) -> IR_Block {
    todo!()
}
