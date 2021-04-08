// use std::rc::Rc;
// use std::cell::RefCell;
// use crate::{core::{Id, IdProvider}, semantic::{Function, Implementation, Parameter, jasm::{Jasm, JasmPrimitiveImplementation, NumberType}}, stdl::jasm::new_jasm_stdl};
// use crate::parsing::{jasm::{JasmParseContext, Symbol}, Symbols};
// use crate::semantic::{UnaryOperator, BinaryOperator, FunctionType};
// use crate::semantic::jasm::JasmType;
// use crate::parsing::Overload;


// // impl TestApplication {

// //     pub fn default() -> JasmParseContext {
// //         Self::new(&[])
// //     }

// //     pub fn new(globals:&[Symbol]) -> JasmParseContext {
// //         let ids = IdProvider::new_cell(1);
// //         let stdl = new_jasm_stdl(&mut *ids.borrow_mut());
// //         ids.borrow_mut().increment_to(1001);
// //         Self::new_with_stdl(ids, stdl, globals)
// //     }

// //     pub fn new_with_stdl(ids:Rc<RefCell<IdProvider>>, stdl:Vec<Function<Jasm>>, globals:&[Symbol]) -> JasmParseContext {
// //         let globals = globals
// //             .to_owned()
// //             .into_iter()
// //             .chain(stdl
// //                 .iter()
// //                 .map(|f|Symbol::new(f.id, &f.name, f.get_type().into())))
// //             .collect();
// //         JasmParseContext::new(ids, globals)
// //     }
// // }


// // PARSING JASM


// pub fn new_default_context() -> JasmParseContext {
//     new_context(&[])
// }

// pub fn new_context(globals:&[Symbol]) -> JasmParseContext {
//     let ids = IdProvider::new_cell(1);
//     let stdl = new_jasm_stdl(&mut *ids.borrow_mut());
//     ids.borrow_mut().increment_to(1001);
//     new_with_stdl(ids, stdl, globals)
// }

// pub fn new_with_stdl(ids:Rc<RefCell<IdProvider>>, stdl:Vec<Function<Jasm>>, globals:&[Symbol]) -> JasmParseContext {
//     let globals = globals
//         .to_owned()
//         .into_iter()
//         .chain(stdl
//             .iter()
//             .map(|f|Symbol::new(f.id, &f.name, f.get_type().into())))
//         .collect();
//     JasmParseContext::new(ids, globals)
// }


// // PASRING JQL

// pub fn new_default_context() -> JqlParseContext {
//     new_context(&[])
// }

// pub fn new_context(globals:&[Symbol]) -> JqlParseContext {
//     let ids = IdProvider::new_cell(1);
//     let jasm_stdl = new_jasm_stdl(&mut *ids.borrow_mut());
//     let jasm_symbols = jasm_stdl
//         .iter()
//         .map(|f|jasm::Symbol::from(f))
//         .collect();

//     let mut jasm_parse = JasmParseContext::new(ids.to_owned(), jasm_symbols);
//     let jql_stdl = new_jql_stdl(&mut jasm_parse);
//     let jql_symbols = globals
//         .to_owned()
//         .into_iter()
//         .chain(jql_stdl
//             .iter()
//             .map(|f|jql::Symbol::new_leaf(f.id, f.name.to_owned(), f.get_type().into())))
//         .collect();

//     ids.borrow_mut().increment_to(1001);
//     let jql_parse_context = JqlParseContext::new(ids, jql_symbols);
//     jql_parse_context
// }


// // BUILD JASM

// pub fn new_default_contexts() -> (JasmParseContext, JasmBuildContext) {
//     new_contexts(Vec::new())
// }

// pub fn new_contexts(externals:Vec<Function<Jasm>>) -> (JasmParseContext, JasmBuildContext) {
//     let context = Context::new();
//     let ids = IdProvider::new_cell(1);
//     let stdl = concat(
//         new_jasm_stdl(&mut *ids.borrow_mut()),
//         externals);

//     ids.borrow_mut().increment_to(1001);

//     let symbols = stdl
//         .iter()
//         .map(|f|Symbol::new(f.id, &f.name, f.get_type().into()))
//         .collect();

//     let parse_context = JasmParseContext::new(ids.to_owned(), symbols);
//     let build_context = JasmBuildContext::new(ids, stdl, context);
//     (parse_context, build_context)
// }

// // BUILD JQL



// pub fn new_default_contexts() -> (JasmParseContext, JqlParseContext, JasmBuildContext, JqlBuildContext) {
//     new_contexts(&[])
// }

// pub fn new_contexts(jql_symbols:&[jql::Symbol]) -> (JasmParseContext, JqlParseContext, JasmBuildContext, JqlBuildContext) {
//     let ids = IdProvider::new_cell(1);
//     let jasm_stdl = new_jasm_stdl(&mut *ids.borrow_mut());
//     let jasm_symbols = jasm_stdl
//         .iter()
//         .map(|f|jasm::Symbol::from(f))
//         .collect::<Symbols<_>>();

//     let mut jasm_parse = JasmParseContext::new(ids.to_owned(), jasm_symbols.to_owned());
//     let jql_stdl = new_jql_stdl(&mut jasm_parse);
//     let jql_symbols = jql_symbols
//         .to_owned()
//         .into_iter()
//         .chain(jql_stdl
//             .iter()
//             .map(|f|jql::Symbol::new_leaf(f.id, f.name.to_owned(), f.get_type().into())))
//         .collect();

//     ids.borrow_mut().increment_to(1001);
//     let context = Context::new();

//     let jql_parse = JqlParseContext::new(ids.to_owned(), jql_symbols);
//     let jasm_build = JasmBuildContext::new(ids.to_owned(), jasm_stdl.to_owned(), context);
//     let jql_build = JqlBuildContext::new(ids, jql_stdl.to_owned(), jasm_symbols);
//     (jasm_parse, jql_parse, jasm_build, jql_build)
// }