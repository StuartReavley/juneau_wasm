use wasmer::{imports, wat2wasm, Instance, Module, Store, Value};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_jit::JIT;



/// This test is lifted straight from the wasmer docs (https://crates.io/crates/wasmer/), to show that wasmer works correctly
#[test]
fn testing_wasmer() {

    // Let's declare the Wasm module with the text representation.
    let wasm_bytes = wat2wasm(
        r#"
(module
  (type $sum_t (func (param i32 i32) (result i32)))
  (func $sum_f (type $sum_t) (param $x i32) (param $y i32) (result i32)
    local.get $x
    local.get $y
    i32.add)
  (export "sum" (func $sum_f)))
"#
        .as_bytes(),
    ).unwrap();

    // Define a compiler configuration.
    //
    // In this situation, the compiler is
    // `wasmer_compiler_cranelift`. The compiler is responsible to
    // compile the Wasm module into executable code.
    let compiler_config = Cranelift::default();

    println!("Creating JIT engine...");
    // Define the engine that will drive everything.
    //
    // In this case, the engine is `wasmer_engine_jit` which roughly
    // means that the executable code will live in memory.
    let engine = JIT::new(compiler_config).engine();

    // Create a store, that holds the engine.
    let store = Store::new(&engine);

    println!("Compiling module...");
    // Here we go.
    //
    // Let's compile the Wasm module. It is at this step that the Wasm
    // text is transformed into Wasm bytes (if necessary), and then
    // compiled to executable code by the compiler, which is then
    // stored in memory by the engine.
    let module = Module::new(&store, wasm_bytes).unwrap();

    // Congrats, the Wasm module is compiled! Now let's execute it for
    // the sake of having a complete example.

    // Create an import object. Since our Wasm module didn't declare
    // any imports, it's an empty object.
    let import_object = imports! {};

    println!("Instantiating module...");
    // And here we go again. Let's instantiate the Wasm module.
    let instance = Instance::new(&module, &import_object).unwrap();

    println!("Calling `sum` function...");
    // The Wasm module exports a function called `sum`.
    let sum = instance.exports.get_function("sum").unwrap();
    let results = sum.call(&[Value::I32(1), Value::I32(2)]).unwrap();

    println!("Results: {:?}", results);
    assert_eq!(results.to_vec(), vec![Value::I32(3)]);

}