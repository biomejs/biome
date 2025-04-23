#![deny(clippy::use_self)]

use biome_js_syntax::JsLanguage;
use biome_rowan::TreeBuilder;

mod generated;
pub use crate::generated::JsSyntaxFactory;
pub mod make;

mod utils;

// Re-exported for tests
#[doc(hidden)]
pub use biome_js_syntax as syntax;

pub type JsSyntaxTreeBuilder = TreeBuilder<'static, JsLanguage, JsSyntaxFactory>;
