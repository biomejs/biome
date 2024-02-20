//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_banned_types;
pub mod no_this_in_static;
pub mod no_useless_fragments;
pub mod no_useless_this_alias;

declare_group! {
    pub Complexity {
        name : "complexity" ,
        rules : [
            self :: no_banned_types :: NoBannedTypes ,
            self :: no_this_in_static :: NoThisInStatic ,
            self :: no_useless_fragments :: NoUselessFragments ,
            self :: no_useless_this_alias :: NoUselessThisAlias ,
        ]
     }
}
