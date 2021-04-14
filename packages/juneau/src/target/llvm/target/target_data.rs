use std::{ffi::CString, fmt::{Display, Formatter}};
use libc::c_uint;
use crate::core::c::{CBox, DisposeRef};
use crate::target::llvm::Type;



native_reference! {
    /// Represents an LLVM Target
    TargetData = llvm_sys::target::LLVMOpaqueTargetData
}

impl TargetData {
    /// Create a target data from a target layout string.
    pub fn new(rep: &str) -> CBox<TargetData> {
        let c_rep = CString::new(rep).unwrap();
        CBox::new(unsafe {llvm_sys::target::LLVMCreateTargetData(c_rep.as_ptr())})
    }
    /// Returns true if the target is big endian.
    pub fn is_big_endian(&self) -> bool {
        let order = unsafe{llvm_sys::target::LLVMByteOrder(self.into())} as c_uint;
        order == 0
    }
    /// Returns the size of a pointer on the target.
    pub fn get_pointer_size(&self) -> usize {
        unsafe { llvm_sys::target::LLVMPointerSize(self.into()) as usize }
    }
    /// Returns the size of the type given in bits.
    pub fn size_of_in_bits(&self, ty: &Type) -> u64 {
        unsafe { llvm_sys::target::LLVMSizeOfTypeInBits(self.into(), ty.into()) }
    }
    /// Returns the size of the type given in bytes.
    pub fn size_of(&self, ty: &Type) -> u64 {
        unsafe { llvm_sys::target::LLVMStoreSizeOfType(self.into(), ty.into()) }
    }
    /// Returns the alignment of the type given in bytes.
    pub fn alignment_of(&self, ty: &Type) -> usize {
        unsafe {llvm_sys::target::LLVMABIAlignmentOfType(self.into(), ty.into()) as usize }
    }
    /// Computes the structure element that contains the byte offset for a target.
    pub fn element_at(&self, struct_ty: &Type, offset: u64) -> usize {
        unsafe {llvm_sys::target::LLVMElementAtOffset(self.into(), struct_ty.into(), offset) as usize }
    }
    /// Compute the byte offset of an element in the struct type given.
    pub fn offset_of(&self, struct_ty: &Type, element: usize) -> u64 {
        unsafe {llvm_sys::target::LLVMOffsetOfElement(self.into(), struct_ty.into(), element as c_uint) }
    }
    /// Returns the string representation of this target data.
    pub fn as_str(&self) -> CBox<str> {
        unsafe {CBox::new(llvm_sys::target::LLVMCopyStringRepOfTargetData(self.into()))}
    }
}
impl Display for TargetData {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str(&self.as_str())
    }
}

impl DisposeRef for TargetData {
    type RefTo = llvm_sys::target::LLVMOpaqueTargetData;
    unsafe fn dispose(ptr: llvm_sys::target::LLVMTargetDataRef) {
       llvm_sys::target::LLVMDisposeTargetData(ptr)
    }
}