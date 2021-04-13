use std::rc::Rc;
use crate::core::Visitor;
use crate::{semantic::jasm::JasmPrimitiveImplementation};
use crate::semantic::{Function, Implementation};
use crate::semantic::jasm::{Jasm, JasmType};
use crate::target::llvm::{self, Builder};
use crate::building::BuildVisitor;
use crate::building::jasm_llvm::JasmModuleBuildVisitor;


impl<'l> Visitor<Rc<Function<Jasm>>, Option<&'l llvm::Function>> for JasmModuleBuildVisitor<'l> {
    fn visit(&mut self, function:Rc<Function<Jasm>>) -> Option<&'l llvm::Function> {
        self.resolve_module_function(function.to_owned(), |visitor| {
            let Function {id, name, parameters, implementation} = function.as_ref();
            let function_type = function.get_type();
            let llvm_function_type = visitor.visit(&function_type);
            use Implementation::*;
            use JasmPrimitiveImplementation::*;

            // println!("FUNCTION {}", crate::semantic::jasm::render::render_jasm(function));
            match implementation {
                Primitive(_, External{ptr, ..}) => Some(visitor.module.add_external_function(name.into(), llvm_function_type, *ptr)),
                Primitive(_, _) => None,
                Semantic((block2, _)) => {
                    let function = visitor.module.add_function(name.into(), llvm_function_type);
                    visitor.push_builder();

                    let block = function.append("entry");
                    visitor.get_builder().position_at_end(block);
            
                    for (parameter, llvm_parameter) in parameters.iter().zip(function.parameters_iter()) {
                        visitor.visit((parameter, llvm_parameter));
                    }
            
                    visitor.visit(block2);
                    visitor.pop_builder();
                    assert!(function.verify(), "function.verify() failed for [{}]:\n\n{:?}\n\n", function.get_name().unwrap_or_default(), function);
                    visitor.pass_manager.map(|p|p.run(function));
                    Some(function)
                }
            }
        })
    }
}