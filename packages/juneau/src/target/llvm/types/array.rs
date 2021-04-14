use libc::c_uint;
use crate::target::llvm::Type;


native_reference! {
    /// An array type.
    ArrayType = llvm_sys::LLVMType
}
get_context! {ArrayType, llvm_sys::core::LLVMGetTypeContext}
to_str!      {ArrayType, llvm_sys::core::LLVMPrintTypeToString}
sub_type!    {ArrayType, llvm_sys::LLVMTypeKind::LLVMArrayTypeKind}


impl ArrayType {
    /// Make a new array type with the length given.
    pub fn new(element: &Type, length: usize) -> &ArrayType {
        unsafe {llvm_sys::core::LLVMArrayType(element.into(), length as c_uint) }.into()
    }
    /// Returns the element type of this array type.
    pub fn get_element(&self) -> &Type {
        unsafe {llvm_sys::core::LLVMGetElementType(self.into())}.into()
    }
    /// Returns the number of elements in this vector type.
    pub fn get_length(&self) -> usize {
        unsafe {llvm_sys::core::LLVMGetArrayLength(self.into()) as usize}
    }
}
