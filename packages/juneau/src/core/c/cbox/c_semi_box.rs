use libc::{c_char, size_t};
use std::marker::PhantomData;
use std::ffi::{CStr, CString};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::borrow::Borrow;
use std::mem::forget;
use super::{DisposeRef};


/// A wrapper for pointers made by C that are now partially owned in Rust.
///
/// This is necessary to allow owned and borrowed representations of C types
/// to be represented by the same type as they are in C with little overhead
pub struct CSemiBox<'a, D:?Sized> where D:DisposeRef + 'a {
    ptr: *mut D::RefTo,
    marker: PhantomData<&'a ()>
}
impl<'a, D:?Sized> CSemiBox<'a, D> where D:DisposeRef + 'a {
    #[inline(always)]
    /// Wrap the pointer in a `CSemiBox`
    pub fn new(ptr: *mut D::RefTo) -> Self {
        CSemiBox {
            ptr: ptr,
            marker: PhantomData
        }
    }
    #[inline(always)]
    /// Returns the internal pointer
    pub unsafe fn as_ptr(&self) -> *mut D::RefTo {
        self.ptr
    }
    #[inline(always)]
    /// Returns the internal pointer
    pub unsafe fn unwrap(self) -> *mut D::RefTo {
        let ptr = self.ptr;
        forget(self);
        ptr
    }
}
impl<'a, D:?Sized> From<*mut D::RefTo> for CSemiBox<'a, D> where D:DisposeRef+'a {
    #[inline(always)]
    fn from(ptr: *mut D::RefTo) -> Self {
        CSemiBox::new(ptr)
    }
}
impl<'a, D:?Sized> Drop for CSemiBox<'a, D> where D:DisposeRef+'a {
    #[inline(always)]
    /// Run the destructor
    fn drop(&mut self) {
        unsafe { <D as DisposeRef>::dispose(self.ptr) }
    }
}
impl<'a, D> Deref for CSemiBox<'a, D> where D:DisposeRef+'a, *mut D::RefTo:Into<&'a D> {
    type Target = D;
    fn deref(&self) -> &D {
        self.ptr.into()
    }
}
impl<'a, D> Borrow<D> for CSemiBox<'a, D> where D:DisposeRef+'a, *mut D::RefTo:Into<&'a D> {
    fn borrow(&self) -> &D {
        self.ptr.into()
    }
}
impl<'a, D> DerefMut for CSemiBox<'a, D> where D:DisposeRef+'a, *mut D::RefTo:Into<&'a D>, *mut D::RefTo:Into<&'a mut D> {
    fn deref_mut(&mut self) -> &mut D {
        self.ptr.into()
    }
}
impl<'a, T> Display for CSemiBox<'a, T> where T:Display+DisposeRef+'a, *mut T::RefTo:Into<&'a T> {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        Display::fmt(self as &T, fmt)
    }
}
impl<'a, T> Debug for CSemiBox<'a, T> where T:Debug+DisposeRef+'a, *mut T::RefTo:Into<&'a T> {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        Debug::fmt(self as &T, fmt)
    }
}
impl<'a, T> PartialEq<T> for CSemiBox<'a, T> where T:'a+DisposeRef+PartialEq, *mut T::RefTo:Into<&'a T> {
    fn eq(&self, other: &T) -> bool {
        (self as &T).eq(other)
    }
}
impl<'a> From<&'a CStr> for CSemiBox<'a, str> {
    fn from(text: &'a CStr) -> CSemiBox<'a, str> {
        CSemiBox::new(text.as_ptr() as *mut c_char)
    }
}