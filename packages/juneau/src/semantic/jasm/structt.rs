use std::fmt::Debug;
use crate::core::Id;
use crate::semantic::Name;


#[derive(Debug, Clone, PartialEq)]
pub struct Struct<T:Debug + Clone + PartialEq> {
    pub id: Id,
    pub name: Name,
    pub parameters: Vec<T>
}
