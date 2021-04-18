use crate::{building::BuildVisitor, core::Id, semantic::{
        jasm::{
            Block, Jasm, JasmExpression, JasmExpressionVisitor, JasmStatement,
            JasmStatementVisitor, JasmType, JasmValue, Struct,
        },
        Function, FunctionType, Name, Parameter, Variable, BinaryOperator, UnaryOperator
    }};
use crate::{
    core::{Visitor, Visits, VisitorWith},
    semantic::{jasm::JasmPrimitiveImplementation, Implementation},
};
use crate::building::jasm_wasm::visitor::*;
use std::{any::Any, rc::Rc, str::FromStr};
use walrus::ir::*;
use walrus::{FunctionBuilder, InstrSeqBuilder, LocalId, Module, ModuleConfig, ValType};
use crate::semantic::jasm::NumberType;


impl JasmExpressionVisitor<()> for WasmBuilderVisitor {
    fn visit_constant(&mut self, value: &JasmValue) -> () {
        self.visit(value)
    }

    fn visit_invocation(
        &mut self,
        id: Id,
        name: &Name,
        arguments: &Vec<JasmExpression>,
        return_typ: &JasmType,
    ) -> () {

        self.visits(arguments);
        let function = self.get_function(id, name);
        let built_function = self.visit(&function);

        let mut func_body = self.function_builder.get_mut().func_body();

        use Implementation::*;
        use JasmPrimitiveImplementation::*;
        use NumberType::*;
        use BinaryOperator::*;
        use UnaryOperator::*;
        use JasmType::*;
        match built_function {
            Some(function) => func_body.call(function),
            None => match function.implementation {

                Primitive(I64, Binary(Add)) => func_body.binop(BinaryOp::I64Add),

                // TODO: Complete the remaining ValType

                _ => panic!(format!("invalid implementation {:?}", &function.implementation))
            }
        };
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