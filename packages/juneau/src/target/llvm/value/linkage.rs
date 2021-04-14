
/// A way of indicating to LLVM how you want a global to interact during linkage.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub enum Linkage {
    /// Default linkage. The global is externally visible and participates in linkage normally.
    External = 0,
    /// Never emitted to the containing module's object file. Used to allow inlining and/or other optimisations to take place, given knowledge of the definition of the global, which is somewhere outside of the module. Otherwise the same as LinkOnceODR. Only allowed on definitions, not declarations.
    AvailableExternally = 1,
    /// Merged with other globals of the same name during linkage. Unreferenced LinkOnce globals may be discarded.
    LinkOnceAny = 2,
    /// Similar to LinkOnceAny, but indicates that it will only be merged with equivalent globals.
    LinkOnceODR = 3,
    /// Same merging semantics as LinkOnceAny. Unlike LinkOnce, unreference globals will not be discarded.
    WeakAny = 5,
    /// Similar to WeakAny, but indicates that it will only be merged with equivalent globals.
    WeakODR = 6,
    /// Only allowed on global array pointers. When two globals with Appending linkage are merged, they are appended together.
    Appending = 7,
    /// Similar to Private, but shows as a local symbol in the object file.
    Internal = 8,
    /// Only directly accessible by objects in the current module. May be renamed as neccessary to avoid collisions, and all references will be updated. Will not show up in the object file's symbol table.
    Private = 9,
    DLLImport = 10,
    DLLExport = 11,
    /// Weak until linked. If not linked, the output symbol is null, instead of undefined.
    ExternalWeak = 12,
    Ghost = 13,
    /// Similar to Weak, but may not have an explicit section, must have a zero initializer, and may not be marked constant. Cannot be used on functions or aliases.
    Common = 14,
    LinkerPrivate = 15,
    LinkerPrivateWeak = 16,
}
impl From<llvm_sys::LLVMLinkage> for Linkage {
    fn from(attr:llvm_sys::LLVMLinkage) -> Linkage {
        unsafe {std::mem::transmute(attr)}
    }
}
impl From<Linkage> for llvm_sys::LLVMLinkage {
    fn from(attr:Linkage) -> Self {
        unsafe {std::mem::transmute(attr)}
    }
}
