use juneau_core::assert_eq_debug;
use crate::parsing::jasm::parse_jasm_function;
use crate::building::jasm_llvm::build_jasm_function;
use super::new_default_contexts;


#[test]
fn test_declaration() {
    let source = "function main():void {let a:i64; return;}";
    let (mut parse_context, mut build_context) = new_default_contexts();
    let jasm = parse_jasm_function(&mut parse_context, source);
    assert_eq_debug!(build_jasm_function(&mut build_context, false, &jasm).0,
r#"; ModuleID = 'main_module'
source_filename = "main_module"

define void @main() {
entry:
  %a = alloca i64
  ret void
}
"#);

    assert_eq_debug!(build_jasm_function(&mut build_context, true, &jasm).0,
r#"; ModuleID = 'main_module'
source_filename = "main_module"

define void @main() {
entry:
  ret void
}
"#);
}
