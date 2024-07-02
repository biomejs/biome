//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;

pub mod no_console;
pub mod no_done_callback;
pub mod no_duplicate_else_if;
pub mod no_evolving_types;
pub mod no_exported_imports;
pub mod no_irregular_whitespace;
pub mod no_label_without_control;
pub mod no_misplaced_assertion;
pub mod no_react_specific_props;
pub mod no_restricted_imports;
pub mod no_substr;
pub mod no_undeclared_dependencies;
pub mod no_unused_function_parameters;
pub mod no_useless_string_concat;
pub mod no_useless_undefined_initialization;
pub mod no_yoda_expression;
pub mod use_adjacent_overload_signatures;
pub mod use_consistent_builtin_instantiation;
pub mod use_date_now;
pub mod use_default_switch_clause;
pub mod use_error_message;
pub mod use_explicit_length_check;
pub mod use_focusable_interactive;
pub mod use_import_extensions;
pub mod use_import_restrictions;
pub mod use_number_to_fixed_digits_argument;
pub mod use_semantic_elements;
pub mod use_sorted_classes;
pub mod use_throw_new_error;
pub mod use_throw_only_error;
pub mod use_top_level_regex;
pub mod use_valid_autocomplete;

declare_lint_group! {
    pub Nursery {
        name : "nursery" ,
        rules : [
            self :: no_console :: NoConsole ,
            self :: no_done_callback :: NoDoneCallback ,
            self :: no_duplicate_else_if :: NoDuplicateElseIf ,
            self :: no_evolving_types :: NoEvolvingTypes ,
            self :: no_exported_imports :: NoExportedImports ,
            self :: no_irregular_whitespace :: NoIrregularWhitespace ,
            self :: no_label_without_control :: NoLabelWithoutControl ,
            self :: no_misplaced_assertion :: NoMisplacedAssertion ,
            self :: no_react_specific_props :: NoReactSpecificProps ,
            self :: no_restricted_imports :: NoRestrictedImports ,
            self :: no_substr :: NoSubstr ,
            self :: no_undeclared_dependencies :: NoUndeclaredDependencies ,
            self :: no_unused_function_parameters :: NoUnusedFunctionParameters ,
            self :: no_useless_string_concat :: NoUselessStringConcat ,
            self :: no_useless_undefined_initialization :: NoUselessUndefinedInitialization ,
            self :: no_yoda_expression :: NoYodaExpression ,
            self :: use_adjacent_overload_signatures :: UseAdjacentOverloadSignatures ,
            self :: use_consistent_builtin_instantiation :: UseConsistentBuiltinInstantiation ,
            self :: use_date_now :: UseDateNow ,
            self :: use_default_switch_clause :: UseDefaultSwitchClause ,
            self :: use_error_message :: UseErrorMessage ,
            self :: use_explicit_length_check :: UseExplicitLengthCheck ,
            self :: use_focusable_interactive :: UseFocusableInteractive ,
            self :: use_import_extensions :: UseImportExtensions ,
            self :: use_import_restrictions :: UseImportRestrictions ,
            self :: use_number_to_fixed_digits_argument :: UseNumberToFixedDigitsArgument ,
            self :: use_semantic_elements :: UseSemanticElements ,
            self :: use_sorted_classes :: UseSortedClasses ,
            self :: use_throw_new_error :: UseThrowNewError ,
            self :: use_throw_only_error :: UseThrowOnlyError ,
            self :: use_top_level_regex :: UseTopLevelRegex ,
            self :: use_valid_autocomplete :: UseValidAutocomplete ,
        ]
     }
}
