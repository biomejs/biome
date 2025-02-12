//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_assist_group;
pub mod use_sorted_properties;
declare_assist_group! { pub Source { name : "source" , rules : [self :: use_sorted_properties :: UseSortedProperties ,] } }
