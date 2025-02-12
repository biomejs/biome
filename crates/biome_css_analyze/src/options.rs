//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::assist;
use crate::lint;
pub type NoDescendingSpecificity = < lint :: nursery :: no_descending_specificity :: NoDescendingSpecificity as biome_analyze :: Rule > :: Options ;
pub type NoDuplicateAtImportRules = < lint :: suspicious :: no_duplicate_at_import_rules :: NoDuplicateAtImportRules as biome_analyze :: Rule > :: Options ;
pub type NoDuplicateCustomProperties = < lint :: nursery :: no_duplicate_custom_properties :: NoDuplicateCustomProperties as biome_analyze :: Rule > :: Options ;
pub type NoDuplicateFontNames = < lint :: suspicious :: no_duplicate_font_names :: NoDuplicateFontNames as biome_analyze :: Rule > :: Options ;
pub type NoDuplicateProperties =
    <lint::nursery::no_duplicate_properties::NoDuplicateProperties as biome_analyze::Rule>::Options;
pub type NoDuplicateSelectorsKeyframeBlock = < lint :: suspicious :: no_duplicate_selectors_keyframe_block :: NoDuplicateSelectorsKeyframeBlock as biome_analyze :: Rule > :: Options ;
pub type NoEmptyBlock =
    <lint::suspicious::no_empty_block::NoEmptyBlock as biome_analyze::Rule>::Options;
pub type NoImportantInKeyframe = < lint :: suspicious :: no_important_in_keyframe :: NoImportantInKeyframe as biome_analyze :: Rule > :: Options ;
pub type NoInvalidDirectionInLinearGradient = < lint :: correctness :: no_invalid_direction_in_linear_gradient :: NoInvalidDirectionInLinearGradient as biome_analyze :: Rule > :: Options ;
pub type NoInvalidGridAreas =
    <lint::correctness::no_invalid_grid_areas::NoInvalidGridAreas as biome_analyze::Rule>::Options;
pub type NoInvalidPositionAtImportRule = < lint :: correctness :: no_invalid_position_at_import_rule :: NoInvalidPositionAtImportRule as biome_analyze :: Rule > :: Options ;
pub type NoIrregularWhitespace =
    <lint::nursery::no_irregular_whitespace::NoIrregularWhitespace as biome_analyze::Rule>::Options;
pub type NoMissingVarFunction =
    <lint::nursery::no_missing_var_function::NoMissingVarFunction as biome_analyze::Rule>::Options;
pub type NoShorthandPropertyOverrides = < lint :: suspicious :: no_shorthand_property_overrides :: NoShorthandPropertyOverrides as biome_analyze :: Rule > :: Options ;
pub type NoUnknownAtRule =
    <lint::nursery::no_unknown_at_rule::NoUnknownAtRule as biome_analyze::Rule>::Options;
pub type NoUnknownFunction =
    <lint::correctness::no_unknown_function::NoUnknownFunction as biome_analyze::Rule>::Options;
pub type NoUnknownMediaFeatureName = < lint :: correctness :: no_unknown_media_feature_name :: NoUnknownMediaFeatureName as biome_analyze :: Rule > :: Options ;
pub type NoUnknownProperty =
    <lint::correctness::no_unknown_property::NoUnknownProperty as biome_analyze::Rule>::Options;
pub type NoUnknownPseudoClass =
    <lint::nursery::no_unknown_pseudo_class::NoUnknownPseudoClass as biome_analyze::Rule>::Options;
pub type NoUnknownPseudoElement = < lint :: nursery :: no_unknown_pseudo_element :: NoUnknownPseudoElement as biome_analyze :: Rule > :: Options ;
pub type NoUnknownTypeSelector = < lint :: nursery :: no_unknown_type_selector :: NoUnknownTypeSelector as biome_analyze :: Rule > :: Options ;
pub type NoUnknownUnit =
    <lint::correctness::no_unknown_unit::NoUnknownUnit as biome_analyze::Rule>::Options;
pub type NoUnmatchableAnbSelector = < lint :: correctness :: no_unmatchable_anb_selector :: NoUnmatchableAnbSelector as biome_analyze :: Rule > :: Options ;
pub type NoValueAtRule =
    <lint::nursery::no_value_at_rule::NoValueAtRule as biome_analyze::Rule>::Options;
pub type UseGenericFontNames =
    <lint::a11y::use_generic_font_names::UseGenericFontNames as biome_analyze::Rule>::Options;
pub type UseSortedProperties =
    <assist::source::use_sorted_properties::UseSortedProperties as biome_analyze::Rule>::Options;
