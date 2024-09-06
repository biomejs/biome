//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;

pub mod no_invalid_grid_areas;
pub mod no_irregular_whitespace;
pub mod no_shorthand_property_overrides;
pub mod no_unknown_function;
pub mod no_unknown_media_feature_name;
pub mod no_unknown_property;
pub mod no_unknown_pseudo_class_selector;
pub mod no_unknown_selector_pseudo_element;
pub mod no_unknown_unit;
pub mod no_value_at_rule;
pub mod use_generic_font_names;

declare_lint_group! {
    pub Nursery {
        name : "nursery" ,
        rules : [
            self :: no_invalid_grid_areas :: NoInvalidGridAreas ,
            self :: no_irregular_whitespace :: NoIrregularWhitespace ,
            self :: no_shorthand_property_overrides :: NoShorthandPropertyOverrides ,
            self :: no_unknown_function :: NoUnknownFunction ,
            self :: no_unknown_media_feature_name :: NoUnknownMediaFeatureName ,
            self :: no_unknown_property :: NoUnknownProperty ,
            self :: no_unknown_pseudo_class_selector :: NoUnknownPseudoClassSelector ,
            self :: no_unknown_selector_pseudo_element :: NoUnknownSelectorPseudoElement ,
            self :: no_unknown_unit :: NoUnknownUnit ,
            self :: no_value_at_rule :: NoValueAtRule ,
            self :: use_generic_font_names :: UseGenericFontNames ,
        ]
     }
}
