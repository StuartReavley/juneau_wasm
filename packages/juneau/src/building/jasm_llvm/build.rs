use std::marker::PhantomData;
use crate::building::Build;
use crate::semantic::jasm::Jasm;
use crate::target::llvm::{Llvm, Value};


pub struct JasmLlvmBuild<'l>(PhantomData<&'l u8>);


impl<'l> Build for JasmLlvmBuild<'l> {
    type Semantic = Jasm;
    type Target = Llvm<'l>;
    type Variable = &'l Value;
}