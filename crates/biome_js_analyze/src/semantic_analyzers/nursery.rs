//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_unused_imports;
pub(crate) mod use_for_of;

declare_group! {
    pub (crate) Nursery {
        name : "nursery" ,
        rules : [
            self :: no_unused_imports :: NoUnusedImports ,
            self :: use_for_of :: UseForOf ,
        ]
     }
}
