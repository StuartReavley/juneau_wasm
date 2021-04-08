use std::io::{Write, Read};

pub mod types;
pub use self::types::*;


// pub trait Serialize {
//     fn serialize<W:Write>(&self, w: &mut W);
// }


pub trait Deserialize {
    fn deserialize<R:Read>(r: &mut R) -> Self;
}