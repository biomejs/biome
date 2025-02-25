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
            let rule = group
                .unwrap_group_as_mut()
                .no_this_in_static
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/google-font-display" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_google_font_display
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/google-font-preconnect" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_google_font_preconnect
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/no-document-import-in-page" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_document_import_in_page
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/no-head-element" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_head_element
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/no-head-import-in-document" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_head_import_in_document
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/no-img-element" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_img_element
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@next/no-unwanted-polyfillio" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
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
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
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
        "@typescript-eslint/ban-types" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_banned_types
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/consistent-type-exports" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
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
                results.has_inspired_rules = true;
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
            if !options.include_nursery {
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
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_consistent_member_accessibility
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/naming-convention" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
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
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_array_literals
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
                results.has_inspired_rules = true;
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
        "@typescript-eslint/no-misused-new" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_misleading_instantiator
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
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_common_js
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-restricted-imports" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_restricted_imports
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-restricted-types" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_restricted_types
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "@typescript-eslint/no-this-alias" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_this_alias
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
                results.has_inspired_rules = true;
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
                results.has_inspired_rules = true;
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
        "@typescript-eslint/require-await" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_await
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "ban-ts-comment" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_ts_ignore
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "barrel-files/avoid-barrel-files" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
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
            let group = rules.style.get_or_insert_with(Default::default);
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
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
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
        "guard-for-in" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_guard_for_in
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import-access/eslint-plugin-import-access" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_package_private_imports
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/exports-last" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_exports_last
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-commonjs" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_common_js
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "import/no-cycle" => {
            if !options.include_nursery {
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
                results.has_inspired_rules = true;
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
                results.has_inspired_rules = true;
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
                results.has_inspired_rules = true;
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
                results.has_inspired_rules = true;
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
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_misplaced_assertion
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
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
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
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
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
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
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
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
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
        "n/no-process-env" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_process_env
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "no-array-constructor" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
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
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_await_in_loop
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
                results.has_inspired_rules = true;
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
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_constant_binary_expression
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
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
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
                results.has_inspired_rules = true;
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
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
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
                results.has_inspired_rules = true;
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
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
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
        "no-new-symbol" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_new_symbol
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
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
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
                .no_multiple_spaces_in_regular_expression_literals
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
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_restricted_imports
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
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
            let group = rules.style.get_or_insert_with(Default::default);
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
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
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
                results.has_inspired_rules = true;
                return false;
            }
            let group = rules.style.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_throw_only_error
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
        "no-useless-catch" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_catch
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
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
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
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_with
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
        "prefer-arrow-callback" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
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
            let group = rules.style.get_or_insert_with(Default::default);
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
        "prefer-regex-literals" => {
            let group = rules.complexity.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_regex_literals
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "prefer-rest-params" => {
            let group = rules.style.get_or_insert_with(Default::default);
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
        "radix" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
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
        "react-hooks/rules-of-hooks" => {
            let group = rules.correctness.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_hook_at_top_level
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
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
                .unwrap_group_as_mut()
                .use_component_export_only_modules
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
        "react/jsx-boolean-value" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
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
                results.has_inspired_rules = true;
                return false;
            }
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
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
        "react/jsx-no-target-blank" => {
            let group = rules.a11y.get_or_insert_with(Default::default);
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
        "solidjs/no-react-specific-props" => {
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_react_specific_props
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
                results.has_inspired_rules = true;
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
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_document_cookie
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
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
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
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_useless_undefined
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
        "unicorn/prefer-at" => {
            if !options.include_inspired {
                results.has_inspired_rules = true;
                return false;
            }
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
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
                results.has_inspired_rules = true;
                return false;
            }
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
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
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .no_substr
                .get_or_insert(Default::default());
            rule.set_level(rule.level().max(rule_severity.into()));
        }
        "unicorn/prefer-string-trim-start-end" => {
            if !options.include_nursery {
                return false;
            }
            let group = rules.nursery.get_or_insert_with(Default::default);
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
            let group = rules.suspicious.get_or_insert_with(Default::default);
            let rule = group
                .unwrap_group_as_mut()
                .use_valid_typeof
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
            return false;
        }
    }
    true
}
