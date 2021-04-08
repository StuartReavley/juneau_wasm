use std::convert::TryInto;



pub trait Base64able {
    fn to_base_64(&self) -> String;
    fn from_base_64(values:&[u8]) -> Self;
}


impl Base64able for u64 {
    fn to_base_64(&self) -> String {
        ::base64::encode(self.to_be_bytes())
    }

    fn from_base_64(values:&[u8]) -> u64 {
        let bytes = ::base64::decode(values).unwrap();
        let bytes:[u8; 8] = bytes.try_into().unwrap();
        u64::from_be_bytes(bytes)
    }
}