use crate::semantic::{Semantic, Target};


pub trait Build {
    type Semantic: Semantic;
    type Target: Target;
    type Variable: Clone;
}