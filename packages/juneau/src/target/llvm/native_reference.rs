// use std::mem::transmute;



// pub trait NativeReference2 {
//     fn get_ptr(&self) -> u64;
// }

// impl<T:NativeReference2+Sized> PartialEq for T {
//     fn eq(self, other: &Self) -> bool {
//         unsafe {transmute::<_, isize>(self) == transmute(other)}
//     }
// }


#[macro_escape]



macro_rules! native_reference(
    ($(#[$attr:meta])* $name:ident = $alias:ty) => (

        $(#[$attr])*
        pub struct $name(std::marker::PhantomData<[u8]>);

        impl Eq for $name {}
        impl PartialEq<$name> for $name {
            fn eq(&self, other:&$name) -> bool {
                use std::mem;
                unsafe {mem::transmute::<_, isize>(self) == mem::transmute(other)}
            }
        }
        impl<'l> PartialEq<$name> for &'l $name {
            fn eq(&self, other:&$name) -> bool {
                use std::mem;
                unsafe {mem::transmute::<_, isize>(self) == mem::transmute(other)}
            }
        }
        impl<'l> From<&'l $name> for *mut $alias {
            fn from(value:&'l $name) -> *mut $alias {
                use std::mem;
                unsafe {mem::transmute(value)}
            }
        }
        impl<'l> From<&'l mut $name> for *mut $alias {
            fn from(value:&'l mut $name) -> *mut $alias {
                use std::mem;
                unsafe {mem::transmute(value)}
            }
        }
        impl<'l> From<*mut $alias> for &'l $name {
            fn from(value:*mut $alias) -> &'l $name {
                use std::mem;
                unsafe {mem::transmute(value)}
            }
        }
        impl<'l> From<*mut $alias> for &'l mut $name {
            fn from(value:*mut $alias) -> &'l mut $name {
                use std::mem;
                unsafe {mem::transmute(value)}
            }
        }
    );
);