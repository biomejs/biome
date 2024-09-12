//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;

pub mod no_common_js;
pub mod no_duplicate_else_if;
pub mod no_dynamic_namespace_import_access;
pub mod no_enum;
pub mod no_exported_imports;
pub mod no_irregular_whitespace;
pub mod no_restricted_imports;
pub mod no_restricted_types;
pub mod no_secrets;
pub mod no_static_element_interactions;
pub mod no_substr;
pub mod no_useless_escape_in_regex;
pub mod use_adjacent_overload_signatures;
pub mod use_aria_props_supported_by_role;
pub mod use_consistent_curly_braces;
pub mod use_consistent_member_accessibility;
pub mod use_import_restrictions;
pub mod use_sorted_classes;
pub mod use_strict_mode;
pub mod use_trim_start_end;
pub mod use_valid_autocomplete;

declare_lint_group! {
    pub Nursery {
        name : "nursery" ,
        rules : [
            self :: no_common_js :: NoCommonJs ,
            self :: no_duplicate_else_if :: NoDuplicateElseIf ,
            self :: no_dynamic_namespace_import_access :: NoDynamicNamespaceImportAccess ,
            self :: no_enum :: NoEnum ,
            self :: no_exported_imports :: NoExportedImports ,
            self :: no_irregular_whitespace :: NoIrregularWhitespace ,
            self :: no_restricted_imports :: NoRestrictedImports ,
            self :: no_restricted_types :: NoRestrictedTypes ,
            self :: no_secrets :: NoSecrets ,
            self :: no_static_element_interactions :: NoStaticElementInteractions ,
            self :: no_substr :: NoSubstr ,
            self :: no_useless_escape_in_regex :: NoUselessEscapeInRegex ,
            self :: use_adjacent_overload_signatures :: UseAdjacentOverloadSignatures ,
            self :: use_aria_props_supported_by_role :: UseAriaPropsSupportedByRole ,
            self :: use_consistent_curly_braces :: UseConsistentCurlyBraces ,
            self :: use_consistent_member_accessibility :: UseConsistentMemberAccessibility ,
            self :: use_import_restrictions :: UseImportRestrictions ,
            self :: use_sorted_classes :: UseSortedClasses ,
            self :: use_strict_mode :: UseStrictMode ,
            self :: use_trim_start_end :: UseTrimStartEnd ,
            self :: use_valid_autocomplete :: UseValidAutocomplete ,
        ]
     }
}
