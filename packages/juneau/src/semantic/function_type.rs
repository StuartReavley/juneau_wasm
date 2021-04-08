use std::fmt::Debug;

use super::Semantic;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FunctionType<S:Semantic> {
    pub parameters: Vec<S::Type>,
    pub retrn: Box<S::Type>
}

impl<S:Semantic> FunctionType<S> {
    pub fn new(parameters:Vec<S::Type>, retrn:S::Type) -> Self {
        let retrn = Box::new(retrn);
        Self {parameters, retrn}
    }
}

