use crate::{building::jasm_wasm::visitor::WasmBuilderVisitor, core::Visitor, semantic::jasm::JasmValue};


impl Visitor<&JasmValue, ()> for WasmBuilderVisitor {
    fn visit(&mut self, value:&JasmValue) -> () {
        let mut func_body = self.function_builder.get_mut().func_body();
        use JasmValue::*;
        match value {
            Null => func_body.i32_const(0),
            Bool(val) => func_body.i32_const(if *val {1} else {0}),
            U8(val) => func_body.i32_const(*val as i32),
            I64(val) => func_body.i64_const(*val),
            U64(val) => func_body.i64_const(*val as i64),
            F64(val) => func_body.f64_const(*val),
            String(val) => func_body.i32_const(0)
        };
    }
}