use crate::{building::BuildVisitor, core::Id, semantic::{
        jasm::{
            Block, Jasm, JasmExpression, JasmExpressionVisitor, JasmStatement,
            JasmStatementVisitor, JasmType, JasmValue, Struct,
        },
        Function, FunctionType, Name, Parameter, Variable, BinaryOperator, UnaryOperator
    }};
use crate::{
    core::{Visitor, Visits, VisitorWith},
    semantic::{jasm::JasmPrimitiveImplementation, Implementation, GetType },
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
        for x in arguments.into_iter().rev() {
            self.visit(x);
        }
        
        
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
                Primitive(I64, Binary(Subtract)) => func_body.binop(BinaryOp::I64Sub),
                Primitive(I64, Binary(Multiply)) => func_body.binop(BinaryOp::I64Mul),
                Primitive(I64, Binary(Divide)) => func_body.binop(BinaryOp::I64DivS),
                Primitive(I64, Binary(Modulo)) => func_body.binop(BinaryOp::I64RemS),
                Primitive(I64, Binary(ShiftRight)) => func_body.binop(BinaryOp::I64ShrS),
                Primitive(I64, Binary(Equal)) => func_body.binop(BinaryOp::I64Eq),
                Primitive(I64, Binary(NotEqual)) => func_body.binop(BinaryOp::I64Ne),
                Primitive(I64, Binary(GreaterThan)) => func_body.binop(BinaryOp::I64GtS),
                Primitive(I64, Binary(GreaterThanOrEqual)) => func_body.binop(BinaryOp::I64GeS),
                Primitive(I64, Binary(LessThan)) => func_body.binop(BinaryOp::I64LtS),
                Primitive(I64, Binary(LessThanOrEqual)) => func_body.binop(BinaryOp::I64LeS),
                
                Primitive(U64, Binary(Add)) => func_body.binop(BinaryOp::I64Add),
                Primitive(U64, Binary(Subtract)) => func_body.binop(BinaryOp::I64Sub),
                Primitive(U64, Binary(Multiply)) => func_body.binop(BinaryOp::I64Mul),
                Primitive(U64, Binary(Divide)) => func_body.binop(BinaryOp::I64DivU),
                Primitive(U64, Binary(Modulo)) => func_body.binop(BinaryOp::I64RemU),
                Primitive(U64, Binary(ShiftRight)) => func_body.binop(BinaryOp::I64ShrU),
                Primitive(U64, Binary(GreaterThan)) => func_body.binop(BinaryOp::I64GtU),
                Primitive(U64, Binary(GreaterThanOrEqual)) => func_body.binop(BinaryOp::I64GeU),
                Primitive(U64, Binary(Equal)) => func_body.binop(BinaryOp::I64Eq),
                Primitive(U64, Binary(NotEqual)) => func_body.binop(BinaryOp::I64Ne),
                Primitive(U64, Binary(LessThan)) => func_body.binop(BinaryOp::I64LtU),
                Primitive(U64, Binary(LessThanOrEqual)) => func_body.binop(BinaryOp::I64LeU),
                
                Primitive(F64, Binary(Add)) => func_body.binop(BinaryOp::F64Add),
                Primitive(F64, Binary(Subtract)) => func_body.binop(BinaryOp::F64Sub),
                Primitive(F64, Binary(Multiply)) => func_body.binop(BinaryOp::F64Mul),
                Primitive(F64, Binary(Divide)) => func_body.binop(BinaryOp::F64Div),
                Primitive(F64, Binary(Equal)) => func_body.binop(BinaryOp::F64Eq),
                Primitive(F64, Binary(NotEqual)) => func_body.binop(BinaryOp::F64Ne),
                Primitive(F64, Binary(GreaterThan)) => func_body.binop(BinaryOp::F64Gt),
                Primitive(F64, Binary(GreaterThanOrEqual)) => func_body.binop(BinaryOp::F64Ge),
                Primitive(F64, Binary(LessThan)) => func_body.binop(BinaryOp::F64Lt),
                Primitive(F64, Binary(LessThanOrEqual)) => func_body.binop(BinaryOp::F64Le),
                
                Primitive(JasmType::Bool, Binary(GreaterThan)) => {
                    match arguments[0].get_type() {
                        I64 => func_body.binop(BinaryOp::I64GtS),
                        U64 => func_body.binop(BinaryOp::I64GtU),
                        F64 => func_body.binop(BinaryOp::F64Gt),
                        _ => panic!(format!("invalid implementation {:?}", &function.implementation)),
                    }
                },
                
                Primitive(JasmType::Bool, Binary(GreaterThanOrEqual)) => {
                    match arguments[0].get_type() {
                        I64 => func_body.binop(BinaryOp::I64GeS),
                        U64 => func_body.binop(BinaryOp::I64GeU),
                        F64 => func_body.binop(BinaryOp::F64Ge),
                        _ => panic!(format!("invalid implementation {:?}", &function.implementation)),
                    }
                },
                
                Primitive(JasmType::Bool, Binary(LessThan)) => {
                    match arguments[0].get_type() {
                        I64 => func_body.binop(BinaryOp::I64LtS),
                        U64 => func_body.binop(BinaryOp::I64LtU),
                        F64 => func_body.binop(BinaryOp::F64Lt),
                        _ => panic!(format!("invalid implementation {:?}", &function.implementation)),
                    }
                },
                
                Primitive(JasmType::Bool, Binary(LessThanOrEqual)) => {
                    match arguments[0].get_type() {
                        I64 => func_body.binop(BinaryOp::I64LeS),
                        U64 => func_body.binop(BinaryOp::I64LeU),
                        F64 => func_body.binop(BinaryOp::F64Le),
                        _ => panic!(format!("invalid implementation {:?}", &function.implementation)),
                    }
                },
                
                Primitive(_, Binary(And)) => func_body.binop(BinaryOp::I64And),
                Primitive(_, Binary(Or)) => func_body.binop(BinaryOp::I64Or),
                Primitive(_, Binary(Xor)) => func_body.binop(BinaryOp::I64Xor),
                Primitive(_, Binary(ShiftLeft)) => func_body.binop(BinaryOp::I64Shl),

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