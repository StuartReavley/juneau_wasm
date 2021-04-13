use crate::{core::c::CSemiBox, dispose};
use super::block::BasicBlock;
use super::context::Context;
use llvm_sys::prelude::{LLVMValueRef};
use libc::{c_char, c_uint};
use std::{ffi::CString, marker::PhantomData};
use super::types::Type;
use super::value::{Function, Predicate, Value};

static NULL_NAME: [c_char; 1] = [0];

native_reference! {
    /// This provides a uniform API for creatinSg instructions and inserting them into a basic block.
    Builder = llvm_sys::LLVMBuilder
}
dispose! {Builder, llvm_sys::LLVMBuilder, llvm_sys::core::LLVMDisposeBuilder}


macro_rules! unary_operator(
    ($name:ident, $func:expr) => (
        pub fn $name<'c>(&self, value:&'c Value) -> &'c Value {
            unsafe {$func(self.into(), value.into(), NULL_NAME.as_ptr())}.into()
        }
    );
);
macro_rules! binary_operator(
    ($name:ident, $func:expr) => (
        pub fn $name<'c>(&self, left:&'c Value, right:&'c Value) -> &'c Value {
            unsafe {$func(self.into(), left.into(), right.into(), NULL_NAME.as_ptr())}.into()
        }
    );
);

