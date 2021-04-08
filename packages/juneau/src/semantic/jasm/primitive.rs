use std::{ffi::c_void, rc::Rc};
use crate::core::{Id, IdContext, IdProvider};
use crate::semantic::{BinaryOperator, UnaryOperator, Function, FunctionType, Parameter, Implementation};
use crate::semantic::jasm::{Jasm, JasmType};


#[derive(Debug, Clone, PartialEq)]
pub enum JasmPrimitiveImplementation {
    External{is_pure: bool, ptr: *mut c_void},
    Unary(NumberType, UnaryOperator),
    Binary(NumberType, BinaryOperator)
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NumberType {
    UnsignedInteger,
    SignedInteger,
    Float,
    Bool
}


impl Function<Jasm> {
    pub fn new_external(ids:&mut impl IdContext, name: &str, return_typ:JasmType, parameter_types: &[JasmType], is_pure:bool, ptr: *mut c_void) -> Rc<Self> {
        Function::new(
            ids.new_id(),
            name.into(),
            parameter_types.iter().enumerate().map(|(i, typ)|Parameter::new(ids.new_id(), &format!("param_{}", i).into(), typ)).collect(),
            Implementation::new_primitive(return_typ, JasmPrimitiveImplementation::External{is_pure, ptr}))
    }
}



// pub struct FunctionDefinition {
//     id:Id,
//     name:String,
//     parameters: Vec<u64>,
//     definition: u64
// }

// impl FunctionDefinition {
//     pub fn new(ids:&mut IdProvider, name:&str) {

//     }
// }


// pub fn new_definitions(ids:&mut IdProvider) {

//     FunctionDefinition::new(ids, "+", )

// }



// // for add:
// //   primitives:
// //      integer variants (i32, u32, i64, u64)
// //      float variants (f64)
// //   implicit casts (maybe not have any of these to start...)
// //      (u32, i32), (i32, u32), etc
// //   

// pub enum PrimitiveFunction {
//     IntegerAdd,
//     FloatAdd
// }


// // bin_op! {build_add, LLVMBuildAdd, LLVMBuildFAdd}
    