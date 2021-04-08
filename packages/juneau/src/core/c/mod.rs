#[macro_use]
mod cbox;

mod util;

pub use cbox::{CBox, CSemiBox, DisposeRef};
pub use util::{SubStruct, ptr_to_null, with_cstr, to_str, to_null_str};
