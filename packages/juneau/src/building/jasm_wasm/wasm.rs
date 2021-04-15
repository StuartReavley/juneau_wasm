use walrus::FunctionId;

use crate::semantic::Target;


#[derive(Debug, Clone, PartialEq)]
pub struct Wasm;


impl Target for Wasm {
    type Function = FunctionId;
}