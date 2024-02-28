//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::analyzers;
use crate::aria_analyzers;
use crate::semantic_analyzers;

pub type NoAccessKey =
    <analyzers::a11y::no_access_key::NoAccessKey as biome_analyze::Rule>::Options;
pub type NoAccumulatingSpread = < semantic_analyzers :: performance :: no_accumulating_spread :: NoAccumulatingSpread as biome_analyze :: Rule > :: Options ;
pub type NoApproximativeNumericConstant = < analyzers :: suspicious :: no_approximative_numeric_constant :: NoApproximativeNumericConstant as biome_analyze :: Rule > :: Options ;
pub type NoArguments =
    <semantic_analyzers::style::no_arguments::NoArguments as biome_analyze::Rule>::Options;
pub type NoAriaHiddenOnFocusable = < aria_analyzers :: a11y :: no_aria_hidden_on_focusable :: NoAriaHiddenOnFocusable as biome_analyze :: Rule > :: Options ;
pub type NoAriaUnsupportedElements = < aria_analyzers :: a11y :: no_aria_unsupported_elements :: NoAriaUnsupportedElements as biome_analyze :: Rule > :: Options ;
pub type NoArrayIndexKey = < semantic_analyzers :: suspicious :: no_array_index_key :: NoArrayIndexKey as biome_analyze :: Rule > :: Options ;
pub type NoAssignInExpressions = < analyzers :: suspicious :: no_assign_in_expressions :: NoAssignInExpressions as biome_analyze :: Rule > :: Options ;
pub type NoAsyncPromiseExecutor = < analyzers :: suspicious :: no_async_promise_executor :: NoAsyncPromiseExecutor as biome_analyze :: Rule > :: Options ;
pub type NoAutofocus = <analyzers::a11y::no_autofocus::NoAutofocus as biome_analyze::Rule>::Options;
pub type NoBannedTypes = < semantic_analyzers :: complexity :: no_banned_types :: NoBannedTypes as biome_analyze :: Rule > :: Options ;
pub type NoBarrelFile =
    <analyzers::nursery::no_barrel_file::NoBarrelFile as biome_analyze::Rule>::Options;
pub type NoBlankTarget =
    <analyzers::a11y::no_blank_target::NoBlankTarget as biome_analyze::Rule>::Options;
pub type NoCatchAssign = < semantic_analyzers :: suspicious :: no_catch_assign :: NoCatchAssign as biome_analyze :: Rule > :: Options ;
pub type NoChildrenProp = < semantic_analyzers :: correctness :: no_children_prop :: NoChildrenProp as biome_analyze :: Rule > :: Options ;
pub type NoClassAssign = < semantic_analyzers :: suspicious :: no_class_assign :: NoClassAssign as biome_analyze :: Rule > :: Options ;
pub type NoCommaOperator =
    <analyzers::style::no_comma_operator::NoCommaOperator as biome_analyze::Rule>::Options;
pub type NoCommentText =
    <analyzers::suspicious::no_comment_text::NoCommentText as biome_analyze::Rule>::Options;
pub type NoCompareNegZero =
    <analyzers::suspicious::no_compare_neg_zero::NoCompareNegZero as biome_analyze::Rule>::Options;
pub type NoConfusingLabels =
    <analyzers::suspicious::no_confusing_labels::NoConfusingLabels as biome_analyze::Rule>::Options;
pub type NoConfusingVoidType = < analyzers :: suspicious :: no_confusing_void_type :: NoConfusingVoidType as biome_analyze :: Rule > :: Options ;
pub type NoConsole =
    <semantic_analyzers::nursery::no_console::NoConsole as biome_analyze::Rule>::Options;
pub type NoConsoleLog =
    <semantic_analyzers::suspicious::no_console_log::NoConsoleLog as biome_analyze::Rule>::Options;
pub type NoConstAssign = < semantic_analyzers :: correctness :: no_const_assign :: NoConstAssign as biome_analyze :: Rule > :: Options ;
pub type NoConstEnum =
    <analyzers::suspicious::no_const_enum::NoConstEnum as biome_analyze::Rule>::Options;
