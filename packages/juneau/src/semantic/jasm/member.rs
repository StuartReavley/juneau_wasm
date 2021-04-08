use crate::core::Id;
use crate::semantic::Name;
use super::JasmExpression;


#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub id: Id,
    pub name: Name,
    pub expression: JasmExpression
}

impl Member {
    pub fn new(id:Id, name:&Name, expression:JasmExpression) -> Self {
        let name = name.to_owned();
        Self {id, name, expression}
    }
}
