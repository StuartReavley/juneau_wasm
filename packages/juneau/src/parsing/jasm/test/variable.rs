use juneau_core::assert_eq_debug;
use crate::core::Id;
use crate::semantic::Variable;
use crate::semantic::jasm::JasmType::*;
use crate::parsing::jasm::parse_jasm_expression;
use super::new_context;


#[test]
fn parse_jasm_variable() {
    let source = "a";
    let mut parse_context = new_context(&[Variable::new(Id::from(63), &"a".into(), &I64)]);
    let jasm = parse_jasm_expression(&mut parse_context, source);
    assert_eq_debug!(jasm, r#"Var(Variable { id: 63, name: "a", typ: I64 })"#);
}