pub type NoConstantCondition = < semantic_analyzers :: correctness :: no_constant_condition :: NoConstantCondition as biome_analyze :: Rule > :: Options ;
pub type NoConstructorReturn = < analyzers :: correctness :: no_constructor_return :: NoConstructorReturn as biome_analyze :: Rule > :: Options ;
pub type NoControlCharactersInRegex = < analyzers :: suspicious :: no_control_characters_in_regex :: NoControlCharactersInRegex as biome_analyze :: Rule > :: Options ;
pub type NoDangerouslySetInnerHtml = < semantic_analyzers :: security :: no_dangerously_set_inner_html :: NoDangerouslySetInnerHtml as biome_analyze :: Rule > :: Options ;
pub type NoDangerouslySetInnerHtmlWithChildren = < semantic_analyzers :: security :: no_dangerously_set_inner_html_with_children :: NoDangerouslySetInnerHtmlWithChildren as biome_analyze :: Rule > :: Options ;
pub type NoDebugger =
    <analyzers::suspicious::no_debugger::NoDebugger as biome_analyze::Rule>::Options;
pub type NoDefaultExport =
    <analyzers::style::no_default_export::NoDefaultExport as biome_analyze::Rule>::Options;
pub type NoDelete = <analyzers::performance::no_delete::NoDelete as biome_analyze::Rule>::Options;
pub type NoDistractingElements = < analyzers :: a11y :: no_distracting_elements :: NoDistractingElements as biome_analyze :: Rule > :: Options ;
pub type NoDoneCallback =
    <analyzers::nursery::no_done_callback::NoDoneCallback as biome_analyze::Rule>::Options;
pub type NoDoubleEquals =
    <analyzers::suspicious::no_double_equals::NoDoubleEquals as biome_analyze::Rule>::Options;
pub type NoDuplicateCase =
    <analyzers::suspicious::no_duplicate_case::NoDuplicateCase as biome_analyze::Rule>::Options;
pub type NoDuplicateClassMembers = < analyzers :: suspicious :: no_duplicate_class_members :: NoDuplicateClassMembers as biome_analyze :: Rule > :: Options ;
pub type NoDuplicateJsxProps = < analyzers :: suspicious :: no_duplicate_jsx_props :: NoDuplicateJsxProps as biome_analyze :: Rule > :: Options ;
pub type NoDuplicateObjectKeys = < analyzers :: suspicious :: no_duplicate_object_keys :: NoDuplicateObjectKeys as biome_analyze :: Rule > :: Options ;
pub type NoDuplicateParameters = < semantic_analyzers :: suspicious :: no_duplicate_parameters :: NoDuplicateParameters as biome_analyze :: Rule > :: Options ;
pub type NoDuplicateTestHooks = < analyzers :: nursery :: no_duplicate_test_hooks :: NoDuplicateTestHooks as biome_analyze :: Rule > :: Options ;
pub type NoEmptyBlockStatements = < analyzers :: suspicious :: no_empty_block_statements :: NoEmptyBlockStatements as biome_analyze :: Rule > :: Options ;
pub type NoEmptyCharacterClassInRegex = < analyzers :: correctness :: no_empty_character_class_in_regex :: NoEmptyCharacterClassInRegex as biome_analyze :: Rule > :: Options ;
pub type NoEmptyInterface =
    <analyzers::suspicious::no_empty_interface::NoEmptyInterface as biome_analyze::Rule>::Options;
pub type NoEmptyPattern =
    <analyzers::correctness::no_empty_pattern::NoEmptyPattern as biome_analyze::Rule>::Options;
pub type NoEmptyTypeParameters = < analyzers :: complexity :: no_empty_type_parameters :: NoEmptyTypeParameters as biome_analyze :: Rule > :: Options ;
pub type NoExcessiveCognitiveComplexity = < analyzers :: complexity :: no_excessive_cognitive_complexity :: NoExcessiveCognitiveComplexity as biome_analyze :: Rule > :: Options ;
pub type NoExcessiveNestedTestSuites = < analyzers :: nursery :: no_excessive_nested_test_suites :: NoExcessiveNestedTestSuites as biome_analyze :: Rule > :: Options ;
pub type NoExplicitAny =
    <analyzers::suspicious::no_explicit_any::NoExplicitAny as biome_analyze::Rule>::Options;
pub type NoExportsInTest =
    <analyzers::nursery::no_exports_in_test::NoExportsInTest as biome_analyze::Rule>::Options;
