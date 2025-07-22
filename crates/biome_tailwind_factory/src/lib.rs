#![deny(clippy::use_self)]

use biome_rowan::TreeBuilder;
use biome_tailwind_syntax::TailwindLanguage;

mod generated;
pub use crate::generated::TailwindSyntaxFactory;
pub mod make;

// Re-exported for tests
#[doc(hidden)]
pub use biome_tailwind_syntax as syntax;

pub type TailwindSyntaxTreeBuilder = TreeBuilder<'static, TailwindLanguage, TailwindSyntaxFactory>;
