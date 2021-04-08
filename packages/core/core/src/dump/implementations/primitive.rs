use crate::Dump;


macro_rules! implement_dump {
    ($($typ:ty),*) => {$(
        impl Dump for $typ {
            fn dump(&self) -> String {
                self.to_string()
            }
        })*
       }
   }

implement_dump!(u8, u16, u32, u64, usize);
implement_dump!(i8, i16, i32, i64, isize);
implement_dump!(bool);


impl Dump for str {
    fn dump(&self) -> String {
        format!("\"{}\"", self)
    }
}
impl Dump for String {
    fn dump(&self) -> String {
        format!("\"{}\"", self)
    }
}