pub type NoExtraBooleanCast = < analyzers :: complexity :: no_extra_boolean_cast :: NoExtraBooleanCast as biome_analyze :: Rule > :: Options ;
pub type NoExtraNonNullAssertion = < analyzers :: suspicious :: no_extra_non_null_assertion :: NoExtraNonNullAssertion as biome_analyze :: Rule > :: Options ;
pub type NoFallthroughSwitchClause = < analyzers :: suspicious :: no_fallthrough_switch_clause :: NoFallthroughSwitchClause as biome_analyze :: Rule > :: Options ;
pub type NoFocusedTests =
    <analyzers::nursery::no_focused_tests::NoFocusedTests as biome_analyze::Rule>::Options;
pub type NoForEach =
    <analyzers::complexity::no_for_each::NoForEach as biome_analyze::Rule>::Options;
pub type NoFunctionAssign = < semantic_analyzers :: suspicious :: no_function_assign :: NoFunctionAssign as biome_analyze :: Rule > :: Options ;
pub type NoGlobalAssign = < semantic_analyzers :: suspicious :: no_global_assign :: NoGlobalAssign as biome_analyze :: Rule > :: Options ;
pub type NoGlobalEval =
    <semantic_analyzers::security::no_global_eval::NoGlobalEval as biome_analyze::Rule>::Options;
pub type NoGlobalIsFinite = < semantic_analyzers :: suspicious :: no_global_is_finite :: NoGlobalIsFinite as biome_analyze :: Rule > :: Options ;
pub type NoGlobalIsNan = < semantic_analyzers :: suspicious :: no_global_is_nan :: NoGlobalIsNan as biome_analyze :: Rule > :: Options ;
pub type NoGlobalObjectCalls = < semantic_analyzers :: correctness :: no_global_object_calls :: NoGlobalObjectCalls as biome_analyze :: Rule > :: Options ;
pub type NoHeaderScope =
    <analyzers::a11y::no_header_scope::NoHeaderScope as biome_analyze::Rule>::Options;
pub type NoImplicitAnyLet =
    <analyzers::suspicious::no_implicit_any_let::NoImplicitAnyLet as biome_analyze::Rule>::Options;
pub type NoImplicitBoolean =
    <analyzers::style::no_implicit_boolean::NoImplicitBoolean as biome_analyze::Rule>::Options;
pub type NoImportAssign = < semantic_analyzers :: suspicious :: no_import_assign :: NoImportAssign as biome_analyze :: Rule > :: Options ;
pub type NoInferrableTypes =
    <analyzers::style::no_inferrable_types::NoInferrableTypes as biome_analyze::Rule>::Options;
pub type NoInnerDeclarations = < analyzers :: correctness :: no_inner_declarations :: NoInnerDeclarations as biome_analyze :: Rule > :: Options ;
pub type NoInteractiveElementToNoninteractiveRole = < aria_analyzers :: a11y :: no_interactive_element_to_noninteractive_role :: NoInteractiveElementToNoninteractiveRole as biome_analyze :: Rule > :: Options ;
pub type NoInvalidConstructorSuper = < analyzers :: correctness :: no_invalid_constructor_super :: NoInvalidConstructorSuper as biome_analyze :: Rule > :: Options ;
pub type NoInvalidNewBuiltin = < semantic_analyzers :: correctness :: no_invalid_new_builtin :: NoInvalidNewBuiltin as biome_analyze :: Rule > :: Options ;
pub type NoInvalidUseBeforeDeclaration = < semantic_analyzers :: correctness :: no_invalid_use_before_declaration :: NoInvalidUseBeforeDeclaration as biome_analyze :: Rule > :: Options ;
pub type NoLabelVar =
    <semantic_analyzers::suspicious::no_label_var::NoLabelVar as biome_analyze::Rule>::Options;
