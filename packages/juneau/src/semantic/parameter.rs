use crate::core::Id;
use super::{GetType, Semantic, Name};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter<S:Semantic> {
    pub id: Id,
    pub name: Name,
    pub typ: S::Type
}

impl<S:Semantic> Parameter<S> {
    pub fn new(id:Id, name:&Name, typ:&S::Type) -> Self {
        let name = name.to_owned();
        let typ = typ.to_owned();
        Self {id, name, typ}
    }
}

impl<S:Semantic> GetType<S::Type> for Parameter<S> {
    fn get_type(&self) -> S::Type {
        self.typ.to_owned()
    }
}
