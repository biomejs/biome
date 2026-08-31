#![deny(clippy::use_self)]

use biome_rowan::TreeBuilder;
use biome_toml_syntax::TomlLanguage;

mod generated;
pub mod make;
pub use crate::generated::TomlSyntaxFactory;

#[doc(hidden)]
pub use biome_toml_syntax as syntax;

pub type TomlSyntaxTreeBuilder = TreeBuilder<'static, TomlLanguage, TomlSyntaxFactory>;