pub type NoMisleadingCharacterClass = < semantic_analyzers :: suspicious :: no_misleading_character_class :: NoMisleadingCharacterClass as biome_analyze :: Rule > :: Options ;
pub type NoMisleadingInstantiator = < analyzers :: suspicious :: no_misleading_instantiator :: NoMisleadingInstantiator as biome_analyze :: Rule > :: Options ;
pub type NoMisplacedAssertion = < analyzers :: nursery :: no_misplaced_assertion :: NoMisplacedAssertion as biome_analyze :: Rule > :: Options ;
pub type NoMisrefactoredShorthandAssign = < analyzers :: suspicious :: no_misrefactored_shorthand_assign :: NoMisrefactoredShorthandAssign as biome_analyze :: Rule > :: Options ;
pub type NoMultipleSpacesInRegularExpressionLiterals = < analyzers :: complexity :: no_multiple_spaces_in_regular_expression_literals :: NoMultipleSpacesInRegularExpressionLiterals as biome_analyze :: Rule > :: Options ;
pub type NoNamespace =
    <analyzers::style::no_namespace::NoNamespace as biome_analyze::Rule>::Options;
pub type NoNamespaceImport =
    <analyzers::nursery::no_namespace_import::NoNamespaceImport as biome_analyze::Rule>::Options;
pub type NoNegationElse =
    <analyzers::style::no_negation_else::NoNegationElse as biome_analyze::Rule>::Options;
pub type NoNewSymbol =
    <semantic_analyzers::correctness::no_new_symbol::NoNewSymbol as biome_analyze::Rule>::Options;
pub type NoNodejsModules =
    <analyzers::nursery::no_nodejs_modules::NoNodejsModules as biome_analyze::Rule>::Options;
pub type NoNonNullAssertion =
    <analyzers::style::no_non_null_assertion::NoNonNullAssertion as biome_analyze::Rule>::Options;
pub type NoNoninteractiveElementToInteractiveRole = < aria_analyzers :: a11y :: no_noninteractive_element_to_interactive_role :: NoNoninteractiveElementToInteractiveRole as biome_analyze :: Rule > :: Options ;
pub type NoNoninteractiveTabindex = < aria_analyzers :: a11y :: no_noninteractive_tabindex :: NoNoninteractiveTabindex as biome_analyze :: Rule > :: Options ;
pub type NoNonoctalDecimalEscape = < analyzers :: correctness :: no_nonoctal_decimal_escape :: NoNonoctalDecimalEscape as biome_analyze :: Rule > :: Options ;
pub type NoParameterAssign = < semantic_analyzers :: style :: no_parameter_assign :: NoParameterAssign as biome_analyze :: Rule > :: Options ;
pub type NoParameterProperties = < analyzers :: style :: no_parameter_properties :: NoParameterProperties as biome_analyze :: Rule > :: Options ;
pub type NoPositiveTabindex = < semantic_analyzers :: a11y :: no_positive_tabindex :: NoPositiveTabindex as biome_analyze :: Rule > :: Options ;
pub type NoPrecisionLoss =
    <analyzers::correctness::no_precision_loss::NoPrecisionLoss as biome_analyze::Rule>::Options;
pub type NoPrototypeBuiltins = < analyzers :: suspicious :: no_prototype_builtins :: NoPrototypeBuiltins as biome_analyze :: Rule > :: Options ;
pub type NoReExportAll =
    <semantic_analyzers::nursery::no_re_export_all::NoReExportAll as biome_analyze::Rule>::Options;
pub type NoRedeclare =
    <semantic_analyzers::suspicious::no_redeclare::NoRedeclare as biome_analyze::Rule>::Options;
pub type NoRedundantAlt =
    <analyzers::a11y::no_redundant_alt::NoRedundantAlt as biome_analyze::Rule>::Options;
pub type NoRedundantRoles =
    <aria_analyzers::a11y::no_redundant_roles::NoRedundantRoles as biome_analyze::Rule>::Options;
pub type NoRedundantUseStrict = < analyzers :: suspicious :: no_redundant_use_strict :: NoRedundantUseStrict as biome_analyze :: Rule > :: Options ;
pub type NoRenderReturnValue = < semantic_analyzers :: correctness :: no_render_return_value :: NoRenderReturnValue as biome_analyze :: Rule > :: Options ;
pub type NoRestrictedGlobals = < semantic_analyzers :: style :: no_restricted_globals :: NoRestrictedGlobals as biome_analyze :: Rule > :: Options ;
pub type NoRestrictedImports = < analyzers :: nursery :: no_restricted_imports :: NoRestrictedImports as biome_analyze :: Rule > :: Options ;
pub type NoSelfAssign =
    <analyzers::correctness::no_self_assign::NoSelfAssign as biome_analyze::Rule>::Options;
