//! Generated file, do not edit by hand, see `xtask/codegen`

use super::{eslint, eslint_to_biome};
pub(crate) fn migrate_eslint_any_rule(
    rules: &mut biome_service::Rules,
    eslint_name: &str,
    rule_severity: eslint::Severity,
    options: &eslint_to_biome::MigrationOptions,
    results: &mut eslint_to_biome::MigrationResults,
) -> bool {
    match eslint_name {
        "@mysticatea/no-this-in-static" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_this_in_static.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("@mysticatea/no-this-in-static");
        }
        "@stylistic/jsx-self-closing-comp" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_self_closing_elements
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@stylistic/jsx-self-closing-comp");
        }
        "@typescript-eslint/array-type" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_consistent_array_type
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("@typescript-eslint/array-type");
        }
        "@typescript-eslint/ban-types" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_banned_types.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("@typescript-eslint/ban-types");
        }
        "@typescript-eslint/consistent-type-exports" => {
            if !options.include_inspired {
                results
                    .inspired_rules
                    .push("@typescript-eslint/consistent-type-exports");
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_export_type.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/consistent-type-exports");
        }
        "@typescript-eslint/consistent-type-imports" => {
            if !options.include_inspired {
                results
                    .inspired_rules
                    .push("@typescript-eslint/consistent-type-imports");
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_import_type.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/consistent-type-imports");
        }
        "@typescript-eslint/naming-convention" => {
            if !options.include_inspired {
                results
                    .inspired_rules
                    .push("@typescript-eslint/naming-convention");
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_naming_convention
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/naming-convention");
        }
        "@typescript-eslint/no-empty-interface" => {
            if !options.include_inspired {
                results
                    .inspired_rules
                    .push("@typescript-eslint/no-empty-interface");
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_empty_interface.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/no-empty-interface");
        }
        "@typescript-eslint/no-explicit-any" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_explicit_any.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/no-explicit-any");
        }
        "@typescript-eslint/no-extra-non-null-assertion" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_extra_non_null_assertion
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/no-extra-non-null-assertion");
        }
        "@typescript-eslint/no-extraneous-class" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_static_only_class.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/no-extraneous-class");
        }
        "@typescript-eslint/no-inferrable-types" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_inferrable_types.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/no-inferrable-types");
        }
        "@typescript-eslint/no-invalid-void-type" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_confusing_void_type
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/no-invalid-void-type");
        }
        "@typescript-eslint/no-misused-new" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_misleading_instantiator
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/no-misused-new");
        }
        "@typescript-eslint/no-namespace" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_namespace.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/no-namespace");
        }
        "@typescript-eslint/no-non-null-assertion" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .no_non_null_assertion
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/no-non-null-assertion");
        }
        "@typescript-eslint/no-redeclare" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_redeclare.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/no-redeclare");
        }
        "@typescript-eslint/no-this-alias" => {
            if !options.include_inspired {
                results
                    .inspired_rules
                    .push("@typescript-eslint/no-this-alias");
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_useless_this_alias
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/no-this-alias");
        }
        "@typescript-eslint/no-unnecessary-type-constraint" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_useless_type_constraint
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/no-unnecessary-type-constraint");
        }
        "@typescript-eslint/no-unsafe-declaration-merging" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_unsafe_declaration_merging
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/no-unsafe-declaration-merging");
        }
        "@typescript-eslint/no-use-before-define" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_invalid_use_before_declaration
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/no-use-before-define");
        }
        "@typescript-eslint/no-useless-constructor" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_useless_constructor
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/no-useless-constructor");
        }
        "@typescript-eslint/no-useless-empty-export" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_useless_empty_export
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/no-useless-empty-export");
        }
        "@typescript-eslint/no-useless-template-literals" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .no_unused_template_literal
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/no-useless-template-literals");
        }
        "@typescript-eslint/parameter-properties" => {
            if !options.include_inspired {
                results
                    .inspired_rules
                    .push("@typescript-eslint/parameter-properties");
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .no_parameter_properties
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/parameter-properties");
        }
        "@typescript-eslint/prefer-as-const" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_as_const_assertion
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/prefer-as-const");
        }
        "@typescript-eslint/prefer-enum-initializers" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_enum_initializers
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/prefer-enum-initializers");
        }
        "@typescript-eslint/prefer-for-of" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_for_of.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/prefer-for-of");
        }
        "@typescript-eslint/prefer-function-type" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_shorthand_function_type
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/prefer-function-type");
        }
        "@typescript-eslint/prefer-literal-enum-member" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_literal_enum_members
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/prefer-literal-enum-member");
        }
        "@typescript-eslint/prefer-namespace-keyword" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .use_namespace_keyword
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/prefer-namespace-keyword");
        }
        "@typescript-eslint/prefer-optional-chain" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.use_optional_chain.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("@typescript-eslint/prefer-optional-chain");
        }
        "barrel-files/avoid-namespace-import" => {
            if !options.include_nursery {
                results
                    .nursery_rules
                    .push("barrel-files/avoid-namespace-import");
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.no_namespace_import.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("barrel-files/avoid-namespace-import");
        }
        "barrel-files/avoid-re-export-all" => {
            if !options.include_nursery {
                results
                    .nursery_rules
                    .push("barrel-files/avoid-re-export-all");
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.no_re_export_all.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("barrel-files/avoid-re-export-all");
        }
        "constructor-super" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_invalid_constructor_super
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("constructor-super");
        }
        "curly" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_block_statements.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("curly");
        }
        "default-case-last" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .use_default_switch_clause_last
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("default-case-last");
        }
        "default-param-last" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_default_parameter_last
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("default-param-last");
        }
        "dot-notation" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.use_literal_keys.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("dot-notation");
        }
        "eqeqeq" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_double_equals.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("eqeqeq");
        }
        "for-direction" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .use_valid_for_direction
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("for-direction");
        }
        "getter-return" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.use_getter_return.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("getter-return");
        }
        "import-access/eslint-plugin-import-access" => {
            if !options.include_inspired {
                results
                    .inspired_rules
                    .push("import-access/eslint-plugin-import-access");
                return false;
            }
            if !options.include_nursery {
                results
                    .nursery_rules
                    .push("import-access/eslint-plugin-import-access");
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .use_import_restrictions
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("import-access/eslint-plugin-import-access");
        }
        "import/no-default-export" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_default_export.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("import/no-default-export");
        }
        "import/no-nodejs-modules" => {
            if !options.include_nursery {
                results.nursery_rules.push("import/no-nodejs-modules");
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.no_nodejs_modules.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("import/no-nodejs-modules");
        }
        "jest/max-nested-describe" => {
            if !options.include_nursery {
                results.nursery_rules.push("jest/max-nested-describe");
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .no_excessive_nested_test_suites
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("jest/max-nested-describe");
        }
        "jest/no-disabled-tests" => {
            if !options.include_inspired {
                results.inspired_rules.push("jest/no-disabled-tests");
                return false;
            }
            if !options.include_nursery {
                results.nursery_rules.push("jest/no-disabled-tests");
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.no_skipped_tests.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("jest/no-disabled-tests");
        }
        "jest/no-done-callback" => {
            if !options.include_nursery {
                results.nursery_rules.push("jest/no-done-callback");
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.no_done_callback.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("jest/no-done-callback");
        }
        "jest/no-export" => {
            if !options.include_inspired {
                results.inspired_rules.push("jest/no-export");
                return false;
            }
            if !options.include_nursery {
                results.nursery_rules.push("jest/no-export");
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.no_exports_in_test.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("jest/no-export");
        }
        "jest/no-focused-tests" => {
            if !options.include_inspired {
                results.inspired_rules.push("jest/no-focused-tests");
                return false;
            }
            if !options.include_nursery {
                results.nursery_rules.push("jest/no-focused-tests");
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.no_focused_tests.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("jest/no-focused-tests");
        }
        "jsx-a11y/alt-text" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_alt_text.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("jsx-a11y/alt-text");
        }
        "jsx-a11y/anchor-has-content" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_anchor_content.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("jsx-a11y/anchor-has-content");
        }
        "jsx-a11y/anchor-is-valid" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_valid_anchor.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("jsx-a11y/anchor-is-valid");
        }
        "jsx-a11y/aria-activedescendant-has-tabindex" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .use_aria_activedescendant_with_tabindex
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("jsx-a11y/aria-activedescendant-has-tabindex");
        }
        "jsx-a11y/aria-props" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_valid_aria_props.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("jsx-a11y/aria-props");
        }
        "jsx-a11y/aria-proptypes" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .use_valid_aria_values
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("jsx-a11y/aria-proptypes");
        }
        "jsx-a11y/aria-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_valid_aria_role.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("jsx-a11y/aria-role");
        }
        "jsx-a11y/aria-unsupported-elements" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .no_aria_unsupported_elements
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("jsx-a11y/aria-unsupported-elements");
        }
        "jsx-a11y/click-events-have-key-events" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .use_key_with_click_events
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("jsx-a11y/click-events-have-key-events");
        }
        "jsx-a11y/heading-has-content" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_heading_content.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("jsx-a11y/heading-has-content");
        }
        "jsx-a11y/html-has-lang" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_html_lang.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("jsx-a11y/html-has-lang");
        }
        "jsx-a11y/iframe-has-title" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_iframe_title.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("jsx-a11y/iframe-has-title");
        }
        "jsx-a11y/lang" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_valid_lang.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("jsx-a11y/lang");
        }
        "jsx-a11y/media-has-caption" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_media_caption.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("jsx-a11y/media-has-caption");
        }
        "jsx-a11y/mouse-events-have-key-events" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .use_key_with_mouse_events
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("jsx-a11y/mouse-events-have-key-events");
        }
        "jsx-a11y/no-access-key" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.no_access_key.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("jsx-a11y/no-access-key");
        }
        "jsx-a11y/no-aria-hidden-on-focusable" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .no_aria_hidden_on_focusable
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("jsx-a11y/no-aria-hidden-on-focusable");
        }
        "jsx-a11y/no-autofocus" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.no_autofocus.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("jsx-a11y/no-autofocus");
        }
        "jsx-a11y/no-distracting-elements" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .no_distracting_elements
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("jsx-a11y/no-distracting-elements");
        }
        "jsx-a11y/no-interactive-element-to-noninteractive-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .no_interactive_element_to_noninteractive_role
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("jsx-a11y/no-interactive-element-to-noninteractive-role");
        }
        "jsx-a11y/no-noninteractive-element-to-interactive-role" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .no_noninteractive_element_to_interactive_role
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("jsx-a11y/no-noninteractive-element-to-interactive-role");
        }
        "jsx-a11y/no-noninteractive-tabindex" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .no_noninteractive_tabindex
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("jsx-a11y/no-noninteractive-tabindex");
        }
        "jsx-a11y/no-redundant-roles" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.no_redundant_roles.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("jsx-a11y/no-redundant-roles");
        }
        "jsx-a11y/role-has-required-aria-props" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group
                .use_aria_props_for_role
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("jsx-a11y/role-has-required-aria-props");
        }
        "jsx-a11y/scope" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.no_header_scope.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("jsx-a11y/scope");
        }
        "jsx-a11y/tabindex-no-positive" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.no_positive_tabindex.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("jsx-a11y/tabindex-no-positive");
        }
        "no-async-promise-executor" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_async_promise_executor
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-async-promise-executor");
        }
        "no-case-declarations" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_switch_declarations
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-case-declarations");
        }
        "no-class-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_class_assign.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-class-assign");
        }
        "no-compare-neg-zero" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_compare_neg_zero.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-compare-neg-zero");
        }
        "no-cond-assign" => {
            if !options.include_inspired {
                results.inspired_rules.push("no-cond-assign");
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_assign_in_expressions
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-cond-assign");
        }
        "no-console" => {
            if !options.include_inspired {
                results.inspired_rules.push("no-console");
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_console_log.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-console");
        }
        "no-const-assign" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_const_assign.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-const-assign");
        }
        "no-constant-condition" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_constant_condition
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-constant-condition");
        }
        "no-constructor-return" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_constructor_return
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-constructor-return");
        }
        "no-control-regex" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_control_characters_in_regex
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-control-regex");
        }
        "no-debugger" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_debugger.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-debugger");
        }
        "no-dupe-args" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_duplicate_parameters
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-dupe-args");
        }
        "no-dupe-class-members" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_duplicate_class_members
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-dupe-class-members");
        }
        "no-dupe-else-if" => {
            if !options.include_nursery {
                results.nursery_rules.push("no-dupe-else-if");
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.no_duplicate_else_if.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-dupe-else-if");
        }
        "no-dupe-keys" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_duplicate_object_keys
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-dupe-keys");
        }
        "no-duplicate-case" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_duplicate_case.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-duplicate-case");
        }
        "no-else-return" => {
            if !options.include_inspired {
                results.inspired_rules.push("no-else-return");
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_useless_else.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-else-return");
        }
        "no-empty" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_empty_block_statements
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-empty");
        }
        "no-empty-character-class" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_empty_character_class_in_regex
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-empty-character-class");
        }
        "no-empty-pattern" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_empty_pattern.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-empty-pattern");
        }
        "no-eval" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group.no_global_eval.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-eval");
        }
        "no-ex-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_catch_assign.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-ex-assign");
        }
        "no-extra-boolean-cast" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_extra_boolean_cast
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-extra-boolean-cast");
        }
        "no-extra-label" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_useless_label.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-extra-label");
        }
        "no-fallthrough" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_fallthrough_switch_clause
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-fallthrough");
        }
        "no-func-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_function_assign.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-func-assign");
        }
        "no-global-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_global_assign.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-global-assign");
        }
        "no-import-assign" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_import_assign.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-import-assign");
        }
        "no-inner-declarations" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_inner_declarations
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-inner-declarations");
        }
        "no-label-var" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_label_var.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-label-var");
        }
        "no-labels" => {
            if !options.include_inspired {
                results.inspired_rules.push("no-labels");
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_confusing_labels.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-labels");
        }
        "no-lone-blocks" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_useless_lone_block_statements
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-lone-blocks");
        }
        "no-lonely-if" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_collapsed_else_if
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-lonely-if");
        }
        "no-loss-of-precision" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_precision_loss.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-loss-of-precision");
        }
        "no-misleading-character-class" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_misleading_character_class
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-misleading-character-class");
        }
        "no-negated-condition" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_negation_else.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-negated-condition");
        }
        "no-new-native-nonconstructor" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_invalid_new_builtin
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-new-native-nonconstructor");
        }
        "no-new-symbol" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_new_symbol.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-new-symbol");
        }
        "no-nonoctal-decimal-escape" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_nonoctal_decimal_escape
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-nonoctal-decimal-escape");
        }
        "no-obj-calls" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_global_object_calls
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-obj-calls");
        }
        "no-param-reassign" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_parameter_assign.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-param-reassign");
        }
        "no-prototype-builtins" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_prototype_builtins
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-prototype-builtins");
        }
        "no-regex-spaces" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_multiple_spaces_in_regular_expression_literals
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-regex-spaces");
        }
        "no-restricted-globals" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .no_restricted_globals
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-restricted-globals");
        }
        "no-restricted-imports" => {
            if !options.include_nursery {
                results.nursery_rules.push("no-restricted-imports");
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .no_restricted_imports
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-restricted-imports");
        }
        "no-self-assign" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_self_assign.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-self-assign");
        }
        "no-self-compare" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_self_compare.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-self-compare");
        }
        "no-sequences" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_comma_operator.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-sequences");
        }
        "no-setter-return" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_setter_return.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-setter-return");
        }
        "no-shadow-restricted-names" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_shadow_restricted_names
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-shadow-restricted-names");
        }
        "no-sparse-array" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_sparse_array.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-sparse-array");
        }
        "no-this-before-super" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_unreachable_super.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-this-before-super");
        }
        "no-undef" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_undeclared_variables
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-undef");
        }
        "no-unneeded-ternary" => {
            if !options.include_nursery {
                results.nursery_rules.push("no-unneeded-ternary");
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group.no_useless_ternary.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-unneeded-ternary");
        }
        "no-unreachable" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_unreachable.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-unreachable");
        }
        "no-unsafe-finally" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_unsafe_finally.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-unsafe-finally");
        }
        "no-unsafe-negation" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_unsafe_negation.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-unsafe-negation");
        }
        "no-unsafe-optional-chaining" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_unsafe_optional_chaining
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-unsafe-optional-chaining");
        }
        "no-unused-labels" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_unused_labels.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-unused-labels");
        }
        "no-unused-private-class-members" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_unused_private_class_members
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("no-unused-private-class-members");
        }
        "no-unused-vars" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_unused_variables.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-unused-vars");
        }
        "no-useless-catch" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_useless_catch.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-useless-catch");
        }
        "no-useless-rename" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_useless_rename.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-useless-rename");
        }
        "no-var" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_var.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-var");
        }
        "no-void" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_void.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-void");
        }
        "no-with" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_with.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("no-with");
        }
        "one-var" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_single_var_declarator
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("one-var");
        }
        "operator-assignment" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_shorthand_assign.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("operator-assignment");
        }
        "prefer-arrow-callback" => {
            if !options.include_inspired {
                results.inspired_rules.push("prefer-arrow-callback");
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.use_arrow_function.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("prefer-arrow-callback");
        }
        "prefer-const" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_const.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("prefer-const");
        }
        "prefer-exponentiation-operator" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_exponentiation_operator
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("prefer-exponentiation-operator");
        }
        "prefer-numeric-literals" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_numeric_literals.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("prefer-numeric-literals");
        }
        "prefer-regex-literals" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.use_regex_literals.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("prefer-regex-literals");
        }
        "prefer-rest-params" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_arguments.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("prefer-rest-params");
        }
        "prefer-template" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_template.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("prefer-template");
        }
        "react-hooks/exhaustive-deps" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .use_exhaustive_dependencies
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("react-hooks/exhaustive-deps");
        }
        "react-hooks/rules-of-hooks" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .use_hook_at_top_level
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("react-hooks/rules-of-hooks");
        }
        "react/button-has-type" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.use_button_type.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("react/button-has-type");
        }
        "react/jsx-boolean-value" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.no_implicit_boolean.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("react/jsx-boolean-value");
        }
        "react/jsx-fragments" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_fragment_syntax.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("react/jsx-fragments");
        }
        "react/jsx-key" => {
            if !options.include_nursery {
                results.nursery_rules.push("react/jsx-key");
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .use_jsx_key_in_iterable
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("react/jsx-key");
        }
        "react/jsx-no-comment-textnodes" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_comment_text.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("react/jsx-no-comment-textnodes");
        }
        "react/jsx-no-duplicate-props" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .no_duplicate_jsx_props
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("react/jsx-no-duplicate-props");
        }
        "react/jsx-no-target-blank" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
            let rule = group.no_blank_target.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("react/jsx-no-target-blank");
        }
        "react/jsx-no-useless-fragment" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_useless_fragments.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("react/jsx-no-useless-fragment");
        }
        "react/no-array-index-key" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_array_index_key.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("react/no-array-index-key");
        }
        "react/no-children-prop" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.no_children_prop.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("react/no-children-prop");
        }
        "react/no-danger" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .no_dangerously_set_inner_html_with_children
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("react/no-danger");
        }
        "react/no-danger-with-children" => {
            let group = rules.security.get_or_insert_with(Default::default);
            let rule = group
                .no_dangerously_set_inner_html
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("react/no-danger-with-children");
        }
        "react/void-dom-elements-no-children" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .no_void_elements_with_children
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("react/void-dom-elements-no-children");
        }
        "require-await" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.use_await.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("require-await");
        }
        "require-yield" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.use_yield.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("require-yield");
        }
        "sonarjs/cognitive-complexity" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_excessive_cognitive_complexity
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("sonarjs/cognitive-complexity");
        }
        "unicorn/filename-case" => {
            if !options.include_inspired {
                results.inspired_rules.push("unicorn/filename-case");
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_filenaming_convention
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("unicorn/filename-case");
        }
        "unicorn/no-array-for-each" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.no_for_each.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("unicorn/no-array-for-each");
        }
        "unicorn/no-instanceof-array" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.use_is_array.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("unicorn/no-instanceof-array");
        }
        "unicorn/no-thenable" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.no_then_property.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("unicorn/no-thenable");
        }
        "unicorn/no-useless-switch-case" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .no_useless_switch_case
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("unicorn/no-useless-switch-case");
        }
        "unicorn/prefer-array-flat-map" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group.use_flat_map.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("unicorn/prefer-array-flat-map");
        }
        "unicorn/prefer-node-protocol" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .use_nodejs_import_protocol
                .get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("unicorn/prefer-node-protocol");
        }
        "unicorn/prefer-number-properties" => {
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group.use_number_namespace.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results
                .migrated_rules
                .push("unicorn/prefer-number-properties");
        }
        "use-isnan" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group.use_is_nan.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("use-isnan");
        }
        "valid-typeof" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group.use_valid_typeof.get_or_insert(Default::default());
            rule.set_level(rule_severity.into());
            results.migrated_rules.push("valid-typeof");
        }
        _ => {
            results.unsupported_rules.push(eslint_name.to_string());
            return false;
        }
    }
    true
}
