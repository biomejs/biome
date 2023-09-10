pub use crate::generated::JsonSyntaxFactory;
use biome_json_syntax::JsonLanguage;
use biome_rowan::TreeBuilder;

mod generated;

// Re-exported for tests
#[doc(hidden)]
pub use biome_json_syntax as syntax;

pub type JsonSyntaxTreeBuilder = TreeBuilder<'static, JsonLanguage, JsonSyntaxFactory>;

pub use generated::node_factory as make;
