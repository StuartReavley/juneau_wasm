use crate::core::Visitor;
use crate::semantic::{Parameter, Implementation};
use crate::semantic::jasm::Jasm;
use super::JasmRenderVisitor;


impl Visitor<&Implementation<Jasm>, String> for JasmRenderVisitor {
    fn visit(&mut self, implementation:&Implementation<Jasm>) -> String {
        use Implementation::*;
        match implementation {
            Semantic((block, _)) => self.visit(block),
            Primitive(_typ, implementation) => format!("{:?}", implementation)
        }
    }
}