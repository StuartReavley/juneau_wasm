use crate::{building::jasm_wasm::visitor::WasmBuilderVisitor, core::{Id, Visitor}, semantic::{
        jasm::{
            Block, Jasm, JasmExpression, JasmExpressionVisitor, JasmStatement,
            JasmStatementVisitor, JasmType, JasmValue, Struct,
        },
        Function, FunctionType, Name, Parameter, Variable,
    }};
use walrus::{ValType, ir::BinaryOp};
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

pub struct WasmOperatorBuilder {
    binops: BinaryOperator,
    return_type: ValType
}

impl WasmOperatorBuilder {
    pub fn new(binops: BinaryOperator, return_type: ValType) -> WasmOperatorBuilder {
        WasmOperatorBuilder {
            binops: binops,
            return_type: return_type
        } 
    }
}

impl From<&WasmOperatorBuilder> for BinaryOp {
    fn from (wasm_operator: &WasmOperatorBuilder) -> Self {
        let WasmOperatorBuilder { binops, return_type} = wasm_operator;
        match binops {
            BinaryOperator::Add => {
                match return_type {
                    ValType::I64 => BinaryOp::I64Add,
                    ValType::F32 => BinaryOp::F32Add,
                    ValType::F64 => BinaryOp::F64Add,
                    _ => panic!("return type {} is not implemented yet", return_type)
                }
            },
            BinaryOperator::Subtract => {
                match return_type {
                    ValType::I64 => BinaryOp::I64Sub,
                    ValType::F32 => BinaryOp::F32Sub,
                    ValType::F64 => BinaryOp::F64Sub,
                    _ => panic!("return type {} is not implemented yet", return_type)
                }
            },
            BinaryOperator::Multiply => {
                match return_type {
                    ValType::I64 => BinaryOp::I64Mul,
                    ValType::F32 => BinaryOp::F32Mul,
                    ValType::F64 => BinaryOp::F64Mul,
                    _ => panic!("return type {} is not implemented yet", return_type)
                }
            },
            BinaryOperator::Divide => {
                match return_type {
                    ValType::I64 => BinaryOp::I64DivS,
                    ValType::F32 => BinaryOp::F32Div,
                    ValType::F64 => BinaryOp::F64Div,
                    _ => panic!("return type {} is not implemented yet", return_type)
                }
            },
            BinaryOperator::Modulo => {
                match return_type {
                    ValType::I64 => BinaryOp::I64RemS,
                    _ => panic!("return type {} is not implemented yet", return_type)
                }
            },
            BinaryOperator::ShiftLeft => {
                match return_type {
                    ValType::I64 => BinaryOp::I64Add,
                    ValType::F32 => BinaryOp::F32Add,
                    ValType::F64 => BinaryOp::F64Add,
                    _ => panic!("return type {} is not implemented yet", return_type)
                }
            },
            BinaryOperator::ShiftRight => {
                match return_type {
                    ValType::I64 => BinaryOp::I64Shl,
                    _ => panic!("return type {} is not implemented yet", return_type)
                }
            },
            BinaryOperator::And => {
                match return_type {
                    ValType::I64 => BinaryOp::I64And,
                    ValType::V128 => BinaryOp::V128And,
                    _ => panic!("return type {} is not implemented yet", return_type)
                }
            },
            BinaryOperator::Or => {
                match return_type {
                    ValType::I64 => BinaryOp::I64Or,
                    ValType::V128 => BinaryOp::V128Or,
                    _ => panic!("return type {} is not implemented yet", return_type)
                }
            },
            BinaryOperator::Xor => {
                match return_type {
                    ValType::I64 => BinaryOp::I64Xor,
                    ValType::V128 => BinaryOp::V128Xor,
                    _ => panic!("return type {} is not implemented yet", return_type)
                }
            },
            BinaryOperator::Equal => {
                match return_type {
                    ValType::I64 => BinaryOp::I64Eq,
                    ValType::F32 => BinaryOp::F32Eq,
                    ValType::F64 => BinaryOp::F64Eq,
                    _ => panic!("return type {} is not implemented yet", return_type)
                }
            },
            BinaryOperator::NotEqual => {
                match return_type {
                    ValType::I64 => BinaryOp::I64Ne,
                    ValType::F32 => BinaryOp::F32Ne,
                    ValType::F64 => BinaryOp::F64Ne,
                    _ => panic!("return type {} is not implemented yet", return_type)
                }
            },
            BinaryOperator::GreaterThan => {
                match return_type {
                    ValType::I64 => BinaryOp::I64GtS,
                    ValType::F32 => BinaryOp::F32Gt,
                    ValType::F64 => BinaryOp::F64Gt,
                    _ => panic!("return type {} is not implemented yet", return_type)
                }
            },
            BinaryOperator::GreaterThanOrEqual => {
                match return_type {
                    ValType::I64 => BinaryOp::I64GeS,
                    ValType::F32 => BinaryOp::F32Ge,
                    ValType::F64 => BinaryOp::F64Ge,
                    _ => panic!("return type {} is not implemented yet", return_type)
                }
            },
            BinaryOperator::LessThan => {
                match return_type {
                    ValType::I64 => BinaryOp::I64LtS,
                    ValType::F32 => BinaryOp::F32Lt,
                    ValType::F64 => BinaryOp::F64Lt,
                    _ => panic!("return type {} is not implemented yet", return_type)
                }
            },
            BinaryOperator::LessThanOrEqual => {
                match return_type {
                    ValType::I64 => BinaryOp::I64LeS,
                    ValType::F32 => BinaryOp::F32Le,
                    ValType::F64 => BinaryOp::F64Le,
                    _ => panic!("return type {} is not implemented yet", return_type)
                }
            },
            _ => panic!("binary operator {} is not implemented yet", binops)
        }
    }
}