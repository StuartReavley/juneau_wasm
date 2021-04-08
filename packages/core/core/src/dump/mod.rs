
mod implementations;


pub trait Dump {
    fn dump(&self) -> String;
}


#[macro_use]
pub mod pretty_assertions;



#[macro_export]
macro_rules! assert_dump_eq {
    ($actual:expr, $expected:expr) => ({
        juneau_core::assert_eq_pretty!(juneau_core::Dump::dump(&$actual), ($expected))
    });
}

#[macro_export]
macro_rules! assert_eq_debug {
    ($actual:expr, $expected:expr $(,)?) => ({
        $crate::assert_eq_object!(format!("{:?}", $actual), $expected)
    });
}

#[macro_export]
macro_rules! assert_eq_object {
    ($actual:expr, $expected:expr $(,)?) => ({
        $crate::assert_eq_pretty!($actual, $expected)
    });
}
