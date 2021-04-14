use juneau_core::assert_eq_debug;
use crate::core::IdProvider;
use crate::semantic::Function;
use crate::semantic::jasm::JasmType;
use crate::parsing::jasm::parse_jasm_function;
use crate::building::jasm_llvm::build_jasm_function;
use super::new_contexts;
use JasmType::I64;


#[test]
fn test_external_invocation() {
    let source = "function main():i64 {return get_seven() + 10;}";
    let (mut parse_context, mut build_context) = new_contexts(vec![Function::new_external(&mut IdProvider::new(65), "get_seven", I64, &[], true, get_seven as *mut _)].into());

    extern fn get_seven() -> i64 {
        7
    }
    let jasm = parse_jasm_function(&mut parse_context, source);
    assert_eq_debug!(build_jasm_function(&mut build_context, true, &jasm).0,
r#"; ModuleID = 'main_module'
source_filename = "main_module"

define i64 @main() {
entry:
  %0 = call i64 @get_seven()
  %1 = add i64 %0, 10
  ret i64 %1
}

declare i64 @get_seven()
"#);
}