//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod noop;

declare_group! {
    pub (crate) Nursery {
        name : "nursery" ,
        rules : [
            self :: noop :: Noop ,
        ]
     }
}
