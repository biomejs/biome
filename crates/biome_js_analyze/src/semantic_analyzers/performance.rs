//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_accumulating_spread;

declare_group! {
    pub Performance {
        name : "performance" ,
        rules : [
            self :: no_accumulating_spread :: NoAccumulatingSpread ,
        ]
     }
}
