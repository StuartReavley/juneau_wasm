use juneau_core::assert_eq_debug;
use crate::parsing::jasm::parse_jasm_function;
use crate::building::jasm_llvm::build_jasm_function;
use super::new_default_contexts;


#[test]
fn test_function_and_invocation() {
    let source = r#"function main():i64 {
        function add(a:i64, b:i64):i64 {return a + b;}
        return add(6, 5);
    }"#;
    let (mut parse_context, mut build_context) = new_default_contexts();
    let jasm = parse_jasm_function(&mut parse_context, source);
    assert_eq_debug!(build_jasm_function(&mut build_context, false, &jasm).0,
r#"; ModuleID = 'main_module'
source_filename = "main_module"

define i64 @main() {
entry:
  %0 = call i64 @add(i64 6, i64 5)
  ret i64 %0
}

define i64 @add(i64 %0, i64 %1) {
entry:
  %a = alloca i64
  store i64 %0, i64* %a
  %b = alloca i64
  store i64 %1, i64* %b
  %2 = load i64, i64* %a
  %3 = load i64, i64* %b
  %4 = add i64 %2, %3
  ret i64 %4
}
"#);


    assert_eq_debug!(build_jasm_function(&mut build_context, true, &jasm).0,
r#"; ModuleID = 'main_module'
source_filename = "main_module"

define i64 @main() {
entry:
  %0 = call i64 @add(i64 6, i64 5)
  ret i64 %0
}

define i64 @add(i64 %0, i64 %1) {
entry:
  %2 = add i64 %1, %0
  ret i64 %2
}
"#);
}