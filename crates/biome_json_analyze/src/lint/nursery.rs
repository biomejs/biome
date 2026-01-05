//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_duplicate_dependencies;
pub mod use_required_scripts;
declare_lint_group! { pub Nursery { name : "nursery" , rules : [self :: no_duplicate_dependencies :: NoDuplicateDependencies , self :: use_required_scripts :: UseRequiredScripts ,] } }
