use crate::{building::BuildVisitor, core::Id, semantic::{
        jasm::{
            Block, Jasm, JasmExpression, JasmExpressionVisitor, JasmStatement,
            JasmStatementVisitor, JasmType, JasmValue, Struct,
        },
        Function, FunctionType, Name, Parameter, Variable, BinaryOperator
    }};
use crate::{
    core::{Visitor, Visits, VisitorWith},
    semantic::{jasm::JasmPrimitiveImplementation, Implementation},
};
use crate::building::jasm_wasm::visitor::*;
use std::{any::Any, rc::Rc, str::FromStr};
use walrus::ir::*;
use walrus::{FunctionBuilder, InstrSeqBuilder, LocalId, Module, ModuleConfig, ValType};

impl JasmExpressionVisitor<()> for WasmBuilderVisitor {
    fn visit_constant(&mut self, value: &JasmValue) -> () {
        match value {
            JasmValue::Null => {
                self.function_builder.get_mut().func_body().i32_const(0);
            },
            JasmValue::Bool(val) => {
                self.function_builder.get_mut().func_body().i32_const(0);
            },
            JasmValue::U8(val) => {
                self.function_builder.get_mut().func_body().i32_const(*val as i32);
            },
            JasmValue::I64(val) => {
                self.function_builder.get_mut().func_body().i64_const(*val);
            },
            JasmValue::U64(val) => {
                self.function_builder.get_mut().func_body().i64_const(*val as i64);
            },
            JasmValue::F64(val) => {
                self.function_builder.get_mut().func_body().f64_const(*val);
            },
            JasmValue::String(val) => {
                self.function_builder.get_mut().func_body().i32_const(0);
            }
        }
    }

    fn visit_invocation(
        &mut self,
        id: Id,
        name: &Name,
        arguments: &Vec<JasmExpression>,
        return_typ: &JasmType,
    ) -> () {
        let return_type = ValType::from(return_typ);
        let operator = String::from(name);

        self.visits(arguments);
        
        if let Ok(binops) = FromStr::from_str(&operator) {
            let ops = match binops {
                BinaryOperator::Add => {
                    match return_type {
                        ValType::I64 => BinaryOp::I64Add,
                        // TODO: Complete the remaining ValType
                        _ => BinaryOp::I64Add
                    }
                },
                _ => BinaryOp::I64Sub
            };
            self.function_builder.get_mut().func_body().binop(ops);
        } else {
            let func_id = self.module.funcs.by_name(&operator).unwrap();
            self.function_builder.get_mut().func_body().call(func_id);
        }
    }

    fn visit_variable(&mut self, variable: &Variable<Jasm>) -> () {
        // here, instead of subtracting 1001, we need to store the variable IDs, and then call 'get_variable'
        let Variable {id, name, ..} = variable;
        let local = self.get_variable(*id, name);
        self.function_builder.get_mut().func_body().local_get(local);
    }

    fn visit_cast(
        &mut self,
        expression: &JasmExpression,
        typ: &JasmType,
    ) -> () {
        todo!()
    }

    fn visit_struct_access(
        &mut self,
        object: &JasmExpression,
        id: Id,
        name: &Name,
        typ: &JasmType,
    ) -> () {
        todo!()
    }

    fn visit_array_access(
        &mut self,
        _object: &JasmExpression,
        _index: &JasmExpression,
    ) -> () {
        todo!()
    }

    fn visit_reference(&mut self, _expression: &JasmExpression) -> () {
        todo!()
    }

    fn visit_dereference(&mut self, _expression: &JasmExpression) -> () {
        todo!()
    }
}