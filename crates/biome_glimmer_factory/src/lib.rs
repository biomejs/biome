//! Factory functions for creating Glimmer AST nodes

mod generated;

pub use biome_glimmer_syntax::GlimmerLanguage;
pub use generated::syntax_factory::GlimmerSyntaxFactory;
pub use generated::{node_factory::*, syntax_factory::*};
