use crate::core::c::CBox;
use crate::dispose;


native_reference! {
    /// Contains all the LLVM entities - mainly modules.
    ///
    /// Every single entity attached to it has its lifetime to enforce the
    /// rule that things from different contexts cannot interact and to
    /// preserve pointer safety.
    Context = llvm_sys::LLVMContext
}

dispose!(Context, llvm_sys::LLVMContext, llvm_sys::core::LLVMContextDispose);


impl Context {
    /// Get a reference to the global context.
    ///
    /// This is marked as unsafe because this can result in undefined behaviour
    /// in a multithreaded context if they all use the same context.
    pub unsafe fn get_global() -> &'static Context {
        llvm_sys::core::LLVMGetGlobalContext().into()
    }
    /// Create a new context, which is owned by the callee block.
    pub fn new() -> CBox<Self> {
        CBox::new(unsafe{llvm_sys::core::LLVMContextCreate()})
    }
}
