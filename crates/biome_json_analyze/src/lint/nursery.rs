//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_quickfix_biome;
declare_lint_group! { pub Nursery { name : "nursery" , rules : [self :: no_quickfix_biome :: NoQuickfixBiome ,] } }
