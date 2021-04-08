use juneau_core::{assert_eq_debug, assert_eq_object};
use crate::semantic::jasm::render::render_jasm;
use crate::parsing::jasm::parse_jasm_statement;
use super::new_default_context;


#[test]
fn parse_jasm_if() {
    let source = "if true {}";
    let mut parse_context = new_default_context();
    let jasm = parse_jasm_statement(&mut parse_context, source);
    assert_eq_debug!(jasm, "If([(Constant(Bool(true)), Block([]))], Block([]))");
    assert_eq_object!(render_jasm(&jasm), r#"if true {

}"#);
}

#[test]
fn parse_jasm_if_else() {
    let source = "if true {} else {}";
    let mut parse_context = new_default_context();
    let jasm = parse_jasm_statement(&mut parse_context, source);
    assert_eq_debug!(jasm, "If([(Constant(Bool(true)), Block([]))], Block([]))");
    assert_eq_object!(render_jasm(&jasm), r#"if true {

}"#);
}