pub type NoSelfCompare =
    <analyzers::suspicious::no_self_compare::NoSelfCompare as biome_analyze::Rule>::Options;
pub type NoSemicolonInJsx = < semantic_analyzers :: nursery :: no_semicolon_in_jsx :: NoSemicolonInJsx as biome_analyze :: Rule > :: Options ;
pub type NoSetterReturn =
    <analyzers::correctness::no_setter_return::NoSetterReturn as biome_analyze::Rule>::Options;
pub type NoShadowRestrictedNames = < analyzers :: suspicious :: no_shadow_restricted_names :: NoShadowRestrictedNames as biome_analyze :: Rule > :: Options ;
pub type NoShoutyConstants = < semantic_analyzers :: style :: no_shouty_constants :: NoShoutyConstants as biome_analyze :: Rule > :: Options ;
pub type NoSkippedTests =
    <analyzers::nursery::no_skipped_tests::NoSkippedTests as biome_analyze::Rule>::Options;
pub type NoSparseArray =
    <analyzers::suspicious::no_sparse_array::NoSparseArray as biome_analyze::Rule>::Options;
pub type NoStaticOnlyClass = < analyzers :: complexity :: no_static_only_class :: NoStaticOnlyClass as biome_analyze :: Rule > :: Options ;
pub type NoStringCaseMismatch = < analyzers :: correctness :: no_string_case_mismatch :: NoStringCaseMismatch as biome_analyze :: Rule > :: Options ;
pub type NoSvgWithoutTitle =
    <analyzers::a11y::no_svg_without_title::NoSvgWithoutTitle as biome_analyze::Rule>::Options;
pub type NoSwitchDeclarations = < analyzers :: correctness :: no_switch_declarations :: NoSwitchDeclarations as biome_analyze :: Rule > :: Options ;
pub type NoThenProperty = < semantic_analyzers :: suspicious :: no_then_property :: NoThenProperty as biome_analyze :: Rule > :: Options ;
pub type NoThisInStatic = < semantic_analyzers :: complexity :: no_this_in_static :: NoThisInStatic as biome_analyze :: Rule > :: Options ;
pub type NoUndeclaredDependencies = < analyzers :: nursery :: no_undeclared_dependencies :: NoUndeclaredDependencies as biome_analyze :: Rule > :: Options ;
pub type NoUndeclaredVariables = < semantic_analyzers :: correctness :: no_undeclared_variables :: NoUndeclaredVariables as biome_analyze :: Rule > :: Options ;
pub type NoUnnecessaryContinue = < analyzers :: correctness :: no_unnecessary_continue :: NoUnnecessaryContinue as biome_analyze :: Rule > :: Options ;
pub type NoUnreachable =
    <analyzers::correctness::no_unreachable::NoUnreachable as biome_analyze::Rule>::Options;
pub type NoUnreachableSuper = < analyzers :: correctness :: no_unreachable_super :: NoUnreachableSuper as biome_analyze :: Rule > :: Options ;
pub type NoUnsafeDeclarationMerging = < semantic_analyzers :: suspicious :: no_unsafe_declaration_merging :: NoUnsafeDeclarationMerging as biome_analyze :: Rule > :: Options ;
pub type NoUnsafeFinally =
    <analyzers::correctness::no_unsafe_finally::NoUnsafeFinally as biome_analyze::Rule>::Options;
pub type NoUnsafeNegation =
    <analyzers::suspicious::no_unsafe_negation::NoUnsafeNegation as biome_analyze::Rule>::Options;
pub type NoUnsafeOptionalChaining = < analyzers :: correctness :: no_unsafe_optional_chaining :: NoUnsafeOptionalChaining as biome_analyze :: Rule > :: Options ;
pub type NoUnusedImports = < semantic_analyzers :: correctness :: no_unused_imports :: NoUnusedImports as biome_analyze :: Rule > :: Options ;
pub type NoUnusedLabels =
    <analyzers::correctness::no_unused_labels::NoUnusedLabels as biome_analyze::Rule>::Options;
