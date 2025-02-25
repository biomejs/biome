//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

pub mod a11y;
pub mod complexity;
pub mod correctness;
pub mod nursery;
pub mod performance;
pub mod security;
pub mod style;
pub mod suspicious;
::biome_analyze::declare_category! { pub Lint { kind : Lint , groups : [self :: a11y :: A11y , self :: complexity :: Complexity , self :: correctness :: Correctness , self :: nursery :: Nursery , self :: performance :: Performance , self :: security :: Security , self :: style :: Style , self :: suspicious :: Suspicious ,] } }
