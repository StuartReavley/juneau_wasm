use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use juneau_core::assert_eq_debug;
use crate::parsing::jasm::{parse_jasm_statement, parse_jasm_expression};
use super::new_default_context;


#[test]
fn parse_struct() {
    let source = "
struct String {
    ptr:      *u8,
    len:      u64,
    capacity: u64
}";

    let mut parse_context = new_default_context();
    let jasm = parse_jasm_statement(&mut parse_context, source);
    assert_eq_debug!(jasm, r#"StructDefinition(Struct { id: 1004, name: "String", parameters: [Parameter { id: 1001, name: "ptr", typ: Pointer(U8) }, Parameter { id: 1002, name: "len", typ: U64 }, Parameter { id: 1003, name: "capacity", typ: U64 }] })"#);
}




    // let hh= DefaultHasher::new();
    // hh.
    // let x = 67i32;
    // let h = x.hash();
    // let source = " {
    //     struct jstring {
    //         ptr: u64,
    //         len: u32,
    //         cap: u32
    //     };
        
    //     let s = jstring {};
    //     s.ptr = 0;
    //     s.len = 0;
    //     s.cap = 0;
    //     "



// hashtable = 
// struct HashItem {
//    in_use: bool,
//    key_field_1: T1,
//    key_field_2: T2,
//    key_field_3: T3
//    accumulator_1: U1,
//    accumulator_2: U2
// }


// group by:
// compute the key
// hash the key:
// let h = new hash
// h.hash(key variant)
// h.hash(key_field1);
// h.hash(key_field2);
// h.hash(key_field3);
// let hsh = f.finalize();

// if bucket is filled:
//   values: key1 = key1'
//   values: key2 = key2'
//   values: key3 = key3'
//   ptrs:   key1 = key1'
//   ptrs:   key2 = key2'
//   ptrs:   key3 = key3'

//   if keys are equal?
//      combine the accumulators...
//   else keys are not equal
//   do the same for hash2
//   if also still full, then request reallocation...

// after, we will enumerate through the hashtable...

// ; The actual type definition for our 'String' type.
// %String = type {
//     i8*,     ; 0: buffer; pointer to the character buffer
//     i32,     ; 1: length; the number of chars in the buffer
//     i32,     ; 2: maxlen; the maximum number of chars in the buffer
//     i32      ; 3: factor; the number of chars to preallocate when growing
// }

// define fastcc void @String_Create_Default(%String* %this) nounwind {
//     ; Initialize 'buffer'.
//     %1 = getelementptr %String* %this, i32 0, i32 0
//     store i8* null, i8** %1

//     ; Initialize 'length'.
//     %2 = getelementptr %String* %this, i32 0, i32 1
//     store i32 0, i32* %2

//     ; Initialize 'maxlen'.
//     %3 = getelementptr %String* %this, i32 0, i32 2
//     store i32 0, i32* %3

//     ; Initialize 'factor'.
//     %4 = getelementptr %String* %this, i32 0, i32 3
//     store i32 16, i32* %4

//     ret void
// }