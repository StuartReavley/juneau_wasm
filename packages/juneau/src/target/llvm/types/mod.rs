

#[macro_export]
macro_rules! sub_type {
    ($ty:ty, $kind:expr) => {
        unsafe impl crate::core::c::SubStruct<crate::target::llvm::Type> for $ty {
            fn is(ty:&crate::target::llvm::Type) -> bool {
                unsafe {
                    let kind = llvm_sys::core::LLVMGetTypeKind(ty.into());
                    kind as libc::c_uint == $kind as libc::c_uint
                }
            }
        }
        crate::deref! {$ty, crate::target::llvm::Type}
    };
}


mod array;
mod function;
mod integer;
mod pointer;
mod structt;
mod typ;
mod vector;


pub use array::*;
pub use function::*;
pub use integer::*;
pub use pointer::*;
pub use structt::*;
pub use typ::*;
pub use vector::*;