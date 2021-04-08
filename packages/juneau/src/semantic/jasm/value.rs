use std::fmt::{Display, Formatter};
use crate::semantic::GetType;
use crate::semantic::jasm::JasmType;


#[derive(Debug, Clone, PartialEq)]
pub enum JasmValue {
    Null,
    Bool(bool),
    U8(u8),
    I64(i64),
    U64(u64),
    F64(f64),
    String(std::string::String)
}

impl GetType<JasmType> for JasmValue {
    fn get_type(&self) -> JasmType {
        use JasmType::*;
        match self {
            JasmValue::Null => Void.into_reference(),
            JasmValue::Bool(_) => Bool,
            JasmValue::U8(_) => U8,
            JasmValue::I64(_) => I64,
            JasmValue::U64(_) => U64,
            JasmValue::F64(_) => F64,
            JasmValue::String(_) => String
        }
    }
}

impl Display for JasmValue {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        use JasmValue::*;
        let text:std::string::String = match self {
            Null => "null".into(),
            Bool(value) => format!("{}", value),
            U8(value) => format!("{}", value),
            I64(value) => format!("{}", value),
            U64(value) => format!("{}", value),
            F64(value) => format!("{}", value),
            String(value) => format!("\"{}\"", value),
        };
        write!(f, "{}", text)
    }
}

impl<T:JasmValueTrait> From<T> for JasmValue {
    fn from(value:T) -> Self {
        value.to_value()
    }
}
impl<T:JasmValueTrait> From<T> for JasmExpression {
    fn from(value:T) -> Self {
        JasmExpression::Constant(value.into())
    }
}
impl From<JasmValue> for JasmExpression {
    fn from(value:JasmValue) -> Self {
        JasmExpression::Constant(value)
    }
}
pub trait JasmValueTrait {
    fn to_value(self) -> JasmValue;
}

use JasmValue::*;

use super::JasmExpression;

impl JasmValueTrait for bool {
    fn to_value(self) -> JasmValue {
        Bool(self)
    }
}
impl JasmValueTrait for u8 {
    fn to_value(self) -> JasmValue {
        U8(self)
    }
}
impl JasmValueTrait for i64 {
    fn to_value(self) -> JasmValue {
        I64(self)
    }
}
impl JasmValueTrait for u64 {
    fn to_value(self) -> JasmValue {
        U64(self)
    }
}
impl JasmValueTrait for f64 {
    fn to_value(self) -> JasmValue {
        F64(self)
    }
}
impl JasmValueTrait for &str {
    fn to_value(self) -> JasmValue {
        String(self.to_owned())
    }
}
