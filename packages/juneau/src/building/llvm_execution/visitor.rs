use std::rc::Rc;
use std::marker::PhantomData;
use crate::{building::ModuleBuildVisitor, core::Visitor};
use crate::core::c::CSemiBox;
use crate::semantic::{Target, Functions};
use crate::target::llvm::{Llvm, JitEngine, JitOptions, ExecutionEngine, ExecutableFunction2, Module, Function};
use super::Execution;


pub struct ExecutionBuildVisitor<'l> {
    engine: Rc<CSemiBox<'l, JitEngine>>,
}

impl<'l> ExecutionBuildVisitor<'l> {
    pub fn new(module:&'l Module) -> Self {
        let engine = Rc::new(JitEngine::new(module, JitOptions {opt_level: 3}).unwrap());
        Self {engine}
    }
}

impl<'l> Visitor<&'l Function, Option<ExecutableFunction2<'l>>> for ExecutionBuildVisitor<'l> {
    fn visit(&mut self, function:&'l Function) -> Option<ExecutableFunction2<'l>> {
        Some(ExecutableFunction2::new(&self.engine, function))
    }
}

impl<'l> ModuleBuildVisitor<Llvm<'l>, Execution<'l>> for ExecutionBuildVisitor<'l> {}
