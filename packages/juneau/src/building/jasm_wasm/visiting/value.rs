use crate::{building::jasm_wasm::visitor::WasmImplementationVisitor, core::Visitor, semantic::jasm::JasmValue};


impl<'b> Visitor<&JasmValue, ()> for WasmImplementationVisitor<'b> {
    fn visit(&mut self, value:&JasmValue) -> () {
        use JasmValue::*;
        match value {
            Null => self.builder.i32_const(0),
            Bool(val) => self.builder.i32_const(if *val {1} else {0}),
            U8(val) => self.builder.i32_const(*val as i32),
            I64(val) => self.builder.i64_const(*val),
            U64(val) => self.builder.i64_const(*val as i64),
            F64(val) => self.builder.f64_const(*val),
            String(val) => self.builder.i32_const(0)
        };
    }
}