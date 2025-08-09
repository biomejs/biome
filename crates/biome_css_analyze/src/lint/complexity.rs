//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_important_styles;
declare_lint_group! { pub Complexity { name : "complexity" , rules : [self :: no_important_styles :: NoImportantStyles ,] } }
