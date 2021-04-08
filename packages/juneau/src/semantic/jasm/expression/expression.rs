use std::fmt::Debug;
use crate::{core::Id, semantic::{Expression, Variable, function}};
use crate::semantic::{Name, Function, GetType, Parameter};
use crate::semantic::jasm::{Jasm, JasmType, JasmValue, JasmStatement, Member, Struct};



#[derive(Debug, Clone, PartialEq)]
pub enum JasmExpression {
    Constant(JasmValue),

    Reference(Box<JasmExpression>),
    Dereference(Box<JasmExpression>),

    Invocation(Id, Name, Vec<JasmExpression>, JasmType), // type is only return type
    Var(Variable<Jasm>),
    StructAccess(Box<JasmExpression>, Id, Name, JasmType), // include type here also?
    ArrayAccess(Box<JasmExpression>, Box<JasmExpression>),  // include type here also?
    Cast(Box<JasmExpression>, JasmType)
}

impl Expression<Jasm> for JasmExpression {
    fn as_function(&self) -> Option<&Function<Jasm>> {
        todo!()
        // match self {
        //     JasmExpression::Function(function) => Some(function),
        //     _ => None
        // }
    }
}

impl GetType<JasmType> for JasmExpression {
    fn get_type(&self) -> JasmType {
        use JasmExpression::*;
        use JasmType::*;
        match self {
            Constant(value) => value.get_type(),
            Invocation(_, _, _, typ) => typ.to_owned(),
            Var(v) => v.get_type(),
            StructAccess(object, id, name, typ) => typ.to_owned(),
            ArrayAccess(array, _) => match array.get_type() {
                Pointer(typ) => *typ.to_owned(),
                _ => panic!("array object not a pointer type")
            },
            Cast(_, typ) => typ.to_owned(),
            Reference(expression) => expression.get_type().into_reference(),
            Dereference(expression) => match expression.get_type() {
                Pointer(typ) => *typ.to_owned(),
                _ => panic!(format!("object not a pointer type {}", expression.get_type()))
            }
        }
    }
}

