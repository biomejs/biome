#![deny(clippy::use_self)]

mod flattening;
mod globals;
mod local_inference;
mod resolver;
mod type_info;

#[cfg(test)]
mod test_util;

pub use resolver::*;
pub use type_info::*;
