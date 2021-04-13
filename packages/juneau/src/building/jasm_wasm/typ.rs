use crate::{
    core::Id,
    semantic::{
        jasm::{
            Block, Jasm, JasmExpression, JasmExpressionVisitor, JasmStatement,
            JasmStatementVisitor, JasmType, JasmValue, Struct,
        },
        Function, FunctionType, Name, Parameter, Variable,
    },
};
use walrus::ValType;
use wasmer::Val;

impl From<&JasmType> for ValType {
    fn from(typ: &JasmType) -> Self {
        match typ {
            JasmType::Bool => ValType::I32,
            JasmType::U8 => ValType::I32,
            JasmType::I64 => ValType::I64,
            JasmType::U64 => ValType::I64,
            JasmType::F64 => ValType::F64,
            JasmType::String => ValType::I32,
            JasmType::Void => panic!("not supported in wasm, only llvm"),
            JasmType::Pointer(jasm_type) => Self::from(jasm_type.as_ref()), // TODO: find alternatives to &**
            JasmType::Function(_func_type) => ValType::Funcref,
            JasmType::Struct(_id, _name) => panic!("not supported in wasm, only llvm"),
            JasmType::Array(jasm_type) => Self::from(jasm_type.as_ref()),
        }
    }
}
