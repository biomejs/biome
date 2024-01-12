use biome_html_syntax::HtmlLanguage;
use biome_rowan::TreeBuilder;

mod generated;
pub use crate::generated::HtmlSyntaxFactory;
pub mod make;

// Re-exported for tests
#[doc(hidden)]
pub use biome_html_syntax as syntax;

pub type JsonSyntaxTreeBuilder = TreeBuilder<'static, HtmlLanguage, HtmlSyntaxFactory>;
