
use std::{marker::Unsize, rc::Rc};
use std::cell::RefCell;

use crate::RcRefCell;

mod downcast;
pub use downcast::*;

mod rc;
pub use rc::*;




pub fn downcast_box<T:Downcast+?Sized, U:'static>(value:Box<T>) -> Option<Box<U>> {
    value.into_any().downcast::<U>().ok()
}
pub fn downcast_mut<T:Downcast+?Sized, U:'static>(value:&mut T) -> Option<&mut U> {
    value.as_any_mut().downcast_mut::<U>()
}
pub fn downcast_ref<T:Downcast+?Sized, U:'static>(value:&T) -> Option<&U> {
    value.as_any().downcast_ref::<U>()
}
