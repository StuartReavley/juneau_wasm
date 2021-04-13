use crate::core::Visitor;
use crate::semantic::jasm::{JasmType, JasmValue};
use crate::target::llvm::{self, ArrayType, Linkage, Constant};
use crate::building::jasm_llvm::JasmModuleBuildVisitor;


impl<'l> Visitor<&JasmValue, &'l llvm::Value> for JasmModuleBuildVisitor<'l> {
    fn visit(&mut self, value:&JasmValue) -> &'l llvm::Value {
        use JasmValue::*;
        match value {
            Null => u8::get_null_ptr(self.into()), // note this is a u8* null
            &Bool(value) => self.visit(value),
            &U8(value) => self.visit(value),
            &I64(value) => self.visit(value),
            &U64(value) => self.visit(value),
            &F64(value) => self.visit(value),
            String(value) => {
                let typ = ArrayType::new(self.visit(&JasmType::U8), value.len());
                let global = self.module.add_global("string", typ);
                global.set_linkage(Linkage::Internal);
                global.set_constant(true);
                global.set_initializer(llvm::Value::new_string(self.into(), value, true));
                global
            }
        }
    }
}

impl<'l, T:Constant> Visitor<T, &'l llvm::Value> for JasmModuleBuildVisitor<'l> {
    fn visit(&mut self, value:T) -> &'l llvm::Value {
        value.as_llvm(self.into())
    }
}