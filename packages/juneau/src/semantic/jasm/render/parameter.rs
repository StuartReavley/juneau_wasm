use crate::core::Visitor;
use crate::semantic::Parameter;
use crate::semantic::jasm::Jasm;
use super::JasmRenderVisitor;


impl Visitor<&Parameter<Jasm>, String> for JasmRenderVisitor {
    fn visit(&mut self, parameter:&Parameter<Jasm>) -> String {
        let Parameter {name, typ, ..} = parameter;
        let typ = self.visit(typ);
        format!("{}:{}", name, typ)
    }
}