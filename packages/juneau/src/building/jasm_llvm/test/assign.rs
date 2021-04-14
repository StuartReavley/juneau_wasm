use juneau_core::assert_eq_debug;
use crate::parsing::jasm::parse_jasm_function;
use crate::building::jasm_llvm::build_jasm_function;
use super::new_default_contexts;


#[test]
fn test_assign() {
    let source = "function main():i64 {
        let a:i64;
        a = 4;
        return a;
    }";
    let (mut parse_context, mut build_context) = new_default_contexts();
    let jasm = parse_jasm_function(&mut parse_context, source);
    assert_eq_debug!(build_jasm_function(&mut build_context, false, &jasm).0,
r#"; ModuleID = 'main_module'
source_filename = "main_module"

define i64 @main() {
entry:
  %a = alloca i64
  store i64 4, i64* %a
  %0 = load i64, i64* %a
  ret i64 %0
}
"#);

    assert_eq_debug!(build_jasm_function(&mut build_context, true, &jasm).0,
r#"; ModuleID = 'main_module'
source_filename = "main_module"

define i64 @main() {
entry:
  ret i64 4
}
"#);
}



#[test]
fn test_assign_for_loop() {
    let source = "function main():i64 {
        let i:i64;
        i = 0;
        while(i < 6) {
            i = i + 1;
        }
        return i;
    }";
    let (mut parse_context, mut build_context) = new_default_contexts();
    let jasm = parse_jasm_function(&mut parse_context, source);
    assert_eq_debug!(build_jasm_function(&mut build_context, false, &jasm).0,
r#"; ModuleID = 'main_module'
source_filename = "main_module"

define i64 @main() {
entry:
  %i = alloca i64
  store i64 0, i64* %i
  br label %while_condition

while_condition:                                  ; preds = %while_body, %entry
  %0 = load i64, i64* %i
  %1 = icmp slt i64 %0, 6
  br i1 %1, label %while_body, label %while_end

while_body:                                       ; preds = %while_condition
  %2 = load i64, i64* %i
  %3 = add i64 %2, 1
  store i64 %3, i64* %i
  br label %while_condition

while_end:                                        ; preds = %while_condition
  %4 = load i64, i64* %i
  ret i64 %4
}
"#);

    assert_eq_debug!(build_jasm_function(&mut build_context, true, &jasm).0,
r#"; ModuleID = 'main_module'
source_filename = "main_module"

define i64 @main() {
entry:
  br label %while_condition

while_condition:                                  ; preds = %while_body, %entry
  %i.0 = phi i64 [ 0, %entry ], [ %1, %while_body ]
  %0 = icmp slt i64 %i.0, 6
  br i1 %0, label %while_body, label %while_end

while_body:                                       ; preds = %while_condition
  %1 = add i64 %i.0, 1
  br label %while_condition

while_end:                                        ; preds = %while_condition
  ret i64 %i.0
}
"#);
}
