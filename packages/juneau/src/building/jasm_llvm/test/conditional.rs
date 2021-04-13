// use crate::parsing::jasm::parse_jasm;
// use crate::building::jasm::generate_executable;
// use super::{TestApplication, application};


// #[test]
// fn test_conditional() {
//     let source = "0 == 1 ? 34 : 55";
//     let mut application = TestApplication::default();
//     let expression = parse_jasm(&mut application.parse_context(), source);
//     let e = generate_executable(&mut application.generate_context(), &expression);
//     assert_eq!(55, e.execute_unit());
// }