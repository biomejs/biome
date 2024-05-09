//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_color_invalid_hex;
pub mod no_css_empty_block;
pub mod no_duplicate_at_import_rules;
pub mod no_duplicate_font_names;
pub mod no_duplicate_selectors_keyframe_block;
pub mod no_important_in_keyframe;
pub mod no_unknown_function;
pub mod no_unknown_property;
pub mod no_unknown_selector_pseudo_element;
pub mod no_unknown_unit;
pub mod no_unmatchable_anb_selector;
pub mod use_generic_font_names;

declare_group! {
    pub Nursery {
        name : "nursery" ,
        rules : [
            self :: no_color_invalid_hex :: NoColorInvalidHex ,
            self :: no_css_empty_block :: NoCssEmptyBlock ,
            self :: no_duplicate_at_import_rules :: NoDuplicateAtImportRules ,
            self :: no_duplicate_font_names :: NoDuplicateFontNames ,
            self :: no_duplicate_selectors_keyframe_block :: NoDuplicateSelectorsKeyframeBlock ,
            self :: no_important_in_keyframe :: NoImportantInKeyframe ,
            self :: no_unknown_function :: NoUnknownFunction ,
            self :: no_unknown_property :: NoUnknownProperty ,
            self :: no_unknown_selector_pseudo_element :: NoUnknownSelectorPseudoElement ,
            self :: no_unknown_unit :: NoUnknownUnit ,
            self :: no_unmatchable_anb_selector :: NoUnmatchableAnbSelector ,
            self :: use_generic_font_names :: UseGenericFontNames ,
        ]
     }
}
