use biome_grit_syntax::GritLanguage;
use biome_rowan::TreeBuilder;

mod generated;
pub use crate::generated::GritSyntaxFactory;

// Re-exported for tests
#[doc(hidden)]
pub use biome_grit_syntax as syntax;

pub type GritSyntaxTreeBuilder = TreeBuilder<'static, GritLanguage, GritSyntaxFactory>;
