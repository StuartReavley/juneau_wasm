use crate::core::Id;
use super::{Name, Expression, GetType, Semantic};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable<S:Semantic> {
    pub id: Id,
    pub name: Name,
    pub typ: S::Type
}

impl<S:Semantic> Variable<S> {
    pub fn new(id:Id, name:&Name, typ:&S::Type) -> Self {
        let name = name.to_owned();
        let typ = typ.to_owned();
        Self {id, name, typ}
    }
}

impl<S:Semantic> GetType<S::Type> for Variable<S> {
    fn get_type(&self) -> S::Type {
        self.typ.to_owned()
    }
}

// impl<S:Semantic> From<Variable<S>> for S::Expression {
//     fn from(variable:Variable<S>) -> Self {
//         Expression<S>::from_variable(variable)
//     }
// }