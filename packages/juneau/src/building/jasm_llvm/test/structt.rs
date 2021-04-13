use juneau_core::assert_eq_debug;
use crate::parsing::jasm::parse_jasm_function;
use crate::building::jasm_llvm::build_jasm_function;
use super::new_default_contexts;


#[test]
fn test_struct() {
    let source = "function main():void {
        struct String {
            ptr:      *u8,
            len:      u64,
            capacity: u64
        }

        return;
    }";

    let (mut parse_context, mut build_context) = new_default_contexts();
    let jasm = parse_jasm_function(&mut parse_context, source);
    assert_eq_debug!(build_jasm_function(&mut build_context, false, &jasm).0,
r#"; ModuleID = 'main_module'
source_filename = "main_module"

define void @main() {
entry:
  ret void
}
"#);

//     assert_eq_debug!(module_from_expression(externals, context, true, expression),
// r#"; ModuleID = 'main_module'
// source_filename = "main_module"

// define i64 @main() {
// entry:
//   ret i64 12
// }
// "#);

//     let e = generate_executable(externals, expression);
//     assert_eq!(12, e.execute_unit());
}



// #[test]
// fn test_struct_object() {
//     let source = " {
//         struct String {
//             ptr:      *u8,
//             len:      u32,
//             capacity: u32
//         };
    
//         let s = String {ptr: (8 as *u8), len: (12 as u32), capacity: (60 as u32)};
//         0
//     }";
    
//     let mut application = TestApplication::default();
//     let expression = parse_jasm_expression(&mut parse_context, source);
//     assert_eq_debug!(module_from_expression(&mut generate_context, false, &expression),
// r#"; ModuleID = 'main_module'
// source_filename = "main_module"

// define i64 @main() {
// entry:
//   %s_ptr = alloca i8*
//   store i8* inttoptr (i64 8 to i8*), i8** %s_ptr
//   %s_len = alloca i32
//   store i32 12, i32* %s_len
//   %s_capacity = alloca i32
//   store i32 60, i32* %s_capacity
//   ret i64 0
// }
// "#);


// // r#"; ModuleID = 'main_module'
// // source_filename = "main_module"

// // %String = type { i8*, i32, i32 }

// // define i64 @main() {
// // entry:
// //   %s = alloca %String
// //   ret i64 0
// // }
// // "#);
// }