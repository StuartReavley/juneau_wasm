use std::ffi::CString;
use libc::{c_char, c_int, c_uint};
use crate::target::llvm::{Context, GetContext, Type};



native_reference! {
    /// A typed value that can be used as an operand in instructions.
    Value = llvm_sys::LLVMValue
}

to_str! {Value, llvm_sys::core::LLVMPrintValueToString}

impl Value {
    /// Create a new constant struct from the values given.
    pub fn new_struct<'a>(context: &'a Context, vals: &[&'a Value], packed: bool) -> &'a Value {
        unsafe {
            llvm_sys::core::LLVMConstStructInContext(
                context.into(),
                vals.as_ptr() as *mut llvm_sys::prelude::LLVMValueRef,
                vals.len() as c_uint,
                packed as c_int,
            )
        }
        .into()
    }
    /// Create a new constant vector from the values given.
    pub fn new_vector<'a>(vals: &[&'a Value]) -> &'a Value {
        unsafe {
            llvm_sys::core::LLVMConstVector(vals.as_ptr() as *mut llvm_sys::prelude::LLVMValueRef, vals.len() as c_uint).into()
        }
    }
    /// Create a new constant C string from the text given.
    pub fn new_string<'l>(context:&'l Context, text:&str, rust_style:bool) -> &'l Value {
        unsafe {
            let ptr = text.as_ptr() as *const c_char;
            let len = text.len() as c_uint;
            llvm_sys::core::LLVMConstStringInContext(context.into(), ptr, len, rust_style as c_int).into()
        }
    }
    /// Create a new constant undefined value of the given type.
    pub fn new_undefined<'l>(typ:&'l Type) -> &'l Value {
        unsafe {llvm_sys::core::LLVMGetUndef(typ.into()).into()}
    }
    /// Returns the name of this value, or `None` if it lacks a name
    pub fn get_name(&self) -> Option<&str> {
        unsafe {
            let mut len = 0;
            let c_name = llvm_sys::core::LLVMGetValueName2(self.into(), &mut len) as *const u8;
            let bytes = std::slice::from_raw_parts(c_name, len);
            let txt = std::str::from_utf8(bytes);
            if let Ok(inner_txt) = txt {
                Some(inner_txt)
            } else {
                None
            }
        }
    }
    /// Sets the name of this value
    pub fn set_name(&self, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe {
            llvm_sys::core::LLVMSetValueName2(self.into(), c_name.as_ptr(), c_name.as_bytes().len() - 1)
        }
    }
    /// Returns the type of this value
    pub fn get_type(&self) -> &Type {
        unsafe {llvm_sys::core::LLVMTypeOf(self.into()) }.into()
    }

    pub fn is_constant(&self) -> bool {
        unsafe {llvm_sys::core::LLVMIsConstant(self.into()) == 1 }
    }

    pub fn replace_all_uses(old: &Value, new: &Value) {
        unsafe {llvm_sys::core::LLVMReplaceAllUsesWith(old.into(), new.into()) }
    }
}


impl GetContext for Value {
    fn get_context(&self) -> &Context {
        self.get_type().get_context()
    }
}