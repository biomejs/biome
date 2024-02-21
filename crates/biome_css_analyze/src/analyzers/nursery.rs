//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod noop;

declare_group! {
    pub Nursery {
        name : "nursery" ,
        rules : [
            self :: noop :: Noop ,
        ]
     }
}
