//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_banned_types;
pub(crate) mod no_useless_fragments;
pub(crate) mod no_useless_this_alias;

declare_group! {
    pub (crate) Complexity {
        name : "complexity" ,
        rules : [
            self :: no_banned_types :: NoBannedTypes ,
            self :: no_useless_fragments :: NoUselessFragments ,
            self :: no_useless_this_alias :: NoUselessThisAlias ,
        ]
     }
}
