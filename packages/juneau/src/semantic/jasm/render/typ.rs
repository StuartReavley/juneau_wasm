use crate::core::Visitor;
use crate::semantic::jasm::JasmType;
use super::JasmRenderVisitor;


impl Visitor<&JasmType, String> for JasmRenderVisitor {
    fn visit(&mut self, typ:&JasmType) -> String {
        format!("{}", typ)
    }
}