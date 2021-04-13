mod new_contexts;
pub use new_contexts::*;

mod assign;
mod binary;
mod conditional;
mod constant;
mod declaration;
mod executable;
mod function;
mod invocation;
mod structt;
mod unary;
mod variable;




// struct Counter {
//     index: usize,
//     len: usize
// }

// unsafe extern fn counter_next(context:*mut Counter) -> bool {
//     (*context).index += 1;
//     (*context).index < (*context).len
// }


// #[test] #[ignore]
// fn test_context() {
//     use JasmType::*;
//     let mut application = TestApplication::new(vec![semantic::Function::new_external(&mut IdProvider::new(123), "counter_next", Bool, &[Pointer(Box::new(Void))], counter_next as *mut _)]);
//     let mut context = Counter {index: 0, len: 10};
//     let source = "counter_next($context)";
//     let expression = parse_jasm_expression(&mut application.parse_context(), source);
//     let e = generate_executable::<i32>(&mut application.generate_context(), &expression);
//     assert_eq!(e.execute(&mut context), 10);
// }

// #[test] #[ignore]
// fn test_while() {
//     use JasmType::*;
//     let mut application = TestApplication::new(vec![semantic::Function::new_external(&mut IdProvider::new(123), "counter_next", Bool, &[Pointer(Box::new(Void))], counter_next as *mut _)]);
//     let mut context = Counter {index: 0, len: 5};
//     let source = "{while(counter_next($context)) 2}";
//     let expression = parse_jasm_expression(&mut application.parse_context(), source);
//     let e = generate_executable::<i32>(&mut application.generate_context(), &expression);
//     assert_eq!(e.execute(&mut context), 0);
//     assert_eq!(5, context.index);
// }



