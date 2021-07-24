use juneau_core::RcRefCell;
use walrus::{Module, ModuleConfig};
use crate::{building::{BuildVisitor, BuildVisitorInner, jasm_wasm::JasmWasmBuild}, core::IdProvider, semantic::{Functions, jasm::Jasm}};


pub struct WasmBuildVisitor {
    pub module: Module,
    inner: BuildVisitorInner<JasmWasmBuild>
}

impl WasmBuildVisitor {
    pub fn new(ids:RcRefCell<IdProvider>, functions:Functions<Jasm>) -> WasmBuildVisitor {
        let config = ModuleConfig::new();
        let module = Module::with_config(config);
        let inner = BuildVisitorInner::new(ids, functions);
        WasmBuildVisitor {
            module,
            inner
        }
    }
}


impl BuildVisitor<JasmWasmBuild> for WasmBuildVisitor {
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

