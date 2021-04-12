use crate::core::Id;
use crate::core::{Visitor, VisitorWith};
use crate::semantic::{
    jasm::{Block, Jasm, JasmExpression, JasmStatement, JasmType, JasmValue, Struct},
    Function, Parameter,
};
use crate::semantic::{Name, Variable};
use std::rc::Rc;
use walrus::{FunctionBuilder, InstrSeqBuilder, LocalId, Module, ModuleConfig, ValType};

pub trait JasmExpressionVisitorWith<E> {
    fn visit_constant(&mut self, module: &mut Module, value: &JasmValue) -> E;
    fn visit_invocation(
        &mut self,
        module: &mut Module,
        id: Id,
        name: &Name,
        arguments: &Vec<JasmExpression>,
        return_typ: &JasmType,
    ) -> E;
    fn visit_variable(&mut self, module: &mut Module, variable: &Variable<Jasm>) -> E;
    fn visit_cast(&mut self, module: &mut Module, expression: &JasmExpression, typ: &JasmType)
        -> E;
    fn visit_struct_access(
        &mut self,
        module: &mut Module,
        object: &JasmExpression,
        id: Id,
        name: &Name,
        typ: &JasmType,
    ) -> E;
    fn visit_array_access(
        &mut self,
        module: &mut Module,
        object: &JasmExpression,
        index: &JasmExpression,
    ) -> E;
    fn visit_reference(&mut self, module: &mut Module, expression: &JasmExpression) -> E;
    fn visit_dereference(&mut self, module: &mut Module, expression: &JasmExpression) -> E;
}

impl<T, V: JasmExpressionVisitorWith<T>> VisitorWith<&JasmExpression, &mut Module, T> for V {
    fn visit_with(&mut self, expression: &JasmExpression, module: &mut Module) -> T {
        use JasmExpression::*;
        match expression {
            Constant(value) => self.visit_constant(module, value),
            Invocation(id, name, arguments, return_typ) => {
                self.visit_invocation(module, *id, name, arguments, return_typ)
            }
            Var(variable) => self.visit_variable(module, variable),
            StructAccess(object, id, name, typ) => {
                self.visit_struct_access(module, object, *id, name, typ)
            }
            ArrayAccess(object, index) => self.visit_array_access(module, object, index),
            Cast(expression, typ) => self.visit_cast(module, expression, typ),
            Reference(expression) => self.visit_reference(module, expression),
            Dereference(expression) => self.visit_dereference(module, expression),
        }
    }
}

pub trait JasmStatementVisitorWith<T> {
    fn visit_empty(&mut self, module: &mut Module) -> T;
    fn visit_declaration(&mut self, module: &mut Module, variable: &Variable<Jasm>) -> T;
    fn visit_assign(
        &mut self,
        module: &mut Module,
        object: &JasmExpression,
        expression: &JasmExpression,
    ) -> T;
    fn visit_if(
        &mut self,
        module: &mut Module,
        thens: &Vec<(JasmExpression, Block)>,
        els: &Block,
    ) -> T;
    fn visit_while(&mut self, module: &mut Module, condition: &JasmExpression, body: &Block) -> T;
    fn visit_struct_definition(
        &mut self,
        module: &mut Module,
        definition: &Struct<Parameter<Jasm>>,
    ) -> T;
    fn visit_function(&mut self, module: &mut Module, function: &Rc<Function<Jasm>>) -> T;
    fn visit_expression(&mut self, module: &mut Module, expression: &JasmExpression) -> T;
    fn visit_return(&mut self, module: &mut Module, expression: &Option<JasmExpression>) -> T;
}

impl<T, V: JasmStatementVisitorWith<T>> VisitorWith<&JasmStatement, &mut Module, T> for V {
    fn visit_with(&mut self, statement: &JasmStatement, module: &mut Module) -> T {
        use JasmStatement::*;
        match statement {
            Empty => self.visit_empty(module),
            Declaration(variable) => self.visit_declaration(module, variable),
            Assign(object, expression) => self.visit_assign(module, object, expression),
            If(thens, els) => self.visit_if(module, thens, els),
            While(condition, body) => self.visit_while(module, condition, body),
            StructDefinition(definition) => self.visit_struct_definition(module, definition),
            Function(function) => self.visit_function(module, function),
            Expression(expression) => self.visit_expression(module, expression),
            Return(expression) => self.visit_return(module, expression),
        }
    }
}
