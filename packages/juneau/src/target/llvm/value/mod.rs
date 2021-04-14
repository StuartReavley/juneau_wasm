



macro_rules! sub_value {
    ($this:ty, $name:expr) => {
        sub_value! {$this, $name, super::Value}
    };
    ($this:ty, $name:expr, $sup:ty) => {
        unsafe impl crate::core::c::SubStruct<$sup> for $this {
            fn is(value: &$sup) -> bool {
                unsafe {!$name(value.into()).is_null() }
            }
            fn from_super(value: &$sup) -> Option<&$this> {
                unsafe {std::mem::transmute($name(value.into())) }
            }
        }
        impl std::ops::Deref for $this {
            type Target = $sup;
            fn deref(&self) -> &$sup {
                use crate::core::c::SubStruct;
                self.to_super()
            }
        }
    };
}




mod alias;
mod attribute;
mod function;
mod global_value;
mod global_variable;
mod linkage;
mod parameter;
mod predicate;
mod value;


pub use alias::*;
pub use attribute::*;
pub use function::*;
pub use global_value::*;
pub use global_variable::*;
pub use linkage::*;
pub use parameter::*;
pub use predicate::*;
pub use value::*;