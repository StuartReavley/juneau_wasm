use crate::target::llvm::Function;


native_reference! {
    /// An argument that is passed to a function.
    Parameter = llvm_sys::LLVMValue
}
sub_value! {Parameter, llvm_sys::core::LLVMIsAArgument}
to_str!    {Parameter, llvm_sys::core::LLVMPrintValueToString}

impl Parameter {
    /// Obtain the function this argument belongs to
    pub fn parent(&self) -> &Function {
        unsafe { llvm_sys::core::LLVMGetParamParent(self.into()).into() }
    }

    /// Set function param alignment
    pub fn set_alignment(&self, alignment: usize) {
        unsafe {
            llvm_sys::core::LLVMSetAlignment(self.into(), alignment as libc::c_uint);
        }
    }
}