use std::ptr;
use libc::c_uint;
use crate::{core::c::{CBox, CSemiBox}, dispose};
use crate::target::llvm::{Constant, Function, Type, Module, GetContext, ExecutionEngine};


/// The options to pass to the MCJIT backend.
#[derive(Copy, Clone)]
pub struct JitOptions {
    /// The degree to which optimizations should be done, between 0 and 3.
    ///
    /// 0 represents no optimizations, 3 represents maximum optimization
    pub opt_level: usize,
}


native_reference! {
    /// The MCJIT backend, which compiles functions and values into machine code.
    JitEngine = llvm_sys::execution_engine::LLVMOpaqueExecutionEngine
    }
dispose! {JitEngine, llvm_sys::execution_engine::LLVMOpaqueExecutionEngine, llvm_sys::execution_engine::LLVMDisposeExecutionEngine}

impl JitEngine {
    /// Run the closure `cb` with the machine code for the function `function`.
    ///
    /// If the function takes multiple arguments, these should be wrapped in a tuple due to
    /// the limitations of Rust's type system.
    ///
    /// This will check that the types match at runtime when in debug mode, but not release mode.
    /// You should make sure to use debug mode if you want it to error when the types don't match.
    pub fn with_function<C, A, R>(&self, function:&Function, cb:C)
    where
        A: Constant,
        R: Constant,
        C: FnOnce(extern "C" fn(A) -> R),
    {
        if cfg!(debug_assertions) {
            let ctx = function.get_context();
            let sig = function.get_signature();
            assert_eq!(Type::get::<R>(ctx), sig.get_return());
            let arg = Type::get::<A>(ctx);

            // TODO: Find out why this is showing an error
            /*
            assert_eq!(sig.get_params(), if let Some(args) = StructType::from_super(arg) {
                args.get_elements()
            } else {
                vec![arg]
            });*/
        }
        unsafe {
            cb(self.get_function::<A, R>(function));
        }
    }
    /// Run the closure `cb` with the machine code for the function `function`.
    pub unsafe fn with_function_unchecked<C, A, R>(&self, function:&Function, cb: C)
    where
        A: Constant,
        R: Constant,
        C: FnOnce(extern "C" fn(A) -> R),
    {
        cb(self.get_function::<A, R>(function));
    }
    /// Returns a pointer to the machine code for the function `function`.
    ///
    /// This is marked as unsafe because the types given as arguments and return could be different
    /// from their internal representation.
    pub unsafe fn get_function<A, R>(&self, function:&Function) -> extern "C" fn(A) -> R {
        let ptr:&u8 = self.get_global(function);
        std::mem::transmute(ptr)
    }
}
impl<'l> ExecutionEngine<'l> for JitEngine {
    type Options = JitOptions;
    fn new(module:&'l Module, options:JitOptions) -> Result<CSemiBox<'l, JitEngine>, CBox<str>> {
        unsafe {
            let mut ee = std::mem::MaybeUninit::zeroed().assume_init();
            let mut out = std::mem::zeroed();
            llvm_sys::execution_engine::LLVMLinkInMCJIT();
            if llvm_sys::target::LLVM_InitializeNativeTarget() == 1 {
                return Err("failed to initialize native target".into());
            }
            if llvm_sys::target::LLVM_InitializeNativeAsmPrinter() == 1 {
                return Err("failed to initialize native asm printer".into());
            }
            let mut options = llvm_sys::execution_engine::LLVMMCJITCompilerOptions {
                OptLevel: options.opt_level as c_uint,
                CodeModel: llvm_sys::target_machine::LLVMCodeModel::LLVMCodeModelDefault,
                NoFramePointerElim: 0,
                EnableFastISel: 1,
                MCJMM: ptr::null_mut(),
            };
            let size = std::mem::size_of::<llvm_sys::execution_engine::LLVMMCJITCompilerOptions>();
            let result = llvm_sys::execution_engine::LLVMCreateMCJITCompilerForModule(
                &mut ee,
                (&*module).into(),
                &mut options,
                size,
                &mut out,
            );
            if result == 0 {
                Ok(ee.into())
            } else {
                Err(CBox::new(out))
            }
        }
    }
}