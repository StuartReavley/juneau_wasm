use libc::c_uint;
use crate::target::llvm::Type;


native_reference! {
    /// A pointer type.
    PointerType = llvm_sys::LLVMType
}
get_context! {PointerType, llvm_sys::core::LLVMGetTypeContext}
to_str!      {PointerType, llvm_sys::core::LLVMPrintTypeToString}
sub_type!    {PointerType, llvm_sys::LLVMTypeKind::LLVMPointerTypeKind}


impl PointerType {
    /// Make a new pointer type with the given element type.
    pub fn new(elem: &Type) -> &Type {
        unsafe {llvm_sys::core::LLVMPointerType(elem.into(), 0 as c_uint)}.into()
    }
    /// Returns the element of this pointer type.
    pub fn get_element(&self) -> &Type {
        unsafe {llvm_sys::core::LLVMGetElementType(self.into()).into()}
    }
}