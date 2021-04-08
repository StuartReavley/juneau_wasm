use std::{fs::File, mem::size_of};
use std::io::Write;
use std::io::Read;
use super::super::{Serialize, Deserialize};


macro_rules! impl_serialize_unsigned {
    ($($t:ty),*) => ($(

        impl Serialize for $t {
            fn serialize<W:Write>(&self, w: &mut W) {
                let mut x = *self;
                loop {
                    let v = (x as u8) & 0b0111_1111;
                    x >>= 7;
                    let next = if x > 0 {0b1000_0000} else {0};
                    w.write_all(&[v | next]).unwrap();
                    if next == 0 {
                        break;
                    }
                }

        // 		T x    = value;
        // 		uchar  next;
        // 		do
        // 			{
        // 			uchar v = uchar(x) & ~(1 << 7);
        // 			x >>= 7;
        // 			next = (x > 0) << 7;
        // 			s.file.put(v | next);
        // 			} while(next);

                
                // w.write_all(&self.to_ne_bytes()).unwrap();
            }
        }

        impl Deserialize for $t {
            fn deserialize<R:Read>(r: &mut R) -> $t {
                let mut buffer = [0u8];
                r.read_exact(&mut buffer).unwrap();
                let mut value:$t = (buffer[0] & 0b0111_1111) as $t;
                let mut count = 7u8;

                while buffer[0] & 0b1000_0000 != 0 {
                    r.read_exact(&mut buffer).unwrap();
                    value|= (buffer[0] & 0b0111_1111) as $t << count;
                    count += 7;
                }
                value

                // let mut buffer = [0; size_of::<$t>()];
                // r.read_exact(&mut buffer).unwrap();
                // return <$t>::from_ne_bytes(buffer);
            }
        }

    )*)
}

macro_rules! impl_serialize_signed {
    ($($t:ty),*) => ($(

        impl Serialize for $t {
            fn serialize<W:Write>(&self, w: &mut W) {
                let x = (self << 1) as u64 ^ (self >> (size_of::<$t>() * 8 - 1)) as u64;
                x.serialize(w);
            }
        }

        impl Deserialize for $t {
            fn deserialize<R:Read>(r: &mut R) -> $t {
                let x = u64::deserialize(r);
                let v = (x >> 1) as $t;
                if x & 0x1 != 0 {!v} else {v}
            }
        }

    )*)
}


impl_serialize_unsigned! [u8, u16, u32, u64, usize];
impl_serialize_signed!   [i8, i16, i32, i64, isize];


impl Serialize for bool {
    fn serialize<W:Write>(&self, w: &mut W) {
        (if *self {1u8} else {0u8}).serialize(w);
    }
}

impl Deserialize for bool {
    fn deserialize<R:Read>(r: &mut R) -> bool {
        return u8::deserialize(r) == 1;
    }
}

impl Serialize for f64 {
    fn serialize<W:Write>(&self, w: &mut W) {
        w.write_all(&self.to_ne_bytes()).unwrap();
    }
}

impl Deserialize for f64 {
    fn deserialize<R:Read>(r: &mut R) -> f64 {
        let mut buffer = [0u8; 8];
        r.read_exact(&mut buffer).unwrap();
        f64::from_ne_bytes(buffer)
    }
}


#[test]
fn u8() {
    let file_name = "serialize_primitive_u8.bin";
    let x:u8 = 7;

    {
        let mut s = File::create(file_name).unwrap();
        x.serialize(&mut s);
    }

    {
        let mut r = File::open(file_name).unwrap();
        let y = u8::deserialize(&mut r);
        assert_eq!(x, y);
    }
}

#[test]
fn i64() {
    let file_name = "serialize_primitive_i64.bin";
    let x:i64 = -384783973;

    {
        let mut w = File::create(file_name).unwrap();
        x.serialize(&mut w);
    }

    {
        let mut r = File::open(file_name).unwrap();
        let y = i64::deserialize(&mut r);
        assert_eq!(x, y);
    }
}