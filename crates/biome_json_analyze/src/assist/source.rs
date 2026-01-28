//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_assist_group;
pub mod organize_package_json;
pub mod use_sorted_keys;
declare_assist_group! { pub Source { name : "source" , rules : [self :: organize_package_json :: OrganizePackageJson , self :: use_sorted_keys :: UseSortedKeys ,] } }
