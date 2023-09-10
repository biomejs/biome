use biome_rowan::TreeBuilder;
use rome_js_syntax::JsLanguage;

mod generated;
pub use crate::generated::JsSyntaxFactory;
pub mod make;

mod utils;

// Re-exported for tests
#[doc(hidden)]
pub use rome_js_syntax as syntax;

pub type JsSyntaxTreeBuilder = TreeBuilder<'static, JsLanguage, JsSyntaxFactory>;
