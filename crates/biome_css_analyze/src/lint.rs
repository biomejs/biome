//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

pub mod a11y;
pub mod correctness;
pub mod nursery;
pub mod suspicious;
::biome_analyze::declare_category! { pub Lint { kind : Lint , groups : [self :: a11y :: A11y , self :: correctness :: Correctness , self :: nursery :: Nursery , self :: suspicious :: Suspicious ,] } }
