use crate::core::c::{CBox, DisposeRef};
use crate::target::llvm::{Context, MemoryBuffer};
use super::BinaryType;



native_reference! {
    /// An external object file that has been parsed by LLVM.
    Binary = llvm_sys::object::LLVMOpaqueBinary
}


impl DisposeRef for Binary {
    type RefTo = llvm_sys::object::LLVMOpaqueBinary;
    unsafe fn dispose(ptr: llvm_sys::object::LLVMBinaryRef) {
        llvm_sys::object::LLVMDisposeBinary(ptr);
    }
}

impl Binary {
    /// Parse the object file at the path given, or return an error string if an error occurs.
    pub fn read(path: &str, ctx: &mut Context) -> Result<CBox<Binary>, CBox<str>> {
        let buf = MemoryBuffer::new_from_file(path)?;
        unsafe {
            let mut err = std::mem::MaybeUninit::zeroed().assume_init();
            let ptr = llvm_sys::object::LLVMCreateBinary(buf.as_ptr(), ctx.into(), &mut err);
            if ptr.is_null() {
                Err(CBox::new(err))
            } else {
                Ok(CBox::new(ptr))
            }
        }
    }
    pub fn binary_type(&self) -> BinaryType {
        unsafe { llvm_sys::object::LLVMBinaryGetType(self.into()) }.into()
    }
}