impl Builder {
    /// Create a new builder in the context given.
    pub fn new<'l>(context:&'l Context) -> CSemiBox<'l, Builder> {
        CSemiBox::new(unsafe {llvm_sys::core::LLVMCreateBuilderInContext(context.into())}.into())
    }

    /// Position the builder at the end of `block`.
    pub fn get_insert_block<'l>(&'l self) -> &'l BasicBlock {
        unsafe {llvm_sys::core::LLVMGetInsertBlock(self.into()).into()}
    }
    
    /// Position the builder at the end of `block`.
    pub fn position_at_end<'l>(&'l self, block:&'l BasicBlock) {
        unsafe {llvm_sys::core::LLVMPositionBuilderAtEnd(self.into(), block.into())}
    }
    /// Build an instruction that returns from the function with void.
    pub fn build_ret_void<'l>(&'l self) -> &'l Value {
        unsafe {llvm_sys::core::LLVMBuildRetVoid(self.into())}.into()
    }
    /// Build an instruction that returns from the function with `value`.
    pub fn build_ret<'l>(&'l self, value:&'l Value) -> &'l Value {
        unsafe {llvm_sys::core::LLVMBuildRet(self.into(), value.into())}.into()
    }
    /// Build an instruction that allocates an array with the element type `elem` and the size `size`.
    ///
    /// The size of this array will be the size of `elem` times `size`.
    pub fn build_array_alloca<'l>(&'l self, element:&'l Type, size:&'l Value) -> &'l Value {
        unsafe {llvm_sys::core::LLVMBuildArrayAlloca(
            self.into(),
            element.into(),
            size.into(),
            NULL_NAME.as_ptr() as *const c_char)}.into()
    }
    /// Build an instruction that allocates a pointer to fit the size of `ty` then returns this pointer.
    ///
    /// Make sure to call `build_free` with the pointer value when you're done with it, or you're
    /// gonna have a bad time.
    pub fn build_allocation<'l>(&self, name:&str, ty:&'l Type) -> &'l Value {
        let c_name = CString::new(name).unwrap();
        unsafe {llvm_sys::core::LLVMBuildAlloca(self.into(), ty.into(), c_name.as_ptr() as *const c_char)}.into()
    }
    /// Build an instruction that frees the `val`, which _MUST_ be a pointer that was returned
    /// from `build_alloca`.
    pub fn build_free(&self, val: &Value) -> &Value {
        unsafe {llvm_sys::core::LLVMBuildFree(self.into(), val.into())}.into()
    }
    /// Build an instruction that store the value `val` in the pointer `ptr`.
    pub fn build_store(&self, val: &Value, ptr: &Value) -> &Value {
        unsafe {llvm_sys::core::LLVMBuildStore(self.into(), val.into(), ptr.into())}.into()
    }
    /// Build an instruction that branches to the block `dest`.
    pub fn build_br(&self, dest: &BasicBlock) -> &Value {
        unsafe {llvm_sys::core::LLVMBuildBr(self.into(), dest.into()).into()}
    }
    /// Build an instruction that branches to `if_block` if `cond` evaluates to true, and `else_block` otherwise.
    pub fn build_cond_br(&self, cond: &Value, if_block: &BasicBlock, else_block: &BasicBlock) -> &Value {
        unsafe {llvm_sys::core::LLVMBuildCondBr(self.into(), cond.into(), if_block.into(), else_block.into())}.into()
    }
    /// Build an instruction that calls the function `func` with the arguments `args`.
    ///
    /// This will return the return value of the function.
    pub fn build_call<'c>(&self, func: &'c Function, args: &[&'c Value]) -> &'c Value {
        unsafe {
            let call = llvm_sys::core::LLVMBuildCall(
                self.into(),
                func.into(),
                args.as_ptr() as *mut LLVMValueRef,
                args.len() as c_uint,
                NULL_NAME.as_ptr(),
            );
            llvm_sys::core::LLVMSetTailCall(call, 0);
            call.into()
        }
    }
    /// Build an instruction that calls the function `func` with the arguments `args`.
    ///
    /// This will return the return value of the function.
    pub fn build_tail_call(&self, func: &Function, args: &[&Value]) -> &Value {
        unsafe {
            let call = llvm_sys::core::LLVMBuildCall(
                self.into(),
                func.into(),
                args.as_ptr() as *mut LLVMValueRef,
                args.len() as c_uint,
                NULL_NAME.as_ptr(),
            );
            llvm_sys::core::LLVMSetTailCall(call, 1);
            call.into()
        }
    }
    /// Build an instruction that converts `val` to an integer `dest`.
    pub fn build_fp_to_si<'c>(&self, val: &'c Value, dest: &'c Type) -> &'c Value {
        unsafe {llvm_sys::core::LLVMBuildFPToSI(self.into(), val.into(), dest.into(), NULL_NAME.as_ptr()).into()}
    }
    /// Build an instruction that converts `val` to a floating point `dest`.
    pub fn build_si_to_fp<'c>(&self, val: &'c Value, dest: &'c Type) -> &'c Value {
        unsafe {llvm_sys::core::LLVMBuildSIToFP(self.into(), val.into(), dest.into(), NULL_NAME.as_ptr()).into()}
    }
    /// Build an instruction that yields to `true_val` if `cond` is equal to `1`, and `false_val` otherwise.
    pub fn build_select<'c>(&self, cond: &'c Value, true_val: &'c Value, false_val: &'c Value) -> &'c Value {
        unsafe {llvm_sys::core::LLVMBuildSelect(
                self.into(),
                cond.into(),
                true_val.into(),
                false_val.into(),
                NULL_NAME.as_ptr())}.into()
    }
    /// Build an instruction that casts a value into a certain type.
    pub fn build_bit_cast<'c>(&self, value: &'c Value, dest: &'c Type) -> &'c Value {
        unsafe {llvm_sys::core::LLVMBuildBitCast(self.into(), value.into(), dest.into(), NULL_NAME.as_ptr())}.into()
    }
    /// Build an instruction that casts an integer to a pointer.
    pub fn build_int_to_ptr<'c>(&self, val: &'c Value, dest: &'c Type) -> &'c Value {
        unsafe {
            llvm_sys::core::LLVMBuildIntToPtr(self.into(), val.into(), dest.into(), NULL_NAME.as_ptr()).into()
        }
    }
    /// Build an instruction that casts a pointer to an integer.
    pub fn build_ptr_to_int<'c>(&self, val: &'c Value, dest: &'c Type) -> &'c Value {
        unsafe {
            llvm_sys::core::LLVMBuildPtrToInt(self.into(), val.into(), dest.into(), NULL_NAME.as_ptr()).into()
        }
    }
    /// Build an instruction that zero extends its operand to the type `dest`.
    pub fn build_zext<'c>(&self, value: &'c Value, dest: &'c Type) -> &'c Value {
        unsafe {
            llvm_sys::core::LLVMBuildZExtOrBitCast(self.into(), value.into(), dest.into(), NULL_NAME.as_ptr())
                .into()
        }
    }
    /// Build an instruction that truncates the high-order bits of value to fit into a certain type.
    pub fn build_trunc<'c>(&self, value: &'c Value, dest: &'c Type) -> &'c Value {
        unsafe {
            llvm_sys::core::LLVMBuildTrunc(self.into(), value.into(), dest.into(), NULL_NAME.as_ptr()).into()
        }
    }
    /// Build an instruction that inserts a value into an aggregate data value.
    pub fn build_insert_value(&self, agg: &Value, elem: &Value, index: usize) -> &Value {
        unsafe {
            llvm_sys::core::LLVMBuildInsertValue(
                self.into(),
                agg.into(),
                elem.into(),
                index as c_uint,
                NULL_NAME.as_ptr(),
            )
            .into()
        }
    }
    /// Build an instruction that computes the address of a subelement of an aggregate data structure.
    ///
    /// Basically type-safe pointer arithmetic.
    pub fn build_gep<'c>(&self, pointer: &'c Value, indices: &[&'c Value]) -> &'c Value {
        unsafe {
            llvm_sys::core::LLVMBuildInBoundsGEP(
                self.into(),
                pointer.into(),
                indices.as_ptr() as *mut LLVMValueRef,
                indices.len() as c_uint,
                NULL_NAME.as_ptr(),
            )
            .into()
        }
    }
    /// Build an instruction that runs whichever block matches the value, or `default` if none of them matched it.
    pub fn build_switch(
        &self,
        value: &Value,
        default: &BasicBlock,
        cases: &[(&Value, &BasicBlock)],
    ) -> &Value {
        unsafe {
            let switch = llvm_sys::core::LLVMBuildSwitch(
                self.into(),
                value.into(),
                default.into(),
                cases.len() as c_uint,
            );
            for case in cases {
                llvm_sys::core::LLVMAddCase(switch, case.0.into(), case.1.into());
            }
            switch.into()
        }
    }
    /// Build a phi node which is used together with branching to select a value depending on the predecessor of the current block
    pub fn build_phi<'ctx>(&self, ty: &'ctx Type, entries: &[(&'ctx Value, &'ctx BasicBlock)]) -> &'ctx Value {
        let phi_node = unsafe {llvm_sys::core::LLVMBuildPhi(self.into(), ty.into(), NULL_NAME.as_ptr())};

        for &(val, preds) in entries {
            unsafe {llvm_sys::core::LLVMAddIncoming(phi_node, &mut val.into(), &mut preds.into(), 1)}
        }
        phi_node.into()
    }

    unary_operator! {build_load, llvm_sys::core::LLVMBuildLoad}
    unary_operator! {build_integer_negate, llvm_sys::core::LLVMBuildNeg}
    unary_operator! {build_float_negate, llvm_sys::core::LLVMBuildFNeg}
    unary_operator! {build_not, llvm_sys::core::LLVMBuildNot}
    
    binary_operator! {build_integer_add, llvm_sys::core::LLVMBuildAdd}
    binary_operator! {build_float_add, llvm_sys::core::LLVMBuildFAdd}
    binary_operator! {build_integer_subtract, llvm_sys::core::LLVMBuildSub}
    binary_operator! {build_float_subtract, llvm_sys::core::LLVMBuildFSub}
    binary_operator! {build_integer_multiply, llvm_sys::core::LLVMBuildMul}
    binary_operator! {build_float_multiply, llvm_sys::core::LLVMBuildFMul}
    binary_operator! {build_unsigned_integer_divide, llvm_sys::core::LLVMBuildUDiv}
    binary_operator! {build_signed_integer_divide, llvm_sys::core::LLVMBuildSDiv}
    binary_operator! {build_float_divide, llvm_sys::core::LLVMBuildFDiv}
    binary_operator! {build_shift_left, llvm_sys::core::LLVMBuildShl}
    binary_operator! {build_arithmetic_shift_right, llvm_sys::core::LLVMBuildAShr}
    binary_operator! {build_and, llvm_sys::core::LLVMBuildAnd}
    binary_operator! {build_or, llvm_sys::core::LLVMBuildOr}
    binary_operator! {build_xor, llvm_sys::core::LLVMBuildXor}


    /// Build an instruction to compare two values with the predicate given.
    pub fn build_integer_compare<'c>(&self, left:&'c Value, right:&'c Value, predicate:Predicate) -> &'c Value {
        assert_eq!(left.get_type(), right.get_type());
        unsafe {llvm_sys::core::LLVMBuildICmp(
            self.into(),
            predicate.into(),
            left.into(),
            right.into(),
            NULL_NAME.as_ptr())
        }
        .into()
    }
    /// Build an instruction to compare two values with the predicate given.
    pub fn build_float_compare<'c>(&self, left:&'c Value, right:&'c Value, predicate:Predicate) -> &'c Value {
        assert_eq!(left.get_type(), right.get_type());
        unsafe {llvm_sys::core::LLVMBuildFCmp(
            self.into(),
            predicate.into(),
            left.into(),
            right.into(),
            NULL_NAME.as_ptr())
        }
        .into()
    }


    
}
