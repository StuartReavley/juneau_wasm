use std::ops::Index;
use libc::c_uint;
use crate::core::c::{SubStruct, with_cstr};
use crate::target::llvm::{GlobalValue, BasicBlock, block::BlockIter, Context, FunctionType, GetContext, Parameter, Value};


native_reference! {
    /// A function is a kind of value that can be called and contains blocks of code.
    ///
    /// To get the value of each argument to a function, you can use the index operator.
    /// For example, `&func[0]` is the value that represents the first argument to the function.
    Function = llvm_sys::LLVMValue
}
sub_value! {Function, llvm_sys::core::LLVMIsAFunction, GlobalValue}
to_str!    {Function, llvm_sys::core::LLVMPrintValueToString}


impl Index<usize> for Function {
    type Output = Parameter;
    fn index(&self, index: usize) -> &Parameter {
        self.get_parameter(index).unwrap()
    }
}
unsafe impl SubStruct<Value> for Function {
    fn is(value: &Value) -> bool {
        FunctionType::is(value.get_type())
    }
}

pub struct ParametersIterator<'c> {
    function: &'c Function,
    index: Option<usize>,
    len: usize
}

impl<'c> Iterator for ParametersIterator<'c> {
    type Item = &'c Parameter;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            None => {
                self.len = self.function.parameters_len();
                self.index = Some(0);
                self.function.get_parameter(0)
            },
            Some(index) => {
                self.index = Some(index + 1);
                self.function.get_parameter(index + 1)
            }
        }
    }
}


impl Function {
    /// Add a basic block with the name given to the function and return it.
    pub fn append<'a>(&'a self, name: &str) -> &'a BasicBlock {
        with_cstr(name, |ptr| unsafe {
            llvm_sys::core::LLVMAppendBasicBlockInContext(self.get_context().into(), self.into(), ptr).into()
        })
    }
    pub fn parameters_len(&self) -> usize {
        unsafe {llvm_sys::core::LLVMCountParams(self.into()) as usize}
    }
    pub fn get_parameter<'c>(&'c self, index:usize) -> Option<&'c Parameter> {
        match index >= self.parameters_len() {
            true  => None,
            false => Some(unsafe {llvm_sys::core::LLVMGetParam(self.into(), index as c_uint)}.into())
        }
    }

    pub fn parameters_iter(&self) -> ParametersIterator {
        ParametersIterator {function:self, index: None, len: 0}
    }

    /// Iterate through this function's basic blocks.
    pub fn blocks(&self) -> BlockIter {
        BlockIter::new(self)
    }
    /// Returns the entry block of this function or `None` if there is none.
    pub fn get_entry(&self) -> Option<&BasicBlock> {
        unsafe {std::mem::transmute(llvm_sys::core::LLVMGetEntryBasicBlock(self.into())) }
    }
    /// Returns the function signature representing this function's signature.
    pub fn get_signature(&self) -> &FunctionType {
        unsafe {
            let ty = llvm_sys::core::LLVMTypeOf(self.into());
            llvm_sys::core::LLVMGetElementType(ty).into()
        }
    }

    pub fn verify(&self) -> bool {
        unsafe {
            llvm_sys::analysis::LLVMVerifyFunction(self.into(), llvm_sys::analysis::LLVMVerifierFailureAction::LLVMPrintMessageAction) == 0
        }
    }
}
impl GetContext for Function {
    fn get_context(&self) -> &Context {
        self.get_type().get_context()
    }
}