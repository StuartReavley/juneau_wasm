use crate::core::FromRef;
use crate::semantic::{Name, Variable, Type};
use crate::semantic::jasm::{Jasm, JasmType};


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Overload {
    pub name: Name,
    pub parameters: Option<Vec<JasmType>>
}


impl Overload {
    pub fn new(name:&Name, parameters:Option<Vec<JasmType>>) -> Self {
        let name = name.to_owned();
        Self {name, parameters}
    }

    pub fn new_variable(name:&Name) -> Self {
        Self::new(name, None)
    }
}

impl From<(&Name, Option<Vec<JasmType>>)> for Overload {
    fn from((name, parameters): (&Name, Option<Vec<JasmType>>)) -> Self {
        Self::new(name, parameters)
    }
}

// impl From<(&String, Option<Vec<JasmType>>)> for Overload {
//     fn from((name, parameters): (&String, Option<Vec<JasmType>>)) -> Self {
//         Self::new(name, parameters)
//     }
// }

impl FromRef<Variable<Jasm>> for Overload {
    fn from_ref(value:&Variable<Jasm>) -> Self {
        let parameters = value.typ.as_function_type().map(|t|t.parameters.to_owned());
        Self::new(&value.name, parameters)
    }
}
