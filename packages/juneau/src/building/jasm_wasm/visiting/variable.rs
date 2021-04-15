use std::collections::HashMap;
use walrus::{LocalId, ValType};

use crate::{building::jasm_wasm::visitor::WasmBuilderVisitor, core::Visitor};
use crate::semantic::Variable;
use crate::semantic::jasm::Jasm;
use crate::building::BuildVisitor;

impl Visitor<&Variable<Jasm>, LocalId> for WasmBuilderVisitor {
    fn visit(&mut self, variable: &Variable<Jasm>) -> LocalId {
        let Variable {id, name, typ} = variable;
        let typ = ValType::from(typ);
        let variable = self.module.locals.add(typ);
        self.insert_variable(*id, variable);
        variable
    }
}