use crate::{core::Visitor, semantic::{Implementation, jasm::JasmPrimitiveImplementation}};
use crate::{core::Id, semantic::{Function, Name, Parameter, Variable, jasm::{Block, Jasm, JasmExpression, JasmExpressionVisitor, JasmStatementVisitor, JasmType, JasmValue, Struct}}};
mod test;




pub fn compile_wasm(jasm:&Function<Jasm>) -> Vec<u8> {
    
    // this is the job entry point
    
    let mut visitor = WasmVisitor {buffer: vec![]};
    visitor.visit(jasm);

    visitor.buffer
}




// SOME SUGGESTIONS TO GET YOU STARTED - DELETE / CHANGE - WHATEVER YOU WANT!

struct WasmVisitor {
    buffer: Vec<u8>
}


impl Visitor<&Function<Jasm>, ()> for WasmVisitor {
    fn visit(&mut self, function:&Function<Jasm>) -> () {
        let Function {id, name, parameters, implementation} = function;

        match implementation {
            Implementation::Semantic((block, typ)) => {
                // EXAMPLE OF VISITING STATEMENTS
                for statement in &block.0 {
                    self.visit(statement);
                }
            }
            Implementation::Primitive(typ, primitive) => {
                match primitive {
                    JasmPrimitiveImplementation::External { is_pure, ptr } => panic!("not supported in wasm, only llvm"),
                    JasmPrimitiveImplementation::Unary(number_type, operator) => {
                        todo!()
                    }
                    JasmPrimitiveImplementation::Binary(number_type, operator) => {
                        todo!()
                    }
                }
            }
        }
    }
}


struct ExpressionResult {

}

impl JasmExpressionVisitor<ExpressionResult> for WasmVisitor {
    fn visit_constant(&mut self, value:&JasmValue) -> ExpressionResult {
        todo!()
    }

    fn visit_invocation(&mut self, id:Id, name:&Name, arguments:&Vec<JasmExpression>, return_typ:&JasmType) -> ExpressionResult {
        
        // EXAMPLE OF VISITING AN EXPRESSION
        for argument in arguments {
            self.visit(argument);
        }
        
        todo!()
    }

    fn visit_variable(&mut self, variable:&Variable<Jasm>) -> ExpressionResult {
        todo!()
    }

    fn visit_cast(&mut self, expression:&JasmExpression, typ:&JasmType) -> ExpressionResult {
        todo!()
    }

    fn visit_struct_access(&mut self, object:&JasmExpression, id:Id, name:&Name, typ:&JasmType) -> ExpressionResult {
        todo!()
    }

    fn visit_array_access(&mut self, object:&JasmExpression, index:&JasmExpression) -> ExpressionResult {
        todo!()
    }

    fn visit_reference(&mut self, expression:&JasmExpression) -> ExpressionResult {
        todo!()
    }

    fn visit_dereference(&mut self, expression:&JasmExpression) -> ExpressionResult {
        todo!()
    }
}


struct StatementResult {

}

impl JasmStatementVisitor<StatementResult> for WasmVisitor {
    fn visit_empty(&mut self) -> StatementResult {
        todo!()
    }

    fn visit_declaration(&mut self, variable:&Variable<Jasm>) -> StatementResult {
        todo!()
    }

    fn visit_assign(&mut self, object:&JasmExpression, expression:&JasmExpression) -> StatementResult {
        todo!()
    }

    fn visit_if(&mut self, thens:&Vec<(JasmExpression, Block)>, els:&Block) -> StatementResult {
        todo!()
    }

    fn visit_while(&mut self, condition:&JasmExpression, body:&Block) -> StatementResult {
        todo!()
    }

    fn visit_struct_definition(&mut self, definition:&Struct<Parameter<Jasm>>) -> StatementResult {
        panic!("not supported in wasm, only llvm")
    }

    fn visit_function(&mut self, function:&std::rc::Rc<Function<Jasm>>) -> StatementResult {
        todo!()
    }

    fn visit_expression(&mut self, expression:&JasmExpression) -> StatementResult {
        todo!()
    }

    fn visit_return(&mut self, expression:&Option<JasmExpression>) -> StatementResult {
        todo!()
    }
}