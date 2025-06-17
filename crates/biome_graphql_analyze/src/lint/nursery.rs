//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod use_named_operation;
pub mod use_naming_convention;
declare_lint_group! { pub Nursery { name : "nursery" , rules : [self :: use_named_operation :: UseNamedOperation , self :: use_naming_convention :: UseNamingConvention ,] } }
