use std::rc::Rc;
use std::fmt::Debug;
use super::{Expression, Function, GetType, Target, Type};


pub trait Semantic where Self: Debug + Clone + PartialEq + Sized {
    type Type: Type<Self> + Debug + Clone + PartialEq + Sized;
    type Implementation: GetType<Self::Type> + Debug + Clone + PartialEq + Sized;
    type PrimitiveImplementation: Debug + Clone + PartialEq + Sized;
    type FunctionType: Debug + Clone + PartialEq + Sized;
    type Variable: GetType<Self::Type> + Debug + Clone + PartialEq + Sized;
}


impl<T:Semantic> Target for T {
    type Function = Rc<Function<T>>;
}




