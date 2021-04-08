use std::marker::PhantomData;

use crate::Dump;





impl<T> Dump for PhantomData<T> {
    fn dump(&self) -> String {
        "[PhantomData]".into()
    }
}