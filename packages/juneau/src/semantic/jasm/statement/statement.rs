use std::rc::Rc;
use crate::{core::Id, semantic::{Expression, Variable, function}};
use crate::semantic::{Function, GetType, Parameter};
use crate::semantic::jasm::{Jasm, JasmExpression, JasmType, Member, Struct};


#[derive(Debug, Clone, PartialEq)]
pub enum JasmStatement {
    Empty,
    Declaration(Variable<Jasm>),
    Assign(JasmExpression, JasmExpression),
    If(Vec<(JasmExpression, Block)>, Block),
    While(JasmExpression, Block),
    StructDefinition(Struct<Parameter<Jasm>>),
    Function(Rc<Function<Jasm>>),
    Expression(JasmExpression),
    Return(Option<JasmExpression>)
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Block(pub Vec<JasmStatement>);

impl Block {
    pub fn new() -> Self {
        Block(Vec::new())
    }
    pub fn push(&mut self, statement:JasmStatement) {
        self.0.push(statement)
    }
}