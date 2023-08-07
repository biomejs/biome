//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;

pub(crate) mod no_accumulating_spread;
pub(crate) mod no_banned_types;
pub(crate) mod no_constant_condition;
pub(crate) mod no_global_is_finite;
pub(crate) mod no_global_is_nan;
pub(crate) mod no_unsafe_declaration_merging;
pub(crate) mod no_useless_this_alias;
pub(crate) mod use_exhaustive_dependencies;
pub(crate) mod use_hook_at_top_level;
pub(crate) mod use_is_array;
pub(crate) mod use_naming_convention;

declare_group! {
    pub (crate) Nursery {
        name : "nursery" ,
        rules : [
            self :: no_accumulating_spread :: NoAccumulatingSpread ,
            self :: no_banned_types :: NoBannedTypes ,
            self :: no_constant_condition :: NoConstantCondition ,
            self :: no_global_is_finite :: NoGlobalIsFinite ,
            self :: no_global_is_nan :: NoGlobalIsNan ,
            self :: no_unsafe_declaration_merging :: NoUnsafeDeclarationMerging ,
            self :: no_useless_this_alias :: NoUselessThisAlias ,
            self :: use_exhaustive_dependencies :: UseExhaustiveDependencies ,
            self :: use_hook_at_top_level :: UseHookAtTopLevel ,
            self :: use_is_array :: UseIsArray ,
            self :: use_naming_convention :: UseNamingConvention ,
        ]
     }
}
