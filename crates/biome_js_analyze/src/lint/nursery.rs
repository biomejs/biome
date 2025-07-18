//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_await_in_loop;
pub mod no_bitwise_operators;
pub mod no_constant_binary_expression;
pub mod no_destructured_props;
pub mod no_excessive_lines_per_function;
pub mod no_floating_promises;
pub mod no_global_dirname_filename;
pub mod no_implicit_coercion;
pub mod no_import_cycles;
pub mod no_magic_numbers;
pub mod no_misused_promises;
pub mod no_nested_component_definitions;
pub mod no_noninteractive_element_interactions;
pub mod no_process_global;
pub mod no_react_prop_assign;
pub mod no_restricted_elements;
pub mod no_secrets;
pub mod no_shadow;
pub mod no_ts_ignore;
pub mod no_unassigned_variables;
pub mod no_unnecessary_conditions;
pub mod no_unresolved_imports;
pub mod no_unwanted_polyfillio;
pub mod no_useless_backref_in_regex;
pub mod no_useless_escape_in_string;
pub mod no_useless_undefined;
pub mod no_vue_reserved_props;
pub mod use_adjacent_getter_setter;
pub mod use_consistent_object_definition;
pub mod use_consistent_response;
pub mod use_exhaustive_switch_cases;
pub mod use_explicit_type;
pub mod use_exports_last;
pub mod use_for_component;
pub mod use_google_font_preconnect;
pub mod use_index_of;
pub mod use_iterable_callback_return;
pub mod use_json_import_attribute;
pub mod use_numeric_separators;
pub mod use_object_spread;
pub mod use_parse_int_radix;
pub mod use_readonly_class_properties;
pub mod use_single_js_doc_asterisk;
pub mod use_sorted_classes;
pub mod use_symbol_description;
pub mod use_unified_type_signature;
pub mod use_unique_element_ids;
declare_lint_group! { pub Nursery { name : "nursery" , rules : [self :: no_await_in_loop :: NoAwaitInLoop , self :: no_bitwise_operators :: NoBitwiseOperators , self :: no_constant_binary_expression :: NoConstantBinaryExpression , self :: no_destructured_props :: NoDestructuredProps , self :: no_excessive_lines_per_function :: NoExcessiveLinesPerFunction , self :: no_floating_promises :: NoFloatingPromises , self :: no_global_dirname_filename :: NoGlobalDirnameFilename , self :: no_implicit_coercion :: NoImplicitCoercion , self :: no_import_cycles :: NoImportCycles , self :: no_magic_numbers :: NoMagicNumbers , self :: no_misused_promises :: NoMisusedPromises , self :: no_nested_component_definitions :: NoNestedComponentDefinitions , self :: no_noninteractive_element_interactions :: NoNoninteractiveElementInteractions , self :: no_process_global :: NoProcessGlobal , self :: no_react_prop_assign :: NoReactPropAssign , self :: no_restricted_elements :: NoRestrictedElements , self :: no_secrets :: NoSecrets , self :: no_shadow :: NoShadow , self :: no_ts_ignore :: NoTsIgnore , self :: no_unassigned_variables :: NoUnassignedVariables , self :: no_unnecessary_conditions :: NoUnnecessaryConditions , self :: no_unresolved_imports :: NoUnresolvedImports , self :: no_unwanted_polyfillio :: NoUnwantedPolyfillio , self :: no_useless_backref_in_regex :: NoUselessBackrefInRegex , self :: no_useless_escape_in_string :: NoUselessEscapeInString , self :: no_useless_undefined :: NoUselessUndefined , self :: no_vue_reserved_props :: NoVueReservedProps , self :: use_adjacent_getter_setter :: UseAdjacentGetterSetter , self :: use_consistent_object_definition :: UseConsistentObjectDefinition , self :: use_consistent_response :: UseConsistentResponse , self :: use_exhaustive_switch_cases :: UseExhaustiveSwitchCases , self :: use_explicit_type :: UseExplicitType , self :: use_exports_last :: UseExportsLast , self :: use_for_component :: UseForComponent , self :: use_google_font_preconnect :: UseGoogleFontPreconnect , self :: use_index_of :: UseIndexOf , self :: use_iterable_callback_return :: UseIterableCallbackReturn , self :: use_json_import_attribute :: UseJsonImportAttribute , self :: use_numeric_separators :: UseNumericSeparators , self :: use_object_spread :: UseObjectSpread , self :: use_parse_int_radix :: UseParseIntRadix , self :: use_readonly_class_properties :: UseReadonlyClassProperties , self :: use_single_js_doc_asterisk :: UseSingleJsDocAsterisk , self :: use_sorted_classes :: UseSortedClasses , self :: use_symbol_description :: UseSymbolDescription , self :: use_unified_type_signature :: UseUnifiedTypeSignature , self :: use_unique_element_ids :: UseUniqueElementIds ,] } }
