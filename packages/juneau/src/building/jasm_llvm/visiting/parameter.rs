use std::collections::HashMap;
use crate::core::Visitor;
use crate::semantic::Parameter;
use crate::semantic::jasm::Jasm;
use crate::target::llvm::{self, Context, Builder, Type, Value};
use crate::building::BuildVisitor;
use crate::building::jasm_llvm::JasmModuleBuildVisitor;


impl<'v, 'l> Visitor<(&'v Parameter<Jasm>, &'l llvm::Parameter), &'l Value> for JasmModuleBuildVisitor<'l> {
    fn visit(&mut self, parameter:(&'v Parameter<Jasm>, &'l llvm::Parameter)) -> &'l Value {
        let (Parameter {id, name, typ}, llvm_parameter) = parameter;
        let builder = self.get_builder();
        let address = builder.build_allocation(name.into(), self.visit(typ));
        builder.build_store(llvm_parameter, address);
        self.insert_variable(*id, address.into());
        address    
    }
}
