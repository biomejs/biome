#![deny(clippy::use_self)]

mod conditionals;
mod flattening;
mod format_type_info;
mod globals;
mod helpers;
mod local_inference;
mod resolver;
mod r#type;
mod type_data;
mod type_store;

pub use conditionals::*;
pub use flattening::MAX_FLATTEN_DEPTH;
pub use globals::{GLOBAL_RESOLVER, GLOBAL_UNKNOWN_ID, GlobalsResolver, NUM_PREDEFINED_TYPES};
pub use resolver::*;
pub use r#type::Type;
pub use type_data::*;
pub use type_store::*;

pub use format_type_info::{FormatTypeContext, FormatTypeOptions};
