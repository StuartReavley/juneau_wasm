use juneau_core::RcRefCell;
use walrus::InstrSeqBuilder;

use crate::{building::{BuildVisitor, BuildVisitorInner, jasm_wasm::JasmWasmBuild}, core::IdProvider, semantic::{Functions, jasm::Jasm}};






pub struct WasmImplementationVisitor<'b> {
    pub builder: InstrSeqBuilder<'b>,
    inner: BuildVisitorInner<JasmWasmBuild>
}

impl<'b> WasmImplementationVisitor<'b> {
    pub fn new(ids:RcRefCell<IdProvider>, functions:Functions<Jasm>, builder:InstrSeqBuilder<'b>) -> WasmImplementationVisitor<'b> {
        let inner = BuildVisitorInner::new(ids, functions);
        WasmImplementationVisitor {
            builder,
            inner
        }
    }
}


impl<'b> BuildVisitor<JasmWasmBuild> for WasmImplementationVisitor<'b> {
    fn get_inner(&self) -> &BuildVisitorInner<JasmWasmBuild> {
        &self.inner
    }
    fn get_inner_mut(&mut self) -> &mut BuildVisitorInner<JasmWasmBuild> {
        &mut self.inner
    }
    fn move_inner(self) -> BuildVisitorInner<JasmWasmBuild> {
        self.inner
    }
}

