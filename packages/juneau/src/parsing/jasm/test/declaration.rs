use juneau_core::{assert_eq_debug, assert_eq_object};
use crate::semantic::jasm::render::render_jasm;
use crate::parsing::jasm::parse_jasm_statement;
use super::new_default_context;


#[test]
fn parse_jasm_declaration() {
    let source = "let a:u64;";
    let mut parse_context = new_default_context();
    let jasm = parse_jasm_statement(&mut parse_context, source);
    assert_eq_debug!(jasm, r#"Declaration(Variable { id: 1001, name: "a", typ: U64 })"#);
    assert_eq_object!(render_jasm(&jasm), r#"let a:u64;"#);
}