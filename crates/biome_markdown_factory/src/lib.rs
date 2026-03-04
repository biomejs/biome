#![deny(clippy::use_self)]

use biome_markdown_syntax::MdLanguage;
use biome_rowan::TreeBuilder;

mod generated;
pub use crate::generated::MarkdownSyntaxFactory;

// Re-exported for tests
#[doc(hidden)]
pub use biome_markdown_syntax as syntax;

pub type DemoSyntaxTreeBuilder = TreeBuilder<'static, MdLanguage, MarkdownSyntaxFactory>;

pub mod make;
