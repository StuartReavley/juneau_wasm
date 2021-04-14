use crate::semantic::BinaryOperator;


/// Comparative operations on values.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Predicate {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}
impl Into<llvm_sys::LLVMIntPredicate> for Predicate {
    fn into(self) -> llvm_sys::LLVMIntPredicate {
        use Predicate::*;
        use llvm_sys::LLVMIntPredicate::*;
        match self {
            Equal => LLVMIntEQ,
            NotEqual => LLVMIntNE,
            GreaterThan => LLVMIntSGT,
            GreaterThanOrEqual => LLVMIntSGE,
            LessThan => LLVMIntSLT,
            LessThanOrEqual => LLVMIntSLE,
        }
    }
}
impl Into<llvm_sys::LLVMRealPredicate> for Predicate {
    fn into(self) -> llvm_sys::LLVMRealPredicate {
        use Predicate::*;
        use llvm_sys::LLVMRealPredicate::*;
        match self {
            Equal => LLVMRealOEQ,
            NotEqual => LLVMRealONE,
            GreaterThan => LLVMRealOGT,
            GreaterThanOrEqual => LLVMRealOGE,
            LessThan => LLVMRealOLT,
            LessThanOrEqual => LLVMRealOLE,
        }
    }
}
