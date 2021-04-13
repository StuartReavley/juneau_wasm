use juneau_core::assert_eq_debug;
use crate::parsing::jasm::parse_jasm_function;
use crate::building::jasm_llvm::build_jasm_function;
use super::new_default_contexts;


#[test]
fn test_constant_integer() {
    let source = "function main():i64 {return 7;}";
    let (mut parse_context, mut build_context) = new_default_contexts();
    let expression = parse_jasm_function(&mut parse_context, source);

    assert_eq_debug!(build_jasm_function(&mut build_context, true, &expression).0,
r#"; ModuleID = 'main_module'
source_filename = "main_module"

define i64 @main() {
entry:
  ret i64 7
}
"#);
}

#[test]
fn test_constant_f64() {
    let source = "function main():f64 {return 7.4;}";
    let (mut parse_context, mut build_context) = new_default_contexts();
    let expression = parse_jasm_function(&mut parse_context, source);

    assert_eq_debug!(build_jasm_function(&mut build_context, true, &expression).0,
r#"; ModuleID = 'main_module'
source_filename = "main_module"

define double @main() {
entry:
  ret double 7.400000e+00
}
"#);
}
