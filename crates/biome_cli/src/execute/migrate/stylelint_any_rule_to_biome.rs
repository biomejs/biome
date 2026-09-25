//! Generated file, do not edit by hand, see `xtask/codegen`

use super::{migration, stylelint_to_biome};
pub(crate) fn migrate_stylelint_any_rule(
    rules: &mut biome_configuration::Rules,
    stylelint_name: &str,
    rule_level: biome_configuration::RulePlainConfiguration,
    options: &migration::MigrationOptions,
    results: &mut stylelint_to_biome::StylelintMigrationResults,
) -> bool {
    match stylelint_name {
        "at-rule-no-unknown" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unknown_at_rules
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "block-no-empty" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_empty_block
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "color-no-hex" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_hex_colors
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "custom-property-no-missing-var-function" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_missing_var_function
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "declaration-block-no-duplicate-custom-properties" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_duplicate_custom_properties
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "declaration-block-no-duplicate-properties" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_duplicate_properties
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "declaration-block-no-shorthand-property-overrides" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_shorthand_property_overrides
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "declaration-no-important" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_important_styles
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "font-family-no-duplicate-names" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_duplicate_font_names
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "font-family-no-missing-generic-family-keyword" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_generic_font_names
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "function-linear-gradient-no-nonstandard-direction" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_invalid_direction_in_linear_gradient
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "function-no-unknown" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unknown_function
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "keyframe-block-no-duplicate-selectors" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_duplicate_selectors_keyframe_block
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "keyframe-declaration-no-important" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_important_in_keyframe
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "media-feature-name-no-unknown" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unknown_media_feature_name
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "media-type-no-deprecated" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_deprecated_media_type
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "named-grid-areas-no-invalid" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_invalid_grid_areas
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "no-descending-specificity" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_descending_specificity
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "no-duplicate-at-import-rules" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_duplicate_at_import_rules
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "no-duplicate-selectors" => {
            if !options.include_nursery {
                results.add(stylelint_name, migration::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_duplicate_selectors
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "no-empty-source" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_empty_source
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "no-invalid-position-at-import-rule" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_invalid_position_at_import_rule
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "no-irregular-whitespace" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_irregular_whitespace
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "property-no-unknown" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unknown_property
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "selector-anb-no-unmatchable" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unmatchable_anb_selector
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "selector-max-class" => {
            if !options.include_nursery {
                results.add(stylelint_name, migration::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_excessive_selector_classes
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "selector-pseudo-class-no-unknown" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unknown_pseudo_class
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "selector-pseudo-element-no-unknown" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unknown_pseudo_element
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "selector-type-no-unknown" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unknown_type_selector
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        "unit-no-unknown" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unknown_unit
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_level));
        }
        _ => {
            results.add(stylelint_name, migration::RuleMigrationResult::Unsupported);
            return false;
        }
    }
    results.add(stylelint_name, migration::RuleMigrationResult::Migrated);
    true
}
