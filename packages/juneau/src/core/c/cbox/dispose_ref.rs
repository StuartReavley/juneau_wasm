use libc::{c_char, c_void, free};



/// Implemented by any type of which its reference represents a C pointer that can be disposed.
pub trait DisposeRef {
    /// What a reference to this type represents as a C pointer.
    type RefTo;
    /// Destroy the contents at the pointer's location.
    ///
    /// This should run some variant of `libc::free(ptr)`
    unsafe fn dispose(ptr: *mut Self::RefTo) {
        free(ptr as *mut c_void);
    }
}


impl DisposeRef for str {
    type RefTo = c_char;
}

#[macro_export]
macro_rules! dispose(
    ($ty:ty, $ref_ty:ty, $func:expr) => (

        impl crate::core::c::DisposeRef for $ty {
            type RefTo = $ref_ty;
            #[inline(always)]
            unsafe fn dispose(ptr: *mut $ref_ty) {
                $func(ptr)
            }
        }
    );
);