use std::borrow::Cow;
use crate::Dump;


impl<'a, T:?Sized + ToOwned + Dump> Dump for Cow<'a, T> where
    <T as ToOwned>::Owned:Dump {
    fn dump(&self) -> String {
        use Cow::*;
        match self {
            Borrowed(value) => format!("Cow::Borrowed({})", value.dump()),
            Owned(value) => format!("Cow::Owned({})", value.dump())
        }
    }
}