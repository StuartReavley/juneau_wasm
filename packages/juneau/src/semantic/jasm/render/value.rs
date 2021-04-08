use crate::core::Visitor;
use crate::semantic::jasm::JasmValue;
use super::JasmRenderVisitor;


impl Visitor<&JasmValue, String> for JasmRenderVisitor {
    fn visit(&mut self, value:&JasmValue) -> String {
        format!("{}", value)
    }
}