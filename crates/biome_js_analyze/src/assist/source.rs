//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_assist_group;
pub mod organize_imports;
pub mod use_sorted_attributes;
declare_assist_group! { pub Source { name : "source" , rules : [self :: organize_imports :: OrganizeImports , self :: use_sorted_attributes :: UseSortedAttributes ,] } }
