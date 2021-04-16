use wasmer::{imports, wat2wasm, Instance, Module, Store, Value};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_jit::JIT;

use crate::building::jasm_wasm::{build_wasm_function, build_wasm_function_to_file};
use crate::parsing::jasm::parse_jasm_function;
use super::new_default_visitors;


#[test]
fn wasm_assign() {
    // This is jasm code in string form. Check out packages/juneau/src/parsing/jasm/test folder for more examples of jasm in string form
    let source = r#"function main():i64 {
        let a:i64;
        a = 4;
        return a;
    }"#; // JASM
    let mut parse_context = new_default_context();
    let jasm = parse_jasm_function(&mut parse_context, source);
    let wasm_bytes = compile_wasm(&jasm);
    
    // Set up Wasmer, which runs the WebAssembly inside our tests
    let compiler_config = Cranelift::default();
    let engine = JIT::new(compiler_config).engine();
    let store = Store::new(&engine);
    let module = Module::new(&store, wasm_bytes).unwrap();
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object).unwrap();
    let add = instance.exports.get_function("main").unwrap();
    // execute the wasm function
    let results = add.call(&[]).unwrap();
    assert_eq!(results.to_vec(), vec![Value::I64(4)]);
}

#[test]
fn wasm_assign_for_loop() {
    // This is jasm code in string form. Check out packages/juneau/src/parsing/jasm/test folder for more examples of jasm in string form
    let source = r#"function main():i64 {
        let i:i64;
        i = 0;
        while(i < 6) {
            i = i + 1;
        }
        return i;
    }"#; // JASM
    let mut parse_context = new_default_context();
    let jasm = parse_jasm_function(&mut parse_context, source);
    println!("{:#?}", &jasm);
    let wasm_bytes = compile_wasm(&jasm);
    
    // Set up Wasmer, which runs the WebAssembly inside our tests
    let compiler_config = Cranelift::default();
    let engine = JIT::new(compiler_config).engine();
    let store = Store::new(&engine);
    let module = Module::new(&store, wasm_bytes).unwrap();
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object).unwrap();
    let add = instance.exports.get_function("main").unwrap();
    // execute the wasm function
    let results = add.call(&[]).unwrap();
    assert_eq!(results.to_vec(), vec![Value::I64(4)]);
}