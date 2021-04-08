use std::rc::Rc;
use crate::semantic::{Function, Parameter, jasm::JasmValue};
use crate::core::Visitor;
use crate::semantic::Variable;
use crate::semantic::jasm::{Jasm, JasmStatement, JasmExpression, Struct, Block};


pub trait JasmStatementVisitor<T> {
    fn visit_empty(&mut self) -> T;
    fn visit_declaration(&mut self, variable:&Variable<Jasm>) -> T;
    fn visit_assign(&mut self, object:&JasmExpression, expression:&JasmExpression) -> T;
    fn visit_if(&mut self, thens:&Vec<(JasmExpression, Block)>, els:&Block) -> T;
    fn visit_while(&mut self, condition:&JasmExpression, body:&Block) -> T;
    fn visit_struct_definition(&mut self, definition:&Struct<Parameter<Jasm>>) -> T;
    fn visit_function(&mut self, function:&Rc<Function<Jasm>>) -> T;
    fn visit_expression(&mut self, expression:&JasmExpression) -> T;
    fn visit_return(&mut self, expression:&Option<JasmExpression>) -> T;
}



impl<T, V:JasmStatementVisitor<T>> Visitor<&JasmStatement, T> for V {
    fn visit(&mut self, statement:&JasmStatement) -> T {
        use JasmStatement::*;
        match statement {
            Empty => self.visit_empty(),
            Declaration(variable) => self.visit_declaration(variable),
            Assign(object, expression) => self.visit_assign(object, expression),
            If(thens, els) => self.visit_if(thens, els),
            While(condition, body) => self.visit_while(condition, body),
            StructDefinition(definition) => self.visit_struct_definition(definition),
            Function(function) => self.visit_function(function),
            Expression(expression) => self.visit_expression(expression),
            Return(expression) => self.visit_return(expression)
        }
    }
}
