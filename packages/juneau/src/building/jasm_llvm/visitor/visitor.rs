use std::collections::HashMap;
use std::{marker::PhantomData, rc::Rc};
use std::cell::RefCell;

use crate::{core::{Id, IdProvider, c::{CBox}}, semantic::{}};
use crate::semantic::{Function, Functions, Implementation};
use crate::semantic::jasm::{Jasm, JasmType};
use crate::target::llvm::{Context, Module, Builder, FunctionPassManager, Type};
use crate::building::{BuildVisitor, BuildVisitorInner};


#[derive(Clone)]
pub struct JasmBuildVisitor<'l> {
    pub ids: Rc<RefCell<IdProvider>>,
    pub functions: Functions<Jasm>,
    pub context: Rc<CBox<Context>>,
    pub is_optimized: bool,
    ph: PhantomData<&'l u8>
}


impl<'l> JasmBuildVisitor<'l> {
    pub fn new(ids:Rc<RefCell<IdProvider>>, functions:Functions<Jasm>, context:Rc<CBox<Context>>) -> Self {
        Self {ids, functions, context, is_optimized: false, ph: PhantomData}
    }
}

impl<'l> From<&mut JasmBuildVisitor<'l>> for &'l Context {
    fn from(context:&mut JasmBuildVisitor) -> Self {
        unsafe{context.context.as_ptr().into()}
    }
}