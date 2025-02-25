//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_await_in_loop;
pub mod no_common_js;
pub mod no_constant_binary_expression;
pub mod no_document_cookie;
pub mod no_document_import_in_page;
pub mod no_duplicate_else_if;
pub mod no_dynamic_namespace_import_access;
pub mod no_enum;
pub mod no_exported_imports;
pub mod no_floating_promises;
pub mod no_global_dirname_filename;
pub mod no_head_element;
pub mod no_head_import_in_document;
pub mod no_img_element;
pub mod no_import_cycles;
pub mod no_irregular_whitespace;
pub mod no_nested_ternary;
pub mod no_noninteractive_element_interactions;
pub mod no_octal_escape;
pub mod no_package_private_imports;
pub mod no_process_env;
pub mod no_process_global;
pub mod no_restricted_imports;
pub mod no_restricted_types;
pub mod no_secrets;
pub mod no_static_element_interactions;
pub mod no_substr;
pub mod no_template_curly_in_string;
pub mod no_ts_ignore;
pub mod no_unwanted_polyfillio;
pub mod no_useless_escape_in_regex;
pub mod no_useless_string_raw;
pub mod no_useless_undefined;
pub mod use_adjacent_overload_signatures;
pub mod use_aria_props_supported_by_role;
pub mod use_at_index;
pub mod use_collapsed_if;
pub mod use_component_export_only_modules;
pub mod use_consistent_curly_braces;
pub mod use_consistent_member_accessibility;
pub mod use_explicit_type;
pub mod use_exports_last;
pub mod use_google_font_display;
pub mod use_google_font_preconnect;
pub mod use_guard_for_in;
pub mod use_parse_int_radix;
pub mod use_sorted_classes;
pub mod use_strict_mode;
pub mod use_trim_start_end;
pub mod use_valid_autocomplete;
declare_lint_group! { pub Nursery { name : "nursery" , rules : [self :: no_await_in_loop :: NoAwaitInLoop , self :: no_common_js :: NoCommonJs , self :: no_constant_binary_expression :: NoConstantBinaryExpression , self :: no_document_cookie :: NoDocumentCookie , self :: no_document_import_in_page :: NoDocumentImportInPage , self :: no_duplicate_else_if :: NoDuplicateElseIf , self :: no_dynamic_namespace_import_access :: NoDynamicNamespaceImportAccess , self :: no_enum :: NoEnum , self :: no_exported_imports :: NoExportedImports , self :: no_floating_promises :: NoFloatingPromises , self :: no_global_dirname_filename :: NoGlobalDirnameFilename , self :: no_head_element :: NoHeadElement , self :: no_head_import_in_document :: NoHeadImportInDocument , self :: no_img_element :: NoImgElement , self :: no_import_cycles :: NoImportCycles , self :: no_irregular_whitespace :: NoIrregularWhitespace , self :: no_nested_ternary :: NoNestedTernary , self :: no_noninteractive_element_interactions :: NoNoninteractiveElementInteractions , self :: no_octal_escape :: NoOctalEscape , self :: no_package_private_imports :: NoPackagePrivateImports , self :: no_process_env :: NoProcessEnv , self :: no_process_global :: NoProcessGlobal , self :: no_restricted_imports :: NoRestrictedImports , self :: no_restricted_types :: NoRestrictedTypes , self :: no_secrets :: NoSecrets , self :: no_static_element_interactions :: NoStaticElementInteractions , self :: no_substr :: NoSubstr , self :: no_template_curly_in_string :: NoTemplateCurlyInString , self :: no_ts_ignore :: NoTsIgnore , self :: no_unwanted_polyfillio :: NoUnwantedPolyfillio , self :: no_useless_escape_in_regex :: NoUselessEscapeInRegex , self :: no_useless_string_raw :: NoUselessStringRaw , self :: no_useless_undefined :: NoUselessUndefined , self :: use_adjacent_overload_signatures :: UseAdjacentOverloadSignatures , self :: use_aria_props_supported_by_role :: UseAriaPropsSupportedByRole , self :: use_at_index :: UseAtIndex , self :: use_collapsed_if :: UseCollapsedIf , self :: use_component_export_only_modules :: UseComponentExportOnlyModules , self :: use_consistent_curly_braces :: UseConsistentCurlyBraces , self :: use_consistent_member_accessibility :: UseConsistentMemberAccessibility , self :: use_explicit_type :: UseExplicitType , self :: use_exports_last :: UseExportsLast , self :: use_google_font_display :: UseGoogleFontDisplay , self :: use_google_font_preconnect :: UseGoogleFontPreconnect , self :: use_guard_for_in :: UseGuardForIn , self :: use_parse_int_radix :: UseParseIntRadix , self :: use_sorted_classes :: UseSortedClasses , self :: use_strict_mode :: UseStrictMode , self :: use_trim_start_end :: UseTrimStartEnd , self :: use_valid_autocomplete :: UseValidAutocomplete ,] } }
