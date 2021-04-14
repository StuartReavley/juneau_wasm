use libc::{c_char, c_uint, c_void};
use std::{env, ffi::{CStr, CString}, io::{Error, ErrorKind}, path::Path, process::Child, process::Command};
use std::marker::PhantomData;
use crate::core::c::{CBox, with_cstr, ptr_to_null, to_str};
use crate::target::llvm::{Context, Type, Function, MemoryBuffer, Value, GlobalValue, Alias};
use super::{AddressSpace, GlobalVariable};



native_reference! {
    /// Represents a single compilation unit of code.
    ///
    /// This is attached to the lifetime of the context that constructs it, but is owned by the `CSemiBox`.
    Module = llvm_sys::LLVMModule
}

get_context!(Module, llvm_sys::core::LLVMGetModuleContext   );
to_str!     (Module, llvm_sys::core::LLVMPrintModuleToString);


impl Module {
    /// Create a new module in the context given with the name given.
    ///
    /// The lifetime of the module will match the lifetime of the context
    /// you instance it in because the context contains it.
    ///
    /// ```rust
    /// use llvm::*;
    /// let context = Context::new();
    /// let context = context.as_semi();
    /// let module = Module::new("name", &context);
    /// println!("{:?}", module)
    /// ```
    pub fn new<'l>(context:&'l Context, name:&str) -> &'l Module {
        let c_name = CString::new(name).unwrap();
        unsafe {llvm_sys::core::LLVMModuleCreateWithNameInContext(c_name.as_ptr(), context.into()).into() }
    }
    /// Add a global to the module with the given type and name.
    pub fn add_global<'l>(&'l self, name:&str, typ:&'l Type) -> &'l GlobalVariable {
        with_cstr(name, |ptr| unsafe {
            llvm_sys::core::LLVMAddGlobal(self.into(), typ.into(), ptr).into()
        })
    }
    /// Add a global variable to the module with the given type, name and initial value.
    pub fn add_global_in_address_space<'l>(&'l self, name:&str, typ: &'l Type, address:AddressSpace) -> &'l GlobalVariable {
        with_cstr(name, |ptr| unsafe {
            llvm_sys::core::LLVMAddGlobalInAddressSpace(self.into(), typ.into(), ptr, address as u32).into()
        })
    }
    /// Add a global variable to the module with the given type, name and initial value.
    pub fn add_global_variable<'l>(&'l self, name:&str, value:&'l Value) -> &'l GlobalVariable {
        let global = self.add_global(name, value.get_type());
        global.set_initializer(value);
        global
    }
    /// Add a global to the module with the given type and name.
    pub fn add_global_alias<'l>(&'l self, name:&str, value:&'l GlobalValue) -> &'l Alias {
        with_cstr(name, |ptr| unsafe {
            llvm_sys::core::LLVMAddAlias(self.into(), value.get_type().into(), value.into(), ptr).into()
        })
    }
    /// Get the global with the name given, or `None` if no global with that name exists.
    pub fn get_global<'l>(&'l self, name: &str) -> Option<&'l GlobalValue> {
        with_cstr(name, |ptr| unsafe {
            let ptr = llvm_sys::core::LLVMGetNamedGlobal(self.into(), ptr);
            ptr_to_null(ptr)
        })
    }
    /// Parse this bitcode file into a module, or return an error string.
    pub fn parse_bitcode<'l>(context:&'l Context, path:&str) -> Result<&'l Module, CBox<str>> {
        unsafe {
            let mut out = std::mem::MaybeUninit::zeroed().assume_init();
            let buf = MemoryBuffer::new_from_file(path)?;
            if llvm_sys::bit_reader::LLVMParseBitcodeInContext2(context.into(), buf.as_ptr(), &mut out) == 1 {
                Err(CBox::from("???"))
            } else {
                Ok(out.into())
            }
        }
    }
    /// Write this module's bitcode to the path given.
    pub fn write_bitcode<'l>(&'l self, path:&str) -> std::io::Result<()> {
        with_cstr(path, |cpath| unsafe {
            if llvm_sys::bit_writer::LLVMWriteBitcodeToFile(self.into(), cpath) != 0 {
                Err(Error::new(
                    ErrorKind::Other,
                    &format!("could not write to {}", path) as &str,
                ))
            } else {
                Ok(())
            }
        })
    }
    /// Parse IR assembly unto a module, or return an error string.
    pub fn parse_ir_from_str<'l>(context:&'l Context, s:&str) -> Result<&'l Module, CBox<str>> {
        unsafe {
            let mut out = std::mem::MaybeUninit::zeroed().assume_init();
            let mut err = std::mem::MaybeUninit::zeroed().assume_init();
            let buf = MemoryBuffer::new_from_str(s, None)?;
            if llvm_sys::ir_reader::LLVMParseIRInContext(context.into(), buf.as_ptr(), &mut out, &mut err)
                == 1
            {
                Err(CBox::new(err))
            } else {
                Ok(out.into())
            }
        }
    }

    pub fn add_external_function<'l>(&'l self, name:&str, typ:&'l Type, function:*mut c_void) -> &'l Function {
        let c_name = CString::new(name).unwrap();
        unsafe {llvm_sys::support::LLVMAddSymbol(c_name.as_ptr(), function)}
        self.add_function(name, typ)
    }

    /// Add a function to the module with the name given.
    pub fn add_function<'l>(&'l self, name:&str, typ:&'l Type) -> &'l mut Function {
        let c_name = CString::new(name).unwrap();
        unsafe { llvm_sys::core::LLVMAddFunction(self.into(), c_name.as_ptr(), typ.into()) }.into()
    }

 
    /// Returns the function with the name given, or `None` if no function with that name exists.
    pub fn get_function<'l>(&'l self, name:&str) -> Option<&'l Function> {
        let c_name = CString::new(name).unwrap();
        unsafe {
            let ty = llvm_sys::core::LLVMGetNamedFunction(self.into(), c_name.as_ptr());
            ptr_to_null(ty)
        }
    }
    /// Returns the type with the name given, or `None`` if no type with that name exists.
    pub fn get_type<'l>(&'l self, name:&str) -> Option<&'l Type> {
        let c_name = CString::new(name).unwrap();
        unsafe {
            let ty = llvm_sys::core::LLVMGetTypeByName(self.into(), c_name.as_ptr());
            ptr_to_null(ty)
        }
    }
    /// Clone this module.
    pub fn clone<'l>(&'l self) -> &'l Module {
        unsafe {llvm_sys::core::LLVMCloneModule(self.into())}.into()
    }

    /// Optimize this module with the given optimization level and size level.
    ///
    /// This runs passes depending on the levels given.
    pub fn optimize<'l>(&'l self, opt_level: usize, size_level: usize) {
        unsafe {
            let builder = llvm_sys::transforms::pass_manager_builder::LLVMPassManagerBuilderCreate();
            llvm_sys::transforms::pass_manager_builder::LLVMPassManagerBuilderSetOptLevel(builder, opt_level as c_uint);
            llvm_sys::transforms::pass_manager_builder::LLVMPassManagerBuilderSetSizeLevel(builder, size_level as c_uint);

            let pass_manager = llvm_sys::core::LLVMCreatePassManager();

            if opt_level > 1 {
                llvm_sys::transforms::pass_manager_builder::LLVMPassManagerBuilderUseInlinerWithThreshold(
                    builder,
                    size_level as c_uint,
                );
            } else {
                // otherwise, we will add the builder to the top of the list of passes.
                // This is not exactly what llvm-opt does, but it is pretty close
                llvm_sys::transforms::ipo::LLVMAddAlwaysInlinerPass(pass_manager);
            }

            llvm_sys::transforms::pass_manager_builder::LLVMPassManagerBuilderPopulateModulePassManager(builder, pass_manager);
            llvm_sys::transforms::pass_manager_builder::LLVMPassManagerBuilderDispose(builder);
            llvm_sys::core::LLVMRunPassManager(pass_manager, self.into());
        }
    }

    /// Returns the target data of this module represented as a string
    pub fn get_target<'l>(&'l self) -> &str {
        unsafe {
            let target = llvm_sys::core::LLVMGetTarget(self.into());
            to_str(target as *mut c_char)
        }
    }

    /// Set the target data of this module to the target data string given.
    pub fn set_target<'l>(&'l self, target:&str) {
        let c_target = CString::new(target).unwrap();
        unsafe { llvm_sys::core::LLVMSetTarget(self.into(), c_target.as_ptr()) }
    }

    /// Verify that the module is safe to run, returning a string detailing the error
    /// when an error occurs.
    pub fn verify<'l>(&'l self) -> Result<(), CBox<str>> {
        unsafe {
            let mut error = std::mem::MaybeUninit::zeroed().assume_init();
            let action = llvm_sys::analysis::LLVMVerifierFailureAction::LLVMReturnStatusAction;
            if llvm_sys::analysis::LLVMVerifyModule(self.into(), action, &mut error) == 1 {
                Err(CBox::new(error))
            } else {
                Ok(())
            }
        }
    }

    /// Compile the module into an object file at the given location.
    ///
    /// Note that this uses the LLVM tool `llc` to do this, which may or may not be
    /// installed on the user's machine.
    pub fn compile<'l>(&'l self, path:&Path, opt_level:usize) -> std::io::Result<Child> {
        let dir = env::temp_dir();
        let path = path.to_str().unwrap();
        let mod_path = dir.join("module.bc");
        let mod_path = mod_path.to_str().unwrap();
        self.write_bitcode(mod_path)?;
        Command::new("llc")
            .arg(&format!("-O={}", opt_level))
            .arg("-filetype=obj")
            .arg("-o")
            .arg(path)
            .arg(mod_path)
            .spawn()
    }

    /// Link a module into this module
    /// This *does not* destroy the source module.
    pub fn link<'l>(&'l self, src:&'l Module) -> Result<(), ()> {
        unsafe {
            let dest = self.into();
            let src = src.into();
            if llvm_sys::linker::LLVMLinkModules2(dest, src) == 1 {
                Err(())
            } else {
                Ok(())
            }
        }
    }

    // Dump the module as IR to stdout.
    pub fn dump<'l>(&'l self) {
        unsafe {llvm_sys::core::LLVMDumpModule(self.into())}
    }
}

impl std::fmt::Display for Module {
    fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let ptr = llvm_sys::core::LLVMPrintModuleToString(self.into());
            let c_str = CStr::from_ptr(ptr);
            f.write_str(c_str.to_str().unwrap())?;
            Ok(())
        }
    }
}


impl<'l> IntoIterator for &'l Module {
    type Item = &'l Function;
    type IntoIter = FunctionIterator<'l>;

    /// Iterate through the functions in the module
    fn into_iter(self) -> FunctionIterator<'l> {
        FunctionIterator::new(self)
    }
}


#[derive(Copy, Clone)]
/// An iterator through the functions contained in a module.
pub struct FunctionIterator<'l> (Option<&'l Function>);

impl<'l> FunctionIterator<'l> {
    pub fn new(module:&'l Module) -> Self {
        Self(unsafe{ptr_to_null(llvm_sys::core::LLVMGetFirstFunction(module.into()))})
    }
} 

impl<'l> Iterator for FunctionIterator<'l> {
    type Item = &'l Function;

    fn next(&mut self) -> Option<&'l Function> {
        let cursor = self.0.and_then(|f|unsafe {ptr_to_null(llvm_sys::core::LLVMGetNextFunction(f.into()))});
        self.0 = cursor;
        cursor
    }
}
