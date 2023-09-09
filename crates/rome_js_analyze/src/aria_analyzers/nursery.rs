//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_aria_unsupported_elements;
pub(crate) mod no_noninteractive_tabindex;
pub(crate) mod no_redundant_roles;
pub(crate) mod use_aria_prop_types;

declare_group! {
    pub (crate) Nursery {
        name : "nursery" ,
        rules : [
            self :: no_aria_unsupported_elements :: NoAriaUnsupportedElements ,
            self :: no_noninteractive_tabindex :: NoNoninteractiveTabindex ,
            self :: no_redundant_roles :: NoRedundantRoles ,
            self :: use_aria_prop_types :: UseAriaPropTypes ,
        ]
     }
}
