use libc::{c_ulonglong, c_int};

use crate::{core::c::CSemiBox, target::llvm::{Constant, Context}};
use super::GenericValue;


/// A value that can be cast into a `GenericValue` and that a `GenericValue` can be cast into.
///
/// Both these methods require contexts because some `Type` constructors are needed for the
/// conversion and these constructors need a context.
pub trait GenericValueCast {
    /// Create a `GenericValue` from this value.
    fn to_generic(self, context: &Context) -> CSemiBox<GenericValue>;
    /// Convert the `GenericValue` into a value of this type again.
    fn from_generic(value: &GenericValue, context: &Context) -> Self;
}

impl GenericValueCast for f64 {
    fn to_generic(self, ctx: &Context) -> CSemiBox<GenericValue> {
        unsafe {
            let ty = llvm_sys::core::LLVMDoubleTypeInContext(ctx.into());
            CSemiBox::new(llvm_sys::execution_engine::LLVMCreateGenericValueOfFloat(ty, self))
        }
    }
    fn from_generic(value: &GenericValue, ctx: &Context) -> f64 {
        unsafe {
            let ty = llvm_sys::core::LLVMDoubleTypeInContext(ctx.into());
            llvm_sys::execution_engine::LLVMGenericValueToFloat(ty, value.into())
        }
    }
}
impl GenericValueCast for f32 {
    fn to_generic(self, ctx: &Context) -> CSemiBox<GenericValue> {
        unsafe {
            let ty = llvm_sys::core::LLVMFloatTypeInContext(ctx.into());
            CSemiBox::new(llvm_sys::execution_engine::LLVMCreateGenericValueOfFloat(ty, self as f64))
        }
    }
    fn from_generic(value: &GenericValue, ctx: &Context) -> f32 {
        unsafe {
            let ty = llvm_sys::core::LLVMFloatTypeInContext(ctx.into());
            llvm_sys::execution_engine::LLVMGenericValueToFloat(ty, value.into()) as f32
        }
    }
}
macro_rules! generic_int(
    ($ty:ty, $signed:expr) => (
        impl GenericValueCast for $ty {
            fn to_generic(self, ctx: &Context) -> CSemiBox<GenericValue> {
                unsafe {
                    let ty = <Self as Constant>::get_type(ctx);
                    CSemiBox::new(llvm_sys::execution_engine::LLVMCreateGenericValueOfInt(ty.into(), self as c_ulonglong, $signed as c_int))
                }
            }
            fn from_generic(value: &GenericValue, _: &Context) -> $ty {
                unsafe {
                    llvm_sys::execution_engine::LLVMGenericValueToInt(value.into(), $signed as c_int) as $ty
                }
            }
        }
    );
    (some $signed:ty, $unsigned:ty) => (
        generic_int!{$signed, true}
        generic_int!{$unsigned, false}
    );
);

generic_int! {some i8, u8}
generic_int! {some i16, u16}
generic_int! {some i32, u32}
generic_int! {some i64, u64}
generic_int! {some isize, usize}


impl GenericValueCast for bool {
    fn to_generic(self, ctx: &Context) -> CSemiBox<GenericValue> {
        unsafe {
            let ty = <Self as Constant>::get_type(ctx);
            CSemiBox::new(llvm_sys::execution_engine::LLVMCreateGenericValueOfInt(
                ty.into(),
                self as c_ulonglong,
                0,
            ))
        }
    }
    fn from_generic(value: &GenericValue, _: &Context) -> bool {
        unsafe {llvm_sys::execution_engine::LLVMGenericValueToInt(value.into(), 0) != 0}
    }
}