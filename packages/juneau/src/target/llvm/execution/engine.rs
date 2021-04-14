use libc::{c_char, c_uint};
use llvm_sys::execution_engine::LLVMGetPointerToGlobal;
use crate::core::c::{CBox, CSemiBox, DisposeRef, with_cstr};
use crate::target::llvm::{Module, Function, Value};
use super::{ExecutableFunction, GenericValue};


/// An abstract interface for implementation execution of LLVM modules.
///
/// This is designed to support both interpreter and just-in-time (JIT) compiler implementations.
pub trait ExecutionEngine<'l>: 'l + Sized + DisposeRef
where
    llvm_sys::execution_engine::LLVMExecutionEngineRef: From<&'l Self>,
{
    /// The options given to the engine upon creation.
    type Options: Copy;
    /// Create a new execution engine with the given `Module` and optiions, or return a
    /// description of the error.
    fn new(module: &'l Module, options: Self::Options) -> Result<CSemiBox<'l, Self>, CBox<str>>;

    /// Add a module to the list of modules to interpret or compile.
    fn add_module(&'l self, module: &'l Module) {
        unsafe {llvm_sys::execution_engine::LLVMAddModule(self.into(), (&*module).into()) }
    }
    /// Remove a module from the list of modules to interpret or compile.
    fn remove_module(&'l self, module: &'l Module) -> &'l Module {
        unsafe {
            let mut out = std::mem::MaybeUninit::zeroed().assume_init();
            llvm_sys::execution_engine::LLVMRemoveModule(self.into(), module.into(), &mut out, std::ptr::null_mut());
            out.into()
        }
    }
    /// Execute all of the static constructors for this program.
    fn run_static_constructors(&'l self) {
        unsafe {llvm_sys::execution_engine::LLVMRunStaticConstructors(self.into())}
    }
    /// Execute all of the static destructors for this program.
    fn run_static_destructors(&'l self) {
        unsafe {llvm_sys::execution_engine::LLVMRunStaticDestructors(self.into())}
    }
    /// Attempt to find a function with the name given, or `None` if there wasn't
    /// a function with that name.
    fn find_function(&'l self, name: &str) -> Option<&'l Function> {
        with_cstr(name, |c_name| unsafe {
            let mut out = std::mem::zeroed();
            llvm_sys::execution_engine::LLVMFindFunction(self.into(), c_name, &mut out);
            std::mem::transmute(out)
        })
    }
    /// Run `function` with the arguments given as ``GenericValue`s, then return the result as one.
    ///
    /// Note that if this engine is a `JitEngine`, it only supports a small fraction of combinations
    /// for the arguments and return value, so be warned.
    ///
    /// To convert the arguments to `GenericValue`s, you should use the `GenericValueCast::to_generic` method.
    /// To convert the return value from a `GenericValue`, you should use the `GenericValueCast::from_generic` method.
    fn run_function(
        &'l self,
        function: &'l Function,
        args: &[&'l GenericValue],
    ) -> &'l GenericValue {
        let ptr = args.as_ptr() as *mut llvm_sys::execution_engine::LLVMGenericValueRef;
        unsafe {llvm_sys::execution_engine::LLVMRunFunction(self.into(), function.into(), args.len() as c_uint, ptr).into()}
    }
    /// Returns a pointer to the global value given.
    ///
    /// This is marked as unsafe because the type cannot be guranteed to be the same as the
    /// type of the global value at this point.
    unsafe fn get_global<T>(&'l self, global: &'l Value) -> &'l T {
        std::mem::transmute(LLVMGetPointerToGlobal(self.into(), global.into()))
    }
    /// Returns a pointer to the global value with the name given.
    ///
    /// This is marked as unsafe because the type cannot be guranteed to be the same as the
    /// type of the global value at this point.
    unsafe fn find_global<T>(&'l self, name: &str) -> Option<&'l T> {
        with_cstr(name, |ptr| {
            std::mem::transmute(llvm_sys::execution_engine::LLVMGetGlobalValueAddress(self.into(), ptr))
        })
    }

    fn get_function_address<'e>(&'e self, function: &Function) -> Option<&'e ExecutableFunction> {
        function.get_name().map(|name|
            unsafe {
                let addr = llvm_sys::execution_engine::LLVMGetFunctionAddress(std::mem::transmute(self), name.as_ptr() as *const c_char);
                std::mem::transmute(addr)
            })
    }
}

