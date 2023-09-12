//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_noninteractive_element_to_interactive_role;
pub(crate) mod use_aria_props_for_role;
pub(crate) mod use_valid_aria_props;
pub(crate) mod use_valid_lang;

declare_group! {
    pub (crate) A11y {
        name : "a11y" ,
        rules : [
            self :: no_noninteractive_element_to_interactive_role :: NoNoninteractiveElementToInteractiveRole ,
            self :: use_aria_props_for_role :: UseAriaPropsForRole ,
            self :: use_valid_aria_props :: UseValidAriaProps ,
            self :: use_valid_lang :: UseValidLang ,
        ]
     }
}
