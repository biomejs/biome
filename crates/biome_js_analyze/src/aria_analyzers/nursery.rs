//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod use_aria_activedescendant_tabindex;

declare_group! {
    pub (crate) Nursery {
        name : "nursery" ,
        rules : [
            self :: use_aria_activedescendant_tabindex :: UseAriaActivedescendantTabindex ,
        ]
     }
}
