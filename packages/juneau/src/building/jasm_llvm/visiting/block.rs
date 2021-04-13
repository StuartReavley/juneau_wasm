use crate::core::{Visitor, Visits};
use crate::building::jasm_llvm::JasmModuleBuildVisitor;
use crate::semantic::jasm::Block;


impl<'l> Visitor<&Block, ()> for JasmModuleBuildVisitor<'l> {
    fn visit(&mut self, block:&Block) {
        self.visits(&block.0);
    }
}