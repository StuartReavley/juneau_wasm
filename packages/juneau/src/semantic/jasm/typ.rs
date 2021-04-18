use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::{core::Id, semantic::Type};
use juneau_core::Error;
use crate::semantic::{Name, FunctionType};
use super::Jasm;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum JasmType {
    Bool,
    U8,
    I64,
    U64,
    F64,
    String,
    Void,
    Pointer(Box<JasmType>),
    Function(FunctionType<Jasm>),
    Struct(Id, Name),
    Array(Box<JasmType>)
}

impl JasmType {
    pub fn into_reference(&self) -> Self {
        Self::Pointer(Box::new(self.to_owned()))
    }
}

impl Type<Jasm> for JasmType {
    fn as_function_type(&self) -> Option<&FunctionType<Jasm>> {
        match self {
            Self::Function(typ) => Some(typ),
            _ => None
        }
    }
}

impl From<FunctionType<Jasm>> for JasmType {
    fn from(typ:FunctionType<Jasm>) -> Self {
        Self::Function(typ)
    }
}

impl FromStr for JasmType {
    type Err = Error;

    fn from_str(value:&str) -> Result<Self, Self::Err> {
        use JasmType::*;
        match value {
            "bool" => Ok(Bool),
            "u8" => Ok(U8),
            "i64" => Ok(I64),
            "u64" => Ok(U64),
            "f64" => Ok(F64),
            "string" => Ok(String),
            "void" => Ok(Void),
            _ => Err(Error::new("cannot convert string"))
        }
    }
}


impl Display for JasmType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        use JasmType::*;
        let text = match self {
            Bool => "bool".into(),
            U8 => "u8".into(),
            I64 => "i64".into(),
            U64 => "u64".into(),
            F64 => "f64".into(),
            String => "string".into(),
            Void => "void".into(),
            Pointer(typ) => format!("*{}", typ),
            Function(function) => format!("{:?}", function),
            Struct(_, name) => format!("{}", name),
            Array(typ) => format!("{}[]", typ)
        };
        write!(f, "{}", text)
    }
}



#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NumberType {
    UnsignedInteger,
    SignedInteger,
    Float,
    Bool
}

impl From<&JasmType> for NumberType {
    fn from(typ:&JasmType) -> Self {
        use JasmType::*;
        match typ {
            Bool => NumberType::Bool,
            U8 => NumberType::UnsignedInteger,
            I64 => NumberType::SignedInteger,
            U64 => NumberType::UnsignedInteger,
            F64 => NumberType::Float,
            String => todo!(),
            Void => todo!(),
            Pointer(_) => todo!(),
            Function(_) => todo!(),
            Struct(_, _) => todo!(),
            Array(_) => todo!()
        }
    }
}
