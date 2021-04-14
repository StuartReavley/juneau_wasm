use juneau_core::assert_eq_debug;
use crate::parsing::jasm::parse_jasm_function;
use crate::building::jasm_llvm::build_jasm_function;
use super::new_default_contexts;


#[test]
fn test_binary() {
    let source = "function main():i64 {return 7 + 5;}";
    let (mut jasm_parse_context, mut jasm_build_context) = new_default_contexts();
    let jasm = parse_jasm_function(&mut jasm_parse_context, source);
    assert_eq_debug!(build_jasm_function(&mut jasm_build_context, false, &jasm).0,
r#"; ModuleID = 'main_module'
source_filename = "main_module"

define i64 @main() {
entry:
  ret i64 12
}
"#);

    assert_eq_debug!(build_jasm_function(&mut jasm_build_context, true, &jasm).0,
r#"; ModuleID = 'main_module'
source_filename = "main_module"

define i64 @main() {
entry:
  ret i64 12
}
"#);
}