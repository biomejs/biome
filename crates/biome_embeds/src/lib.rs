pub mod bindings;
mod data;
pub mod references;
pub(crate) mod visitor;

pub use data::EmbeddedData;
pub use visitor::{EmbeddedSnippet, collect_embedded_data};
