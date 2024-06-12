//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_assists_group;

pub mod use_sorted_keys;

declare_assists_group! {
    pub Nursery {
        name : "nursery" ,
        rules : [
            self :: use_sorted_keys :: UseSortedKeys ,
        ]
     }
}
