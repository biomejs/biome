//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_aria_hidden_on_focusable;
pub(crate) mod no_static_element_interactions;
pub(crate) mod use_valid_aria_role;

declare_group! {
    pub (crate) Nursery {
        name : "nursery" ,
        rules : [
            self :: no_aria_hidden_on_focusable :: NoAriaHiddenOnFocusable ,
            self :: no_static_element_interactions :: NoStaticElementInteractions ,
            self :: use_valid_aria_role :: UseValidAriaRole ,
        ]
     }
}
