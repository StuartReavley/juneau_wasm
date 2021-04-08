

pub fn ieee754_f32(value:f32) -> [u8; 4] {
    value.to_le_bytes()
}

pub fn ieee754_f64(value:f64) -> [u8; 8] {
    value.to_le_bytes()
}

pub fn encode_u8_string(value:&[u8]) -> Vec<u8> {
    let mut values = vec![value.len() as u8]; // eugh, longer than 256 chars?
    values.extend(value);
    values
}

  
//   export const signedLEB128 = (n: number) => {
//     const buffer = [];
//     let more = true;
//     while (more) {
//       let byte = n & 0x7f;
//       n >>>= 7;
//       if ((n === 0 && (byte & 0x40) === 0) || (n === -1 && (byte & 0x40) !== 0)) {
//         more = false;
//       } else {
//         byte |= 0x80;
//       }
//       buffer.push(byte);
//     }
//     return buffer;
//   };

pub fn unsignedLEB128(mut value:u64) -> Vec<u8> {
    let mut buffer = Vec::new();

    while value > 0 || buffer.is_empty() {
        let byte = (value & 0x7f) as u8;
        value >>= 7;
        buffer.push(if value != 0 {byte | 0x80} else {byte});
    }

    buffer
}
