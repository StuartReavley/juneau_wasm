use crate::{building::jasm_wasm::visitor::WasmBuilderVisitor, core::{Id, Visitor}, semantic::{
        jasm::{
            Block, Jasm, JasmExpression, JasmExpressionVisitor, JasmStatement,
            JasmStatementVisitor, JasmType, JasmValue, Struct,
        },
        Function, FunctionType, Name, Parameter, Variable,
    }};
use walrus::ValType;
use wasmer::Val;


impl Visitor<&JasmType, ValType> for WasmBuilderVisitor {
    fn visit(&mut self, typ:&JasmType) -> ValType {
        use JasmType::*;
        match typ {
            Bool => ValType::I32,
            U8 => ValType::I32,
            I64 => ValType::I64,
            U64 => ValType::I64,
            F64 => ValType::F64,
            String => ValType::I32,
            Void => panic!("not supported in wasm, only llvm"),
            Pointer(jasm_type) => self.visit(jasm_type.as_ref()),
            Function(_func_type) => ValType::Funcref,
            Struct(_id, _name) => panic!("not supported in wasm, only llvm"),
            Array(jasm_type) => self.visit(jasm_type.as_ref())
        }
    }
}