use biome_php_syntax::PhpLanguage;
use biome_rowan::TreeBuilder;

mod generated;
pub use crate::generated::PhpSyntaxFactory;
pub mod make;

// Re-exported for tests
#[doc(hidden)]
pub use biome_php_syntax as syntax;

pub type PhpSyntaxTreeBuilder = TreeBuilder<'static, PhpLanguage, PhpSyntaxFactory>;