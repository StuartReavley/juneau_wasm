use crate::{target::llvm::Predicate, semantic::BinaryOperator};


impl From<BinaryOperator> for Predicate {
    fn from(operator:BinaryOperator) -> Self {
        use BinaryOperator::*;
        match operator {
            Equal => Self::Equal,
            NotEqual => Self::NotEqual,
            GreaterThan => Self::GreaterThan,
            GreaterThanOrEqual => Self::GreaterThanOrEqual,
            LessThan => Self::LessThan,
            LessThanOrEqual => Self::LessThanOrEqual,
            _ => panic!("not a predicate")
        }
    }
}