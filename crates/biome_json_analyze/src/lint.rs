//! Generated file, do not edit by hand, see `xtask/codegen`

pub mod nursery;
pub mod style;
pub mod suspicious;
::biome_analyze::declare_category! { pub Lint { kind : Lint , language : :: biome_json_syntax :: JsonLanguage , groups : [self :: nursery :: Nursery , self :: style :: Style , self :: suspicious :: Suspicious ,] } }
