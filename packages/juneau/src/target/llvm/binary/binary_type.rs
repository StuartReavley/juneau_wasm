

#[derive(Debug, Copy, Clone)]
pub enum BinaryType {
    /// Archive file
    Archive,
    /// Mach-O Universal Binary file
    MachOUniversalBinary,
    /// COFF Import file
    COFFImportFile,
    /// LLVM IR
    IR,
    /// Windows resource (.res) file
    WinRes,
    /// COFF Object file
    COFF,
    /// ELF 32-bit, little endian
    ELF32L,
    /// ELF 32-bit, big endian
    ELF32B,
    /// ELF 64-bit, little endian
    ELF64L,
    /// ELF 64-bit, big endian
    ELF64B,
    /// MachO 32-bit, little endian
    MachO32L,
    /// MachO 32-bit, big endian
    MachO32B,
    /// MachO 64-bit, little endian
    MachO64L,

    /// MachO 64-bit, big endian
    MachO64B,
    /// Web assembly
    Wasm,
}


impl From<llvm_sys::object::LLVMBinaryType> for BinaryType {
    fn from(ty:llvm_sys::object::LLVMBinaryType) -> Self {
        use llvm_sys::object::LLVMBinaryType::*;
        use BinaryType::*;
        match ty {
            LLVMBinaryTypeArchive => Archive,
            LLVMBinaryTypeMachOUniversalBinary => MachOUniversalBinary,
            LLVMBinaryTypeCOFFImportFile => COFFImportFile,
            LLVMBinaryTypeIR => IR,
            LLVMBinaryTypeWinRes => WinRes,
            LLVMBinaryTypeCOFF => COFF,
            LLVMBinaryTypeELF32L => ELF32L,
            LLVMBinaryTypeELF32B => ELF32B,
            LLVMBinaryTypeELF64L => ELF64L,
            LLVMBinaryTypeELF64B => ELF64B,
            LLVMBinaryTypeMachO32L => MachO32L,
            LLVMBinaryTypeMachO32B => MachO32B,
            LLVMBinaryTypeMachO64L => MachO64L,
            LLVMBinaryTypeMachO64B => MachO64B,
            LLVMBinaryTypeWasm => Wasm,
        }
    }
}
