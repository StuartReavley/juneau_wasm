use libc::{c_char, size_t};
use std::ffi::{CStr, CString};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::borrow::Borrow;
use std::mem::{transmute, forget};
use super::{DisposeRef, CSemiBox};


/// A wrapper for pointers made by C that are now completely owned by Rust, so
/// they are not limited by any lifetimes.
///
/// This is necessary to allow owned and borrowed representations of C types
/// to be represented by the same type as they are in C with little overhead.
pub struct CBox<D:?Sized> where D:DisposeRef {
    ptr: *mut D::RefTo
}
impl<D:?Sized> CBox<D> where D:DisposeRef {
    #[inline(always)]
    /// Wrap the pointer in a `CBox`.
    pub fn new(ptr: *mut D::RefTo) -> Self {
        CBox {
            ptr: ptr
        }
    }
    #[inline(always)]
    /// Returns the internal pointer.
    pub unsafe fn as_ptr(&self) -> *mut D::RefTo {
        self.ptr
    }
    #[inline(always)]
    /// Returns the internal pointer.
    pub unsafe fn unwrap(self) -> *mut D::RefTo {
        let ptr = self.ptr;
        forget(self);
        ptr
    }
    /// Returns the box as a 'CSemiBox'.
    pub fn as_semi<'a>(&'a self) -> &CSemiBox<'a, D> {
        unsafe {transmute(self)}
    }
    /// Returns the box as a 'CSemiBox'.
    pub fn as_semi_mut<'a>(&'a mut self) -> &mut CSemiBox<'a, D> {
        unsafe {transmute(self)}
    }
}
impl<'a> From<&'a str> for CBox<str> {
    /// Copy this text using malloc and strcpy.
    fn from(text: &'a str) -> CBox<str> {
        unsafe {
            let cstr = CString::new(text).unwrap();
            let ptr = libc::malloc(text.len() as size_t + 1) as *mut c_char;
            libc::strcpy(ptr, cstr.as_ptr());
            CBox::new(ptr)
        }
    }
}

impl<'a> Deref for CBox<str> {
    type Target = str;
    fn deref(&self) -> &str {
        unsafe {
            let text = CStr::from_ptr(self.ptr);
            std::str::from_utf8_unchecked(text.to_bytes())
        }
    }
}
impl Clone for CBox<str> {
    fn clone(&self) -> CBox<str> {
        unsafe {
            let ptr = libc::malloc(self.len() as size_t + 1) as *mut c_char;
            libc::strcpy(ptr, self.ptr);
            CBox::new(ptr)
        }
    }
}
impl Display for CBox<str> {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str(self.deref())
    }
}
impl Debug for CBox<str> {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str(self.deref())
    }
}

impl<T> Deref for CBox<T> where T:DisposeRef {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe{transmute(self.ptr)}
    }
}
impl<T> Borrow<T> for CBox<T> where T:DisposeRef {
    fn borrow(&self) -> &T {
        unsafe{transmute(self.ptr)}
    }
}
impl<T> DerefMut for CBox<T> where T:DisposeRef {
    fn deref_mut(&mut self) -> &mut T {
        unsafe{transmute(self.ptr)}
    }
}
impl<'a, T> PartialEq<T> for CBox<T> where T:'a+DisposeRef+PartialEq, *mut T::RefTo:Into<&'a T> {
    fn eq(&self, other: &T) -> bool {
        unsafe{transmute::<_, &T>(self.ptr) == other}
    }
}