use std::rc::Rc;
use crate::core::{Visitor, Visits};
use crate::semantic::{Function, Parameter, Variable};
use crate::semantic::jasm::{Jasm, JasmStatementVisitor, JasmExpressionVisitor, JasmStatement, JasmExpression, JasmType, Struct, Block};
use super::JasmRenderVisitor;



impl Visitor<&Block, String> for JasmRenderVisitor {
    fn visit(&mut self, block:&Block) -> String {
        self.increase_indent();
        let value:String = self.visit(&block.0);
        self.decrease_indent();
        format!("{{\n{}\n{}}}", value, self.get_indent())
    }
}

impl Visitor<&Vec<JasmStatement>, String> for JasmRenderVisitor {
    fn visit(&mut self, statements:&Vec<JasmStatement>) -> String {
        let values = self.visits(statements);
        self.join_new_line(values)
    }
}


impl JasmStatementVisitor<String> for JasmRenderVisitor {
    fn visit_empty(&mut self) -> String {
        format!(";")
    }

    fn visit_declaration(&mut self, variable:&Variable<Jasm>) -> String{
        let Variable {id, name, typ} = variable;
        format!("let {}:{};", name, self.visit(typ))
    }

    fn visit_assign(&mut self, object:&JasmExpression, expression:&JasmExpression) -> String {
        format!("{} = {};", self.visit(object), self.visit(expression))
    }

    fn visit_if(&mut self, thens:&Vec<(JasmExpression, Block)>, els:&Block) -> String {
        let mut blocks = thens
            .iter()
            .map(|(condition, then)| {
                let condition = self.visit(condition);
                let then = self.visit(then);
                format!("if {} {}", condition, then)
            })
            .collect::<Vec<_>>();

        if !els.0.is_empty() {
            blocks.push(self.visit(els));
        }
        
        blocks.join(" else ")
    }

    fn visit_while(&mut self, condition:&JasmExpression, body:&Block) -> String {
        let condition = self.visit(condition);
        let body = self.visit(body);
        format!("while {} {}", condition, body)
    }

    fn visit_struct_definition(&mut self, definition:&Struct<Parameter<Jasm>>) -> String {
        todo!()
    }

    fn visit_function(&mut self, function:&Rc<Function<Jasm>>) -> String {
        self.visit(function)
    }

    fn visit_expression(&mut self, expression:&JasmExpression) -> String {
        format!("{};", self.visit(expression))
    }

    fn visit_return(&mut self, expression:&Option<JasmExpression>) -> String {
        match expression {
            None => format!("return;"),
            Some(expression) => format!("return {};", self.visit(expression))
        }
    }
}

