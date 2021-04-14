use std::{ffi::CString, ops::Deref};
use libc::{c_char, size_t};
use crate::core::c::{CBox, DisposeRef, with_cstr};


native_reference! {
    MemoryBuffer = llvm_sys::LLVMMemoryBuffer
}

impl MemoryBuffer {
    pub fn new_from_file(path: &str) -> Result<CBox<MemoryBuffer>, CBox<str>> {
        with_cstr(path, |path| unsafe {
            let mut output = std::mem::MaybeUninit::zeroed().assume_init();
            let mut error = std::mem::MaybeUninit::zeroed().assume_init();
            if llvm_sys::core::LLVMCreateMemoryBufferWithContentsOfFile(path, &mut output, &mut error) == 1 {
                Err(CBox::new(error))
            } else {
                Ok(CBox::new(output))
            }
        })
    }

    pub fn new_from_str(buf: &str, name: Option<&str>) -> Result<CBox<MemoryBuffer>, CBox<str>> {
        unsafe {
            let in_name = name
                .map(|n| n.as_bytes().to_vec())
                .map(|mut v| {
                    v.push(0);
                    CString::from_vec_unchecked(v)
                })
                .unwrap_or(CString::default());

            let out = llvm_sys::core::LLVMCreateMemoryBufferWithMemoryRangeCopy(
                buf.as_ptr() as *const c_char,
                buf.len() as size_t,
                in_name.as_ptr(),
            );
            Ok(CBox::new(out))
        }
    }
}
impl Deref for MemoryBuffer {
    type Target = str;
    fn deref(&self) -> &str {
        unsafe {
            #[allow(dead_code)]
            struct StrSlice {
                data: *const c_char,
                len: usize,
            }
            std::mem::transmute(StrSlice {
                data: llvm_sys::core::LLVMGetBufferStart(self.into()),
                len: llvm_sys::core::LLVMGetBufferSize(self.into()) as usize,
            })
        }
    }
}
impl DisposeRef for MemoryBuffer {
    type RefTo = llvm_sys::LLVMMemoryBuffer;
    unsafe fn dispose(ptr:*mut Self::RefTo) {
        llvm_sys::core::LLVMDisposeMemoryBuffer(ptr)
    }
}
