use juneau_core::{assert_eq_debug, assert_eq_object};
use crate::semantic::jasm::render::render_jasm;
use crate::parsing::jasm::parse_jasm_expression;
use super::new_default_context;


#[test]
fn parse_jasm_cast() {
    let source = "23.3 as i64";
    let mut parse_context = new_default_context();
    let jasm = parse_jasm_expression(&mut parse_context, source);
    assert_eq_debug!(jasm, "Cast(Constant(F64(23.3)), I64)");
    assert_eq_object!(render_jasm(&jasm), r#"23.3 as i64"#);
}