use crate::semantic::{Module, jasm::Jasm};

use super::{EMPTY_ARRAY, FUNCTION_TYPE, Section, Valtype, create_section, empty_array, encode_vector, magicModuleHeader, moduleVersion};


pub fn flatten<T>(values:Vec<Vec<T>>) -> Vec<T> {
    values.into_iter().flatten().collect()
}


pub struct WasmBuildContext {
    buffer:Vec<u8>
}

impl WasmBuildContext {
    pub fn new() -> Self {
        let buffer = Vec::new();
        Self {buffer}
    }

    pub fn write(&mut self, values:&[u8]) {
        self.buffer.extend(values.iter())
    }
}

pub fn run_emitter(jasm:&Module<Jasm>) {
//   export const emitter: Emitter = (ast: TransformedProgram) => {

//     // Function types are vectors of parameters and return types. Currently
//     // WebAssembly only supports single return values
    let print_function_type = flatten(vec![
        vec![FUNCTION_TYPE],
        encode_vector(vec![Valtype::f32 as u8]),
        vec![EMPTY_ARRAY]
    ]);
  
//     // TODO: optimise - some of the procs might have the same type signature
//     const funcTypes = ast.map(proc => [
//       functionType,
//       ...encodeVector(proc.args.map(_ => Valtype.f32)),
//       emptyArray
//     ]);

    let mut context = WasmBuildContext::new();

    // UNCLEAR WE WANT TO WRITE THESE...
    for function in jasm.functions {
        context.write(&[FUNCTION_TYPE]);

        //       ...encodeVector(proc.args.map(_ => Valtype.f32)),

        context.write(&[EMPTY_ARRAY]);
    }
  
//     // the type section is a vector of function types
    let typeSection = create_section(
        Section::Type,
        encode_vector([printFunctionType, ...funcTypes])
    );
  
//     // the function section is a vector of type indices that indicate the type of each function
//     // in the code section
//     const funcSection = createSection(
//       Section.func,
//       encodeVector(ast.map((_, index) => index + 1 /* type index */))
//     );
  
//     // the import section is a vector of imported functions
//     const printFunctionImport = [
//       ...encodeString("env"),
//       ...encodeString("print"),
//       ExportType.func,
//       0x00 // type index
//     ];
  
//     const memoryImport = [
//       ...encodeString("env"),
//       ...encodeString("memory"),
//       ExportType.mem,
//       /* limits https://webassembly.github.io/spec/core/binary/types.html#limits -
//         indicates a min memory size of one page */
//       0x00,
//       0x01
//     ];
  
    let import_section = create_section(
        Section::Import,
        encode_vector(vec![printFunctionImport, memoryImport])
    );
  
//     // the export section is a vector of exported functions
    let export_section = create_section(
        Section::Export,
        encodeVector(vec![
            [
            ...encodeString("run"),
            ExportType.func,
            ast.findIndex(a => a.name === "main") + 1
            ]
        ])
        );
  
//     // the code section contains vectors of functions
//     const codeSection = createSection(
//       Section.code,
//       encodeVector(ast.map(a => codeFromProc(a, ast)))
//     );
  
//     return Uint8Array.from([
//       ...magicModuleHeader,
    context.write(&magicModuleHeader);
//       ...moduleVersion,
    context.write(&moduleVersion);

//       ...typeSection,
//       ...importSection,
    context.write(import_section);
//       ...funcSection,
//       ...exportSection,
//       ...codeSection
//     ]);
//   };

    
}
