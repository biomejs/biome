//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_empty_source;
pub mod use_deprecated_date;
declare_lint_group! { pub Nursery { name : "nursery" , rules : [self :: no_empty_source :: NoEmptySource , self :: use_deprecated_date :: UseDeprecatedDate ,] } }
