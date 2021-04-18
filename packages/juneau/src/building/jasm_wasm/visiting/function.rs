use crate::{building::{BuildVisitor, BuildVisitorInner, jasm_wasm::visitor::WasmBuilderVisitor}, core::{Id, IdContext, IdProvider, Visits}, semantic::{BinaryOperator, Function, FunctionType, Functions, Name, Parameter, Variable, jasm::{
    Block, Jasm, JasmExpression, JasmExpressionVisitor, JasmStatement,
    JasmStatementVisitor, JasmType, JasmValue, Struct,
}}};
use crate::{
core::{Visitor, VisitorWith},
semantic::{jasm::JasmPrimitiveImplementation, Implementation},
};
use std::{any::Any, rc::Rc, str::FromStr};
use walrus::{FunctionId, ir::*};
use walrus::{FunctionBuilder, InstrSeqBuilder, LocalId, Module, ModuleConfig, ValType};


impl Visitor<&Rc<Function<Jasm>>, Option<FunctionId>> for WasmBuilderVisitor {
    fn visit(&mut self, function: &Rc<Function<Jasm>>) -> Option<FunctionId> {
        self.resolve_module_function(function.to_owned(), |visitor| {
            let Function {name, parameters, implementation, ..} = function.as_ref();
            use Implementation::*;
            use JasmPrimitiveImplementation::*;
            match implementation {
                Primitive(_, External{ptr, ..}) => {
                    todo!()
                }
                Primitive(_, _) => None,
                Semantic((block, _)) => {
                    let mut function_builder = {
                        let FunctionType { parameters, retrn } = function.get_type();
                        // Convert from JasmType into ValType
                        let params = visitor.visits(&parameters);
                        let results = visitor.visit(&*retrn);
                        FunctionBuilder::new(&mut visitor.module.types, &params, &[results])
                    };
            
                    let name:String = name.into();
                    function_builder.name(name.clone());
            
                    visitor.function_builder.push(function_builder);
                    
                    let parameters = visitor.visits(parameters);
            
                    visitor.visits(&block.0);
                            
                    let function = visitor.function_builder.pop().finish(parameters, &mut visitor.module.funcs);
                    // Export the final function.
                    visitor.module.exports.add(&name, function);
                    Some(function)
                }
            }
        })
    }
}
