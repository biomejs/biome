//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_duplicate_json_keys;

declare_group! {
    pub (crate) Nursery {
        name : "nursery" ,
        rules : [
            self :: no_duplicate_json_keys :: NoDuplicateJsonKeys ,
        ]
     }
}
