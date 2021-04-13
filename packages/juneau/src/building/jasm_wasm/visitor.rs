use crate::{
    core::Id,
    semantic::{
        jasm::{
            Block, Jasm, JasmExpression, JasmExpressionVisitor, JasmStatement,
            JasmStatementVisitor, JasmType, JasmValue, Struct,
        },
        Function, FunctionType, Name, Parameter, Variable, BinaryOperator
    },
};
use crate::{
    core::{Visitor, VisitorWith},
    semantic::{jasm::JasmPrimitiveImplementation, Implementation},
};
use std::{any::Any, rc::Rc, str::FromStr};
use walrus::ir::*;
use walrus::{FunctionBuilder, InstrSeqBuilder, LocalId, Module, ModuleConfig, ValType};

pub struct WasmBuilderVisitor {
    pub module: Module,
    pub function_builder: FunctionBuilder
}

impl WasmBuilderVisitor {
    pub fn new(config:ModuleConfig, params:&[ValType], results:&[ValType]) -> WasmBuilderVisitor {
        let mut module = Module::with_config(config);
        let mut function_builder = FunctionBuilder::new(&mut module.types, params, results);
        WasmBuilderVisitor {
            module: module,
            function_builder: function_builder
        }
    }

    
    pub fn name(&mut self, name:String) -> () {
        self.function_builder.name(name);
    }
}

impl Visitor<&Function<Jasm>, ()> for WasmBuilderVisitor {
    fn visit(&mut self, function: &Function<Jasm>) -> () {
        let Function {
            id,
            name,
            parameters,
            implementation,
        } = function;

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
                JasmPrimitiveImplementation::Unary(number_type, operator) => {
                    println!(
                        "Unary\nnumber_type: {:#?}\ntype: {:?}\noperator: {:?}\n",
                        number_type, typ, operator
                    );
                    todo!()
                }
                JasmPrimitiveImplementation::Binary(number_type, operator) => {
                    println!(
                        "Unary\nnumber_type: {:#?}\ntype: {:?}\noperator: {:?}\n",
                        number_type, typ, operator
                    );
                    todo!()
                }
            },
        }
    }
}
