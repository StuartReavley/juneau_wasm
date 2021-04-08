use juneau_core::{assert_eq_debug, assert_eq_object};
use crate::core::Id;
use crate::semantic::Variable;
use crate::semantic::jasm::JasmType::*;
use crate::semantic::jasm::render::render_jasm;
use crate::parsing::jasm::parse_jasm_statement;
use super::new_context;


#[test]
fn parse_jasm_assign() {
    let source = "a = 4;";
    let mut context = new_context(&[Variable::new(Id::from(63), &"a".into(), &I64)]);
    let jasm = parse_jasm_statement(&mut context, source);
    assert_eq_debug!(jasm, r#"Assign(Var(Variable { id: 63, name: "a", typ: I64 }), Constant(I64(4)))"#);
    assert_eq_object!(render_jasm(&jasm), r#"a = 4;"#);
}
