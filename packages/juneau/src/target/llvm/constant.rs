use std::ffi::CStr;
use libc::{c_char, c_uint, c_ulonglong};
use llvm_sys::core::LLVMInt8TypeInContext;
use crate::target::llvm::{Context, Value, Type, PointerType, StructType, FunctionType, VectorType};


/// A type that can be represented as a constant in LLVM IR.
pub trait Constant where Self:Copy {
    /// Compile this value into a constant in the context given.
    fn as_llvm<'l>(self, context:&'l Context) -> &'l Value;
    /// Get the type descriptor for this type in the context given.
    fn get_type<'l>(context:&'l Context) -> &'l Type;

    fn get_null_ptr<'l>(context:&'l Context) -> &'l Value {
        unsafe {llvm_sys::core::LLVMConstNull(PointerType::new(Self::get_type(context)).into())}.into()
    }
}
macro_rules! compile_int(
    ($uty:ty, $sty:ty, $ctx:ident => $ty_ex:expr) => (
        impl Constant for $uty {
            fn as_llvm<'l>(self, context:&'l Context) -> &'l Value {
                unsafe {llvm_sys::core::LLVMConstInt(Self::get_type(context).into(), self as c_ulonglong, 0) }.into()
            }
            fn get_type<'l>($ctx: &'l Context) -> &'l Type {
                let $ctx = $ctx.into();
                unsafe { $ty_ex }.into()
            }
        }
        impl Constant for $sty {
            fn as_llvm<'l>(self, context:&'l Context) -> &'l Value {
                unsafe {llvm_sys::core::LLVMConstInt(Self::get_type(context).into(), self as c_ulonglong, 0) }.into()
            }
            fn get_type<'l>($ctx: &'l Context) -> &'l Type {
                
                let $ctx = $ctx.into();
                unsafe { $ty_ex }.into()
            }
        }
    );
    ($uty:ty, $sty:ty, $func:ident) => (
        compile_int!{$uty, $sty, ctx => llvm_sys::core::$func(ctx)}
    );
);
impl Constant for bool {
    fn as_llvm<'l>(self, context:&'l Context) -> &'l Value {
        unsafe {llvm_sys::core::LLVMConstInt(Self::get_type(context).into(), self as c_ulonglong, 0) }.into()
    }
    fn get_type<'l>(context:&'l Context) -> &'l Type {
        unsafe {llvm_sys::core::LLVMInt1TypeInContext(context.into())}.into()
    }
}
impl Constant for f32 {
    fn as_llvm<'l>(self, context:&'l Context) -> &'l Value {
        unsafe {llvm_sys::core::LLVMConstReal(Self::get_type(context).into(), self as f64) }.into()
    }
    fn get_type<'l>(context:&'l Context) -> &'l Type {
        unsafe {llvm_sys::core::LLVMFloatTypeInContext(context.into()) }.into()
    }
}
impl Constant for f64 {
    fn as_llvm<'l>(self, context:&'l Context) -> &'l Value {
        unsafe {llvm_sys::core::LLVMConstReal(Self::get_type(context).into(), self) }.into()
    }
    fn get_type<'l>(context:&'l Context) -> &'l Type {
        unsafe {llvm_sys::core::LLVMDoubleTypeInContext(context.into()) }.into()
    }
}
impl Constant for char {
    fn as_llvm<'l>(self, context:&'l Context) -> &'l Value {
        unsafe {
            llvm_sys::core::LLVMConstInt(
                Self::get_type(context).into(),
                self as u32 as c_ulonglong,
                0,
            )
        }
        .into()
    }
    fn get_type<'l>(context:&'l Context) -> &'l Type {
        unsafe {llvm_sys::core::LLVMInt32TypeInContext(context.into()) }.into()
    }
}
impl Constant for *const c_char {
    fn as_llvm<'l>(self, context: &'l Context) -> &'l Value {
        unsafe {
            let len = CStr::from_ptr(self).to_bytes().len();
            llvm_sys::core::LLVMConstStringInContext(context.into(), self, len as c_uint, 0).into()
        }
    }
    fn get_type<'l>(context: &'l Context) -> &'l Type {
        PointerType::new(Type::get::<c_char>(context))
    }
}
impl Constant for *const str {
    fn as_llvm<'l>(self, context:&'l Context) -> &'l Value {
        unsafe {std::mem::transmute::<_, &str>(self) }.as_llvm(context)
    }
    fn get_type<'l>(context:&'l Context) -> &'l Type {
        <&str as Constant>::get_type(context)
    }
}
impl<'b> Constant for &'b str {
    fn as_llvm<'l>(self, context:&'l Context) -> &'l Value {
        self.as_bytes().as_llvm(context)
    }
    fn get_type<'l>(context:&'l Context) -> &'l Type {
        <&'b [u8] as Constant>::get_type(context)
    }
}

