use juneau_core::assert_eq_debug;
use crate::core::Id;
use crate::semantic::{FunctionType, Variable};
use crate::semantic::jasm::JasmType::*;
use crate::parsing::jasm::parse_jasm_expression;
use super::new_context;



#[test]
fn parse_jasm_invocation() {
    let source = "sum(2, 4)";
    let mut parse_context = new_context(&[Variable::new(Id::from(8001), &"sum".into(), &FunctionType::new(vec![I64, I64], I64).into())]);
    let jasm = parse_jasm_expression(&mut parse_context, source);
    assert_eq_debug!(jasm, r#"Invocation(8001, "sum", [Constant(I64(2)), Constant(I64(4))], I64)"#);
}