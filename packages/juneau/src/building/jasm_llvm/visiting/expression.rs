use std::collections::HashMap;
use std::ffi::c_void;
use crate::{building::jasm_llvm::JasmModuleBuildVisitor, core::Id, semantic::{Parameter, jasm::{Jasm, JasmExpressionVisitor, JasmStatement, JasmStatementVisitor, JasmValue, Member}}};
use crate::core::{Visitor, Visits};
use crate::semantic::{Name, GetType, Variable, Implementation};
use crate::semantic::jasm::{Struct, UnaryOperator, JasmType, JasmExpression, JasmPrimitiveImplementation};
use crate::target::llvm::{FunctionPassManager, StructType, GetContext, Predicate, BasicBlock, Builder, Context, ExecutionEngine, Function, Module, Value, Type};
use crate::building::BuildVisitor;
use super::parameter;


impl<'l> JasmExpressionVisitor<&'l Value> for JasmModuleBuildVisitor<'l> {
    fn visit_constant(&mut self, value:&JasmValue) -> &'l Value {
        self.visit(value)
    }

    fn visit_invocation(&mut self, id:Id, name:&Name, arguments:&Vec<JasmExpression>, return_typ:&JasmType) -> &'l Value {
        let builder = self.get_builder();
        let arguments = self.visits(arguments);
        let function = self.get_function(id, name);
        let built_function = self.visit(function.to_owned());
        use Implementation::*;
        use JasmPrimitiveImplementation::*;
        use crate::semantic::jasm::NumberType::*;
        use crate::semantic::BinaryOperator::*;
        use crate::semantic::UnaryOperator::*;
        From::from(match built_function {
            Some(function) => builder.build_call(function, &arguments),
            None => match function.implementation {
                Primitive(_, Unary(UnsignedInteger, Not)) => builder.build_not(arguments[0]),
                Primitive(_, Unary(SignedInteger, Not)) => builder.build_not(arguments[0]),
                Primitive(_, Unary(UnsignedInteger, Negate)) => builder.build_integer_negate(arguments[0]),
                Primitive(_, Unary(SignedInteger, Negate)) => builder.build_integer_negate(arguments[0]),
                Primitive(_, Unary(Float, Negate)) => builder.build_float_negate(arguments[0]),

                Primitive(_, Binary(UnsignedInteger  , Add)) => builder.build_integer_add(arguments[0], arguments[1]),
                Primitive(_, Binary(SignedInteger  , Add)) => builder.build_integer_add(arguments[0], arguments[1]),
                Primitive(_, Binary(Float  , Add)) => builder.build_float_add(arguments[0], arguments[1]),
                Primitive(_, Binary(UnsignedInteger  , Subtract)) => builder.build_integer_subtract(arguments[0], arguments[1]),
                Primitive(_, Binary(SignedInteger  , Subtract)) => builder.build_integer_subtract(arguments[0], arguments[1]),
                Primitive(_, Binary(Float  , Subtract)) => builder.build_float_subtract(arguments[0], arguments[1]),
                Primitive(_, Binary(UnsignedInteger  , Multiply)) => builder.build_integer_multiply(arguments[0], arguments[1]),
                Primitive(_, Binary(SignedInteger  , Multiply)) => builder.build_integer_multiply(arguments[0], arguments[1]),
                Primitive(_, Binary(Float  , Multiply)) => builder.build_float_multiply(arguments[0], arguments[1]),
                Primitive(_, Binary(UnsignedInteger  , Divide)) => builder.build_unsigned_integer_divide(arguments[0], arguments[1]),
                Primitive(_, Binary(SignedInteger  , Divide)) => builder.build_signed_integer_divide(arguments[0], arguments[1]),
                Primitive(_, Binary(Float  , Divide)) => builder.build_float_divide(arguments[0], arguments[1]),
                
                Primitive(_, Binary(UnsignedInteger, ShiftLeft)) => builder.build_shift_left(arguments[0], arguments[1]),
                Primitive(_, Binary(SignedInteger, ShiftLeft)) => builder.build_shift_left(arguments[0], arguments[1]),
                Primitive(_, Binary(UnsignedInteger, ShiftRight)) => builder.build_arithmetic_shift_right(arguments[0], arguments[1]),
                Primitive(_, Binary(SignedInteger, ShiftRight)) => builder.build_arithmetic_shift_right(arguments[0], arguments[1]),

                Primitive(_, Binary(Bool, And)) => builder.build_and(arguments[0], arguments[1]),
                Primitive(_, Binary(UnsignedInteger, And)) => builder.build_and(arguments[0], arguments[1]),
                Primitive(_, Binary(SignedInteger, And)) => builder.build_and(arguments[0], arguments[1]),
                Primitive(_, Binary(Bool, Or)) => builder.build_or(arguments[0], arguments[1]),
                Primitive(_, Binary(UnsignedInteger, Or)) => builder.build_or(arguments[0], arguments[1]),
                Primitive(_, Binary(SignedInteger, Or)) => builder.build_or(arguments[0], arguments[1]),
                Primitive(_, Binary(Bool, Xor)) => builder.build_xor(arguments[0], arguments[1]),
                Primitive(_, Binary(UnsignedInteger, Xor)) => builder.build_xor(arguments[0], arguments[1]),
                Primitive(_, Binary(SignedInteger, Xor)) => builder.build_xor(arguments[0], arguments[1]),

                Primitive(_, Binary(UnsignedInteger, predicate)) => builder.build_integer_compare(arguments[0], arguments[1], predicate.into()),
                Primitive(_, Binary(SignedInteger, predicate)) => builder.build_integer_compare(arguments[0], arguments[1], predicate.into()),
                Primitive(_, Binary(Float  , predicate)) => builder.build_float_compare(arguments[0], arguments[1], predicate.into()),
                _ => panic!(format!("invalid implementation {:?}", &function.implementation))
            }
        })
    }

    fn visit_variable(&mut self, variable:&Variable<Jasm>) -> &'l Value {
        let Variable {id, name, ..} = variable;
        let address = self.get_variable(*id, name);
        self.get_builder().build_load(address)
    }

    fn visit_cast(&mut self, expression:&JasmExpression, typ:&JasmType) -> &'l Value {
        let source = expression.get_type();
        let target = typ;
        let typ = self.visit(typ);
        let expression = self.visit(expression);
        let builder = self.get_builder();
        use JasmType::*;
        From::from(match (&source, target) {
            (U8, U8) => expression,
            // (U8, I32) => builder.build_zext(expression, typ),
            // (U8, U32) => builder.build_zext(expression, typ),
            (U8, I64) => builder.build_zext(expression, typ),
            (U8, U64) => builder.build_zext(expression, typ),

            (I64, U8) => builder.build_trunc(expression, typ),
            // (I64, I32) => builder.build_trunc(expression, typ),
            // (I64, U32) => builder.build_trunc(expression, typ),
            (I64, I64) => expression,
            (I64, U64) => builder.build_zext(expression, typ),

            (U64, U8) => builder.build_trunc(expression, typ),
            // (U64, I32) => builder.build_trunc(expression, typ),
            // (U64, U32) => builder.build_trunc(expression, typ),
            (U64, I64) => builder.build_trunc(expression, typ),
            (U64, U64) => expression,

            (U8, F64) => builder.build_si_to_fp(expression, typ),
            // (I32, F64) => builder.build_si_to_fp(expression, typ),
            // (U32, F64) => builder.build_si_to_fp(expression, typ),
            (I64, F64) => builder.build_si_to_fp(expression, typ),
            (U64, F64) => builder.build_si_to_fp(expression, typ),

            (F64, U8) => builder.build_fp_to_si(expression, typ),
            // (F64, I32) => builder.build_fp_to_si(expression, typ),
            // (F64, U32) => builder.build_fp_to_si(expression, typ),
            (F64, I64) => builder.build_fp_to_si(expression, typ),
            (F64, U64) => builder.build_fp_to_si(expression, typ),

            (I64, Pointer(_)) => builder.build_int_to_ptr(expression, typ),
            (Pointer(_), I64) => builder.build_ptr_to_int(expression, typ),
            _ => {panic!(format!("{:?} {:?}", &source, &target))}
        })
    }

    fn visit_struct_access(&mut self, object:&JasmExpression, id:Id, name:&Name, typ:&JasmType) -> &'l Value {
        todo!()
    }

    fn visit_array_access(&mut self, object:&JasmExpression, index:&JasmExpression) -> &'l Value {
        todo!()
    }

    fn visit_reference(&mut self, expression:&JasmExpression) -> &'l Value {
        use JasmExpression::*;
        match expression {
            Var(Variable {id, name, ..}) => self.get_variable(*id, name),
            Dereference(expression) => self.visit(expression.as_ref()),
            _ => panic!()
        }
    }

    fn visit_dereference(&mut self, expression:&JasmExpression) -> &'l Value {
        let address = self.visit(expression);
        self.get_builder().build_load(address)
    }
}
