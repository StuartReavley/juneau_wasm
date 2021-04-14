



macro_rules! to_str(
    ($ty:ty, $func:expr) => (
        impl std::fmt::Debug for $ty {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                fmt.write_str(unsafe {
                    let c_str = $func(self.into());
                    crate::core::c::to_str(c_str)
                })
            }
        }
    );
);