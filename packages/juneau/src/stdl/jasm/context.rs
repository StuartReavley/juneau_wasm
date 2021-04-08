


// pub struct JasmStdlContext {
//     pub ids: IdProvider,
//     pub globals: Symbols<Symbol>,
// }



// use crate::{core::{Id, IdProvider}, semantic::{Function, Implementation, Parameter, jasm::{Jasm, JasmPrimitiveImplementation, NumberType}}, stdl::jasm::new_jasm_stdl};
// use crate::parsing::{jasm::{JasmParseContext, Symbol}, Symbols};
// use crate::semantic::{UnaryOperator, BinaryOperator, FunctionType};
// use crate::semantic::jasm::JasmType;
// use crate::parsing::Overload;


// pub struct TestApplication {
//     pub ids: IdProvider,
//     pub globals: Symbols<Symbol>,
// }

// impl Default for TestApplication {
//     fn default() -> Self {
//         Self::new(&[])
//     }
// }

// impl TestApplication {
//     pub fn new(globals:&[Symbol]) -> Self {
//         let mut ids = IdProvider::new(1);
//         let stdl = new_jasm_stdl(&mut ids);
//         ids.increment_to(1001);
//         Self::new_with_stdl(ids, stdl, globals)
//     }

//     pub fn new_with_stdl(ids:IdProvider, stdl:Vec<Function<Jasm>>, globals:&[Symbol]) -> Self {
//         let globals = globals
//             .to_owned()
//             .into_iter()
//             .chain(stdl
//                 .iter()
//                 .map(|f|Symbol::new(f.id, &f.name, f.get_type().into())))
//             .collect();
//         Self {ids, globals}
//     }

//     pub fn parse_context(&mut self) -> JasmParseContext {
//         JasmParseContext::new(&mut self.ids, &self.globals)
//     }
// }
