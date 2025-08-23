#[allow(unused_imports)]
use inkwell;
use crate::types::statement::*;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::OptimizationLevel;

#[derive(Debug)]
pub(crate) enum IrBlock {
    Return,
}
struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

fn expression_ir(expr: ExpressionStatement) -> IrBlock {
    todo!()
}

fn variable_ir(var: VariableStatement) -> IrBlock {
    todo!()
}


fn if_ir(if_statement: IfStatement) -> IrBlock {
    todo!()
}

fn block_ir(block_statement: BlockStatement) -> IrBlock {
    todo!()
}

fn function_ir(function_declaration: FunctionStatement) -> IrBlock {
    //TODO: Create Function in LLVM
}

pub(crate) fn statements_to_ir(statements: Vec<Statement>) -> IrBlock {

    for statement in statements {
        match statement {
        Statement::Expression(expression) => expression_ir(expression),
        Statement::Variable(var) => variable_ir(var),
        Statement::If(if_statement) => if_ir(if_statement),
        Statement::Block(block) => block_ir(block),
        Statement::While(_) => todo!(),
        Statement::Function(func) => function_ir(func), 
        Statement::Return(_) => todo!(),
        };
    }

    IrBlock::Return
}
