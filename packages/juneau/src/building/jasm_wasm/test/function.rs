use wasmer::{imports, wat2wasm, Instance, Module, Store, Value};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_jit::JIT;

use crate::building::jasm_wasm::{build_wasm_function, build_wasm_function_to_file};
use crate::parsing::jasm::parse_jasm_function;
use super::new_default_visitors;


#[test]
fn wasm_function() {
    // This is jasm code in string form. Check out packages/juneau/src/parsing/jasm/test folder for more examples of jasm in string form
    let source = "function add(a:i64, b:i64):i64 {return a + b;}"; // JASM
    let (mut parse_visitor, mut build_visitor) = new_default_visitors();
    let jasm = parse_jasm_function(&mut parse_visitor, source);
    let wasm_bytes = build_wasm_function(&mut build_visitor, &jasm);
    
    // Set up Wasmer, which runs the WebAssembly inside our tests
    let compiler_config = Cranelift::default();
    let engine = JIT::new(compiler_config).engine();
    let store = Store::new(&engine);
    let module = Module::new(&store, wasm_bytes).unwrap();
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object).unwrap();
    let add = instance.exports.get_function("add").unwrap();
    // execute the wasm function
    let results = add.call(&[Value::I64(1), Value::I64(2)]).unwrap();
    assert_eq!(results.to_vec(), vec![Value::I64(3)]);
}

fn wasm_function_and_invocation() {
    // This is jasm code in string form. Check out packages/juneau/src/parsing/jasm/test folder for more examples of jasm in string form
    let source = r#"function main():i64 {
        function add(a:i64, b:i64):i64 {return a + b;}
        return add(6, 5);
    }"#; // JASM

    let (mut parse_visitor, mut build_visitor) = new_default_visitors();
    let jasm = parse_jasm_function(&mut parse_visitor, source);
    let wasm_bytes = build_wasm_function(&mut build_visitor, &jasm);
    // println!("{:#?}", jasm);
    
    // Set up Wasmer, which runs the WebAssembly inside our tests
    let compiler_config = Cranelift::default();
    let engine = JIT::new(compiler_config).engine();
    let store = Store::new(&engine);
    let module = Module::new(&store, wasm_bytes).unwrap();
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object).unwrap();
    let main = instance.exports.get_function("main").unwrap();
    // execute the wasm function
    let results = main.call(&[]).unwrap();
    assert_eq!(results.to_vec(), vec![Value::I64(11)]);
}