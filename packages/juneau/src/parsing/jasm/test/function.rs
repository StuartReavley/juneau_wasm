use juneau_core::{assert_eq_debug, assert_eq_object, hash_map};
use crate::{core::Template, parsing::jasm::parse_jasm_statement, semantic::jasm::render::render_jasm};
use crate::parsing::jasm::parse_jasm_function;
use super::new_default_context;


#[test]
fn parse_function() {
    let source = "function add(a:i64, b:i64):i64 {return a + b;}";
    let mut parse_context = new_default_context();
    let jasm = parse_jasm_function(&mut parse_context, source);
    assert_eq_debug!(jasm, r#"Function { id: 1003, name: "add", parameters: [Parameter { id: 1001, name: "a", typ: I64 }, Parameter { id: 1002, name: "b", typ: I64 }], implementation: Semantic((Block([Return(Some(Invocation(15, "+", [Var(Variable { id: 1001, name: "a", typ: I64 }), Var(Variable { id: 1002, name: "b", typ: I64 })], I64)))]), I64)) }"#);
    assert_eq_object!(render_jasm(&jasm), r#"function add(a:i64, b:i64):i64 {
    return +(a, b);
}"#);
}

#[test]
fn parse_function_and_invocation() {
    let source = r#"function main():i64 {
        function add(a:i64, b:i64):i64 {return a + b;}
        return add(6, 5);
    }"#;
    let mut parse_context = new_default_context();
    let jasm = parse_jasm_function(&mut parse_context, source);
    assert_eq_debug!(jasm, r#"Function { id: 1004, name: "main", parameters: [], implementation: Semantic((Block([Function(Function { id: 1003, name: "add", parameters: [Parameter { id: 1001, name: "a", typ: I64 }, Parameter { id: 1002, name: "b", typ: I64 }], implementation: Semantic((Block([Return(Some(Invocation(15, "+", [Var(Variable { id: 1001, name: "a", typ: I64 }), Var(Variable { id: 1002, name: "b", typ: I64 })], I64)))]), I64)) }), Return(Some(Invocation(1003, "add", [Constant(I64(6)), Constant(I64(5))], I64)))]), I64)) }"#);
    assert_eq_object!(render_jasm(&jasm), r#"function main():i64 {
    function add(a:i64, b:i64):i64 {
        return +(a, b);
    }
    return add(6, 5);
}"#);
}


#[test]
fn parse_jql_add_function() {
    let source = Template::render(r#"
        function ${name}(
            a_typ:u8,
            a_ptr:*u8,
            a_boolean:bool,
            a_integer:i64,
            a_float:f64,
            b_typ:u8,
            b_ptr:*u8,
            b_boolean:bool,
            b_integer:i64,
            b_float:f64,
            out_typ:*u8,
            out_ptr:*u8,
            out_boolean:*bool,
            out_integer:*i64,
            out_float:*f64):void {

            if a_typ == ${null} or b_typ == ${null} {
                *out_typ = ${null};
                *out_ptr = null;
                *out_boolean = false;
                *out_integer = 0;
                *out_float = 0.0;
            } else if a_typ == ${error} {
                *out_typ = ${error};
                *out_ptr = null;
                *out_boolean = false;
                *out_integer = a_integer;
                *out_float = 0.0;
            } else if b_typ == ${error} {
                *out_typ = ${error};
                *out_ptr = null;
                *out_boolean = false;
                *out_integer = b_integer;
                *out_float = 0.0;
            } else if a_typ == ${boolean} {
                *out_typ = ${error};
                *out_ptr = null;
                *out_boolean = false;
                *out_integer = 1;
                *out_float = 0.0;
            } else if a_typ == ${integer} {
                if b_typ == ${integer} {
                    *out_typ = ${integer};
                    *out_ptr = null;
                    *out_boolean = false;
                    *out_integer = a_integer ${operator} b_integer;
                    *out_float = 0.0;                    
                } else if b_typ == ${float} {
                    *out_typ = ${float};
                    *out_ptr = null;
                    *out_boolean = false;
                    *out_integer = 0;
                    *out_float = (a_integer as f64) ${operator} b_float;                 
                }
            } else if a_typ == ${float} {
                if b_typ == ${integer} {
                    *out_typ = ${float};
                    *out_ptr = null;
                    *out_boolean = false;
                    *out_integer = 0;
                    *out_float = a_float ${operator} (b_integer as f64);                    
                } else if b_typ == ${float} {
                    *out_typ = ${float};
                    *out_ptr = null;
                    *out_boolean = false;
                    *out_integer = 0;
                    *out_float = a_float ${operator} b_float;                 
                }
            }

        return;
        }"#, hash_map!{
            "name".into() => "add".into(),
            "null".into() => "(0 as u8)".into(),
            "boolean".into() => "(1 as u8)".into(),
            "integer".into() => "(2 as u8)".into(),
            "float".into() => "(3 as u8)".into(),
            "error".into() => "(6 as u8)".into(),
            "operator".into() => "+".into()
        });



    let mut parse_context = new_default_context();
    let jasm = parse_jasm_function(&mut parse_context, &source);
    assert_eq_object!(render_jasm(&jasm), r#"function add(a_typ:u8, a_ptr:*u8, a_boolean:bool, a_integer:i64, a_float:f64, b_typ:u8, b_ptr:*u8, b_boolean:bool, b_integer:i64, b_float:f64, out_typ:*u8, out_ptr:*u8, out_boolean:*bool, out_integer:*i64, out_float:*f64):void {
    if or(==(a_typ, 0 as u8), ==(b_typ, 0 as u8)) {
        *(out_typ) = 0 as u8;
        *(out_ptr) = null;
        *(out_boolean) = false;
        *(out_integer) = 0;
        *(out_float) = 0;
    } else if ==(a_typ, 6 as u8) {
        *(out_typ) = 6 as u8;
        *(out_ptr) = null;
        *(out_boolean) = false;
        *(out_integer) = a_integer;
        *(out_float) = 0;
    } else if ==(b_typ, 6 as u8) {
        *(out_typ) = 6 as u8;
        *(out_ptr) = null;
        *(out_boolean) = false;
        *(out_integer) = b_integer;
        *(out_float) = 0;
    } else if ==(a_typ, 1 as u8) {
        *(out_typ) = 6 as u8;
        *(out_ptr) = null;
        *(out_boolean) = false;
        *(out_integer) = 1;
        *(out_float) = 0;
    } else if ==(a_typ, 2 as u8) {
        if ==(b_typ, 2 as u8) {
            *(out_typ) = 2 as u8;
            *(out_ptr) = null;
            *(out_boolean) = false;
            *(out_integer) = +(a_integer, b_integer);
            *(out_float) = 0;
        } else if ==(b_typ, 3 as u8) {
            *(out_typ) = 3 as u8;
            *(out_ptr) = null;
            *(out_boolean) = false;
            *(out_integer) = 0;
            *(out_float) = +(a_integer as f64, b_float);
        }
    } else if ==(a_typ, 3 as u8) {
        if ==(b_typ, 2 as u8) {
            *(out_typ) = 3 as u8;
            *(out_ptr) = null;
            *(out_boolean) = false;
            *(out_integer) = 0;
            *(out_float) = +(a_float, b_integer as f64);
        } else if ==(b_typ, 3 as u8) {
            *(out_typ) = 3 as u8;
            *(out_ptr) = null;
            *(out_boolean) = false;
            *(out_integer) = 0;
            *(out_float) = +(a_float, b_float);
        }
    }
    return;
}"#);
}