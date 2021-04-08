use crate::semantic::{Function, FunctionType, GetType, Semantic, Target, Variable};
use super::{Block, JasmExpression, JasmPrimitiveImplementation, JasmStatement, JasmType};


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Jasm();

impl Semantic for Jasm {
    type Type = JasmType;
    type Implementation = (Block, JasmType);
    type PrimitiveImplementation = JasmPrimitiveImplementation;
    type FunctionType = FunctionType<Self>;
    type Variable = Variable<Self>;
}

impl GetType<JasmType> for (Block, JasmType) {
    fn get_type(&self) -> JasmType {
        self.1.to_owned()
    }
}
