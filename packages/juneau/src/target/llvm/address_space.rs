
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum AddressSpace {
    Generic = 0,
    Global = 1,
    Shared = 3,
    Const = 4,
    Local = 5,
}
