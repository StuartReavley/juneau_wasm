use std::rc::Rc;
use crate::core::{Visitor, Visits};
use crate::semantic::Function;
use crate::semantic::jasm::Jasm;
use super::JasmRenderVisitor;


impl Visitor<&Rc<Function<Jasm>>, String> for JasmRenderVisitor {
    fn visit(&mut self, function:&Rc<Function<Jasm>>) -> String {
        let Function {name, parameters, implementation, ..} = function.as_ref();
        let typ = self.visit(&*function.get_type().retrn);
        let parameters = self.visits(parameters).join(", ");
        let implementation = self.visit(implementation);
        format!("function {}({}):{} {}", name, parameters, typ, implementation)
    }
}