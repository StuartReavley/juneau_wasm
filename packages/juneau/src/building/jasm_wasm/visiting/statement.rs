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
    core::{Visitor, VisitorWith, Visits},
    semantic::{jasm::JasmPrimitiveImplementation, Implementation},
};
use crate::building::jasm_wasm::visitor::*;
use crate::building::BuildVisitor;
use std::{any::Any, rc::Rc, str::FromStr};
use walrus::ir::*;
use walrus::{FunctionBuilder, InstrSeqBuilder, LocalId, Module, ModuleConfig, ValType};

impl<'b> JasmStatementVisitor<()> for WasmImplementationVisitor<'b> {
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
        self.builder.local_set(local);
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
        // // we count the length of instruction sequence before adding condition.
        // let curr_seq_len = self.function_builder.get_mut().func_body().instrs().len();
 

        // self.visit(condition);
        
        
        // // TODO: Find the easiest way to split the wasm sequences.
        // // 
        // // 1. We get all the sequence after visiting conditions
        // // 2. We split the sequence based on the initial sequence in `curr_seq_len`
        // let mut curr_seq = self.function_builder.get_mut().func_body().instrs_mut().to_owned();
        // let (_, new_seq) = curr_seq.split_at_mut(curr_seq_len);
        // let new_seq_len = new_seq.len();

        // // 3. We visit the while's body
        // // 4. We obtain all sequence which includes all the sequences from the start
        // self.visits(&body.0);
        // let mut curr_body_seq = self.function_builder.get_mut().func_body().instrs_mut().to_owned();

        // // 5. We split the sequence again similar to step 2 above, but we got the rest of the sequence
        // // 6. We then split the rest of the sequence based on the length of the condition's sequence
        // let (_, all_seq) = curr_body_seq.split_at_mut(curr_seq_len);
        // let (cond_seq, body_seq) = all_seq.split_at_mut(new_seq_len);
        // println!("{:?}", condition);
        

        self.builder.block(None, |  done | {
            let done_id = done.id().to_owned();
            
            done.loop_(None, | loop_ | {
                let loop_id = loop_.id();
                &self.visit(condition);
                // for (seq, loc) in cond_seq.to_owned().iter() {
                //     loop_.instr(seq.to_owned());
                // };
                loop_.br_if(done_id);
                self.visits(&body.0);
                // for (seq, loc) in body_seq.to_owned().iter() {
                //     loop_.instr(seq.to_owned());
                // };
                loop_.br(loop_id);
            });
            
        });

        // println!("{:?}\n", body);
        println!("While statement\n");
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
        self.builder.return_();
    }
}