use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::{core::{Id, IdContext, IdProvider, Stack, c::{CBox, CSemiBox}}, semantic::{Parameter, jasm::Struct}};
use crate::semantic::{Name, Function, Functions, Implementation};
use crate::semantic::jasm::{Jasm, JasmType};
use crate::target::llvm::{Llvm, Context, Module, Builder, FunctionPassManager, Type};
use crate::building::{BuildVisitor, BuildVisitorInner, ModuleBuildVisitor};
use crate::building::jasm_llvm::JasmLlvmBuild;


pub struct JasmModuleBuildVisitor<'l> {
    pub module: &'l Module,
    pub pass_manager:Option<&'l FunctionPassManager>,
    builders: Vec<Rc<CSemiBox<'l, Builder>>>,
    context: Rc<CBox<Context>>,
    inner: BuildVisitorInner<JasmLlvmBuild<'l>>,
    struct_indices: HashMap<Id, usize>,
    structs: HashMap<Id, Struct<Parameter<Jasm>>>,
    types: HashMap<Id, &'l Type>
}


impl<'l> JasmModuleBuildVisitor<'l> {
    pub fn new(ids:Rc<RefCell<IdProvider>>, functions:Functions<Jasm>, context:Rc<CBox<Context>>, module:&'l Module, pass_manager:Option<&'l FunctionPassManager>) -> Self {
        let builders = Vec::new();
        let structs = HashMap::new();
        let struct_indices = HashMap::new();
        let types = HashMap::new();
        let inner = BuildVisitorInner::new(ids, functions);
        Self {context, module, builders, inner, structs, struct_indices, types, pass_manager}
    }

    pub fn insert_type(&mut self, id:&Id, typ:&'l Type) {
        self.types.insert(*id, typ);
    }
    pub fn get_type(&self, id:&Id, name:&Name) -> &'l Type {
        *self.types.get(id).expect(&format!("type not found [{:?}]", name))
    }

    pub fn insert_struct(&mut self, definition:&Struct<Parameter<Jasm>>) {
        let Struct {id, name, parameters} = definition;
        self.structs.insert(*id, definition.to_owned());
        for (index, parameter) in parameters.iter().enumerate() {
            self.struct_indices.insert(parameter.id, index);
        }
    }
    pub fn get_struct(&self, id:&Id, name:&Name) -> &Struct<Parameter<Jasm>> {
        self.structs.get(id).expect(&format!("not found struct [{:?}]", name))
    }
    pub fn get_struct_indice(&self, id:&Id, name:&Name) -> usize {
        *self.struct_indices.get(id).expect(&format!("not found struct parameter [{:?}]", name))
    }

    pub fn push_builder(&mut self) {
        let context = self.into();
        self.builders.push(Rc::new(Builder::new(context)))
    }
    pub fn get_builder(&self) -> Rc<CSemiBox<'l, Builder>> {
        self.builders.last().unwrap().to_owned()
    }
    pub fn pop_builder(&mut self) {
        self.builders.pop();
    }



}

impl<'l> IdContext for JasmModuleBuildVisitor<'l> {
    fn new_id(&mut self) -> Id {
        self.get_inner_mut().new_id()
    }
}

impl<'l> BuildVisitor<JasmLlvmBuild<'l>> for JasmModuleBuildVisitor<'l> {
    fn get_inner(&self) -> &BuildVisitorInner<JasmLlvmBuild<'l>> {
        &self.inner
    }
    fn get_inner_mut(&mut self) -> &mut BuildVisitorInner<JasmLlvmBuild<'l>> {
        &mut self.inner
    }
    fn move_inner(self) -> BuildVisitorInner<JasmLlvmBuild<'l>> {
        self.inner
    }
}


impl<'l> From<&mut JasmModuleBuildVisitor<'l>> for &'l Context {
    fn from(context:&mut JasmModuleBuildVisitor) -> Self {
        unsafe{context.context.as_ptr().into()}
    }
}

impl<'l> ModuleBuildVisitor<Jasm, Llvm<'l>> for JasmModuleBuildVisitor<'l> {
    fn get_module_functions(&self) -> Option<Functions<Llvm<'l>>> {
        Some(self.get_inner().module_functions.to_owned())
    }
}

