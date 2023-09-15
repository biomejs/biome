//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_accumulating_spread;
pub(crate) mod no_global_is_finite;
pub(crate) mod no_global_is_nan;
pub(crate) mod use_exhaustive_dependencies;
pub(crate) mod use_hook_at_top_level;
pub(crate) mod use_is_array;

declare_group! {
    pub (crate) Nursery {
        name : "nursery" ,
        rules : [
            self :: no_accumulating_spread :: NoAccumulatingSpread ,
            self :: no_global_is_finite :: NoGlobalIsFinite ,
            self :: no_global_is_nan :: NoGlobalIsNan ,
            self :: use_exhaustive_dependencies :: UseExhaustiveDependencies ,
            self :: use_hook_at_top_level :: UseHookAtTopLevel ,
            self :: use_is_array :: UseIsArray ,
        ]
     }
}
