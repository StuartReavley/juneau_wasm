use crate::core::Visitor;
use crate::parsing::jasm::parse_jasm_function;
use crate::building::ModuleBuildVisitor;
use crate::building::jasm_llvm::build_jasm_function;
use crate::building::jasm_llvm::test::new_default_contexts;
use crate::building::llvm_execution::ExecutionBuildVisitor;



#[test]
fn test_function_and_invocation() {
    let source = r#"function main():i64 {
        function add(a:i64, b:i64):i64 {return a + b;}
        return add(6, 5);
    }"#;
    let (mut parse_context, mut build_context) = new_default_contexts();
    let jasm = parse_jasm_function(&mut parse_context, source);
    let (llvm_module, module) = build_jasm_function(&mut build_context, false, &jasm);
    let executable_visitor = ExecutionBuildVisitor::new(llvm_module);
    let module = executable_visitor.visit(&module);
    let result:i64 = module.get_main().call0();
    assert_eq!(result, 11);
}