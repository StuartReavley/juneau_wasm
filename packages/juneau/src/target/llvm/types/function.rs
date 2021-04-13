use libc::c_uint;
use crate::{core::c::SubStruct, deref};
use crate::target::llvm::{Type, PointerType};


native_reference! {
    /// A function signature type.
    FunctionType = llvm_sys::LLVMType
}
get_context! {FunctionType, llvm_sys::core::LLVMGetTypeContext}
to_str!      {FunctionType, llvm_sys::core::LLVMPrintTypeToString}
deref!       {FunctionType, Type}


unsafe impl SubStruct<Type> for FunctionType {
    fn is(mut ty:&Type) -> bool {
        unsafe {
            while let Some(ptr) = PointerType::from_super(ty) {
                ty = ptr.get_element();
            }
            let kind = llvm_sys::core::LLVMGetTypeKind(ty.into());
            kind as c_uint == llvm_sys::LLVMTypeKind::LLVMFunctionTypeKind as c_uint
        }
    }
}
impl FunctionType {
    /// Make a new function signature with the return type and arguments given.
    pub fn new<'a>(parameters: &[&'a Type], retrn: &'a Type) -> &'a FunctionType {
        unsafe {
            llvm_sys::core::LLVMFunctionType(
                retrn.into(),
                parameters.as_ptr() as *mut llvm_sys::prelude::LLVMTypeRef,
                parameters.len() as c_uint,
                0,
            ).into()
        }
    }



    /// Returns the number of parameters this signature takes.
    pub fn num_params(&self) -> usize {
        unsafe {llvm_sys::core::LLVMCountParamTypes(self.into()) as usize}
    }
    /// Returns a vector of this signature's parameters' types.
    pub fn get_params(&self) -> Vec<&Type> {
        unsafe {
            let count = llvm_sys::core::LLVMCountParamTypes(self.into());
            let mut types: Vec<llvm_sys::prelude::LLVMTypeRef> = (0..count)
                .map(|_| std::mem::MaybeUninit::zeroed().assume_init())
                .collect();
            llvm_sys::core::LLVMGetParamTypes(self.into(), types.as_mut_ptr() as *mut llvm_sys::prelude::LLVMTypeRef);
            types.iter().map(|t| std::mem::transmute(t)).collect()
        }
    }
    /// Returns the type that this function returns.
    pub fn get_return(&self) -> &Type {
        unsafe {llvm_sys::core::LLVMGetReturnType(self.into()).into()}
    }
}