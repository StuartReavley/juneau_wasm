use std::rc::Rc;
use crate::{building::jasm_wasm::visitor::WasmBuilderVisitor, core::{concat, IdProvider}};
use crate::semantic::{Function, Functions};
use crate::semantic::jasm::Jasm;
use crate::parsing::jasm::JasmParseVisitor;
use crate::stdl::jasm::new_jasm_stdl;



pub fn new_default_visitors() -> (JasmParseVisitor, WasmBuilderVisitor) {
    new_visitors(Functions::new())
}

pub fn new_visitors(externals:Functions<Jasm>) -> (JasmParseVisitor, WasmBuilderVisitor) {
    let ids = IdProvider::new_cell(1);
    let (functions, symbols) = new_jasm_stdl(&mut *ids.borrow_mut(), &externals, &[]);
    ids.borrow_mut().increment_to(1001);
    let parse_visitor = JasmParseVisitor::new(ids.to_owned(), symbols);
    let build_visitor = WasmBuilderVisitor::new(ids, functions);
    (parse_visitor, build_visitor)
}