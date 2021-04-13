use crate::core::c::ptr_to_null;
use crate::target::llvm::Value;
use super::GlobalValue;


native_reference! {
    /// A global variable
    GlobalVariable = llvm_sys::LLVMValue
}
sub_value! {GlobalVariable, llvm_sys::core::LLVMIsAGlobalVariable, GlobalValue}
to_str! {GlobalVariable, llvm_sys::core::LLVMPrintValueToString}


impl GlobalVariable {
    /// Set the initial value of the global
    pub fn set_initializer(&self, val: &Value) {
        unsafe {llvm_sys::core::LLVMSetInitializer(self.into(), val.into())}
    }
    /// Set the initial value of the global
    pub fn get_initializer(&self) -> Option<&Value> {
        unsafe {ptr_to_null(llvm_sys::core::LLVMGetInitializer(self.into()))}
    }
    /// Set whether this global is a constant.
    pub fn set_constant(&self, is_constant: bool) {
        let llvm_bool = if is_constant {1} else {0};
        unsafe {llvm_sys::core::LLVMSetGlobalConstant(self.into(), llvm_bool)}
    }
    /// Returns true if this global is a constant.
    pub fn get_constant(&self) -> bool {
        unsafe {llvm_sys::core::LLVMIsGlobalConstant(self.into()) != 0}
    }
}