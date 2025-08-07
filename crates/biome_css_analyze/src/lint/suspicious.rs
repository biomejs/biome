//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_duplicate_at_import_rules;
pub mod no_duplicate_custom_properties;
pub mod no_duplicate_font_names;
pub mod no_duplicate_properties;
pub mod no_duplicate_selectors_keyframe_block;
pub mod no_empty_block;
pub mod no_important_in_keyframe;
pub mod no_irregular_whitespace;
pub mod no_shorthand_property_overrides;
pub mod no_unknown_at_rules;
pub mod no_useless_escape_in_string;
declare_lint_group! { pub Suspicious { name : "suspicious" , rules : [self :: no_duplicate_at_import_rules :: NoDuplicateAtImportRules , self :: no_duplicate_custom_properties :: NoDuplicateCustomProperties , self :: no_duplicate_font_names :: NoDuplicateFontNames , self :: no_duplicate_properties :: NoDuplicateProperties , self :: no_duplicate_selectors_keyframe_block :: NoDuplicateSelectorsKeyframeBlock , self :: no_empty_block :: NoEmptyBlock , self :: no_important_in_keyframe :: NoImportantInKeyframe , self :: no_irregular_whitespace :: NoIrregularWhitespace , self :: no_shorthand_property_overrides :: NoShorthandPropertyOverrides , self :: no_unknown_at_rules :: NoUnknownAtRules , self :: no_useless_escape_in_string :: NoUselessEscapeInString ,] } }
