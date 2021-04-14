use std::collections::HashMap;
use walrus::{LocalId, ValType};

use crate::{building::jasm_wasm::visitor::WasmBuilderVisitor, core::Visitor};
use crate::semantic::Parameter;
use crate::semantic::jasm::Jasm;
use crate::building::BuildVisitor;

impl Visitor<&Parameter<Jasm>, LocalId> for WasmBuilderVisitor {
    fn visit(&mut self, parameter: &Parameter<Jasm>) -> LocalId {
        let Parameter {id, name, typ} = parameter;
        let typ = ValType::from(typ);
        let parameter = self.module.locals.add(typ);
        self.insert_variable(*id, parameter);
        parameter
    }
}