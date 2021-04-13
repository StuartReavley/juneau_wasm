mod visiting;
pub use visiting::*;

mod test;

mod typ;

mod visitor;
use visitor::*;

mod wasm;
pub use wasm::*;

mod build;
pub use build::*;


use crate::{
    core::Id,
    semantic::{
        jasm::{
            Block, Jasm, JasmExpression, JasmExpressionVisitor, JasmStatement,
            JasmStatementVisitor, JasmType, JasmValue, Struct,
        },
        Function, FunctionType, Name, Parameter, Variable, BinaryOperator
    },
};
use crate::{
    core::{Visitor, VisitorWith},
    semantic::{jasm::JasmPrimitiveImplementation, Implementation},
};
use std::{any::Any, rc::Rc, str::FromStr};
use walrus::ir::*;
use walrus::{FunctionBuilder, InstrSeqBuilder, LocalId, Module, ModuleConfig, ValType};


pub fn compile_wasm(jasm: &Function<Jasm>) -> Vec<u8> {

        
    // Build instance of wasm builder visitor
    let mut func_jasm = WasmBuilderVisitor::new();

    func_jasm.visit(jasm);
    

    
    // emit wasm into buffer
    func_jasm.module.emit_wasm()
}

pub fn compile_wasm_to_file(jasm: &Function<Jasm>) -> anyhow::Result<()> {
    // Build instance of wasm builder visitor
    let mut func_jasm = WasmBuilderVisitor::new();

    func_jasm.visit(jasm);
    

        
    // emit wasm into buffer
    func_jasm.module.emit_wasm_file("target/out.wasm")
}