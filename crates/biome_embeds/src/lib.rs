pub mod bindings;
mod data;
mod embedded;
pub mod references;
pub(crate) mod visitor;

pub use data::EmbeddedData;
pub use embedded::{EmbeddedSource, EmbeddedSourceData};
pub use visitor::{EmbeddedSnippet, collect_embedded_data};
