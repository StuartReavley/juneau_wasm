use std::rc::Rc;
use std::collections::HashMap;
use crate::core::{Id, IdContext};
use crate::semantic::{BinaryOperator, UnaryOperator, Function, Functions, Parameter, Implementation, Variable};
use crate::semantic::jasm::{Jasm, NumberType, JasmType, JasmPrimitiveImplementation};
use crate::parsing::Symbols;

mod new;
pub use new::*;


pub fn new_jasm_stdl(ids:&mut impl IdContext, other_functions:&Functions<Jasm>, symbols:&[Variable<Jasm>]) -> (Functions<Jasm>, Symbols<Jasm>) {
    let mut functions = Functions::new();
    let number_types = {
        use NumberType::*;
        use JasmType::*;
        [(UnsignedInteger, U64), (SignedInteger, I64), (Float, F64)]
    };

    let binary_operators = {use BinaryOperator::*;[Add, Subtract, Multiply, Divide]};
    let unary_operators = {use UnaryOperator::*;[Negate]};
    for (number_type, typ) in &number_types {
        for operator in &binary_operators {
            functions.insert_function(new_binary(ids, *operator, *number_type, typ, typ));
        }
        for operator in &unary_operators {
            functions.insert_function(new_unary(ids, *operator, *number_type, typ, typ));
        }
    }

    let predicates = {use BinaryOperator::*;[Equal, NotEqual, GreaterThan, GreaterThanOrEqual, LessThan, LessThanOrEqual]};
    let predicate_types = {
        use NumberType::*;
        use JasmType::*;
        [(UnsignedInteger, U8), (UnsignedInteger, U64), (SignedInteger, I64), (Float, F64), (NumberType::Bool, JasmType::Bool)]
    };
    use JasmType::*;
    for (number_type, typ) in &predicate_types {
        for operator in &predicates {
            functions.insert_function(new_binary(ids, *operator, *number_type, typ, &Bool));
        }
    }

    let integer_binary_operators = {use BinaryOperator::*;[ShiftLeft, ShiftRight, And, Or, Xor]};
    let integer_unary_operators = {use UnaryOperator::*;[Not]};
    let integer_types = {
        use NumberType::*;
        use JasmType::*;
        [(UnsignedInteger, U8), (UnsignedInteger, U64), (SignedInteger, I64)]
    };

    for (number_type, typ) in &integer_types {
        for operator in &integer_binary_operators {
            functions.insert_function(new_binary(ids, *operator, *number_type, typ, typ));
        }
        for operator in &integer_unary_operators {
            functions.insert_function(new_unary(ids, *operator, *number_type, typ, typ));
        }
    }

    let bool_binary_operators = {use BinaryOperator::*;[And, Or, Xor]};
    let bool_unary_operators = {use UnaryOperator::*;[Not]};

    for operator in &bool_binary_operators {
        functions.insert_function(new_binary(ids, *operator, NumberType::Bool, &Bool, &Bool));
    }
    for operator in &bool_unary_operators {
        functions.insert_function(new_unary(ids, *operator, NumberType::Bool, &Bool, &Bool));
    }

    functions.union_self(other_functions);
    let symbols = functions
        .iter()
        .map(|f|Variable::new(f.id, &f.name, &f.get_type().into()))
        .chain(symbols.to_owned().into_iter())
        .collect();

    (functions, symbols)
}


fn new_binary(ids:&mut impl IdContext, operator:BinaryOperator, number_type:NumberType, parameter:&JasmType, retrn:&JasmType) -> Rc<Function<Jasm>> {
    Function::new(
        ids.new_id(),
        operator.to_string().into(),
        vec![
            Parameter::new(ids.new_id(), &"a".into(), parameter),
            Parameter::new(ids.new_id(), &"b".into(), parameter)
        ],
        Implementation::Primitive(retrn.to_owned(), JasmPrimitiveImplementation::Binary(number_type, operator).into()))
}


fn new_unary(ids:&mut impl IdContext, operator:UnaryOperator, number_type:NumberType, parameter:&JasmType, retrn:&JasmType) -> Rc<Function<Jasm>> {
    Function::new(
        ids.new_id(),
        operator.to_string().into(),
        vec![
            Parameter::new(ids.new_id(), &"a".into(), parameter),
        ],
        Implementation::Primitive(retrn.to_owned(), JasmPrimitiveImplementation::Unary(number_type, operator).into()))
}