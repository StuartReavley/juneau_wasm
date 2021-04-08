use super::{Function, Semantic};


pub trait Expression<S:Semantic> {
    fn as_function(&self) -> Option<&Function<S>>;
    // fn from_variable(variable:S::Variable) -> S::Expression;
}



pub trait ExpressionType {
    fn new_variable_type() -> Self;
    fn new_invocation_type() -> Self;
}