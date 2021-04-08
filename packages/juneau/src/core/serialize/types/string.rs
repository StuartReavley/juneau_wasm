use std::fs::File;
use std::io::{Write, Read};
use super::super::{Serialize, Deserialize};


impl Serialize for String {
    fn serialize<W:Write>(&self, w:&mut W) {
        self.as_str().serialize(w)
    }
}

impl Serialize for &str {
    fn serialize<W:Write>(&self, w:&mut W) {
        let bytes = (*self).as_bytes();
        let len:usize = bytes.len();
        len.serialize(w);
        w.write_all(bytes).unwrap();
    }
}

impl Deserialize for String {
    fn deserialize<R:Read>(r:&mut R) -> String {
        let len = usize::deserialize(r);
        let mut bytes = vec![0u8; len];
        r.read_exact(&mut bytes).unwrap();
        return String::from_utf8(bytes).unwrap();
    }
}


#[test]
fn basic() {
    let file_name = "serialize_string_basic.bin";
    let x = "hello";

    {
        let mut w = File::create(file_name).unwrap();
        x.serialize(&mut w);
    }

    {
        let mut r = File::open(file_name).unwrap();
        let y = String::deserialize(&mut r);
        assert_eq!(x, y);
    }
}