pub type NoUnusedPrivateClassMembers = < analyzers :: correctness :: no_unused_private_class_members :: NoUnusedPrivateClassMembers as biome_analyze :: Rule > :: Options ;
pub type NoUnusedTemplateLiteral = < analyzers :: style :: no_unused_template_literal :: NoUnusedTemplateLiteral as biome_analyze :: Rule > :: Options ;
pub type NoUnusedVariables = < semantic_analyzers :: correctness :: no_unused_variables :: NoUnusedVariables as biome_analyze :: Rule > :: Options ;
pub type NoUselessCatch =
    <analyzers::complexity::no_useless_catch::NoUselessCatch as biome_analyze::Rule>::Options;
pub type NoUselessConstructor = < analyzers :: complexity :: no_useless_constructor :: NoUselessConstructor as biome_analyze :: Rule > :: Options ;
pub type NoUselessElse =
    <analyzers::style::no_useless_else::NoUselessElse as biome_analyze::Rule>::Options;
pub type NoUselessEmptyExport = < analyzers :: complexity :: no_useless_empty_export :: NoUselessEmptyExport as biome_analyze :: Rule > :: Options ;
pub type NoUselessFragments = < semantic_analyzers :: complexity :: no_useless_fragments :: NoUselessFragments as biome_analyze :: Rule > :: Options ;
pub type NoUselessLabel =
    <analyzers::complexity::no_useless_label::NoUselessLabel as biome_analyze::Rule>::Options;
pub type NoUselessLoneBlockStatements = < analyzers :: complexity :: no_useless_lone_block_statements :: NoUselessLoneBlockStatements as biome_analyze :: Rule > :: Options ;
pub type NoUselessRename =
    <analyzers::complexity::no_useless_rename::NoUselessRename as biome_analyze::Rule>::Options;
pub type NoUselessSwitchCase = < analyzers :: complexity :: no_useless_switch_case :: NoUselessSwitchCase as biome_analyze :: Rule > :: Options ;
pub type NoUselessTernary =
    <analyzers::nursery::no_useless_ternary::NoUselessTernary as biome_analyze::Rule>::Options;
pub type NoUselessThisAlias = < semantic_analyzers :: complexity :: no_useless_this_alias :: NoUselessThisAlias as biome_analyze :: Rule > :: Options ;
pub type NoUselessTypeConstraint = < analyzers :: complexity :: no_useless_type_constraint :: NoUselessTypeConstraint as biome_analyze :: Rule > :: Options ;
pub type NoVar = <semantic_analyzers::style::no_var::NoVar as biome_analyze::Rule>::Options;
pub type NoVoid = <analyzers::complexity::no_void::NoVoid as biome_analyze::Rule>::Options;
pub type NoVoidElementsWithChildren = < semantic_analyzers :: correctness :: no_void_elements_with_children :: NoVoidElementsWithChildren as biome_analyze :: Rule > :: Options ;
pub type NoVoidTypeReturn =
    <analyzers::correctness::no_void_type_return::NoVoidTypeReturn as biome_analyze::Rule>::Options;
pub type NoWith = <analyzers::complexity::no_with::NoWith as biome_analyze::Rule>::Options;
pub type UseAltText = <analyzers::a11y::use_alt_text::UseAltText as biome_analyze::Rule>::Options;
pub type UseAnchorContent =
    <analyzers::a11y::use_anchor_content::UseAnchorContent as biome_analyze::Rule>::Options;
pub type UseAriaActivedescendantWithTabindex = < aria_analyzers :: a11y :: use_aria_activedescendant_with_tabindex :: UseAriaActivedescendantWithTabindex as biome_analyze :: Rule > :: Options ;
pub type UseAriaPropsForRole = < aria_analyzers :: a11y :: use_aria_props_for_role :: UseAriaPropsForRole as biome_analyze :: Rule > :: Options ;
pub type UseArrowFunction =
    <analyzers::complexity::use_arrow_function::UseArrowFunction as biome_analyze::Rule>::Options;
pub type UseAsConstAssertion =
    <analyzers::style::use_as_const_assertion::UseAsConstAssertion as biome_analyze::Rule>::Options;
pub type UseAwait = <analyzers::suspicious::use_await::UseAwait as biome_analyze::Rule>::Options;
pub type UseBlockStatements =
    <analyzers::style::use_block_statements::UseBlockStatements as biome_analyze::Rule>::Options;
pub type UseButtonType =
    <semantic_analyzers::a11y::use_button_type::UseButtonType as biome_analyze::Rule>::Options;
