use std::io::{Write, Read};
use super::super::{Serialize, Deserialize};


impl<T:Serialize> Serialize for Vec<T> {
    fn serialize<W:Write>(&self, w:&mut W) {
        self.len().serialize(w);
		for value in self {
            value.serialize(w);
        }
    }
}


impl<T:Deserialize> Deserialize for Vec<T> {
    fn deserialize<R:Read>(r:&mut R) -> Vec<T> {
        let mut values = Vec::new();
        let len = usize::deserialize(r);
        values.reserve(len);
        for _ in 0..len {
            values.push(T::deserialize(r));
        }

        values
    }
}
