use wasmer::{imports, wat2wasm, Instance, Module, Store, Value};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_jit::JIT;

use crate::building::jasm_wasm::{compile_wasm, compile_wasm_to_file};
use crate::parsing::jasm::parse_jasm_function;
use crate::parsing::jasm::test::new_default_context;

#[test]
fn test_1() {
    // This is jasm code in string form. Check out packages/juneau/src/parsing/jasm/test folder for more examples of jasm in string form
    let source = "function add(a:i64, b:i64):i64 {return a + b;}"; // JASM
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
    let add = instance.exports.get_function("add").unwrap();
    // execute the wasm function
    let results = add.call(&[Value::I64(1), Value::I64(2)]).unwrap();
    assert_eq!(results.to_vec(), vec![Value::I64(3)]);
}