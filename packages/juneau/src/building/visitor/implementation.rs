use crate::semantic::Semantic;


#[derive(Debug, Clone, PartialEq)]
pub enum ImplementationBuild<S:Semantic, T:Semantic> {
    Function(T::Function),
    Primitive(S::PrimitiveImplementation)
}

impl<S:Semantic, T:Semantic> ImplementationBuild<S, T> {
    pub fn new(primitive:S::PrimitiveImplementation) -> Self {
        Self::Primitive(primitive)
    }
}