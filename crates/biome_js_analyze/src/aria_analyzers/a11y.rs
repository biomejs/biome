//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_aria_hidden_on_focusable;
pub mod no_aria_unsupported_elements;
pub mod no_interactive_element_to_noninteractive_role;
pub mod no_noninteractive_element_to_interactive_role;
pub mod no_noninteractive_tabindex;
pub mod no_redundant_roles;
pub mod use_aria_activedescendant_with_tabindex;
pub mod use_aria_props_for_role;
pub mod use_valid_aria_props;
pub mod use_valid_aria_role;
pub mod use_valid_aria_values;
pub mod use_valid_lang;

declare_group! {
    pub A11y {
        name : "a11y" ,
        rules : [
            self :: no_aria_hidden_on_focusable :: NoAriaHiddenOnFocusable ,
            self :: no_aria_unsupported_elements :: NoAriaUnsupportedElements ,
            self :: no_interactive_element_to_noninteractive_role :: NoInteractiveElementToNoninteractiveRole ,
            self :: no_noninteractive_element_to_interactive_role :: NoNoninteractiveElementToInteractiveRole ,
            self :: no_noninteractive_tabindex :: NoNoninteractiveTabindex ,
            self :: no_redundant_roles :: NoRedundantRoles ,
            self :: use_aria_activedescendant_with_tabindex :: UseAriaActivedescendantWithTabindex ,
            self :: use_aria_props_for_role :: UseAriaPropsForRole ,
            self :: use_valid_aria_props :: UseValidAriaProps ,
            self :: use_valid_aria_role :: UseValidAriaRole ,
            self :: use_valid_aria_values :: UseValidAriaValues ,
            self :: use_valid_lang :: UseValidLang ,
        ]
     }
}
