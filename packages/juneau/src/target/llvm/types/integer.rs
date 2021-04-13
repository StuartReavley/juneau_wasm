use libc::c_uint;
use crate::target::llvm::Context;


native_reference! {
    /// An integer type.
    IntegerType = llvm_sys::LLVMType
}
get_context! {IntegerType, llvm_sys::core::LLVMGetTypeContext}
to_str!      {IntegerType, llvm_sys::core::LLVMPrintTypeToString}
sub_type!    {IntegerType, llvm_sys::LLVMTypeKind::LLVMPointerTypeKind}


impl IntegerType {
    /// Make a new integer type that will be the size of the given number of bits.
    pub fn new(context: &Context, numbits: usize) -> &IntegerType {
        unsafe {llvm_sys::core::LLVMIntTypeInContext(context.into(), numbits as c_uint)}.into()
    }
    /// Returns how long an integer of this type is, in bits.
    pub fn get_width(&self) -> usize {
        unsafe {llvm_sys::core::LLVMGetIntTypeWidth(self.into()) as usize}
    }
}