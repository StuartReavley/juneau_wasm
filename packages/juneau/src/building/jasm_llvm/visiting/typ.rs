use crate::core::{Visitor, Visits};
use crate::semantic::FunctionType;
use crate::semantic::jasm::{Jasm, JasmType};
use crate::target::llvm::{self, Type, PointerType, ArrayType, Constant};
use crate::building::jasm_llvm::JasmModuleBuildVisitor;


impl<'l> Visitor<&JasmType, &'l Type> for JasmModuleBuildVisitor<'l> {
    fn visit(&mut self, typ:&JasmType) -> &'l Type {
        use JasmType::*;
        match typ {
            Bool => bool::get_type(self.into()),
            U8 => u8::get_type(self.into()),
            I64 => i64::get_type(self.into()),
            U64 => u64::get_type(self.into()),
            F64 => f64::get_type(self.into()),
            String => todo!(),
            Void => <()>::get_type(self.into()),
            Pointer(typ) => PointerType::new(self.visit(typ.as_ref())),
            Function(typ) => self.visit(typ),
            Struct(id, name) => self.get_type(id, name),
            Array(typ) => PointerType::new(self.visit(typ.as_ref())) // hmmm array type requires fixed length...
        }
    }
}

impl<'l> Visitor<&FunctionType<Jasm>, &'l llvm::FunctionType> for JasmModuleBuildVisitor<'l> {
    fn visit(&mut self, typ:&FunctionType<Jasm>) -> &'l llvm::FunctionType {
        let FunctionType {parameters, retrn} = typ;
        let parameters = self.visits(parameters);
        let retrn:&'l llvm::Type = self.visit(retrn.as_ref());
        llvm::FunctionType::new(&parameters, retrn)
    }
}
