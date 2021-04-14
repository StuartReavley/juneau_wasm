use crate::dispose;




native_reference! {
    /// A wrapped value that can be passed to an interpreted function or returned from one
    GenericValue = llvm_sys::execution_engine::LLVMOpaqueGenericValue
}

dispose! {GenericValue, llvm_sys::execution_engine::LLVMOpaqueGenericValue, llvm_sys::execution_engine::LLVMDisposeGenericValue}
