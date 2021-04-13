use libc::c_uint;
use crate::target::llvm::Type;



native_reference! {
    /// A vector type.
    VectorType = llvm_sys::LLVMType
}
get_context! {VectorType, llvm_sys::core::LLVMGetTypeContext}
to_str!      {VectorType, llvm_sys::core::LLVMPrintTypeToString}
sub_type!    {VectorType, llvm_sys::LLVMTypeKind::LLVMVectorTypeKind}


impl VectorType {
    /// Make a new vector type with the length given.
    pub fn new(element: &Type, length: usize) -> &VectorType {
        unsafe {llvm_sys::core::LLVMVectorType(element.into(), length as c_uint) }.into()
    }
    /// Returns the element type of this vector type.
    pub fn get_element(&self) -> &Type {
        unsafe {std::mem::transmute(llvm_sys::core::LLVMGetElementType(self.into())) }
    }
    /// Returns the number of elements in this vector type.
    pub fn get_size(&self) -> usize {
        unsafe {llvm_sys::core::LLVMGetVectorSize(self.into()) as usize}
    }
}