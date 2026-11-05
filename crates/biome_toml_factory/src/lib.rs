#![deny(clippy::use_self)]

use biome_rowan::TreeBuilder;
use biome_toml_syntax::TomlLanguage;

mod generated;
pub use crate::generated::TomlSyntaxFactory;
pub mod make;

// Re-exported for tests
#[doc(hidden)]
pub use biome_toml_syntax as syntax;

pub type TomlSyntaxTreeBuilder = TreeBuilder<'static, TomlLanguage, TomlSyntaxFactory>;
pub use crate::generated::node_factory::*;
