use std::rc::Rc;
use crate::core::Visitor;
use crate::building::jasm_llvm::{JasmBuildVisitor, JasmModuleBuildVisitor};
use crate::building::ModuleBuildVisitor;
use crate::semantic::{Functions, Module, jasm::Jasm};
use crate::target::llvm::{self, Llvm, FunctionPassManager};


impl<'l> Visitor<&Module<Jasm>, (&'l llvm::Module, Module<Llvm<'l>>)> for JasmBuildVisitor<'l> {
    fn visit(&mut self, module:&Module<Jasm>) -> (&'l llvm::Module, Module<Llvm<'l>>) {

        let llvm_module = llvm::Module::new(self.into(), "main_module");
        let pass_manager = match self.is_optimized {
            false => None,
            true => {
                let pass_manager = FunctionPassManager::new(llvm_module);
                pass_manager.add_promote_memory_to_register_pass();
                pass_manager.add_basic_alias_analysis_pass();
                pass_manager.add_instruction_combining_pass();
                pass_manager.add_reassociate_pass();
                pass_manager.add_GVN_pass();
                pass_manager.add_CFG_simplification_pass();
                pass_manager.add_aggressive_DCE_pass();
                pass_manager.add_CFG_simplification_pass();
                pass_manager.initialize();
                Some(&*pass_manager)
            }
        };
    
        let functions = Functions::union(&self.functions, &module.functions);
        let JasmBuildVisitor {ids, context, ..} = self.to_owned();
        let mut visitor = JasmModuleBuildVisitor::new(ids, functions, context, llvm_module, pass_manager);
    
        let module = visitor.visit(module);
        // let main = self.get_main();
        // let main_id = main.id;
        // let function = main.visit(&mut context).unwrap();
        // let module = Module::new_with_functions(main_id, Rc::new(function), context.get_module_functions());
        llvm_module.verify().unwrap();
        (llvm_module, module)
    }
}
