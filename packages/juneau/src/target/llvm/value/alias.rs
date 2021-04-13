use super::GlobalValue;


native_reference! {
    /// An alias to another global value.
    Alias = llvm_sys::LLVMValue
}
sub_value! {Alias, llvm_sys::core::LLVMIsAGlobalAlias, GlobalValue}
to_str!    {Alias, llvm_sys::core::LLVMPrintValueToString}

