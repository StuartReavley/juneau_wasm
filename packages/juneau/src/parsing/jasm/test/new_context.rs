use crate::core::IdProvider;
use crate::semantic::{Variable, Functions};
use crate::semantic::jasm::Jasm;
use crate::parsing::jasm::JasmParseVisitor;
use crate::stdl::jasm::new_jasm_stdl;



pub fn new_default_context() -> JasmParseVisitor {
    new_context(&[])
}

pub fn new_context(symbols:&[Variable<Jasm>]) -> JasmParseVisitor {
    let ids = IdProvider::new_cell(1);
    let (functions, symbols) = new_jasm_stdl(&mut *ids.borrow_mut(), &Functions::new(), symbols);
    ids.borrow_mut().increment_to(1001);
    JasmParseVisitor::new(ids, symbols)
}
