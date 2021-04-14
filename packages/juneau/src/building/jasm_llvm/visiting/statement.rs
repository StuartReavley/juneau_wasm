use std::rc::Rc;
use std::collections::HashMap;
use std::ffi::c_void;
use crate::{core::Id, semantic::{Function, Parameter, jasm::{Block, Jasm, JasmExpressionVisitor, JasmStatement, JasmStatementVisitor, Member}}};
use crate::core::Visitor;
use crate::semantic::{GetType, Variable, Implementation};
use crate::semantic::jasm::{Struct, UnaryOperator, JasmType, JasmExpression, JasmPrimitiveImplementation};
use crate::target::llvm::{FunctionPassManager, StructType, GetContext, Predicate, BasicBlock, Builder, Context, ExecutionEngine, Module, Value, Type};
use crate::building::{BuildVisitor};
use crate::building::jasm_llvm::JasmModuleBuildVisitor;
use super::parameter;


impl<'l> JasmStatementVisitor<()> for JasmModuleBuildVisitor<'l> {
    fn visit_empty(&mut self) {
    }

    fn visit_declaration(&mut self, variable:&Variable<Jasm>) -> () {
        let Variable {id, name, typ} = variable;
        let llvm_typ = self.visit(typ);
        let address = self.get_builder().build_allocation(name.into(), llvm_typ);
        self.insert_variable(*id, address);
    }

    fn visit_if(&mut self, thens:&Vec<(JasmExpression, Block)>, els:&Block) {
        let builder = self.get_builder();
        let block = builder.get_insert_block();
        let parent_function = block.get_parent().unwrap();
        let end_block = parent_function.append("if_end");

        for (condition, then) in thens {
            let then_block = parent_function.append("if_then");
            let els_block = parent_function.append("if_else");
            let condition = self.visit(condition);
            builder.build_cond_br(condition, then_block, els_block);

            then_block.move_after(builder.get_insert_block());
            builder.position_at_end(then_block);
            self.visit(then);
            builder.build_br(end_block);

            els_block.move_after(builder.get_insert_block());
            builder.position_at_end(els_block);
        }

        let els = self.visit(els);
        builder.build_br(end_block);

        end_block.move_after(builder.get_insert_block());
        builder.position_at_end(end_block);
    }

    fn visit_while(&mut self, condition:&JasmExpression, body:&Block) {
        let builder = self.get_builder();
        let block = builder.get_insert_block();
        let parent_function = block.get_parent().unwrap();
        let condition_block = parent_function.append("while_condition");
        let body_block = parent_function.append("while_body");
        let end_block = parent_function.append("while_end");

        builder.build_br(condition_block);
        builder.position_at_end(condition_block);
        let condition = self.visit(condition);
        builder.build_cond_br(condition, body_block, end_block);                            

        body_block.move_after(builder.get_insert_block());
        builder.position_at_end(body_block);
        self.visit(body);
        builder.build_br(condition_block);

        end_block.move_after(builder.get_insert_block());
        builder.position_at_end(end_block);
    }

    fn visit_struct_definition(&mut self, definition:&Struct<Parameter<Jasm>>) {
        let Struct {id, name, parameters} = definition;
        self.insert_struct(definition);
        let parameters = parameters.iter().map(|p|self.visit(&p.typ)).collect::<Vec<_>>();
        let struct_type = StructType::new_named(self.into(), name.into(), &parameters, false);
        self.insert_type(id, struct_type);
    }

    fn visit_function(&mut self, function:&Rc<Function<Jasm>>) {
        self.visit(function.to_owned());
    }

    fn visit_assign(&mut self, object:&JasmExpression, expression:&JasmExpression) {
        use JasmExpression::*;
        let address = match object {
            Var(Variable{id, name, ..}) => self.get_variable(*id, name),
            StructAccess(_, _, _, _) => generate_get_element_pointer(self, object),
            ArrayAccess(_, _) => generate_get_element_pointer(self, object),
            Dereference(expression) => self.visit(expression.as_ref()),
            object => panic!(format!("invalid assignment {:?}", object))
        };
        let expression = self.visit(expression);
        self.get_builder().build_store(expression, address);
    }

    fn visit_expression(&mut self, expression:&JasmExpression) {
        self.visit(expression);
    }

    fn visit_return(&mut self, expression:&Option<JasmExpression>) {
        let builder = self.get_builder();
        match expression {
            None => builder.build_ret_void(),
            Some(expression) => builder.build_ret(self.visit(expression))
        };
    }
}



fn generate_get_element_pointer<'l>(visitor:&mut JasmModuleBuildVisitor<'l>, expression:&JasmExpression) -> &'l Value {
    let (address, indices, typ) = get_pointer_indices(visitor, expression);
    visitor.get_builder().build_gep(address, &indices)
}

fn get_pointer_indices<'l>(visitor:&mut JasmModuleBuildVisitor<'l>, e:&JasmExpression) -> (&'l Value, Vec<&'l Value>, JasmType) {
    use JasmExpression::*;
    match e {
        Var(Variable{id, name, typ}) => {
            let address = visitor.get_variable(*id, name);
            (address, vec![], typ.to_owned())
        },
        StructAccess(object, parameter_id, parameter_name, typ) => {
            let (address, mut indices, _) = get_pointer_indices(visitor, object);
            let index = visitor.get_struct_indice(parameter_id, parameter_name);
            indices.push(visitor.visit(index));
            (address, indices, typ.to_owned())
        },
        ArrayAccess(object, index) => {
            let (address, mut indices, typ) = get_pointer_indices(visitor, object);
            let index = visitor.visit(index.as_ref());
            indices.push(index);
            match typ {
                JasmType::Array(typ) => (address, indices, (*typ).to_owned()),
                _ => panic!("trying to index into non-array type")
            }
        },
        _ => panic!("invalid assignment")
    }
}
