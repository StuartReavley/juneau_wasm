use juneau_core::{assert_eq_debug, assert_eq_object};
use crate::semantic::jasm::render::render_jasm;
use crate::parsing::jasm::parse_jasm_expression;
use super::new_default_context;


#[test]
fn parse_jasm_constant_u64() {
    let source = "23";
    let mut parse_context = new_default_context();
    let jasm = parse_jasm_expression(&mut parse_context, source);
    assert_eq_debug!(jasm, "Constant(I64(23))");
    assert_eq_object!(render_jasm(&jasm), r#"23"#);
}

#[test]
fn parse_jasm_constant_i64() {
    let source = "-23";
    let mut parse_context = new_default_context();
    let jasm = parse_jasm_expression(&mut parse_context, source);
    assert_eq_debug!(jasm, "Constant(I64(-23))");
    assert_eq_object!(render_jasm(&jasm), r#"-23"#);
}

#[test]
fn parse_jasm_constant_f64() {
    let source = "-23.3";
    let mut parse_context = new_default_context();
    let jasm = parse_jasm_expression(&mut parse_context, source);
    assert_eq_debug!(jasm, "Constant(F64(-23.3))");
    assert_eq_object!(render_jasm(&jasm), r#"-23.3"#);
}

#[test]
fn parse_jasm_constant_bool() {
    let source = "true";
    let mut parse_context = new_default_context();
    let jasm = parse_jasm_expression(&mut parse_context, source);
    assert_eq_debug!(jasm, "Constant(Bool(true))");
    assert_eq_object!(render_jasm(&jasm), r#"true"#);
}

#[test]
fn parse_jasm_constant_string() {
    let source = r#""bob""#;
    let mut parse_context = new_default_context();
    let jasm = parse_jasm_expression(&mut parse_context, source);
    assert_eq_debug!(jasm, r#"Constant(String("bob"))"#);
    assert_eq_object!(render_jasm(&jasm), r#""bob""#);
}

#[test]
fn parse_jasm_constant_null() {
    let source = "null";
    let mut parse_context = new_default_context();
    let jasm = parse_jasm_expression(&mut parse_context, source);
    assert_eq_debug!(jasm, "Constant(Null)");
    assert_eq_object!(render_jasm(&jasm), r#"null"#);
}
