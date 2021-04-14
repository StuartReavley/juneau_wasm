use llvm_sys::prelude::{LLVMModuleRef, LLVMPassManagerRef};
use super::{Function, Module, native_reference};



native_reference! {
    /// Represents a single compilation unit of code.
    ///
    /// This is attached to the lifetime of the context that constructs it, but is owned by the `CSemiBox`.
    FunctionPassManager = llvm_sys::LLVMPassManager
}

crate::dispose!(FunctionPassManager, llvm_sys::LLVMPassManager, llvm_sys::core::LLVMDisposePassManager);


impl FunctionPassManager {
    pub fn new<'c>(module:&'c Module) -> &'c mut FunctionPassManager {
        unsafe {llvm_sys::core::LLVMCreateFunctionPassManagerForModule(module.into()).into()}
    }

    pub fn initialize(&mut self) -> bool {
        unsafe {llvm_sys::core::LLVMInitializeFunctionPassManager(self.into()) > 0}
    }

    pub fn run<'g, 'c>(&'g self, function: &'c Function) -> bool {
        unsafe {llvm_sys::core::LLVMRunFunctionPassManager(self.into(), function.into()) > 0}
    }

    pub fn finalize(&mut self) -> bool {
        unsafe {llvm_sys::core::LLVMFinalizeFunctionPassManager(self.into()) > 0}
    }

    #[allow(non_snake_case)]
    pub fn add_aggressive_DCE_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddAggressiveDCEPass(self.into())}
    }

    pub fn add_alignment_from_assumptions_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddAlignmentFromAssumptionsPass(self.into())}
    }

    #[allow(non_snake_case)]
    pub fn add_CFG_simplification_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddCFGSimplificationPass(self.into())}
    }

    pub fn add_dead_store_elimination_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddDeadStoreEliminationPass(self.into())}
    }

    pub fn add_scalarizer_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddScalarizerPass(self.into())}
    }

    pub fn add_merged_load_store_motion_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddMergedLoadStoreMotionPass(self.into())}
    }

    #[allow(non_snake_case)]
    pub fn add_GVN_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddGVNPass(self.into())}
    }

    pub fn add_ind_var_simplify_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddIndVarSimplifyPass(self.into())}
    }

    pub fn add_instruction_combining_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddInstructionCombiningPass(self.into())}
    }

    pub fn add_jump_threading_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddJumpThreadingPass(self.into())}
    }

    #[allow(non_snake_case)]
    pub fn add_LICM_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddLICMPass(self.into())}
    }

    pub fn add_loop_deletion_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddLoopDeletionPass(self.into())}
    }

    pub fn add_loop_idiom_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddLoopIdiomPass(self.into())}
    }

    pub fn add_loop_rotate_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddLoopRotatePass(self.into())}
    }

    pub fn add_loop_reroll_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddLoopRerollPass(self.into())}
    }

    pub fn add_loop_unroll_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddLoopUnrollPass(self.into())}
    }

    pub fn add_loop_unswitch_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddLoopUnswitchPass(self.into())}
    }

    pub fn add_mem_cpy_opt_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddMemCpyOptPass(self.into())}
    }

    pub fn add_partially_inline_lib_calls_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddPartiallyInlineLibCallsPass(self.into())}
    }

    pub fn add_lower_switch_pass(&mut self) {
        unsafe {llvm_sys::transforms::util::LLVMAddLowerSwitchPass(self.into())}
    }

    pub fn add_promote_memory_to_register_pass(&mut self) {
        unsafe {llvm_sys::transforms::util::LLVMAddPromoteMemoryToRegisterPass(self.into())}
    }

    pub fn add_reassociate_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddReassociatePass(self.into())}
    }

    #[allow(non_snake_case)]
    pub fn add_SCCP_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddSCCPPass(self.into())}
    }

    pub fn add_scalar_repl_aggregates_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddScalarReplAggregatesPass(self.into())}
    }

    #[allow(non_snake_case)]
    pub fn add_scalar_repl_aggregates_pass_SSA(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddScalarReplAggregatesPassSSA(self.into())}
    }

    pub fn add_scalar_repl_aggregates_pass_with_threshold(&mut self, threshold: std::os::raw::c_int) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddScalarReplAggregatesPassWithThreshold(self.into(), threshold)}
    }

    pub fn add_simplify_lib_calls_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddSimplifyLibCallsPass(self.into())}
    }

    pub fn add_tail_call_elimination_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddTailCallEliminationPass(self.into())}
    }

    pub fn add_constant_propagation_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddConstantPropagationPass(self.into())}
    }

    pub fn add_demote_memory_to_register_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddDemoteMemoryToRegisterPass(self.into())}
    }

    pub fn add_verifier_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddVerifierPass(self.into())}
    }

    pub fn add_correlated_value_propagation_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddCorrelatedValuePropagationPass(self.into())}
    }

    #[allow(non_snake_case)]
    pub fn add_early_CSE_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddEarlyCSEPass(self.into())}
    }

    pub fn add_lower_expect_intrinsic_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddLowerExpectIntrinsicPass(self.into())}
    }

    pub fn add_type_based_alias_analysis_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddTypeBasedAliasAnalysisPass(self.into())}
    }

    #[allow(non_snake_case)]
    pub fn add_scoped_no_alias_AA_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddScopedNoAliasAAPass(self.into())}
    }

    pub fn add_basic_alias_analysis_pass(&mut self) {
        unsafe {llvm_sys::transforms::scalar::LLVMAddBasicAliasAnalysisPass(self.into())}
    }
}


