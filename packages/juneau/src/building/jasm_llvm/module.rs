use std::rc::Rc;
use crate::core::Visitor;
use crate::{building::BuildVisitor, core::IdContext, semantic::{GetType, jasm::Block}};
use crate::semantic::{Module, Function, Functions, Implementation};
use crate::semantic::jasm::{Jasm, JasmExpression, JasmStatement};
use crate::target::llvm::{self, Llvm, FunctionPassManager, Builder};
use crate::building::jasm_llvm::{JasmBuildVisitor, JasmModuleBuildVisitor};
use crate::building::jasm_llvm::*;


pub fn build_jasm_expression<'l>(visitor:&mut JasmBuildVisitor<'l>, is_optimized:bool, expression:&JasmExpression) -> (&'l llvm::Module, Module<Llvm<'l>>) {
    let typ = expression.get_type();
    let implementation = Implementation::new((Block(vec![JasmStatement::Return(Some(expression.to_owned()))]), typ));
    let function = Function::new(visitor.ids.new_id(), "main".into(), Vec::new(), implementation);
    build_jasm_function(visitor, is_optimized, &function)
}

pub fn build_jasm_function<'l>(visitor:&mut JasmBuildVisitor<'l>, is_optimized:bool, function:&Rc<Function<Jasm>>) -> (&'l llvm::Module, Module<Llvm<'l>>) {
    let module = Module::new_with_main(function.id, function.to_owned());
    build_jasm(visitor, is_optimized, &module)
}

pub fn build_jasm<'l>(visitor:&mut JasmBuildVisitor<'l>, is_optimized:bool, module:&Module<Jasm>) -> (&'l llvm::Module, Module<Llvm<'l>>) {
    visitor.is_optimized = is_optimized;
    visitor.visit(module)
}