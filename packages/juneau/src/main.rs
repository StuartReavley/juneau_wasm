#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(try_blocks)] // needed for Antlr


use std::{fs::File, io::{BufReader, Cursor}};
// use semantic::expression::{Constant, Invocation, ValueVariable, KeyVariable};
// use calculation::{nodes::ExpressionField, nodes::where_::execute_where, values::VectorValues};
// use calculation::delta::Delta;
// use calculation::nodes::where_::WhereNode;

mod core;
mod semantic;
mod parsing;
mod building;
mod stdl;
mod target;

fn main() {
    let _some_value = 123;
    println!("Hello World!");

}



// #[test]
// fn should_fail() {
//     unimplemented!();
// }


// #[test]
// fn master() {
//     // let r = Cursor::new(b"id,age\n1,35\n2,36\n3,37");
//     let r = BufReader::new(File::open("flights.1987.tiny.csv").unwrap());
//     let csv = read_csv(r);
//     let values = csv.iter().map(|input| VectorValues::from(input)).collect::<Vec<_>>();
//     let len = values[0].values.len();
//     let keys = vec![VectorValues {values: (0..(len as i32)).collect()}];
    

//     let mut node = WhereNode {
//         id: 0,
//         where_expression: Box::new(Invocation::new("==", vec![
//             Box::new(Constant::new(2)),
//             Box::new(ValueVariable::new(2))
//         ])),
//         keys: vec![
//             ExpressionField::new(0, vec![], Box::new(KeyVariable::new(0)))
//         ],
//         values: vec![]
//     };

//     let input = Delta {
//         is_nexts: [
//             VectorValues {values: vec![0; len]},
//             VectorValues {values: vec![1; len]}
//         ],
//         keys,
//         values: values.into_iter().map(|values| [VectorValues {values: vec![0; len]}, values]).collect::<Vec<_>>()
//     };

//     let output = execute_where(&mut node, &input);
//     assert_eq!(2, output.is_nexts[0].values.len());
// }
