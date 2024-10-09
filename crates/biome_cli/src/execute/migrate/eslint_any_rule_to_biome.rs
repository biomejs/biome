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
        "@mysticatea/no-this-in-static" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_this_in_static.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@next/no-head-element" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.no_head_element.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@next/no-head-import-in-document" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .no_head_import_in_document
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@next/no-img-element" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.no_img_element.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@stylistic/jsx-self-closing-comp" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_self_closing_elements
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/adjacent-overload-signatures" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .use_adjacent_overload_signatures
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/array-type" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_consistent_array_type
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/ban-types" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_banned_types.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/consistent-type-exports" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_export_type.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/consistent-type-imports" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_import_type.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/default-param-last" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_default_parameter_last
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/dot-notation" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.use_literal_keys.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/explicit-function-return-type" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .use_explicit_function_return_type
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/explicit-member-accessibility" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .use_consistent_member_accessibility
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/naming-convention" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_naming_convention
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-dupe-class-members" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_duplicate_class_members
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-empty-function" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_empty_block_statements
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-empty-interface" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_empty_interface.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-explicit-any" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_explicit_any.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-extra-non-null-assertion" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_extra_non_null_assertion
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-extraneous-class" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_static_only_class.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-inferrable-types" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_inferrable_types.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-invalid-void-type" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_confusing_void_type
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-loss-of-precision" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_precision_loss.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-misused-new" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_misleading_instantiator
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-namespace" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_namespace.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-non-null-assertion" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .no_non_null_assertion
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-redeclare" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_redeclare.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-require-imports" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.no_common_js.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-restricted-imports" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .no_restricted_imports
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-restricted-types" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.no_restricted_types.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-this-alias" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_useless_this_alias
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-unnecessary-type-constraint" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_useless_type_constraint
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-unsafe-declaration-merging" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_unsafe_declaration_merging
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-unused-vars" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_unused_variables.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-use-before-define" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_invalid_use_before_declaration
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-useless-constructor" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_useless_constructor
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/no-useless-empty-export" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_useless_empty_export
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/only-throw-error" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_throw_only_error.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/parameter-properties" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .no_parameter_properties
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/prefer-as-const" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_as_const_assertion
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/prefer-enum-initializers" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_enum_initializers
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/prefer-for-of" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_for_of.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/prefer-function-type" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_shorthand_function_type
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/prefer-literal-enum-member" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_literal_enum_members
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/prefer-namespace-keyword" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .use_namespace_keyword
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/prefer-optional-chain" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.use_optional_chain.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "@typescript-eslint/require-await" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.use_await.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "barrel-files/avoid-barrel-files" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group.no_barrel_file.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "barrel-files/avoid-namespace-import" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_namespace_import.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "barrel-files/avoid-re-export-all" => {
            let group = rules.performance.get_or_insert_with(Default::default);
            let rule = group.no_re_export_all.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "constructor-super" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_invalid_constructor_super
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "curly" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_block_statements.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "default-case" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_default_switch_clause
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "default-case-last" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .use_default_switch_clause_last
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "default-param-last" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_default_parameter_last
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "dot-notation" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.use_literal_keys.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "eqeqeq" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_double_equals.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "for-direction" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .use_valid_for_direction
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "getter-return" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.use_getter_return.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "import-access/eslint-plugin-import-access" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .use_import_restrictions
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "import/no-commonjs" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.no_common_js.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "import/no-default-export" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_default_export.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "import/no-nodejs-modules" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_nodejs_modules.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jest/max-nested-describe" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_excessive_nested_test_suites
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jest/no-disabled-tests" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_skipped_tests.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jest/no-done-callback" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_done_callback.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jest/no-duplicate-hooks" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_duplicate_test_hooks
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jest/no-export" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_exports_in_test.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jest/no-focused-tests" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_focused_tests.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jest/no-standalone-expect" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_misplaced_assertion
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/alt-text" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_alt_text.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/anchor-has-content" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_anchor_content.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/anchor-is-valid" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_valid_anchor.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/aria-activedescendant-has-tabindex" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .use_aria_activedescendant_with_tabindex
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/aria-props" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_valid_aria_props.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/aria-proptypes" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .use_valid_aria_values
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/aria-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_valid_aria_role.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/aria-unsupported-elements" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .no_aria_unsupported_elements
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/autocomplete-valid" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .use_valid_autocomplete
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/click-events-have-key-events" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .use_key_with_click_events
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/heading-has-content" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_heading_content.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/html-has-lang" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_html_lang.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/iframe-has-title" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_iframe_title.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/img-redundant-alt" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.no_redundant_alt.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/interactive-supports-focus" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .use_focusable_interactive
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/label-has-associated-control" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .no_label_without_control
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/lang" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_valid_lang.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/media-has-caption" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_media_caption.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/mouse-events-have-key-events" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .use_key_with_mouse_events
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/no-access-key" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.no_access_key.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/no-aria-hidden-on-focusable" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .no_aria_hidden_on_focusable
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/no-autofocus" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.no_autofocus.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/no-distracting-elements" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .no_distracting_elements
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/no-interactive-element-to-noninteractive-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .no_interactive_element_to_noninteractive_role
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/no-noninteractive-element-to-interactive-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .no_noninteractive_element_to_interactive_role
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/no-noninteractive-tabindex" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .no_noninteractive_tabindex
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/no-redundant-roles" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.no_redundant_roles.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/no-static-element-interactions" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .no_static_element_interactions
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/prefer-tag-over-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .use_semantic_elements
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/role-has-required-aria-props" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .use_aria_props_for_role
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/role-supports-aria-props" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .use_aria_props_supported_by_role
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/scope" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.no_header_scope.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "jsx-a11y/tabindex-no-positive" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.no_positive_tabindex.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "n/no-process-env" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.no_process_env.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-array-constructor" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.use_array_literals.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-async-promise-executor" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_async_promise_executor
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-case-declarations" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_switch_declarations
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-class-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_class_assign.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-compare-neg-zero" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_compare_neg_zero.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-cond-assign" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_assign_in_expressions
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-console" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_console.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-const-assign" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_const_assign.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-constant-condition" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_constant_condition
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-constructor-return" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_constructor_return
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-control-regex" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_control_characters_in_regex
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-debugger" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_debugger.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-dupe-args" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_duplicate_parameters
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-dupe-class-members" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_duplicate_class_members
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-dupe-else-if" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.no_duplicate_else_if.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-dupe-keys" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_duplicate_object_keys
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-duplicate-case" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_duplicate_case.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-else-return" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_useless_else.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-empty" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_empty_block_statements
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-empty-character-class" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_empty_character_class_in_regex
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-empty-function" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_empty_block_statements
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-empty-pattern" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_empty_pattern.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-empty-static-block" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_empty_block_statements
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-eval" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group.no_global_eval.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-ex-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_catch_assign.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-extra-boolean-cast" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_extra_boolean_cast
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-extra-label" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_useless_label.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-fallthrough" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_fallthrough_switch_clause
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-func-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_function_assign.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-global-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_global_assign.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-import-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_import_assign.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-inner-declarations" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_inner_declarations
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-irregular-whitespace" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .no_irregular_whitespace
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-label-var" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_label_var.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-labels" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_confusing_labels.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-lone-blocks" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_useless_lone_block_statements
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-lonely-if" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_collapsed_else_if
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-loss-of-precision" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_precision_loss.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-misleading-character-class" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_misleading_character_class
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-negated-condition" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_negation_else.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-nested-ternary" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.no_nested_ternary.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-new-native-nonconstructor" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_invalid_builtin_instantiation
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-new-symbol" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_new_symbol.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-new-wrappers" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_consistent_builtin_instantiation
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-nonoctal-decimal-escape" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_nonoctal_decimal_escape
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-obj-calls" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_global_object_calls
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-octal-escape" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.no_octal_escape.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-param-reassign" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_parameter_assign.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-prototype-builtins" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_prototype_builtins
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-redeclare" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_redeclare.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-regex-spaces" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_multiple_spaces_in_regular_expression_literals
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-restricted-globals" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .no_restricted_globals
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-restricted-imports" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .no_restricted_imports
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-secrets/no-secrets" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.no_secrets.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-self-assign" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_self_assign.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-self-compare" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_self_compare.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-sequences" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_comma_operator.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-setter-return" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_setter_return.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-shadow-restricted-names" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_shadow_restricted_names
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-sparse-arrays" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_sparse_array.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-template-curly-in-string" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .no_template_curly_in_string
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-this-before-super" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_unreachable_super.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-throw-literal" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_throw_only_error.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-undef" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_undeclared_variables
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-undef-init" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_useless_undefined_initialization
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-unneeded-ternary" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_useless_ternary.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-unreachable" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_unreachable.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-unsafe-finally" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_unsafe_finally.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-unsafe-negation" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_unsafe_negation.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-unsafe-optional-chaining" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_unsafe_optional_chaining
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-unused-labels" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_unused_labels.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-unused-private-class-members" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_unused_private_class_members
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-unused-vars" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_unused_variables.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-use-before-define" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_invalid_use_before_declaration
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-useless-catch" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_useless_catch.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-useless-concat" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_useless_string_concat
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-useless-constructor" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_useless_constructor
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-useless-escape" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .no_useless_escape_in_regex
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-useless-rename" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_useless_rename.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-var" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_var.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-void" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_void.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "no-with" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_with.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "one-var" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_single_var_declarator
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "operator-assignment" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_shorthand_assign.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "prefer-arrow-callback" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.use_arrow_function.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "prefer-const" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_const.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "prefer-exponentiation-operator" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_exponentiation_operator
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "prefer-numeric-literals" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_numeric_literals.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "prefer-regex-literals" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.use_regex_literals.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "prefer-rest-params" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_arguments.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "prefer-template" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_template.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "react-hooks/exhaustive-deps" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .use_exhaustive_dependencies
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "react-hooks/rules-of-hooks" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .use_hook_at_top_level
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "react-refresh/only-export-components" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .use_component_export_only_modules
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "react/button-has-type" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_button_type.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "react/jsx-boolean-value" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_implicit_boolean.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "react/jsx-curly-brace-presence" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .use_consistent_curly_braces
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "react/jsx-fragments" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_fragment_syntax.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "react/jsx-key" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .use_jsx_key_in_iterable
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "react/jsx-no-comment-textnodes" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_comment_text.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "react/jsx-no-duplicate-props" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_duplicate_jsx_props
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "react/jsx-no-target-blank" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.no_blank_target.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "react/jsx-no-useless-fragment" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_useless_fragments.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "react/no-array-index-key" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_array_index_key.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "react/no-children-prop" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_children_prop.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "react/no-danger" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .no_dangerously_set_inner_html
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "react/no-danger-with-children" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .no_dangerously_set_inner_html_with_children
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "react/void-dom-elements-no-children" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_void_elements_with_children
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "require-await" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.use_await.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "require-yield" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.use_yield.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "solidjs/no-react-specific-props" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_react_specific_props
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "sonarjs/cognitive-complexity" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_excessive_cognitive_complexity
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "sonarjs/prefer-while" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_while.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "unicorn/error-message" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.use_error_message.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "unicorn/explicit-length-check" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_explicit_length_check
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "unicorn/filename-case" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_filenaming_convention
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "unicorn/new-for-builtins" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_invalid_builtin_instantiation
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "unicorn/no-array-for-each" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_for_each.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "unicorn/no-for-loop" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_for_of.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "unicorn/no-instanceof-array" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.use_is_array.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "unicorn/no-static-only-class" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_static_only_class.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "unicorn/no-thenable" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_then_property.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "unicorn/no-useless-switch-case" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_useless_switch_case
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "unicorn/prefer-array-flat-map" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.use_flat_map.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "unicorn/prefer-at" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.use_at_index.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "unicorn/prefer-date-now" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.use_date_now.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "unicorn/prefer-node-protocol" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_nodejs_import_protocol
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "unicorn/prefer-number-properties" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_number_namespace.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "unicorn/prefer-string-slice" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.no_substr.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "unicorn/prefer-string-trim-start-end" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.use_trim_start_end.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "unicorn/require-number-to-fixed-digits-argument" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .use_number_to_fixed_digits_argument
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "unicorn/throw-new-error" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_throw_new_error.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "unused-imports/no-unused-imports" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_unused_imports.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "unused-imports/no-unused-vars" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_unused_variables.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "use-isnan" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.use_is_nan.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "valid-typeof" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.use_valid_typeof.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        "yoda" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_yoda_expression.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
        }
        _ => {
            return false;
        }
    }
    true
}