pub type UseCollapsedElseIf =
    <analyzers::style::use_collapsed_else_if::UseCollapsedElseIf as biome_analyze::Rule>::Options;
pub type UseConsistentArrayType = < analyzers :: style :: use_consistent_array_type :: UseConsistentArrayType as biome_analyze :: Rule > :: Options ;
pub type UseConst =
    <semantic_analyzers::style::use_const::UseConst as biome_analyze::Rule>::Options;
pub type UseDefaultParameterLast = < analyzers :: style :: use_default_parameter_last :: UseDefaultParameterLast as biome_analyze :: Rule > :: Options ;
pub type UseDefaultSwitchClauseLast = < analyzers :: suspicious :: use_default_switch_clause_last :: UseDefaultSwitchClauseLast as biome_analyze :: Rule > :: Options ;
pub type UseEnumInitializers =
    <analyzers::style::use_enum_initializers::UseEnumInitializers as biome_analyze::Rule>::Options;
pub type UseExhaustiveDependencies = < semantic_analyzers :: correctness :: use_exhaustive_dependencies :: UseExhaustiveDependencies as biome_analyze :: Rule > :: Options ;
pub type UseExponentiationOperator = < analyzers :: style :: use_exponentiation_operator :: UseExponentiationOperator as biome_analyze :: Rule > :: Options ;
pub type UseExportType =
    <semantic_analyzers::style::use_export_type::UseExportType as biome_analyze::Rule>::Options;
pub type UseFilenamingConvention = < analyzers :: style :: use_filenaming_convention :: UseFilenamingConvention as biome_analyze :: Rule > :: Options ;
pub type UseFlatMap =
    <analyzers::complexity::use_flat_map::UseFlatMap as biome_analyze::Rule>::Options;
pub type UseForOf =
    <semantic_analyzers::style::use_for_of::UseForOf as biome_analyze::Rule>::Options;
pub type UseFragmentSyntax = < semantic_analyzers :: style :: use_fragment_syntax :: UseFragmentSyntax as biome_analyze :: Rule > :: Options ;
pub type UseGetterReturn =
    <analyzers::suspicious::use_getter_return::UseGetterReturn as biome_analyze::Rule>::Options;
pub type UseHeadingContent =
    <analyzers::a11y::use_heading_content::UseHeadingContent as biome_analyze::Rule>::Options;
pub type UseHookAtTopLevel = < semantic_analyzers :: correctness :: use_hook_at_top_level :: UseHookAtTopLevel as biome_analyze :: Rule > :: Options ;
pub type UseHtmlLang =
    <analyzers::a11y::use_html_lang::UseHtmlLang as biome_analyze::Rule>::Options;
pub type UseIframeTitle =
    <analyzers::a11y::use_iframe_title::UseIframeTitle as biome_analyze::Rule>::Options;
pub type UseImportRestrictions = < analyzers :: nursery :: use_import_restrictions :: UseImportRestrictions as biome_analyze :: Rule > :: Options ;
pub type UseImportType =
    <semantic_analyzers::style::use_import_type::UseImportType as biome_analyze::Rule>::Options;
pub type UseIsArray =
    <semantic_analyzers::suspicious::use_is_array::UseIsArray as biome_analyze::Rule>::Options;
pub type UseIsNan =
    <semantic_analyzers::correctness::use_is_nan::UseIsNan as biome_analyze::Rule>::Options;
pub type UseJsxKeyInIterable = < semantic_analyzers :: nursery :: use_jsx_key_in_iterable :: UseJsxKeyInIterable as biome_analyze :: Rule > :: Options ;
pub type UseKeyWithClickEvents = < analyzers :: a11y :: use_key_with_click_events :: UseKeyWithClickEvents as biome_analyze :: Rule > :: Options ;
pub type UseKeyWithMouseEvents = < analyzers :: a11y :: use_key_with_mouse_events :: UseKeyWithMouseEvents as biome_analyze :: Rule > :: Options ;
pub type UseLiteralEnumMembers = < analyzers :: style :: use_literal_enum_members :: UseLiteralEnumMembers as biome_analyze :: Rule > :: Options ;
pub type UseLiteralKeys =
    <analyzers::complexity::use_literal_keys::UseLiteralKeys as biome_analyze::Rule>::Options;
pub type UseMediaCaption =
    <analyzers::a11y::use_media_caption::UseMediaCaption as biome_analyze::Rule>::Options;
