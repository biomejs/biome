//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_useless_fragments;

declare_group! {
    pub (crate) Complexity {
        name : "complexity" ,
        rules : [
            self :: no_useless_fragments :: NoUselessFragments ,
        ]
     }
}
