use std::rc::Rc;
use crate::core::Id;
use crate::semantic::{Name, Semantic, Parameter, FunctionType, GetType, Implementation};


#[derive(Debug, Clone, PartialEq)]
pub struct Function<S:Semantic> {
    pub id: Id,
    pub name: Name,
    pub parameters: Vec<Parameter<S>>,
    pub implementation: Implementation<S>,
}


impl<S:Semantic> Function<S> {
    pub fn new(id: Id, name:Name, parameters:Vec<Parameter<S>>, implementation:Implementation<S>) -> Rc<Self> {
        Rc::new(Self {id, name, parameters, implementation})
    }

    pub fn get_type(&self) -> FunctionType<S> {
        let Self {parameters, implementation, ..} = self;
        let parameters = parameters.get_type();
        let retrn = implementation.get_type();
        FunctionType::new(parameters, retrn)
    }
}

