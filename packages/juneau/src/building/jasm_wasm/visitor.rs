use crate::{building::{BuildVisitor, BuildVisitorInner}, core::{Id, IdContext, IdProvider, Stack}, semantic::{BinaryOperator, Function, FunctionType, Functions, Name, Parameter, Variable, jasm::{
            Block, Jasm, JasmExpression, JasmExpressionVisitor, JasmStatement,
            JasmStatementVisitor, JasmType, JasmValue, Struct,
        }}};
use crate::{
    core::{Visitor, VisitorWith},
    semantic::{jasm::JasmPrimitiveImplementation, Implementation},
};
use std::{any::Any, cell::RefCell, rc::Rc, str::FromStr};
use walrus::ir::*;
use walrus::{FunctionBuilder, InstrSeqBuilder, LocalId, Module, ModuleConfig, ValType};

use super::JasmWasmBuild;

pub struct WasmBuilderVisitor {
    pub module: Module,
    pub function_builder: Stack<FunctionBuilder>,
    // BuildVisitorInner is a helper struct to keep track of functions and variables
    inner: BuildVisitorInner<JasmWasmBuild>
}

impl WasmBuilderVisitor {
    pub fn new(ids:Rc<RefCell<IdProvider>>, functions:Functions<Jasm>) -> WasmBuilderVisitor {
        // Construct a new Walrus module.
        let config = ModuleConfig::new();
        let module = Module::with_config(config);
        let function_builder = Stack::new();
        let inner = BuildVisitorInner::new(ids, functions);
        WasmBuilderVisitor {
            module,
            function_builder,
            inner
        }
    }
}


impl BuildVisitor<JasmWasmBuild> for WasmBuilderVisitor {
    fn get_inner(&self) -> &BuildVisitorInner<JasmWasmBuild> {
        &self.inner
    }
    fn get_inner_mut(&mut self) -> &mut BuildVisitorInner<JasmWasmBuild> {
        &mut self.inner
    }
    fn move_inner(self) -> BuildVisitorInner<JasmWasmBuild> {
        self.inner
    }
}

