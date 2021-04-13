mod visiting;
pub use visiting::*;

mod test;

mod typ;

mod visitor;
use visitor::*;

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
    let FunctionType { parameters, retrn } = jasm.get_type();

    // Construct a new Walrus module.
    let config = ModuleConfig::new();
    
    // Convert from JasmType into ValType
    let params: Vec<ValType> = parameters.iter().map(|x| ValType::from(x)).collect();
    let results = ValType::from(&*retrn);
    
    // Build instance of wasm builder visitor
    let mut func_jasm = WasmBuilderVisitor::new(config, &params, &[results]);

    let func_name = String::from(&jasm.name);
    func_jasm.name(func_name.clone());
    
    let param: Vec<LocalId> = params
    .iter()
    .map(|x| func_jasm.module.locals.add(*x))
    .collect();
    // func_jasm.visit_with(jasm, &mut module);
    func_jasm.visit(jasm);
    

    let final_func = func_jasm.function_builder.finish(param, &mut func_jasm.module.funcs);

    // Export the final function.
    func_jasm.module.exports.add(&func_name, final_func);
    
    // emit wasm into buffer
    func_jasm.module.emit_wasm()
}

pub fn compile_wasm_to_file(jasm: &Function<Jasm>) -> anyhow::Result<()> {
    let FunctionType { parameters, retrn } = jasm.get_type();

    // Construct a new Walrus module.
    let config = ModuleConfig::new();
    
    // Convert from JasmType into ValType
    let params: Vec<ValType> = parameters.iter().map(|x| ValType::from(x)).collect();
    let results = ValType::from(&*retrn);
    
    // Build instance of wasm builder visitor
    let mut func_jasm = WasmBuilderVisitor::new(config, &params, &[results]);

    let func_name = String::from(&jasm.name);
    func_jasm.name(func_name.clone());
    
    let param: Vec<LocalId> = params
    .iter()
    .map(|x| func_jasm.module.locals.add(*x))
    .collect();
    // func_jasm.visit_with(jasm, &mut module);
    func_jasm.visit(jasm);
    

    let final_func = func_jasm.function_builder.finish(param, &mut func_jasm.module.funcs);

    // Export the final function.
    func_jasm.module.exports.add(&func_name, final_func);
        
    // emit wasm into buffer
    func_jasm.module.emit_wasm_file("target/out.wasm")
}