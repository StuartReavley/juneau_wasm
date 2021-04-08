use std::fmt::{Display, Formatter};
use std::str::FromStr;
use juneau_core::Error;


#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    ShiftLeft,
    ShiftRight,
    And,
    Or,
    Xor,

    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

impl BinaryOperator {
    pub fn get_name(&self) -> &'static str {
        use BinaryOperator::*;
        match self {
            Add => "add",
            Subtract => "subtract",
            Multiply => "multiply",
            Divide => "divide",
            Modulo => "modulo",
            ShiftLeft => "shift_left",
            ShiftRight => "shift_right",
            And => "and",
            Or => "or",
            Xor => "xor",
            Equal => "equal",
            NotEqual => "not_equal",
            GreaterThan => "greater_than",
            GreaterThanOrEqual => "greater_than_or_equal",
            LessThan => "less_than",
            LessThanOrEqual => "less_than_or_equal"
        }
    }
}


impl FromStr for BinaryOperator {
    type Err = Error;

    fn from_str(value:&str) -> Result<Self, Self::Err> {
        use BinaryOperator::*;
        match value {
            "+" => Ok(Add),
            "-" => Ok(Subtract),
            "*" => Ok(Multiply),
            "/" => Ok(Divide),
            "%" => Ok(Modulo),

            "==" => Ok(Equal),
            "!=" => Ok(NotEqual),
            ">" => Ok(GreaterThan),
            ">=" => Ok(GreaterThanOrEqual),
            "<" => Ok(LessThan),
            "<=" => Ok(LessThanOrEqual),

            "<<" => Ok(ShiftLeft),
            ">>" => Ok(ShiftRight),
            "and" => Ok(And),
            "or" => Ok(Or),
            "xor" => Ok(Xor),

            _ => Err(Error::new("invalid"))
        }
    }
}


impl Display for BinaryOperator {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        use BinaryOperator::*;
        let text = match self {
            Add => "+",
            Subtract => "-",
            Multiply => "*",
            Divide => "/",
            Modulo => "%",

            Equal => "==",
            NotEqual => "!=",
            GreaterThan => ">",
            GreaterThanOrEqual => ">=",
            LessThan => "<",
            LessThanOrEqual => "<=",

            ShiftLeft => "<<",
            ShiftRight => ">>",
            And => "and",
            Or => "or",
            Xor => "xor"
        };
        write!(f, "{}", text)
    }
}