//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_assist_group;
pub mod no_duplicate_classes;
declare_assist_group! { pub Source { name : "source" , rules : [self :: no_duplicate_classes :: NoDuplicateClasses ,] } }
