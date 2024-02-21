//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_positive_tabindex;
pub mod use_button_type;

declare_group! {
    pub A11y {
        name : "a11y" ,
        rules : [
            self :: no_positive_tabindex :: NoPositiveTabindex ,
            self :: use_button_type :: UseButtonType ,
        ]
     }
}
