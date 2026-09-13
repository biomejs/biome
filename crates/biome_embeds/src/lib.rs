pub mod bindings;
mod data;
pub mod references;
#[cfg(test)]
pub(crate) mod testing;
pub(crate) mod visitor;
pub mod vue_directives;

pub use data::{EmbeddedData, VueDirectiveResolution, vue_directive_binding_name};
pub use visitor::{EmbeddedSnippet, collect_embedded_data};
