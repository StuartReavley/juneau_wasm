use std::marker::PhantomData;
use crate::semantic::Target;
use crate::target::llvm::Function;


#[derive(Debug, Clone, PartialEq)]
pub struct Llvm<'l>(PhantomData<&'l u8>);


impl<'l> Target for Llvm<'l> {
    type Function = &'l Function;
}