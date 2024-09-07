//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;

pub mod no_irregular_whitespace;
pub mod no_unknown_pseudo_class_selector;
pub mod no_unknown_selector_pseudo_element;
pub mod no_value_at_rule;
pub mod use_generic_font_names;

declare_lint_group! {
    pub Nursery {
        name : "nursery" ,
        rules : [
            self :: no_irregular_whitespace :: NoIrregularWhitespace ,
            self :: no_unknown_pseudo_class_selector :: NoUnknownPseudoClassSelector ,
            self :: no_unknown_selector_pseudo_element :: NoUnknownSelectorPseudoElement ,
            self :: no_value_at_rule :: NoValueAtRule ,
            self :: use_generic_font_names :: UseGenericFontNames ,
        ]
     }
}
