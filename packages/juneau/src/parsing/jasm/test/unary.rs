use juneau_core::assert_eq_debug;
use crate::parsing::jasm::parse_jasm_expression;
use super::new_default_context;


#[test]
fn parse_jasm_unary_expression() {
    let source = "-(7)";
    let mut parse_context = new_default_context();
    let jasm = parse_jasm_expression(&mut parse_context, source);
    assert_eq_debug!(jasm, r#"Invocation(27, "-", [Constant(I64(7))], I64)"#);
}