impl<'b> Constant for &'b [u8] {
    fn as_llvm<'l>(self, context: &'l Context) -> &'l Value {
        unsafe {
            let ptr = self.as_ptr() as *const c_char;
            let len = self.len() as c_uint;
            let llvm_ptr = llvm_sys::core::LLVMConstStringInContext(context.into(), ptr, len, 1).into();
            let size = self.len().as_llvm(context);
            Value::new_struct(context, &[llvm_ptr, size], false)
        }
    }
    fn get_type<'l>(ctx: &'l Context) -> &'l Type {
        let usize_t = usize::get_type(ctx);
        StructType::new(ctx, &[usize_t, usize_t], false)
    }
}



compile_int! {u8, i8, LLVMInt8TypeInContext}
compile_int! {u16, i16, LLVMInt16TypeInContext}
compile_int! {u32, i32, LLVMInt32TypeInContext}
compile_int! {u64, i64, LLVMInt64TypeInContext}
compile_int! {usize, isize, context => llvm_sys::core::LLVMIntTypeInContext(context, std::mem::size_of::<isize>() as c_uint * 8)}
impl Constant for () {
    fn as_llvm<'l>(self, context: &'l Context) -> &'l Value {
        0u64.as_llvm(context)
        // unsafe {llvm_sys::core::LLVMConstNull(Self::get_type(context).into()) }.into()
    }
    fn get_type<'l>(context:&'l Context) -> &'l Type {
        unsafe {llvm_sys::core::LLVMVoidTypeInContext(context.into()) }.into()
    }
}

macro_rules! compile_tuple(
    ($($name:ident = $oname:ident),+) => (
        impl<$($name),+> Constant for ($($name),+) where $($name:Constant),+ {
            fn as_llvm<'l>(self, context:&'l Context) -> &'l Value {
                let ($($oname, )+) = self;
                Value::new_struct(context, &[$($oname.as_llvm(context)),+], false)
            }
            fn get_type<'l>(context:&'l Context) -> &'l Type {
                StructType::new(context, &[$($name::get_type(context)),+], false)
            }
        }
    )
);
compile_tuple! {A = a, B = b}
compile_tuple! {A = a, B = b, C = c}
compile_tuple! {A = a, B = b, C = c, D = d}
compile_tuple! {A = a, B = b, C = c, D = d, E = e}
compile_tuple! {A = a, B = b, C = c, D = d, E = e, F = f}
compile_tuple! {A = a, B = b, C = c, D = d, E = e, F = f, G = g}

macro_rules! compile_array(
    ($ty:ty, $num:expr) => (
        impl<T> Constant for $ty where T: Copy + Constant {
            fn as_llvm<'l>(self, context:&'l Context) -> &'l Value {
                let values:Vec<_> = self.iter().map(|&value| value.as_llvm(context)).collect();
                unsafe {llvm_sys::core::LLVMConstVector(values.as_ptr() as *mut llvm_sys::prelude::LLVMValueRef, $num) }.into()
            }
            fn get_type<'l>(context:&'l Context) -> &'l Type {
                VectorType::new(Type::get::<T>(context), $num)
            }
        }
    )
);
compile_array! {[T; 0], 0}
compile_array! {[T; 1], 1}
compile_array! {[T; 2], 2}
compile_array! {[T; 3], 3}
compile_array! {[T; 4], 4}
compile_array! {[T; 5], 5}
compile_array! {[T; 6], 6}

macro_rules! compile_func(
    ($($name:ident),*) => (
        impl<R, $($name),*> Constant for fn($($name),*) -> R where R:Constant, $($name:Constant),* {
            fn as_llvm<'l>(self, context: &'l Context) -> &'l Value {
                unsafe {
                    let as_usize: usize = std::mem::transmute(self);
                    let value = as_usize.as_llvm(context);
                    llvm_sys::core::LLVMConstIntToPtr(value.into(), Self::get_type(context).into())
                }.into()
            }
            fn get_type<'l>(context: &'l Context) -> &'l Type {
                FunctionType::new(&[$($name::get_type(context)),*], R::get_type(context))
            }
        }
        impl<R, $($name),*> Constant for extern fn($($name),*) -> R where R:Constant, $($name:Constant),* {
            fn as_llvm<'l>(self, context: &'l Context) -> &'l Value {
                unsafe {
                    let as_usize: usize = std::mem::transmute(self);
                    let value = as_usize.as_llvm(context);
                    llvm_sys::core::LLVMConstIntToPtr(value.into(), Self::get_type(context).into())
                }.into()
            }
            fn get_type<'l>(context: &'l Context) -> &'l Type {
                FunctionType::new(&[$($name::get_type(context)),*], R::get_type(context))
            }
        }
    )
);
compile_func! {}
compile_func! {A}
compile_func! {A, B}
compile_func! {A, B, C}
compile_func! {A, B, C, D}
compile_func! {A, B, C, D, E}
compile_func! {A, B, C, D, E, F}
compile_func! {A, B, C, D, E, F, G}
