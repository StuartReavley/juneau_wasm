use std::rc::Rc;
use crate::core::{concat, IdProvider};
use crate::target::llvm::Context;
use crate::semantic::{Function, Functions};
use crate::semantic::jasm::Jasm;
use crate::parsing::jasm::JasmParseVisitor;
use crate::stdl::jasm::new_jasm_stdl;
use crate::building::jasm_llvm::JasmBuildVisitor;



pub fn new_default_contexts() -> (JasmParseVisitor, JasmBuildVisitor<'static>) {
    new_contexts(Functions::new())
}

pub fn new_contexts(externals:Functions<Jasm>) -> (JasmParseVisitor, JasmBuildVisitor<'static>) {
    let context = Rc::new(Context::new());
    let ids = IdProvider::new_cell(1);
    let (functions, symbols) = new_jasm_stdl(&mut *ids.borrow_mut(), &externals, &[]);
    ids.borrow_mut().increment_to(1001);
    let parse_context = JasmParseVisitor::new(ids.to_owned(), symbols);
    let build_context = JasmBuildVisitor::new(ids, functions, context);
    (parse_context, build_context)
}