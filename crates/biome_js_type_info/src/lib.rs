#![deny(clippy::use_self)]

mod flattening;
mod format_type_info;
mod globals;
mod local_inference;
mod resolver;
mod type_info;

pub use globals::{GLOBAL_RESOLVER, GLOBAL_UNKNOWN_ID, GlobalsResolver, NUM_PREDEFINED_TYPES};
pub use resolver::*;
pub use type_info::*;

pub use format_type_info::{FormatTypeContext, FormatTypeOptions};
