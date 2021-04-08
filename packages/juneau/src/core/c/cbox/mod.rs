mod c_box;
mod c_semi_box;
#[macro_use]
mod dispose_ref;


pub use c_box::{CBox};
pub use c_semi_box::{CSemiBox};
pub use dispose_ref::{DisposeRef};


#[macro_export]
macro_rules! deref(
    ($ty:ty, $to:ty) => (
        impl std::ops::Deref for $ty {
            type Target = $to;
            fn deref(&self) -> &$to {
                unsafe {std::mem::transmute(self)}
            }
        }
    );
);