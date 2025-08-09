//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

pub mod correctness;
pub mod style;
pub mod suspicious;
::biome_analyze::declare_category! { pub Lint { kind : Lint , groups : [self :: correctness :: Correctness , self :: style :: Style , self :: suspicious :: Suspicious ,] } }
