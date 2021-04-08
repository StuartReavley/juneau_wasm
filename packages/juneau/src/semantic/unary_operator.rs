use std::{fmt::{Display, Formatter}, str::FromStr};
use juneau_core::Error;


#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum UnaryOperator {
    Negate,
    Not
}


impl FromStr for UnaryOperator {
    type Err = Error;

    fn from_str(value:&str) -> Result<Self, Self::Err> {
        use UnaryOperator::*;
        match value {
            "-" => Ok(Negate),
            "!" => Ok(Not),
            _ => Err(Error::new("invalid"))
        }
    }
}


impl Display for UnaryOperator {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        use UnaryOperator::*;
        let text = match self {
            Negate => "-",
            Not => "!",
        };
        write!(f, "{}", text)
    }
}