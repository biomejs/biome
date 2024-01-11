//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_aria_hidden_on_focusable;
pub(crate) mod no_aria_unsupported_elements;
pub(crate) mod no_interactive_element_to_noninteractive_role;
pub(crate) mod no_noninteractive_element_to_interactive_role;
pub(crate) mod no_noninteractive_tabindex;
pub(crate) mod no_redundant_roles;
pub(crate) mod use_aria_activedescendant_with_tabindex;
pub(crate) mod use_aria_props_for_role;
pub(crate) mod use_valid_aria_props;
pub(crate) mod use_valid_aria_role;
pub(crate) mod use_valid_aria_values;
pub(crate) mod use_valid_lang;

declare_group! {
    pub (crate) A11y {
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
