use biome_markdown_syntax::DemoLanguage;
use biome_rowan::TreeBuilder;

mod generated;
pub use crate::generated::DemoSyntaxFactory;

// Re-exported for tests
#[doc(hidden)]
pub use biome_markdown_syntax as syntax;

pub type DemoSyntaxTreeBuilder = TreeBuilder<'static, DemoLanguage, DemoSyntaxFactory>;
