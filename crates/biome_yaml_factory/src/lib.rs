use biome_rowan::TreeBuilder;
use biome_yaml_syntax::YamlLanguage;

mod generated;
pub mod make;
pub use crate::generated::YamlSyntaxFactory;

// Re-exported for tests
#[doc(hidden)]
pub use biome_yaml_syntax as syntax;

pub type YamlSyntaxTreeBuilder = TreeBuilder<'static, YamlLanguage, YamlSyntaxFactory>;
