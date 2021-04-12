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
use std::{any::Any, rc::Rc, str::FromStr};
use walrus::ir::*;
use walrus::{FunctionBuilder, InstrSeqBuilder, LocalId, Module, ModuleConfig, ValType};
mod test;
mod typ;
mod visitor;
use visitor::{JasmExpressionVisitorWith, JasmStatementVisitorWith};

pub fn compile_wasm(jasm: &Function<Jasm>) -> Vec<u8> {
    // Construct a new Walrus module.
    let config = ModuleConfig::new();
    let mut module = Module::with_config(config);
    let FunctionType { parameters, retrn } = jasm.get_type();

    // Convert from JasmType into ValType
    let params: Vec<ValType> = parameters.iter().map(|x| ValType::from(x)).collect();
    let results = ValType::from(&*retrn);

    // Build instances of Jasm local function
    let mut func_jasm = FunctionBuilder::new(&mut module.types, &params, &[results]);
    
    let func_name = String::from(&jasm.name);
    func_jasm.name(func_name.clone());
    
    let param: Vec<LocalId> = params
    .iter()
    .map(|x| module.locals.add(*x))
    .collect();
    func_jasm.visit_with(jasm, &mut module);

    let final_func = func_jasm.finish(param, &mut module.funcs);

    // Export the final function.
    module.exports.add(&func_name, final_func);
    
    // emit wasm into buffer
    module.emit_wasm()
}

pub fn compile_wasm_to_file(jasm: &Function<Jasm>) -> anyhow::Result<()> {
        // Construct a new Walrus module.
        let config = ModuleConfig::new();
        let mut module = Module::with_config(config);
        let FunctionType { parameters, retrn } = jasm.get_type();
    
        // Convert from JasmType into ValType
        let params: Vec<ValType> = parameters.iter().map(|x| ValType::from(x)).collect();
        let results = ValType::from(&*retrn);
    
        // Build instances of Jasm local function
        let mut func_jasm = FunctionBuilder::new(&mut module.types, &params, &[results]);
        let func_name = String::from(&jasm.name);
        func_jasm.name(func_name.clone());
        
        let param: Vec<LocalId> = params
        .iter()
        .map(|x| module.locals.add(*x))
        .collect();
        func_jasm.visit_with(jasm, &mut module);

        let final_func = func_jasm.finish(param, &mut module.funcs);
    
        // Export the final function.
        module.exports.add(&func_name, final_func);
        
        // emit wasm into buffer
        module.emit_wasm_file("target/out.wasm")
}

// SOME SUGGESTIONS TO GET YOU STARTED - DELETE / CHANGE - WHATEVER YOU WANT!

struct WasmVisitor {
    buffer: Vec<u8>,
}

impl VisitorWith<&Function<Jasm>, &mut Module, ()> for FunctionBuilder {
    fn visit_with(&mut self, function: &Function<Jasm>, module: &mut Module) -> () {
        let Function {
            id,
            name,
            parameters,
            implementation,
        } = function;

        match implementation {
            Implementation::Semantic((block, typ)) => {
                // EXAMPLE OF VISITING STATEMENTS
                for statement in &block.0 {
                    self.visit_with(statement, module);
                }
            }
            Implementation::Primitive(typ, primitive) => match primitive {
                JasmPrimitiveImplementation::External { is_pure, ptr } => {
                    panic!("not supported in wasm, only llvm")
                }
                JasmPrimitiveImplementation::Unary(number_type, operator) => {
                    println!(
                        "Unary\nnumber_type: {:#?}\ntype: {:?}\noperator: {:?}\n",
                        number_type, typ, operator
                    );
                    todo!()
                }
                JasmPrimitiveImplementation::Binary(number_type, operator) => {
                    println!(
                        "Unary\nnumber_type: {:#?}\ntype: {:?}\noperator: {:?}\n",
                        number_type, typ, operator
                    );
                    todo!()
                }
            },
        }
    }
}

impl JasmExpressionVisitorWith<()> for FunctionBuilder {
    fn visit_constant(&mut self, module: &mut Module, value: &JasmValue) -> () {
        todo!()
    }

    fn visit_invocation(
        &mut self,
        module: &mut Module,
        id: Id,
        name: &Name,
        arguments: &Vec<JasmExpression>,
        return_typ: &JasmType,
    ) -> () {
        let return_type = ValType::from(return_typ);
        for argument in arguments {
            self.visit_with(argument, module);
        }
        let operator = String::from(name);
        let ops = match FromStr::from_str(&operator) {
            Ok(BinaryOperator::Add) => {
                match return_type {
                    ValType::I64 => BinaryOp::I64Add,
                    // TODO: Complete the remaining ValType
                    _ => BinaryOp::I64Sub
                }
            },
            _ => BinaryOp::I64Sub
        };
        self.func_body().binop(ops);
    }

    fn visit_variable(&mut self, module: &mut Module, variable: &Variable<Jasm>) -> () {
        let index = (variable.id.value - 1001) as usize;
        let locals: Vec<&Local> = module.locals.iter().collect();
        self.func_body().local_get(locals[index].id());
    }

    fn visit_cast(
        &mut self,
        module: &mut Module,
        expression: &JasmExpression,
        typ: &JasmType,
    ) -> () {
        todo!()
    }

    fn visit_struct_access(
        &mut self,
        module: &mut Module,
        object: &JasmExpression,
        id: Id,
        name: &Name,
        typ: &JasmType,
    ) -> () {
        todo!()
    }

    fn visit_array_access(
        &mut self,
        module: &mut Module,
        object: &JasmExpression,
        index: &JasmExpression,
    ) -> () {
        todo!()
    }

    fn visit_reference(&mut self, module: &mut Module, expression: &JasmExpression) -> () {
        todo!()
    }

    fn visit_dereference(&mut self, module: &mut Module, expression: &JasmExpression) -> () {
        todo!()
    }
}

impl JasmStatementVisitorWith<()> for FunctionBuilder {
    fn visit_empty(&mut self, module: &mut Module) -> () {
        println!("Empty statement\n");
        todo!()
    }

    fn visit_declaration(&mut self, module: &mut Module, variable: &Variable<Jasm>) -> () {
        println!("Declaration statement\n");
        todo!()
    }

    fn visit_assign(
        &mut self,
        module: &mut Module,
        object: &JasmExpression,
        expression: &JasmExpression,
    ) -> () {
        println!("Assign statement\n");
        todo!()
    }

    fn visit_if(
        &mut self,
        module: &mut Module,
        thens: &Vec<(JasmExpression, Block)>,
        els: &Block,
    ) -> () {
        println!("If statement\n");
        todo!()
    }

    fn visit_while(&mut self, module: &mut Module, condition: &JasmExpression, body: &Block) -> () {
        println!("While statement\n");
        todo!()
    }

    fn visit_struct_definition(
        &mut self,
        module: &mut Module,
        definition: &Struct<Parameter<Jasm>>,
    ) -> () {
        panic!("not supported in wasm, only llvm")
    }

    fn visit_function(
        &mut self,
        module: &mut Module,
        function: &std::rc::Rc<Function<Jasm>>,
    ) -> () {
        println!("Function statement\n");
        todo!()
    }

    fn visit_expression(&mut self, module: &mut Module, expression: &JasmExpression) -> () {
        println!("Expression statement\n");
        todo!()
    }

    fn visit_return(&mut self, module: &mut Module, expression: &Option<JasmExpression>) -> () {
        if let Some(jexpr) = expression {
            self.visit_with(jexpr, module);
        }
    }
}
