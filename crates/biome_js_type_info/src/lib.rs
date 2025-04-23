#![deny(clippy::use_self)]

mod flattening;
mod format_type_info;
mod globals;
mod local_inference;
mod resolver;
mod type_info;

pub use resolver::*;
pub use type_info::*;

pub use format_type_info::{FormatTypeContext, FormatTypeOptions};
