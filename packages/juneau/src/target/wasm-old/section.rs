

// https://webassembly.github.io/spec/core/binary/modules.html#sections
pub enum Section {
    Custom = 0,
    Type = 1,
    Import = 2,
    Func = 3,
    Table = 4,
    Memory = 5,
    Global = 6,
    Export = 7,
    Start = 8,
    Element = 9,
    Code = 10,
    Data = 11
}


//   // https://webassembly.github.io/spec/core/binary/modules.html#sections
//   // sections are encoded by their type followed by their vector contents
//   const createSection = (sectionType: Section, data: any[]) => [
//     sectionType,
//     ...encodeVector(data)
//   ];

pub fn create_section(section_type:Section, data:&[u8]) -> Vec<u8> {
    let mut values = Vec::new();
    values.push(section_type as u8);
    values.extend(data);
    values
}