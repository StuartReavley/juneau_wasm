use std::rc::Rc;
use crate::core::Id;
use crate::core::Visitor;
use crate::semantic::{Name, Variable};
use crate::semantic::jasm::{Jasm, JasmExpression, JasmType, JasmValue};


pub trait JasmExpressionVisitor<E> {
    fn visit_constant(&mut self, value:&JasmValue) -> E;
    fn visit_invocation(&mut self, id:Id, name:&Name, arguments:&Vec<JasmExpression>, return_typ:&JasmType) -> E;
    fn visit_variable(&mut self, variable:&Variable<Jasm>) -> E;
    fn visit_cast(&mut self, expression:&JasmExpression, typ:&JasmType) -> E;
    fn visit_struct_access(&mut self, object:&JasmExpression, id:Id, name:&Name, typ:&JasmType) -> E;
    fn visit_array_access(&mut self, object:&JasmExpression, index:&JasmExpression) -> E;
    fn visit_reference(&mut self, expression:&JasmExpression) -> E;
    fn visit_dereference(&mut self, expression:&JasmExpression) -> E;
}

impl<T, V:JasmExpressionVisitor<T>> Visitor<&JasmExpression, T> for V {
    fn visit(&mut self, expression:&JasmExpression) -> T {
        use JasmExpression::*;
        match expression {
            Constant(value) => self.visit_constant(value),
            Invocation(id, name, arguments, return_typ) => self.visit_invocation(*id, name, arguments, return_typ),
            Var(variable) => self.visit_variable(variable),
            StructAccess(object, id, name, typ) => self.visit_struct_access(object, *id, name, typ),
            ArrayAccess(object, index) => self.visit_array_access(object, index),
            Cast(expression, typ) => self.visit_cast(expression, typ),
            Reference(expression) => self.visit_reference(expression),
            Dereference(expression) => self.visit_dereference(expression)
        }
    }
}
