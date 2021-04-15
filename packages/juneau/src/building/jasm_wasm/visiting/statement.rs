use crate::{
    core::Id,
    semantic::{
        jasm::{
            Block, Jasm, JasmExpression, JasmExpressionVisitor, JasmStatement,
            JasmStatementVisitor, JasmType, JasmValue, Struct,
        },
        Function, FunctionType, Name, Parameter, Variable, BinaryOperator
    },
};
use crate::{
    core::{Visitor, VisitorWith},
    semantic::{jasm::JasmPrimitiveImplementation, Implementation},
};
use crate::building::jasm_wasm::visitor::*;
use crate::building::BuildVisitor;
use std::{any::Any, rc::Rc, str::FromStr};
use walrus::ir::*;
use walrus::{FunctionBuilder, InstrSeqBuilder, LocalId, Module, ModuleConfig, ValType};

impl JasmStatementVisitor<()> for WasmBuilderVisitor {
    fn visit_empty(&mut self) -> () {
        println!("Empty statement\n");
        todo!()
    }

    fn visit_declaration(&mut self, variable: &Variable<Jasm>) -> () {
        self.visit(variable);
    }

    fn visit_assign(
        &mut self,
        object: &JasmExpression,
        expression: &JasmExpression,
    ) -> () {
        use JasmExpression::*;
        let local = match object {
            Var(Variable{id, name, ..}) => self.get_variable(*id, name),
            object => panic!(format!("invalid assignment {:?}", object))
        };
        self.visit(expression);
        self.function_builder.get_mut().func_body().local_set(local);
    }

    fn visit_if(
        &mut self,
        thens: &Vec<(JasmExpression, Block)>,
        els: &Block,
    ) -> () {
        println!("If statement\n");
        todo!()
    }

    fn visit_while(&mut self, condition: &JasmExpression, body: &Block) -> () {
        println!("While statement\n");
        todo!()
    }

    fn visit_struct_definition(
        &mut self,
        definition: &Struct<Parameter<Jasm>>,
    ) -> () {
        panic!("not supported in wasm, only llvm")
    }

    fn visit_function(
        &mut self,
        function: &Rc<Function<Jasm>>,
    ) -> () {
        self.visit(function);
    }

    fn visit_expression(&mut self, expression: &JasmExpression) -> () {
        println!("Expression statement\n");
        todo!()
    }

    fn visit_return(&mut self, expression: &Option<JasmExpression>) -> () {
        if let Some(jexpr) = expression {
            self.visit(jexpr);
        }

        // https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-instr-control
        self.function_builder.get_mut().func_body().return_();
    }
}