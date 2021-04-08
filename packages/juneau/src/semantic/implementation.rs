use super::{Semantic, GetType};


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Implementation<S:Semantic> {
    Semantic(S::Implementation),
    Primitive(S::Type, S::PrimitiveImplementation)
}

impl<S:Semantic> Implementation<S> {
    pub fn new(implementation:S::Implementation) -> Self {
        Self::Semantic(implementation)
    }
    pub fn new_primitive(typ:S::Type, primitive:S::PrimitiveImplementation) -> Self {
        Self::Primitive(typ, primitive)
    }
}

impl<S:Semantic> GetType<S::Type> for Implementation<S> {
    fn get_type(&self) -> S::Type {
        use Implementation::*;
        match self {
            Semantic(s) => s.get_type(),
            Primitive(typ, _) => typ.to_owned()
        }
    }
}
