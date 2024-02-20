//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_aria_hidden_on_focusable;
pub mod use_valid_aria_role;

declare_group! {
    pub Nursery {
        name : "nursery" ,
        rules : [
            self :: no_aria_hidden_on_focusable :: NoAriaHiddenOnFocusable ,
            self :: use_valid_aria_role :: UseValidAriaRole ,
        ]
     }
}
