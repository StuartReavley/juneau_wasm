use crate::{building::{BuildVisitor, BuildVisitorInner, jasm_wasm::visitor::WasmBuilderVisitor}, core::{Id, IdContext, IdProvider, Visits}, semantic::{BinaryOperator, Function, FunctionType, Functions, Name, Parameter, Variable, jasm::{
    Block, Jasm, JasmExpression, JasmExpressionVisitor, JasmStatement,
    JasmStatementVisitor, JasmType, JasmValue, Struct,
}}};
use crate::{
core::{Visitor, VisitorWith},
semantic::{jasm::JasmPrimitiveImplementation, Implementation},
};
use std::{any::Any, rc::Rc, str::FromStr};
use walrus::ir::*;
use walrus::{FunctionBuilder, InstrSeqBuilder, LocalId, Module, ModuleConfig, ValType};


impl Visitor<&Function<Jasm>, ()> for WasmBuilderVisitor {
    fn visit(&mut self, function: &Function<Jasm>) -> () {

        let FunctionType { parameters, retrn } = function.get_type();

        // Convert from JasmType into ValType
        let params: Vec<ValType> = parameters.iter().map(|x| ValType::from(x)).collect();
        let results = ValType::from(&*retrn);
        let mut function_builder = FunctionBuilder::new(&mut self.module.types, &params, &[results]);


        let Function {
            id,
            name,
            parameters,
            implementation,
        } = function;

        let name:String = name.into();
        function_builder.name(name.clone());

        self.function_builder.push(function_builder);


        let parameters = self.visits(parameters);
        

        match implementation {
            Implementation::Semantic((block, typ)) => {
                // EXAMPLE OF VISITING STATEMENTS
                for statement in &block.0 {
                    self.visit(statement);
                }
            }
            Implementation::Primitive(typ, primitive) => match primitive {
                JasmPrimitiveImplementation::External { is_pure, ptr } => {
                    panic!("not supported in wasm, only llvm")
                }
                JasmPrimitiveImplementation::Unary(operator) => {
                    println!(
                        "Unary\nnumber_type: {:?}\noperator: {:?}\n",
                        typ, operator
                    );
                    todo!()
                }
                JasmPrimitiveImplementation::Binary(operator) => {
                    println!(
                        "Unary\nnumber_type: {:?}\noperator: {:?}\n",
                        typ, operator
                    );
                    todo!()
                }
            },
        }

        let final_func = self.function_builder.pop().finish(parameters, &mut self.module.funcs);


        // Export the final function.
        self.module.exports.add(&name, final_func);
    
    }
}
