//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_interactive_element_to_noninteractive_role;
pub(crate) mod use_aria_activedescendant_with_tabindex;
pub(crate) mod use_valid_aria_role;

declare_group! {
    pub (crate) Nursery {
        name : "nursery" ,
        rules : [
            self :: no_interactive_element_to_noninteractive_role :: NoInteractiveElementToNoninteractiveRole ,
            self :: use_aria_activedescendant_with_tabindex :: UseAriaActivedescendantWithTabindex ,
            self :: use_valid_aria_role :: UseValidAriaRole ,
        ]
     }
}
