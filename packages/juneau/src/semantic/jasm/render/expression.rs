use crate::core::Id;
use crate::core::{Visitor, Visits};
use crate::semantic::{Name, Function, Parameter, Variable};
use crate::semantic::jasm::{Jasm, JasmStatementVisitor, JasmExpressionVisitor, JasmStatement, JasmExpression, JasmType, Struct, JasmValue};
use super::JasmRenderVisitor;


impl JasmExpressionVisitor<String> for JasmRenderVisitor {
    fn visit_constant(&mut self, value:&JasmValue) -> String {
        self.visit(value)
    }

    fn visit_invocation(&mut self, id:Id, name:&Name, arguments:&Vec<JasmExpression>, return_typ:&JasmType) -> String {
        format!("{}({})", name, self.visits(arguments).join(", "))
    }

    fn visit_variable(&mut self, variable:&Variable<Jasm>) -> String {
        format!("{}", variable.name)
    }

    fn visit_cast(&mut self, expression:&JasmExpression, typ:&JasmType) -> String {
        format!("{} as {}", self.visit(expression), self.visit(typ))
    }

    fn visit_struct_access(&mut self, object:&JasmExpression, id:Id, name:&Name, typ:&JasmType) -> String {
        format!("{}.{}", self.visit(object), name)
    }

    fn visit_array_access(&mut self, object:&JasmExpression, index:&JasmExpression) -> String {
        todo!()
    }

    // fn visit_block(&mut self, statements:&Vec<JasmStatement>, retrn:&JasmExpression) -> String {
    //     self.increase_indent();
    //     let mut values = statements.visit(self);
    //     values.push(retrn.visit(self));
    //     let values = join_new_line(self, values);
    //     self.decrease_indent();
    //     format!("{{\n{}}}", values)
    // }

    fn visit_reference(&mut self, expression:&JasmExpression) -> String {
        format!("&{}", self.visit(expression))
    }

    fn visit_dereference(&mut self, expression:&JasmExpression) -> String {
        format!("*({})", self.visit(expression))
    }
}
