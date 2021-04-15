use crate::semantic::Target;


#[derive(Debug, Clone, PartialEq)]
pub struct Wasm;


impl Target for Wasm {
    // Not used yet - soon we'll want to compile jasm modules to wasm modules
    type Function = ();
}