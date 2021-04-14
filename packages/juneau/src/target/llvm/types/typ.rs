use libc::c_uint;
use crate::target::llvm::{Constant, Context, TargetData};


native_reference! {
    /// Defines how a value should be laid out in memory.
    Type = llvm_sys::LLVMType
}
get_context! {Type, llvm_sys::core::LLVMGetTypeContext}
to_str!      {Type, llvm_sys::core::LLVMPrintTypeToString}


impl Type {
    #[inline(always)]
    /// Get the type given as an LLVM type descriptor in the context given.
    pub fn get<'l, T:Constant>(context:&'l Context) -> &'l Type {
        T::get_type(context)
    }
    /// Returns true if the size of the type is known at compile-time.
    ///
    /// This is equivalent to the type implementing `Sized` in Rust
    pub fn is_sized<'l>(&'l self) -> bool {
        unsafe {llvm_sys::core::LLVMTypeIsSized(self.into()) != 0}
    }
    /// Returns true if this type is a function.
    ///
    /// This is equivalent to `FunctionType::is`.
    pub fn is_function(&self) -> bool {
        let kind = unsafe {llvm_sys::core::LLVMGetTypeKind(self.into()) };
        kind as c_uint == llvm_sys::LLVMTypeKind::LLVMFunctionTypeKind as c_uint
    }
    /// Returns true if this type is a struct.
    ///
    /// This is equivalent to `StructType::is`.
    pub fn is_struct(&self) -> bool {
        let kind = unsafe {llvm_sys::core::LLVMGetTypeKind(self.into()) };
        kind as c_uint == llvm_sys::LLVMTypeKind::LLVMStructTypeKind as c_uint
    }
    /// Returns true if this type is void.
    pub fn is_void(&self) -> bool {
        let kind = unsafe {llvm_sys::core::LLVMGetTypeKind(self.into()) };
        kind as c_uint == llvm_sys::LLVMTypeKind::LLVMVoidTypeKind as c_uint
    }
    /// Returns true if this type is a pointer.
    ///
    /// This is equivalent to `PointerType::is`.
    pub fn is_pointer(&self) -> bool {
        let kind = unsafe {llvm_sys::core::LLVMGetTypeKind(self.into()) };
        kind as c_uint == llvm_sys::LLVMTypeKind::LLVMPointerTypeKind as c_uint
    }
    /// Returns true if this type is an integer.
    pub fn is_integer(&self) -> bool {
        let kind = unsafe {llvm_sys::core::LLVMGetTypeKind(self.into()) };
        kind as c_uint == llvm_sys::LLVMTypeKind::LLVMIntegerTypeKind as c_uint
    }
    /// Returns true if this type is any floating-point number.
    pub fn is_float(&self) -> bool {
        let kind = unsafe {llvm_sys::core::LLVMGetTypeKind(self.into()) } as c_uint;
        kind == llvm_sys::LLVMTypeKind::LLVMHalfTypeKind as c_uint
            || kind == llvm_sys::LLVMTypeKind::LLVMFloatTypeKind as c_uint
            || kind == llvm_sys::LLVMTypeKind::LLVMDoubleTypeKind as c_uint
    }
    /// Returns the size of the type in bytes.
    pub fn get_size(&self, target: &TargetData) -> usize {
        unsafe {llvm_sys::target::LLVMABISizeOfType(target.into(), self.into()) as usize}
    }
}