use juneau_core::assert_eq_debug;
use crate::semantic::jasm::render::render_jasm;
use crate::parsing::jasm::parse_jasm_expression;
use super::new_default_context;


#[test]
fn parse_jasm_binary_expression() {
    let source = "2 + 7";
    let mut parse_context = new_default_context();
    let jasm = parse_jasm_expression(&mut parse_context, source);
    assert_eq_debug!(jasm, r#"Invocation(15, "+", [Constant(I64(2)), Constant(I64(7))], I64)"#);
    juneau_core::assert_eq_object!(render_jasm(&jasm), r#"+(2, 7)"#);
}