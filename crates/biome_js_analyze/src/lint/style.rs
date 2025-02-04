//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_arguments;
pub mod no_comma_operator;
pub mod no_default_export;
pub mod no_done_callback;
pub mod no_implicit_boolean;
pub mod no_inferrable_types;
pub mod no_namespace;
pub mod no_namespace_import;
pub mod no_negation_else;
pub mod no_non_null_assertion;
pub mod no_parameter_assign;
pub mod no_parameter_properties;
pub mod no_restricted_globals;
pub mod no_shouty_constants;
pub mod no_unused_template_literal;
pub mod no_useless_else;
pub mod no_yoda_expression;
pub mod use_as_const_assertion;
pub mod use_block_statements;
pub mod use_collapsed_else_if;
pub mod use_consistent_array_type;
pub mod use_consistent_builtin_instantiation;
pub mod use_const;
pub mod use_default_parameter_last;
pub mod use_default_switch_clause;
pub mod use_enum_initializers;
pub mod use_explicit_length_check;
pub mod use_exponentiation_operator;
pub mod use_export_type;
pub mod use_filenaming_convention;
pub mod use_for_of;
pub mod use_fragment_syntax;
pub mod use_import_type;
pub mod use_literal_enum_members;
pub mod use_naming_convention;
pub mod use_node_assert_strict;
pub mod use_nodejs_import_protocol;
pub mod use_number_namespace;
pub mod use_numeric_literals;
pub mod use_self_closing_elements;
pub mod use_shorthand_array_type;
pub mod use_shorthand_assign;
pub mod use_shorthand_function_type;
pub mod use_single_case_statement;
pub mod use_single_var_declarator;
pub mod use_template;
pub mod use_throw_new_error;
pub mod use_throw_only_error;
declare_lint_group! { pub Style { name : "style" , rules : [self :: no_arguments :: NoArguments , self :: no_comma_operator :: NoCommaOperator , self :: no_default_export :: NoDefaultExport , self :: no_done_callback :: NoDoneCallback , self :: no_implicit_boolean :: NoImplicitBoolean , self :: no_inferrable_types :: NoInferrableTypes , self :: no_namespace :: NoNamespace , self :: no_namespace_import :: NoNamespaceImport , self :: no_negation_else :: NoNegationElse , self :: no_non_null_assertion :: NoNonNullAssertion , self :: no_parameter_assign :: NoParameterAssign , self :: no_parameter_properties :: NoParameterProperties , self :: no_restricted_globals :: NoRestrictedGlobals , self :: no_shouty_constants :: NoShoutyConstants , self :: no_unused_template_literal :: NoUnusedTemplateLiteral , self :: no_useless_else :: NoUselessElse , self :: no_yoda_expression :: NoYodaExpression , self :: use_as_const_assertion :: UseAsConstAssertion , self :: use_block_statements :: UseBlockStatements , self :: use_collapsed_else_if :: UseCollapsedElseIf , self :: use_consistent_array_type :: UseConsistentArrayType , self :: use_consistent_builtin_instantiation :: UseConsistentBuiltinInstantiation , self :: use_const :: UseConst , self :: use_default_parameter_last :: UseDefaultParameterLast , self :: use_default_switch_clause :: UseDefaultSwitchClause , self :: use_enum_initializers :: UseEnumInitializers , self :: use_explicit_length_check :: UseExplicitLengthCheck , self :: use_exponentiation_operator :: UseExponentiationOperator , self :: use_export_type :: UseExportType , self :: use_filenaming_convention :: UseFilenamingConvention , self :: use_for_of :: UseForOf , self :: use_fragment_syntax :: UseFragmentSyntax , self :: use_import_type :: UseImportType , self :: use_literal_enum_members :: UseLiteralEnumMembers , self :: use_naming_convention :: UseNamingConvention , self :: use_node_assert_strict :: UseNodeAssertStrict , self :: use_nodejs_import_protocol :: UseNodejsImportProtocol , self :: use_number_namespace :: UseNumberNamespace , self :: use_numeric_literals :: UseNumericLiterals , self :: use_self_closing_elements :: UseSelfClosingElements , self :: use_shorthand_array_type :: UseShorthandArrayType , self :: use_shorthand_assign :: UseShorthandAssign , self :: use_shorthand_function_type :: UseShorthandFunctionType , self :: use_single_case_statement :: UseSingleCaseStatement , self :: use_single_var_declarator :: UseSingleVarDeclarator , self :: use_template :: UseTemplate , self :: use_throw_new_error :: UseThrowNewError , self :: use_throw_only_error :: UseThrowOnlyError ,] } }