pub type UseNamespaceKeyword = < analyzers :: suspicious :: use_namespace_keyword :: UseNamespaceKeyword as biome_analyze :: Rule > :: Options ;
pub type UseNamingConvention = < semantic_analyzers :: style :: use_naming_convention :: UseNamingConvention as biome_analyze :: Rule > :: Options ;
pub type UseNodeAssertStrict = < analyzers :: nursery :: use_node_assert_strict :: UseNodeAssertStrict as biome_analyze :: Rule > :: Options ;
pub type UseNodejsImportProtocol = < analyzers :: style :: use_nodejs_import_protocol :: UseNodejsImportProtocol as biome_analyze :: Rule > :: Options ;
pub type UseNumberNamespace = < semantic_analyzers :: style :: use_number_namespace :: UseNumberNamespace as biome_analyze :: Rule > :: Options ;
pub type UseNumericLiterals =
    <analyzers::style::use_numeric_literals::UseNumericLiterals as biome_analyze::Rule>::Options;
pub type UseOptionalChain =
    <analyzers::complexity::use_optional_chain::UseOptionalChain as biome_analyze::Rule>::Options;
pub type UseRegexLiterals =
    <analyzers::complexity::use_regex_literals::UseRegexLiterals as biome_analyze::Rule>::Options;
pub type UseSelfClosingElements = < analyzers :: style :: use_self_closing_elements :: UseSelfClosingElements as biome_analyze :: Rule > :: Options ;
pub type UseShorthandArrayType = < analyzers :: style :: use_shorthand_array_type :: UseShorthandArrayType as biome_analyze :: Rule > :: Options ;
pub type UseShorthandAssign =
    <analyzers::style::use_shorthand_assign::UseShorthandAssign as biome_analyze::Rule>::Options;
pub type UseShorthandFunctionType = < analyzers :: style :: use_shorthand_function_type :: UseShorthandFunctionType as biome_analyze :: Rule > :: Options ;
pub type UseSimpleNumberKeys = < analyzers :: complexity :: use_simple_number_keys :: UseSimpleNumberKeys as biome_analyze :: Rule > :: Options ;
pub type UseSimplifiedLogicExpression = < analyzers :: complexity :: use_simplified_logic_expression :: UseSimplifiedLogicExpression as biome_analyze :: Rule > :: Options ;
pub type UseSingleCaseStatement = < analyzers :: style :: use_single_case_statement :: UseSingleCaseStatement as biome_analyze :: Rule > :: Options ;
pub type UseSingleVarDeclarator = < analyzers :: style :: use_single_var_declarator :: UseSingleVarDeclarator as biome_analyze :: Rule > :: Options ;
pub type UseSortedClasses = < semantic_analyzers :: nursery :: use_sorted_classes :: UseSortedClasses as biome_analyze :: Rule > :: Options ;
pub type UseTemplate =
    <analyzers::style::use_template::UseTemplate as biome_analyze::Rule>::Options;
pub type UseValidAnchor =
    <analyzers::a11y::use_valid_anchor::UseValidAnchor as biome_analyze::Rule>::Options;
pub type UseValidAriaProps =
    <aria_analyzers::a11y::use_valid_aria_props::UseValidAriaProps as biome_analyze::Rule>::Options;
pub type UseValidAriaRole =
    <aria_analyzers::a11y::use_valid_aria_role::UseValidAriaRole as biome_analyze::Rule>::Options;
pub type UseValidAriaValues = < aria_analyzers :: a11y :: use_valid_aria_values :: UseValidAriaValues as biome_analyze :: Rule > :: Options ;
pub type UseValidForDirection = < analyzers :: correctness :: use_valid_for_direction :: UseValidForDirection as biome_analyze :: Rule > :: Options ;
pub type UseValidLang =
    <aria_analyzers::a11y::use_valid_lang::UseValidLang as biome_analyze::Rule>::Options;
pub type UseValidTypeof =
    <analyzers::suspicious::use_valid_typeof::UseValidTypeof as biome_analyze::Rule>::Options;
pub type UseWhile = <analyzers::style::use_while::UseWhile as biome_analyze::Rule>::Options;
pub type UseYield = <analyzers::correctness::use_yield::UseYield as biome_analyze::Rule>::Options;
