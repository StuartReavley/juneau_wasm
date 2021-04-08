use std::rc::Rc;
use std::collections::HashMap;
use crate::core::Visitor;
use crate::{core::{Id, IdContext}};
use crate::semantic::{Name, Semantic, Target, Function, Functions, Module};
use crate::building::{BuildVisitorInner, Build};

use super::ModuleBuildVisitor;


pub trait BuildVisitor<B:Build> where Self:Sized {
    fn get_inner(&self) -> &BuildVisitorInner<B>;
    fn get_inner_mut(&mut self) -> &mut BuildVisitorInner<B>;
    fn move_inner(self) -> BuildVisitorInner<B>;

    fn insert_variable(&mut self, id:Id, variable:B::Variable) {
        self.get_inner_mut().insert_variable(id, variable);
    }
    fn get_variable(&self, id:Id, name:&Name) -> B::Variable {
        self.get_inner().get_variable(id, name)
    }
    fn get_function(&mut self, id:Id, name:&Name) -> <B::Semantic as Target>::Function {
        self.get_inner().get_function(id, name)
    }
    fn resolve_module_function(&mut self, function:<B::Semantic as Target>::Function, f:impl FnOnce(&mut Self) -> Option<<B::Target as Target>::Function>) -> Option<<B::Target as Target>::Function> {
        BuildVisitorInner::resolve_module_function(self, function, f)
    }
}
