use std::collections::HashMap;
use std::collections::hash_map::Entry::*;
use juneau_core::RcRefCell;
use crate::building::Build;
use crate::{core::{Id, IdContext, IdProvider}};
use crate::semantic::{Name, Function, Functions, Implementation, FunctionType, Target};
use super::BuildVisitor;


pub struct BuildVisitorInner<B:Build> {
    ids: RcRefCell<IdProvider>,
    variables: HashMap<Id, B::Variable>,
    functions: Functions<B::Semantic>,
    pub module_functions: Functions<B::Target>
}

impl<B:Build> BuildVisitorInner<B> {
    pub fn new(ids:RcRefCell<IdProvider>, functions:Functions<B::Semantic>) -> Self {
        let variables = HashMap::new();
        let module_functions = Functions::new();
        Self {ids, variables, functions, module_functions}
    }
    pub fn insert_variable(&mut self, id:Id, variable:B::Variable) {
        self.variables.insert(id, variable);
    }
    pub fn get_variable(&self, id:Id, name:&Name) -> B::Variable {
        self.variables.get(&id).expect(&format!("variable not found [{:?}]", name)).to_owned()
    }

    pub fn get_function(&self, id:Id, name:&Name) -> <B::Semantic as Target>::Function {
        self.functions.get(id, name)
    }

    pub fn resolve_module_function<V:BuildVisitor<B>>(visitor:&mut V, function:<B::Semantic as Target>::Function, f:impl FnOnce(&mut V) -> Option<<B::Target as Target>::Function>) -> Option<<B::Target as Target>::Function> {
        let id = function.id;
        visitor.get_inner_mut().functions.insert(id, function);
        visitor.get_inner().module_functions.get_option(id)
            .map(|f|f.to_owned())    
            .or_else(|| {
                f(visitor).map(|function| {
                    visitor.get_inner_mut().module_functions.insert(id, function.to_owned());
                    function
                })
            })
    }
}

impl<B:Build> IdContext for BuildVisitorInner<B> {
    fn new_id(&mut self) -> Id {
        self.ids.new_id()
    }
}