mod visiting;
pub use visiting::*;

pub mod test;


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


pub fn build_wasm_function(visitor:&mut WasmBuildVisitor, jasm: &Rc<Function<Jasm>>) -> Vec<u8> {

    visitor.visit(jasm);

    // emit wasm into buffer
    visitor.module.emit_wasm()
}

pub fn build_wasm_function_to_file(visitor:&mut WasmBuildVisitor, jasm: &Rc<Function<Jasm>>) -> anyhow::Result<()> {

    visitor.visit(jasm);
    
    // emit wasm into buffer
    visitor.module.emit_wasm_file("target/out.wasm")
}