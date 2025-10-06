//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_biome_first_exception;
pub mod no_duplicate_object_keys;
pub mod no_quickfix_biome;
pub mod use_biome_ignore_folder;
declare_lint_group! { pub Suspicious { name : "suspicious" , rules : [self :: no_biome_first_exception :: NoBiomeFirstException , self :: no_duplicate_object_keys :: NoDuplicateObjectKeys , self :: no_quickfix_biome :: NoQuickfixBiome , self :: use_biome_ignore_folder :: UseBiomeIgnoreFolder ,] } }
