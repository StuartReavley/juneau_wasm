use std::fmt::Debug;
use std::hash::Hash;
use crate::core::FromRef;
use crate::semantic::{Variable, Semantic};


pub trait SymbolSemantic: Semantic + Sized {
    type Overload: FromRef<Variable<Self>> + Debug + Hash + Eq + Clone;
}
