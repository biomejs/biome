//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_delete;

declare_group! {
    pub (crate) Performance {
        name : "performance" ,
        rules : [
            self :: no_delete :: NoDelete ,
        ]
     }
}
