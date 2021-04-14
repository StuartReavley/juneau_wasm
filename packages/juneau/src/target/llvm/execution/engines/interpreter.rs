use crate::dispose;
use crate::core::c::{CSemiBox, CBox};
use crate::target::llvm::{ExecutionEngine, Module};



native_reference! {
    /// The interpreter backend
    Interpreter = llvm_sys::execution_engine::LLVMOpaqueExecutionEngine
}

dispose! {Interpreter, llvm_sys::execution_engine::LLVMOpaqueExecutionEngine, llvm_sys::execution_engine::LLVMDisposeExecutionEngine}


impl<'a> ExecutionEngine<'a> for Interpreter {
    type Options = ();
    fn new(module: &'a Module, _: ()) -> Result<CSemiBox<'a, Interpreter>, CBox<str>> {
        unsafe {
            let mut ee = std::mem::MaybeUninit::zeroed().assume_init();
            let mut out = std::mem::zeroed();
            llvm_sys::execution_engine::LLVMLinkInInterpreter();
            let result =
                llvm_sys::execution_engine::LLVMCreateInterpreterForModule(&mut ee, (&*module).into(), &mut out);
            if result == 0 {
                Ok(ee.into())
            } else {
                Err(CBox::new(out))
            }
        }
    }
}