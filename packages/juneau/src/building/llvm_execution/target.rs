use std::marker::PhantomData;
use crate::semantic::Target;
use crate::target::llvm::ExecutableFunction2;



pub struct Execution<'l>(PhantomData<&'l u8>);

impl<'l> Target for Execution<'l> {
    type Function = ExecutableFunction2<'l>;
}