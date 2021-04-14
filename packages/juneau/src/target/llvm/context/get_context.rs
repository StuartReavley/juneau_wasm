use super::Context;


/// Implemented by everything that is owned by a context.
pub trait GetContext {
    /// Returns a reference to the context that owns this value.
    ///
    /// ```rust
    /// use llvm::*;
    /// let context = Context::new();
    /// let module = Module::new("main", &context.as_semi());
    /// assert!(context == *module.get_context());
    /// ```
    fn get_context(&self) -> &Context;
}

#[macro_export]
macro_rules! get_context(
    ($ty:ty, $func:expr) => (
        impl crate::target::llvm::GetContext for $ty {
            fn get_context(&self) -> &crate::target::llvm::Context {
                unsafe {$func(self.into())}.into()
            }
        }
    );
);