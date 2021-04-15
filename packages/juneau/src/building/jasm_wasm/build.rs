use std::marker::PhantomData;
use walrus::{Local, LocalId};

use crate::building::Build;
use crate::semantic::jasm::Jasm;

use super::Wasm;


pub struct JasmWasmBuild;


impl Build for JasmWasmBuild {
    type Semantic = Jasm;
    type Target = Wasm;
    type Variable = LocalId;
}