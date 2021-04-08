
// https://webassembly.github.io/spec/core/binary/types.html
pub enum ValueType {
    I32 = 0x7f,
    I64 = 0x7e,
    F32 = 0x7d,
    F64 = 0x7c
}

pub struct ResultType(Vec<ValueType>);