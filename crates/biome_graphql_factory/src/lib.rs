use biome_graphql_syntax::GraphqlLanguage;
use biome_rowan::TreeBuilder;

mod generated;
pub mod make;
pub use crate::generated::GraphqlSyntaxFactory;

// Re-exported for tests
#[doc(hidden)]
pub use biome_graphql_syntax as syntax;

pub type GritSyntaxTreeBuilder = TreeBuilder<'static, GraphqlLanguage, GraphqlSyntaxFactory>;
