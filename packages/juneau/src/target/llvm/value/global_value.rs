use super::Linkage;


native_reference! {
    /// A value with global scope (eg: Function, Alias, Global variable)
    GlobalValue = llvm_sys::LLVMValue
}
sub_value! {GlobalValue, llvm_sys::core::LLVMIsAGlobalValue}
to_str! {GlobalValue, llvm_sys::core::LLVMPrintValueToString}


impl GlobalValue {
    /// Set the linkage type for this global
    pub fn set_linkage(&self, linkage:Linkage) {
        unsafe {
            llvm_sys::core::LLVMSetLinkage(self.into(), linkage.into());
        }
    }
    /// Returns the linkage type for this global
    pub fn get_linkage(&self) -> Linkage {
        unsafe { llvm_sys::core::LLVMGetLinkage(self.into()).into() }
    }
    /// Returns true if this global is a declaration (as opposed to a definition).
    pub fn is_declaration(&self) -> bool {
        unsafe {
            // FIXME: There should be a constant somewhere, instead of '1'
            llvm_sys::core::LLVMIsDeclaration(self.into()) == 1
        }
    }
}