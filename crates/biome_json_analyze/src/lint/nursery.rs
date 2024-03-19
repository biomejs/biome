//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_duplicate_json_keys;

declare_group! {
    pub Nursery {
        name : "nursery" ,
        rules : [
            self :: no_duplicate_json_keys :: NoDuplicateJsonKeys ,
        ]
     }
}
