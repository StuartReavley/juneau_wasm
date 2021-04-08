use crate::semantic::jasm::Jasm;
use crate::parsing::SymbolSemantic;
use super::Overload;


impl SymbolSemantic for Jasm {
    type Overload = Overload;
}