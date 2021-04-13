use libc::{c_int, c_uint};
use crate::{target::llvm::{Context, Type}, core::c::with_cstr};


native_reference! {
    /// A structure type, such as a tuple or struct.
    StructType = llvm_sys::LLVMType
    }
get_context! {StructType, llvm_sys::core::LLVMGetTypeContext}
to_str!      {StructType, llvm_sys::core::LLVMPrintTypeToString}
sub_type!    {StructType, llvm_sys::LLVMTypeKind::LLVMStructTypeKind}

impl StructType {
    /// Make a new struct with the given fields and packed representation.
    pub fn new<'a>(context: &'a Context, fields: &[&'a Type], packed: bool) -> &'a StructType {
        unsafe {
            llvm_sys::core::LLVMStructTypeInContext(
                context.into(),
                fields.as_ptr() as *mut llvm_sys::prelude::LLVMTypeRef,
                fields.len() as c_uint,
                packed as c_int,
            )
        }
        .into()
    }
    /// Make a new named struct with the given fields and packed representation.
    pub fn new_named<'a>(
        context: &'a Context,
        name: &str,
        fields: &[&'a Type],
        packed: bool,
    ) -> &'a StructType {
        with_cstr(name, |name| unsafe {
            let ty = llvm_sys::core::LLVMStructCreateNamed(context.into(), name);
            llvm_sys::core::LLVMStructSetBody(
                ty,
                fields.as_ptr() as *mut llvm_sys::prelude::LLVMTypeRef,
                fields.len() as c_uint,
                packed as c_int,
            );
            ty.into()
        })
    }
    /// Make a new named struct without defining its fields yet.
    ///
    /// You can use this opaque struct without defining its fields (for creating recursive types, etc.). When you want to define its fields (which may include itself), use set_elements().
    pub fn new_opaque<'a>(context: &'a Context, name: &str) -> &'a StructType {
        with_cstr(name, |name| unsafe {
            llvm_sys::core::LLVMStructCreateNamed(context.into(), name).into()
        })
    }
    /// Set the elements that make up this struct.
    ///
    /// Can only be called once, and only on a StructType created through the new_opaque() function.
    /// Returns Err(()) if the struct is not opaque.
    pub fn set_elements<'a>(&self, fields: &[&'a Type], packed: bool) -> Result<(), ()> {
        unsafe {
            if llvm_sys::core::LLVMIsOpaqueStruct(self.into()) != 0 {
                llvm_sys::core::LLVMStructSetBody(
                    self.into(),
                    fields.as_ptr() as *mut llvm_sys::prelude::LLVMTypeRef,
                    fields.len() as c_uint,
                    packed as c_int,
                );
                Ok(())
            } else {
                Err(())
            }
        }
    }
    /// Returns the elements that make up this struct.
    pub fn get_elements(&self) -> Vec<&Type> {
        unsafe {
            let size = llvm_sys::core::LLVMCountStructElementTypes(self.into());
            let mut els: Vec<llvm_sys::prelude::LLVMTypeRef> = (0..size)
                .map(|_| std::mem::MaybeUninit::zeroed().assume_init())
                .collect();
            llvm_sys::core::LLVMGetStructElementTypes(self.into(), els.as_mut_ptr() as *mut llvm_sys::prelude::LLVMTypeRef);
            els.iter().map(|&el_ptr| std::mem::transmute(el_ptr)).collect()
        }
    }
}