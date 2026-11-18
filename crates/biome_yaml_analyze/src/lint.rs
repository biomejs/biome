//! Generated file, do not edit by hand, see `xtask/codegen`

pub mod nursery;
pub mod suspicious;
::biome_analyze::declare_category! { pub Lint { kind : Lint , language : :: biome_yaml_syntax :: YamlLanguage , groups : [self :: nursery :: Nursery , self :: suspicious :: Suspicious ,] } }
