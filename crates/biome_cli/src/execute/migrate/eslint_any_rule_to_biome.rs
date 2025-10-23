//! Generated file, do not edit by hand, see `xtask/codegen`

use super::{eslint_eslint, eslint_to_biome};
pub(crate) fn migrate_eslint_any_rule(
    rules: &mut biome_configuration::Rules,
    eslint_name: &str,
    rule_severity: eslint_eslint::Severity,
    options: &eslint_to_biome::MigrationOptions,
    results: &mut eslint_to_biome::MigrationResults,
) -> bool {
    match eslint_name {
        "@eslint-react/no-forward-ref" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_react_forward_ref
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/no-nested-component-definitions" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_nested_component_definitions
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/no-nested-components" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_nested_component_definitions
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@eslint-react/no-useless-fragment" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_fragments
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@mysticatea/no-this-in-static" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_this_in_static
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/google-font-display" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_google_font_display
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/google-font-preconnect" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_google_font_preconnect
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/no-async-client-component" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_next_async_client_component
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/no-document-import-in-page" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_document_import_in_page
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/no-head-element" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_head_element
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/no-head-import-in-document" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_head_import_in_document
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/no-img-element" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_img_element
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/no-unwanted-polyfillio" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unwanted_polyfillio
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@stylistic/jsx-self-closing-comp" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_self_closing_elements
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/adjacent-overload-signatures" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_adjacent_overload_signatures
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/array-type" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_consistent_array_type
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/ban-ts-comment" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_ts_ignore
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/ban-types" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_banned_types
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/consistent-type-definitions" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_consistent_type_definitions
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/consistent-type-exports" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_export_type
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/consistent-type-imports" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_import_type
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/default-param-last" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_default_parameter_last
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/dot-notation" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_literal_keys
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/explicit-function-return-type" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_explicit_type
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/explicit-member-accessibility" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_consistent_member_accessibility
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/explicit-module-boundary-types" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_explicit_type
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/naming-convention" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_naming_convention
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-array-constructor" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_array_literals
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-deprecated" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_deprecated_imports
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-dupe-class-members" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_duplicate_class_members
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-empty-function" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_empty_block_statements
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-empty-interface" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_empty_interface
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-explicit-any" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_explicit_any
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-extra-non-null-assertion" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_extra_non_null_assertion
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-extraneous-class" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_static_only_class
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-floating-promises" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_floating_promises
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-inferrable-types" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_inferrable_types
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-invalid-void-type" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_confusing_void_type
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-loss-of-precision" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_precision_loss
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-magic-numbers" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_magic_numbers
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-misused-new" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_misleading_instantiator
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-misused-promises" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_misused_promises
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-namespace" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_namespace
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-non-null-asserted-optional-chain" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_non_null_asserted_optional_chain
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-non-null-assertion" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_non_null_assertion
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-redeclare" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_redeclare
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-require-imports" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_common_js
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-restricted-imports" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_restricted_imports
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-restricted-types" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_restricted_types
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-this-alias" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_this_alias
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unnecessary-condition" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unnecessary_conditions
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unnecessary-type-constraint" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_type_constraint
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unsafe-declaration-merging" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unsafe_declaration_merging
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-unused-vars" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unused_variables
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-use-before-define" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_invalid_use_before_declaration
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-useless-constructor" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_constructor
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-useless-empty-export" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_empty_export
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/only-throw-error" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_throw_only_error
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/parameter-properties" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_parameter_properties
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-as-const" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_as_const_assertion
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-enum-initializers" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_enum_initializers
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-for-of" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_for_of
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-function-type" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_shorthand_function_type
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-literal-enum-member" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_literal_enum_members
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-namespace-keyword" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_namespace_keyword
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-optional-chain" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_optional_chain
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/prefer-readonly" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_readonly_class_properties
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/require-await" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_await
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/switch-exhaustiveness-check" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_exhaustive_switch_cases
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/unified-signatures" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_unified_type_signatures
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "array-callback-return" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_iterable_callback_return
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "arrow-body-style" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_consistent_arrow_return
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "barrel-files/avoid-barrel-files" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_barrel_file
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "barrel-files/avoid-namespace-import" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_namespace_import
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "barrel-files/avoid-re-export-all" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_re_export_all
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "constructor-super" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_invalid_constructor_super
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "curly" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_block_statements
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "default-case" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_default_switch_clause
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "default-case-last" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_default_switch_clause_last
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "default-param-last" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_default_parameter_last
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "deno-lint/no-process-global" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_process_global
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "dot-notation" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_literal_keys
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "eqeqeq" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_double_equals
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "for-direction" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_valid_for_direction
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "getter-return" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_getter_return
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "grouped-accessor-pairs" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_grouped_accessor_pairs
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "guard-for-in" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_guard_for_in
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import-access/eslint-plugin-import-access" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_private_imports
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/exports-last" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_exports_last
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/named" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unresolved_imports
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-commonjs" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_common_js
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-cycle" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_import_cycles
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-default-export" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_default_export
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-deprecated" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_deprecated_imports
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-extraneous-dependencies" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_undeclared_dependencies
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-nodejs-modules" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_nodejs_modules
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/max-nested-describe" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_excessive_nested_test_suites
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-disabled-tests" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_skipped_tests
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-done-callback" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_done_callback
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-duplicate-hooks" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_duplicate_test_hooks
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-export" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_exports_in_test
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-focused-tests" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_focused_tests
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jest/no-standalone-expect" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_misplaced_assertion
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsdoc/no-multi-asterisks" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_single_js_doc_asterisk
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/alt-text" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_alt_text
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/anchor-has-content" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_anchor_content
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/anchor-is-valid" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_valid_anchor
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/aria-activedescendant-has-tabindex" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_aria_activedescendant_with_tabindex
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/aria-props" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_valid_aria_props
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/aria-proptypes" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_valid_aria_values
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/aria-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_valid_aria_role
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/aria-unsupported-elements" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_aria_unsupported_elements
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/autocomplete-valid" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_valid_autocomplete
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/click-events-have-key-events" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_key_with_click_events
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/heading-has-content" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_heading_content
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/html-has-lang" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_html_lang
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/iframe-has-title" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_iframe_title
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/img-redundant-alt" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_redundant_alt
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/interactive-supports-focus" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_focusable_interactive
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/label-has-associated-control" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_label_without_control
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/lang" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_valid_lang
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/media-has-caption" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_media_caption
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/mouse-events-have-key-events" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_key_with_mouse_events
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-access-key" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_access_key
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-aria-hidden-on-focusable" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_aria_hidden_on_focusable
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-autofocus" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_autofocus
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-distracting-elements" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_distracting_elements
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-interactive-element-to-noninteractive-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_interactive_element_to_noninteractive_role
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-noninteractive-element-interactions" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_noninteractive_element_interactions
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-noninteractive-element-to-interactive-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_noninteractive_element_to_interactive_role
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-noninteractive-tabindex" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_noninteractive_tabindex
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-redundant-roles" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_redundant_roles
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/no-static-element-interactions" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_static_element_interactions
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/prefer-tag-over-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_semantic_elements
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/role-has-required-aria-props" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_aria_props_for_role
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/role-supports-aria-props" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_aria_props_supported_by_role
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/scope" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_header_scope
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "jsx-a11y/tabindex-no-positive" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_positive_tabindex
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "max-lines-per-function" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_excessive_lines_per_function
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "max-params" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_max_params
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "n/no-process-env" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_process_env
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-alert" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_alert
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-array-constructor" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_array_literals
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-async-promise-executor" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_async_promise_executor
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-await-in-loop" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_await_in_loops
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-bitwise" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_bitwise_operators
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-case-declarations" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_switch_declarations
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-class-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_class_assign
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-compare-neg-zero" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_compare_neg_zero
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-cond-assign" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_assign_in_expressions
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-console" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_console
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-const-assign" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_const_assign
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-constant-binary-expression" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_constant_binary_expressions
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-constant-condition" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_constant_condition
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-constructor-return" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_constructor_return
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-control-regex" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_control_characters_in_regex
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-debugger" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_debugger
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-dupe-args" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_duplicate_parameters
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-dupe-class-members" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_duplicate_class_members
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-dupe-else-if" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_duplicate_else_if
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-dupe-keys" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_duplicate_object_keys
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-duplicate-case" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_duplicate_case
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-else-return" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_else
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-empty" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_empty_block_statements
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-empty-character-class" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_empty_character_class_in_regex
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-empty-function" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_empty_block_statements
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-empty-pattern" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_empty_pattern
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-empty-static-block" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_empty_block_statements
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-eval" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_global_eval
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-ex-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_catch_assign
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-extra-boolean-cast" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_extra_boolean_cast
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-extra-label" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_label
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-fallthrough" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_fallthrough_switch_clause
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-func-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_function_assign
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-global-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_global_assign
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-implicit-coercion" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_implicit_coercions
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-import-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_import_assign
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-inner-declarations" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_inner_declarations
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-irregular-whitespace" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_irregular_whitespace
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-label-var" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_label_var
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-labels" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_confusing_labels
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-lone-blocks" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_lone_block_statements
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-lonely-if" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_collapsed_else_if
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-loss-of-precision" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_precision_loss
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-misleading-character-class" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_misleading_character_class
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-negated-condition" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_negation_else
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-nested-ternary" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_nested_ternary
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-new-native-nonconstructor" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_invalid_builtin_instantiation
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-new-wrappers" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_consistent_builtin_instantiation
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-nonoctal-decimal-escape" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_nonoctal_decimal_escape
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-obj-calls" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_global_object_calls
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-octal-escape" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_octal_escape
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-param-reassign" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_parameter_assign
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-prototype-builtins" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_prototype_builtins
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-redeclare" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_redeclare
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-regex-spaces" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_adjacent_spaces_in_regex
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-restricted-globals" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_restricted_globals
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-restricted-imports" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_restricted_imports
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-secrets/no-secrets" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_secrets
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-self-assign" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_self_assign
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-self-compare" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_self_compare
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-sequences" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_comma_operator
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-setter-return" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_setter_return
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-shadow" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_shadow
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-shadow-restricted-names" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_shadow_restricted_names
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-sparse-arrays" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_sparse_array
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-template-curly-in-string" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_template_curly_in_string
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-this-before-super" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unreachable_super
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-throw-literal" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_throw_only_error
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unassigned-vars" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unassigned_variables
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-undef" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_undeclared_variables
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-undef-init" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_undefined_initialization
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unneeded-ternary" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_ternary
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unreachable" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unreachable
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unsafe-finally" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unsafe_finally
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unsafe-negation" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unsafe_negation
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unsafe-optional-chaining" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unsafe_optional_chaining
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unused-expressions" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unused_expressions
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unused-labels" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unused_labels
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unused-private-class-members" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unused_private_class_members
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-unused-vars" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unused_variables
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-use-before-define" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_invalid_use_before_declaration
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-backreference" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_regex_backrefs
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-catch" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_catch
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-computed-key" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_literal_keys
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-concat" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_string_concat
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-constructor" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_constructor
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-escape" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_escape_in_regex
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-useless-rename" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_rename
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-var" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_var
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-void" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_void
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-with" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_with
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "object-shorthand" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_consistent_object_definitions
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "one-var" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_single_var_declarator
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "operator-assignment" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_shorthand_assign
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "package-json-dependencies/duplicate-dependencies" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_duplicate_dependencies
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "package-json/unique-dependencies" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_duplicate_dependencies
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-arrow-callback" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_arrow_function
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-const" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_const
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-exponentiation-operator" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_exponentiation_operator
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-numeric-literals" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_numeric_literals
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-object-has-own" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_prototype_builtins
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-object-spread" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_object_spread
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-regex-literals" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_regex_literals
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-rest-params" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_arguments
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-template" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_template
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/jsx-a" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_valid_anchor
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/jsx-img" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_image_size
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/no-use-visible-task" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_qwik_use_visible_task
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/prefer-classlist" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_qwik_classlist
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/use-method-usage" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_qwik_method_usage
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "qwik/valid-lexical-scope" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_qwik_valid_lexical_scope
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "radix" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_parse_int_radix
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-hooks/exhaustive-deps" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_exhaustive_dependencies
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-hooks/react-compiler" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_react_prop_assignments
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-hooks/rules-of-hooks" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_hook_at_top_level
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-prefer-function-component/react-prefer-function-component" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_react_function_components
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-refresh/only-export-components" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_component_export_only_modules
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-x/no-forward-ref" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_react_forward_ref
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-x/no-nested-components" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_nested_component_definitions
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react-x/no-useless-fragment" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_fragments
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/button-has-type" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_button_type
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/forbid-elements" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_restricted_elements
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-boolean-value" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_implicit_boolean
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-curly-brace-presence" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_consistent_curly_braces
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-fragments" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_fragment_syntax
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-key" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_jsx_key_in_iterable
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-no-comment-textnodes" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_comment_text
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-no-duplicate-props" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_duplicate_jsx_props
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-no-literals" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_jsx_literals
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-no-target-blank" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_blank_target
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/jsx-no-useless-fragment" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_fragments
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/no-array-index-key" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_array_index_key
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/no-children-prop" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_children_prop
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/no-danger" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_dangerously_set_inner_html
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/no-danger-with-children" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_dangerously_set_inner_html_with_children
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "react/void-dom-elements-no-children" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_void_elements_with_children
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "regexp/no-useless-backreference" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_regex_backrefs
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "require-await" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_await
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "require-yield" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_yield
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "solidjs/no-destructure" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_solid_destructured_props
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "solidjs/no-react-specific-props" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_react_specific_props
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "solidjs/prefer-for" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_solid_for_component
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "sonarjs/cognitive-complexity" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_excessive_cognitive_complexity
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "sonarjs/prefer-while" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_while
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "symbol-description" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_symbol_description
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/error-message" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_error_message
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/explicit-length-check" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_explicit_length_check
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/filename-case" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_filenaming_convention
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/new-for-builtins" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_invalid_builtin_instantiation
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-array-for-each" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_for_each
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-document-cookie" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_document_cookie
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-empty-file" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_empty_source
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-for-loop" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_for_of
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-instanceof-array" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_is_array
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-lonely-if" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_collapsed_if
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-static-only-class" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_static_only_class
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-thenable" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_then_property
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-useless-switch-case" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_switch_case
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/no-useless-undefined" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_undefined
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/numeric-separators-style" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_numeric_separators
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-array-flat-map" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_flat_map
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-array-index-of" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_index_of
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-at" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_at_index
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-date-now" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_date_now
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-module" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_global_dirname_filename
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-node-protocol" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_nodejs_import_protocol
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-number-properties" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_number_namespace
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-string-slice" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_substr
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-string-trim-start-end" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_trim_start_end
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/require-number-to-fixed-digits-argument" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_number_to_fixed_digits_argument
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/throw-new-error" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_throw_new_error
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unused-imports/no-unused-imports" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unused_imports
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unused-imports/no-unused-vars" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_unused_variables
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "use-isnan" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_is_nan
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "valid-typeof" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_valid_typeof
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/max-nested-describe" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_excessive_nested_test_suites
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/no-disabled-tests" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_skipped_tests
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/no-done-callback" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_done_callback
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/no-duplicate-hooks" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_duplicate_test_hooks
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/no-focused-tests" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_focused_tests
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vitest/no-standalone-expect" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_misplaced_assertion
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/define-macros-order" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_vue_define_macros_order
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/multi-word-component-names" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_vue_multi_word_component_names
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-deprecated-data-object-declaration" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_vue_data_object_declaration
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-dupe-keys" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_vue_duplicate_keys
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-reserved-keys" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_vue_reserved_keys
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-reserved-props" => {
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_vue_reserved_props
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "vue/no-shared-component-data" => {
            if !options.include_inspired {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Inspired);
                return false;
            }
            if !options.include_nursery {
                results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Nursery);
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_vue_data_object_declaration
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "yoda" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_yoda_expression
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        _ => {
            results.add(
                eslint_name,
                eslint_to_biome::RuleMigrationResult::Unsupported,
            );
            return false;
        }
    }
    results.add(eslint_name, eslint_to_biome::RuleMigrationResult::Migrated);
    true
}
