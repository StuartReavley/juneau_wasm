use crate::core::c::to_str;
use libc::c_char;





native_reference! {
    Target = llvm_sys::target_machine::LLVMTarget
}

impl Target {
    /// Returns the name of this target.
    pub fn get_name(&self) -> &str {
        unsafe {to_str(llvm_sys::target_machine::LLVMGetTargetName(self.into()) as *mut c_char)}
    }
    /// Returns the description of this target.
    pub fn get_description(&self) -> &str {
        unsafe {
            to_str(llvm_sys::target_machine::LLVMGetTargetDescription(self.into()) as *mut c_char)
        }
    }
    /// Returns true if this target has an assembly generation backend implemented.
    pub fn has_asm_backend(&self) -> bool {
        unsafe {llvm_sys::target_machine::LLVMTargetHasAsmBackend(self.into()) != 0 }
    }
    /// Returns true if this target supports JIT compilation.
    pub fn has_jit(&self) -> bool {
        unsafe {llvm_sys::target_machine::LLVMTargetHasJIT(self.into()) != 0 }
    }
    /// Returns true if this target has a target machine.
    pub fn has_target_machine(&self) -> bool {
        unsafe {llvm_sys::target_machine::LLVMTargetHasTargetMachine(self.into()) != 0 }
    }
}
