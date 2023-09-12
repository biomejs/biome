//! Generated file, do not edit by hand, see `xtask/codegen`

pub(crate) mod a11y;
pub(crate) mod complexity;
pub(crate) mod correctness;
pub(crate) mod nursery;
pub(crate) mod security;
pub(crate) mod style;
pub(crate) mod suspicious;
::biome_analyze::declare_category! { pub (crate) SemanticAnalyzers { kind : Lint , groups : [self :: a11y :: A11y , self :: complexity :: Complexity , self :: correctness :: Correctness , self :: nursery :: Nursery , self :: security :: Security , self :: style :: Style , self :: suspicious :: Suspicious ,] } }
