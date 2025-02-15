//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::assist;
use crate::lint;
pub type NoAccessKey = <lint::a11y::no_access_key::NoAccessKey as biome_analyze::Rule>::Options;
pub type NoAccumulatingSpread = < lint :: performance :: no_accumulating_spread :: NoAccumulatingSpread as biome_analyze :: Rule > :: Options ;
pub type NoApproximativeNumericConstant = < lint :: suspicious :: no_approximative_numeric_constant :: NoApproximativeNumericConstant as biome_analyze :: Rule > :: Options ;
pub type NoArguments = <lint::style::no_arguments::NoArguments as biome_analyze::Rule>::Options;
pub type NoAriaHiddenOnFocusable = < lint :: a11y :: no_aria_hidden_on_focusable :: NoAriaHiddenOnFocusable as biome_analyze :: Rule > :: Options ;
pub type NoAriaUnsupportedElements = < lint :: a11y :: no_aria_unsupported_elements :: NoAriaUnsupportedElements as biome_analyze :: Rule > :: Options ;
pub type NoArrayIndexKey =
    <lint::suspicious::no_array_index_key::NoArrayIndexKey as biome_analyze::Rule>::Options;
pub type NoAssignInExpressions = < lint :: suspicious :: no_assign_in_expressions :: NoAssignInExpressions as biome_analyze :: Rule > :: Options ;
pub type NoAsyncPromiseExecutor = < lint :: suspicious :: no_async_promise_executor :: NoAsyncPromiseExecutor as biome_analyze :: Rule > :: Options ;
pub type NoAutofocus = <lint::a11y::no_autofocus::NoAutofocus as biome_analyze::Rule>::Options;
pub type NoAwaitInLoop =
    <lint::nursery::no_await_in_loop::NoAwaitInLoop as biome_analyze::Rule>::Options;
pub type NoBannedTypes =
    <lint::complexity::no_banned_types::NoBannedTypes as biome_analyze::Rule>::Options;
pub type NoBarrelFile =
    <lint::performance::no_barrel_file::NoBarrelFile as biome_analyze::Rule>::Options;
pub type NoBlankTarget =
    <lint::a11y::no_blank_target::NoBlankTarget as biome_analyze::Rule>::Options;
pub type NoCatchAssign =
    <lint::suspicious::no_catch_assign::NoCatchAssign as biome_analyze::Rule>::Options;
pub type NoChildrenProp =
    <lint::correctness::no_children_prop::NoChildrenProp as biome_analyze::Rule>::Options;
pub type NoClassAssign =
    <lint::suspicious::no_class_assign::NoClassAssign as biome_analyze::Rule>::Options;
pub type NoCommaOperator =
    <lint::style::no_comma_operator::NoCommaOperator as biome_analyze::Rule>::Options;
pub type NoCommentText =
    <lint::suspicious::no_comment_text::NoCommentText as biome_analyze::Rule>::Options;
pub type NoCommonJs = <lint::nursery::no_common_js::NoCommonJs as biome_analyze::Rule>::Options;
pub type NoCompareNegZero =
    <lint::suspicious::no_compare_neg_zero::NoCompareNegZero as biome_analyze::Rule>::Options;
pub type NoConfusingLabels =
    <lint::suspicious::no_confusing_labels::NoConfusingLabels as biome_analyze::Rule>::Options;
pub type NoConfusingVoidType =
    <lint::suspicious::no_confusing_void_type::NoConfusingVoidType as biome_analyze::Rule>::Options;
pub type NoConsole = <lint::suspicious::no_console::NoConsole as biome_analyze::Rule>::Options;
pub type NoConstAssign =
    <lint::correctness::no_const_assign::NoConstAssign as biome_analyze::Rule>::Options;
pub type NoConstEnum =
    <lint::suspicious::no_const_enum::NoConstEnum as biome_analyze::Rule>::Options;
pub type NoConstantBinaryExpression = < lint :: nursery :: no_constant_binary_expression :: NoConstantBinaryExpression as biome_analyze :: Rule > :: Options ;
pub type NoConstantCondition =
    <lint::correctness::no_constant_condition::NoConstantCondition as biome_analyze::Rule>::Options;
pub type NoConstantMathMinMaxClamp = < lint :: correctness :: no_constant_math_min_max_clamp :: NoConstantMathMinMaxClamp as biome_analyze :: Rule > :: Options ;
pub type NoConstructorReturn =
    <lint::correctness::no_constructor_return::NoConstructorReturn as biome_analyze::Rule>::Options;
pub type NoControlCharactersInRegex = < lint :: suspicious :: no_control_characters_in_regex :: NoControlCharactersInRegex as biome_analyze :: Rule > :: Options ;
pub type NoDangerouslySetInnerHtml = < lint :: security :: no_dangerously_set_inner_html :: NoDangerouslySetInnerHtml as biome_analyze :: Rule > :: Options ;
pub type NoDangerouslySetInnerHtmlWithChildren = < lint :: security :: no_dangerously_set_inner_html_with_children :: NoDangerouslySetInnerHtmlWithChildren as biome_analyze :: Rule > :: Options ;
pub type NoDebugger = <lint::suspicious::no_debugger::NoDebugger as biome_analyze::Rule>::Options;
pub type NoDefaultExport =
    <lint::style::no_default_export::NoDefaultExport as biome_analyze::Rule>::Options;
pub type NoDelete = <lint::performance::no_delete::NoDelete as biome_analyze::Rule>::Options;
pub type NoDistractingElements =
    <lint::a11y::no_distracting_elements::NoDistractingElements as biome_analyze::Rule>::Options;
pub type NoDocumentCookie =
    <lint::nursery::no_document_cookie::NoDocumentCookie as biome_analyze::Rule>::Options;
pub type NoDocumentImportInPage = < lint :: nursery :: no_document_import_in_page :: NoDocumentImportInPage as biome_analyze :: Rule > :: Options ;
pub type NoDoneCallback =
    <lint::style::no_done_callback::NoDoneCallback as biome_analyze::Rule>::Options;
pub type NoDoubleEquals =
    <lint::suspicious::no_double_equals::NoDoubleEquals as biome_analyze::Rule>::Options;
pub type NoDuplicateCase =
    <lint::suspicious::no_duplicate_case::NoDuplicateCase as biome_analyze::Rule>::Options;
pub type NoDuplicateClassMembers = < lint :: suspicious :: no_duplicate_class_members :: NoDuplicateClassMembers as biome_analyze :: Rule > :: Options ;
pub type NoDuplicateElseIf =
    <lint::nursery::no_duplicate_else_if::NoDuplicateElseIf as biome_analyze::Rule>::Options;
pub type NoDuplicateJsxProps =
    <lint::suspicious::no_duplicate_jsx_props::NoDuplicateJsxProps as biome_analyze::Rule>::Options;
pub type NoDuplicateObjectKeys = < lint :: suspicious :: no_duplicate_object_keys :: NoDuplicateObjectKeys as biome_analyze :: Rule > :: Options ;
pub type NoDuplicateParameters = < lint :: suspicious :: no_duplicate_parameters :: NoDuplicateParameters as biome_analyze :: Rule > :: Options ;
pub type NoDuplicateTestHooks = < lint :: suspicious :: no_duplicate_test_hooks :: NoDuplicateTestHooks as biome_analyze :: Rule > :: Options ;
pub type NoDynamicNamespaceImportAccess = < lint :: nursery :: no_dynamic_namespace_import_access :: NoDynamicNamespaceImportAccess as biome_analyze :: Rule > :: Options ;
pub type NoEmptyBlockStatements = < lint :: suspicious :: no_empty_block_statements :: NoEmptyBlockStatements as biome_analyze :: Rule > :: Options ;
pub type NoEmptyCharacterClassInRegex = < lint :: correctness :: no_empty_character_class_in_regex :: NoEmptyCharacterClassInRegex as biome_analyze :: Rule > :: Options ;
pub type NoEmptyInterface =
    <lint::suspicious::no_empty_interface::NoEmptyInterface as biome_analyze::Rule>::Options;
pub type NoEmptyPattern =
    <lint::correctness::no_empty_pattern::NoEmptyPattern as biome_analyze::Rule>::Options;
pub type NoEmptyTypeParameters = < lint :: complexity :: no_empty_type_parameters :: NoEmptyTypeParameters as biome_analyze :: Rule > :: Options ;
pub type NoEnum = <lint::nursery::no_enum::NoEnum as biome_analyze::Rule>::Options;
pub type NoEvolvingTypes =
    <lint::suspicious::no_evolving_types::NoEvolvingTypes as biome_analyze::Rule>::Options;
pub type NoExcessiveCognitiveComplexity = < lint :: complexity :: no_excessive_cognitive_complexity :: NoExcessiveCognitiveComplexity as biome_analyze :: Rule > :: Options ;
pub type NoExcessiveNestedTestSuites = < lint :: complexity :: no_excessive_nested_test_suites :: NoExcessiveNestedTestSuites as biome_analyze :: Rule > :: Options ;
pub type NoExplicitAny =
    <lint::suspicious::no_explicit_any::NoExplicitAny as biome_analyze::Rule>::Options;
pub type NoExportedImports =
    <lint::nursery::no_exported_imports::NoExportedImports as biome_analyze::Rule>::Options;
pub type NoExportsInTest =
    <lint::suspicious::no_exports_in_test::NoExportsInTest as biome_analyze::Rule>::Options;
pub type NoExtraBooleanCast =
    <lint::complexity::no_extra_boolean_cast::NoExtraBooleanCast as biome_analyze::Rule>::Options;
pub type NoExtraNonNullAssertion = < lint :: suspicious :: no_extra_non_null_assertion :: NoExtraNonNullAssertion as biome_analyze :: Rule > :: Options ;
pub type NoFallthroughSwitchClause = < lint :: suspicious :: no_fallthrough_switch_clause :: NoFallthroughSwitchClause as biome_analyze :: Rule > :: Options ;
pub type NoFlatMapIdentity =
    <lint::correctness::no_flat_map_identity::NoFlatMapIdentity as biome_analyze::Rule>::Options;
pub type NoFloatingPromises =
    <lint::nursery::no_floating_promises::NoFloatingPromises as biome_analyze::Rule>::Options;
pub type NoFocusedTests =
    <lint::suspicious::no_focused_tests::NoFocusedTests as biome_analyze::Rule>::Options;
pub type NoForEach = <lint::complexity::no_for_each::NoForEach as biome_analyze::Rule>::Options;
pub type NoFunctionAssign =
    <lint::suspicious::no_function_assign::NoFunctionAssign as biome_analyze::Rule>::Options;
pub type NoGlobalAssign =
    <lint::suspicious::no_global_assign::NoGlobalAssign as biome_analyze::Rule>::Options;
pub type NoGlobalDirnameFilename = < lint :: nursery :: no_global_dirname_filename :: NoGlobalDirnameFilename as biome_analyze :: Rule > :: Options ;
pub type NoGlobalEval =
    <lint::security::no_global_eval::NoGlobalEval as biome_analyze::Rule>::Options;
pub type NoGlobalIsFinite =
    <lint::suspicious::no_global_is_finite::NoGlobalIsFinite as biome_analyze::Rule>::Options;
pub type NoGlobalIsNan =
    <lint::suspicious::no_global_is_nan::NoGlobalIsNan as biome_analyze::Rule>::Options;
pub type NoGlobalObjectCalls = < lint :: correctness :: no_global_object_calls :: NoGlobalObjectCalls as biome_analyze :: Rule > :: Options ;
pub type NoHeadElement =
    <lint::nursery::no_head_element::NoHeadElement as biome_analyze::Rule>::Options;
pub type NoHeadImportInDocument = < lint :: nursery :: no_head_import_in_document :: NoHeadImportInDocument as biome_analyze :: Rule > :: Options ;
pub type NoHeaderScope =
    <lint::a11y::no_header_scope::NoHeaderScope as biome_analyze::Rule>::Options;
pub type NoImgElement =
    <lint::nursery::no_img_element::NoImgElement as biome_analyze::Rule>::Options;
pub type NoImplicitAnyLet =
    <lint::suspicious::no_implicit_any_let::NoImplicitAnyLet as biome_analyze::Rule>::Options;
pub type NoImplicitBoolean =
    <lint::style::no_implicit_boolean::NoImplicitBoolean as biome_analyze::Rule>::Options;
pub type NoImportAssign =
    <lint::suspicious::no_import_assign::NoImportAssign as biome_analyze::Rule>::Options;
pub type NoImportCycles =
    <lint::nursery::no_import_cycles::NoImportCycles as biome_analyze::Rule>::Options;
pub type NoInferrableTypes =
    <lint::style::no_inferrable_types::NoInferrableTypes as biome_analyze::Rule>::Options;
pub type NoInnerDeclarations =
    <lint::correctness::no_inner_declarations::NoInnerDeclarations as biome_analyze::Rule>::Options;
pub type NoInteractiveElementToNoninteractiveRole = < lint :: a11y :: no_interactive_element_to_noninteractive_role :: NoInteractiveElementToNoninteractiveRole as biome_analyze :: Rule > :: Options ;
pub type NoInvalidBuiltinInstantiation = < lint :: correctness :: no_invalid_builtin_instantiation :: NoInvalidBuiltinInstantiation as biome_analyze :: Rule > :: Options ;
pub type NoInvalidConstructorSuper = < lint :: correctness :: no_invalid_constructor_super :: NoInvalidConstructorSuper as biome_analyze :: Rule > :: Options ;
pub type NoInvalidNewBuiltin = < lint :: correctness :: no_invalid_new_builtin :: NoInvalidNewBuiltin as biome_analyze :: Rule > :: Options ;
pub type NoInvalidUseBeforeDeclaration = < lint :: correctness :: no_invalid_use_before_declaration :: NoInvalidUseBeforeDeclaration as biome_analyze :: Rule > :: Options ;
pub type NoIrregularWhitespace =
    <lint::nursery::no_irregular_whitespace::NoIrregularWhitespace as biome_analyze::Rule>::Options;
pub type NoLabelVar = <lint::suspicious::no_label_var::NoLabelVar as biome_analyze::Rule>::Options;
pub type NoLabelWithoutControl =
    <lint::a11y::no_label_without_control::NoLabelWithoutControl as biome_analyze::Rule>::Options;
pub type NoMisleadingCharacterClass = < lint :: suspicious :: no_misleading_character_class :: NoMisleadingCharacterClass as biome_analyze :: Rule > :: Options ;
pub type NoMisleadingInstantiator = < lint :: suspicious :: no_misleading_instantiator :: NoMisleadingInstantiator as biome_analyze :: Rule > :: Options ;
pub type NoMisplacedAssertion = < lint :: suspicious :: no_misplaced_assertion :: NoMisplacedAssertion as biome_analyze :: Rule > :: Options ;
pub type NoMisrefactoredShorthandAssign = < lint :: suspicious :: no_misrefactored_shorthand_assign :: NoMisrefactoredShorthandAssign as biome_analyze :: Rule > :: Options ;
pub type NoMultipleSpacesInRegularExpressionLiterals = < lint :: complexity :: no_multiple_spaces_in_regular_expression_literals :: NoMultipleSpacesInRegularExpressionLiterals as biome_analyze :: Rule > :: Options ;
pub type NoNamespace = <lint::style::no_namespace::NoNamespace as biome_analyze::Rule>::Options;
pub type NoNamespaceImport =
    <lint::style::no_namespace_import::NoNamespaceImport as biome_analyze::Rule>::Options;
pub type NoNegationElse =
    <lint::style::no_negation_else::NoNegationElse as biome_analyze::Rule>::Options;
pub type NoNestedTernary =
    <lint::nursery::no_nested_ternary::NoNestedTernary as biome_analyze::Rule>::Options;
pub type NoNewSymbol =
    <lint::correctness::no_new_symbol::NoNewSymbol as biome_analyze::Rule>::Options;
pub type NoNodejsModules =
    <lint::correctness::no_nodejs_modules::NoNodejsModules as biome_analyze::Rule>::Options;
pub type NoNonNullAssertion =
    <lint::style::no_non_null_assertion::NoNonNullAssertion as biome_analyze::Rule>::Options;
pub type NoNoninteractiveElementInteractions = < lint :: nursery :: no_noninteractive_element_interactions :: NoNoninteractiveElementInteractions as biome_analyze :: Rule > :: Options ;
pub type NoNoninteractiveElementToInteractiveRole = < lint :: a11y :: no_noninteractive_element_to_interactive_role :: NoNoninteractiveElementToInteractiveRole as biome_analyze :: Rule > :: Options ;
pub type NoNoninteractiveTabindex = < lint :: a11y :: no_noninteractive_tabindex :: NoNoninteractiveTabindex as biome_analyze :: Rule > :: Options ;
pub type NoNonoctalDecimalEscape = < lint :: correctness :: no_nonoctal_decimal_escape :: NoNonoctalDecimalEscape as biome_analyze :: Rule > :: Options ;
pub type NoOctalEscape =
    <lint::nursery::no_octal_escape::NoOctalEscape as biome_analyze::Rule>::Options;
pub type NoPackagePrivateImports = < lint :: nursery :: no_package_private_imports :: NoPackagePrivateImports as biome_analyze :: Rule > :: Options ;
pub type NoParameterAssign =
    <lint::style::no_parameter_assign::NoParameterAssign as biome_analyze::Rule>::Options;
pub type NoParameterProperties =
    <lint::style::no_parameter_properties::NoParameterProperties as biome_analyze::Rule>::Options;
pub type NoPositiveTabindex =
    <lint::a11y::no_positive_tabindex::NoPositiveTabindex as biome_analyze::Rule>::Options;
pub type NoPrecisionLoss =
    <lint::correctness::no_precision_loss::NoPrecisionLoss as biome_analyze::Rule>::Options;
pub type NoProcessEnv =
    <lint::nursery::no_process_env::NoProcessEnv as biome_analyze::Rule>::Options;
pub type NoProcessGlobal =
    <lint::nursery::no_process_global::NoProcessGlobal as biome_analyze::Rule>::Options;
pub type NoPrototypeBuiltins =
    <lint::suspicious::no_prototype_builtins::NoPrototypeBuiltins as biome_analyze::Rule>::Options;
pub type NoReExportAll =
    <lint::performance::no_re_export_all::NoReExportAll as biome_analyze::Rule>::Options;
pub type NoReactSpecificProps = < lint :: suspicious :: no_react_specific_props :: NoReactSpecificProps as biome_analyze :: Rule > :: Options ;
pub type NoRedeclare =
    <lint::suspicious::no_redeclare::NoRedeclare as biome_analyze::Rule>::Options;
pub type NoRedundantAlt =
    <lint::a11y::no_redundant_alt::NoRedundantAlt as biome_analyze::Rule>::Options;
pub type NoRedundantRoles =
    <lint::a11y::no_redundant_roles::NoRedundantRoles as biome_analyze::Rule>::Options;
pub type NoRedundantUseStrict = < lint :: suspicious :: no_redundant_use_strict :: NoRedundantUseStrict as biome_analyze :: Rule > :: Options ;
pub type NoRenderReturnValue = < lint :: correctness :: no_render_return_value :: NoRenderReturnValue as biome_analyze :: Rule > :: Options ;
pub type NoRestrictedGlobals =
    <lint::style::no_restricted_globals::NoRestrictedGlobals as biome_analyze::Rule>::Options;
pub type NoRestrictedImports =
    <lint::nursery::no_restricted_imports::NoRestrictedImports as biome_analyze::Rule>::Options;
pub type NoRestrictedTypes =
    <lint::nursery::no_restricted_types::NoRestrictedTypes as biome_analyze::Rule>::Options;
pub type NoSecrets = <lint::nursery::no_secrets::NoSecrets as biome_analyze::Rule>::Options;
pub type NoSelfAssign =
    <lint::correctness::no_self_assign::NoSelfAssign as biome_analyze::Rule>::Options;
pub type NoSelfCompare =
    <lint::suspicious::no_self_compare::NoSelfCompare as biome_analyze::Rule>::Options;
pub type NoSetterReturn =
    <lint::correctness::no_setter_return::NoSetterReturn as biome_analyze::Rule>::Options;
pub type NoShadowRestrictedNames = < lint :: suspicious :: no_shadow_restricted_names :: NoShadowRestrictedNames as biome_analyze :: Rule > :: Options ;
pub type NoShoutyConstants =
    <lint::style::no_shouty_constants::NoShoutyConstants as biome_analyze::Rule>::Options;
pub type NoSkippedTests =
    <lint::suspicious::no_skipped_tests::NoSkippedTests as biome_analyze::Rule>::Options;
pub type NoSparseArray =
    <lint::suspicious::no_sparse_array::NoSparseArray as biome_analyze::Rule>::Options;
pub type NoStaticElementInteractions = < lint :: nursery :: no_static_element_interactions :: NoStaticElementInteractions as biome_analyze :: Rule > :: Options ;
pub type NoStaticOnlyClass =
    <lint::complexity::no_static_only_class::NoStaticOnlyClass as biome_analyze::Rule>::Options;
pub type NoStringCaseMismatch = < lint :: correctness :: no_string_case_mismatch :: NoStringCaseMismatch as biome_analyze :: Rule > :: Options ;
pub type NoSubstr = <lint::nursery::no_substr::NoSubstr as biome_analyze::Rule>::Options;
pub type NoSuspiciousSemicolonInJsx = < lint :: suspicious :: no_suspicious_semicolon_in_jsx :: NoSuspiciousSemicolonInJsx as biome_analyze :: Rule > :: Options ;
pub type NoSvgWithoutTitle =
    <lint::a11y::no_svg_without_title::NoSvgWithoutTitle as biome_analyze::Rule>::Options;
pub type NoSwitchDeclarations = < lint :: correctness :: no_switch_declarations :: NoSwitchDeclarations as biome_analyze :: Rule > :: Options ;
pub type NoTemplateCurlyInString = < lint :: nursery :: no_template_curly_in_string :: NoTemplateCurlyInString as biome_analyze :: Rule > :: Options ;
pub type NoThenProperty =
    <lint::suspicious::no_then_property::NoThenProperty as biome_analyze::Rule>::Options;
pub type NoThisInStatic =
    <lint::complexity::no_this_in_static::NoThisInStatic as biome_analyze::Rule>::Options;
pub type NoTsIgnore = <lint::nursery::no_ts_ignore::NoTsIgnore as biome_analyze::Rule>::Options;
pub type NoUndeclaredDependencies = < lint :: correctness :: no_undeclared_dependencies :: NoUndeclaredDependencies as biome_analyze :: Rule > :: Options ;
pub type NoUndeclaredVariables = < lint :: correctness :: no_undeclared_variables :: NoUndeclaredVariables as biome_analyze :: Rule > :: Options ;
pub type NoUnnecessaryContinue = < lint :: correctness :: no_unnecessary_continue :: NoUnnecessaryContinue as biome_analyze :: Rule > :: Options ;
pub type NoUnreachable =
    <lint::correctness::no_unreachable::NoUnreachable as biome_analyze::Rule>::Options;
pub type NoUnreachableSuper =
    <lint::correctness::no_unreachable_super::NoUnreachableSuper as biome_analyze::Rule>::Options;
pub type NoUnsafeDeclarationMerging = < lint :: suspicious :: no_unsafe_declaration_merging :: NoUnsafeDeclarationMerging as biome_analyze :: Rule > :: Options ;
pub type NoUnsafeFinally =
    <lint::correctness::no_unsafe_finally::NoUnsafeFinally as biome_analyze::Rule>::Options;
pub type NoUnsafeNegation =
    <lint::suspicious::no_unsafe_negation::NoUnsafeNegation as biome_analyze::Rule>::Options;
pub type NoUnsafeOptionalChaining = < lint :: correctness :: no_unsafe_optional_chaining :: NoUnsafeOptionalChaining as biome_analyze :: Rule > :: Options ;
pub type NoUnusedFunctionParameters = < lint :: correctness :: no_unused_function_parameters :: NoUnusedFunctionParameters as biome_analyze :: Rule > :: Options ;
pub type NoUnusedImports =
    <lint::correctness::no_unused_imports::NoUnusedImports as biome_analyze::Rule>::Options;
pub type NoUnusedLabels =
    <lint::correctness::no_unused_labels::NoUnusedLabels as biome_analyze::Rule>::Options;
pub type NoUnusedPrivateClassMembers = < lint :: correctness :: no_unused_private_class_members :: NoUnusedPrivateClassMembers as biome_analyze :: Rule > :: Options ;
pub type NoUnusedTemplateLiteral = < lint :: style :: no_unused_template_literal :: NoUnusedTemplateLiteral as biome_analyze :: Rule > :: Options ;
pub type NoUnusedVariables =
    <lint::correctness::no_unused_variables::NoUnusedVariables as biome_analyze::Rule>::Options;
pub type NoUnwantedPolyfillio =
    <lint::nursery::no_unwanted_polyfillio::NoUnwantedPolyfillio as biome_analyze::Rule>::Options;
pub type NoUselessCatch =
    <lint::complexity::no_useless_catch::NoUselessCatch as biome_analyze::Rule>::Options;
pub type NoUselessConstructor = < lint :: complexity :: no_useless_constructor :: NoUselessConstructor as biome_analyze :: Rule > :: Options ;
pub type NoUselessElse =
    <lint::style::no_useless_else::NoUselessElse as biome_analyze::Rule>::Options;
pub type NoUselessEmptyExport = < lint :: complexity :: no_useless_empty_export :: NoUselessEmptyExport as biome_analyze :: Rule > :: Options ;
pub type NoUselessEscapeInRegex = < lint :: nursery :: no_useless_escape_in_regex :: NoUselessEscapeInRegex as biome_analyze :: Rule > :: Options ;
pub type NoUselessFragments =
    <lint::complexity::no_useless_fragments::NoUselessFragments as biome_analyze::Rule>::Options;
pub type NoUselessLabel =
    <lint::complexity::no_useless_label::NoUselessLabel as biome_analyze::Rule>::Options;
pub type NoUselessLoneBlockStatements = < lint :: complexity :: no_useless_lone_block_statements :: NoUselessLoneBlockStatements as biome_analyze :: Rule > :: Options ;
pub type NoUselessRename =
    <lint::complexity::no_useless_rename::NoUselessRename as biome_analyze::Rule>::Options;
pub type NoUselessStringConcat = < lint :: complexity :: no_useless_string_concat :: NoUselessStringConcat as biome_analyze :: Rule > :: Options ;
pub type NoUselessStringRaw =
    <lint::nursery::no_useless_string_raw::NoUselessStringRaw as biome_analyze::Rule>::Options;
pub type NoUselessSwitchCase =
    <lint::complexity::no_useless_switch_case::NoUselessSwitchCase as biome_analyze::Rule>::Options;
pub type NoUselessTernary =
    <lint::complexity::no_useless_ternary::NoUselessTernary as biome_analyze::Rule>::Options;
pub type NoUselessThisAlias =
    <lint::complexity::no_useless_this_alias::NoUselessThisAlias as biome_analyze::Rule>::Options;
pub type NoUselessTypeConstraint = < lint :: complexity :: no_useless_type_constraint :: NoUselessTypeConstraint as biome_analyze :: Rule > :: Options ;
pub type NoUselessUndefined =
    <lint::nursery::no_useless_undefined::NoUselessUndefined as biome_analyze::Rule>::Options;
pub type NoUselessUndefinedInitialization = < lint :: complexity :: no_useless_undefined_initialization :: NoUselessUndefinedInitialization as biome_analyze :: Rule > :: Options ;
pub type NoVar = <lint::suspicious::no_var::NoVar as biome_analyze::Rule>::Options;
pub type NoVoid = <lint::complexity::no_void::NoVoid as biome_analyze::Rule>::Options;
pub type NoVoidElementsWithChildren = < lint :: correctness :: no_void_elements_with_children :: NoVoidElementsWithChildren as biome_analyze :: Rule > :: Options ;
pub type NoVoidTypeReturn =
    <lint::correctness::no_void_type_return::NoVoidTypeReturn as biome_analyze::Rule>::Options;
pub type NoWith = <lint::complexity::no_with::NoWith as biome_analyze::Rule>::Options;
pub type NoYodaExpression =
    <lint::style::no_yoda_expression::NoYodaExpression as biome_analyze::Rule>::Options;
pub type OrganizeImports =
    <assist::source::organize_imports::OrganizeImports as biome_analyze::Rule>::Options;
pub type UseAdjacentOverloadSignatures = < lint :: nursery :: use_adjacent_overload_signatures :: UseAdjacentOverloadSignatures as biome_analyze :: Rule > :: Options ;
pub type UseAltText = <lint::a11y::use_alt_text::UseAltText as biome_analyze::Rule>::Options;
pub type UseAnchorContent =
    <lint::a11y::use_anchor_content::UseAnchorContent as biome_analyze::Rule>::Options;
pub type UseAriaActivedescendantWithTabindex = < lint :: a11y :: use_aria_activedescendant_with_tabindex :: UseAriaActivedescendantWithTabindex as biome_analyze :: Rule > :: Options ;
pub type UseAriaPropsForRole =
    <lint::a11y::use_aria_props_for_role::UseAriaPropsForRole as biome_analyze::Rule>::Options;
pub type UseAriaPropsSupportedByRole = < lint :: nursery :: use_aria_props_supported_by_role :: UseAriaPropsSupportedByRole as biome_analyze :: Rule > :: Options ;
pub type UseArrayLiterals =
    <lint::correctness::use_array_literals::UseArrayLiterals as biome_analyze::Rule>::Options;
pub type UseArrowFunction =
    <lint::complexity::use_arrow_function::UseArrowFunction as biome_analyze::Rule>::Options;
pub type UseAsConstAssertion =
    <lint::style::use_as_const_assertion::UseAsConstAssertion as biome_analyze::Rule>::Options;
pub type UseAtIndex = <lint::nursery::use_at_index::UseAtIndex as biome_analyze::Rule>::Options;
pub type UseAwait = <lint::suspicious::use_await::UseAwait as biome_analyze::Rule>::Options;
pub type UseBlockStatements =
    <lint::style::use_block_statements::UseBlockStatements as biome_analyze::Rule>::Options;
pub type UseButtonType =
    <lint::a11y::use_button_type::UseButtonType as biome_analyze::Rule>::Options;
pub type UseCollapsedElseIf =
    <lint::style::use_collapsed_else_if::UseCollapsedElseIf as biome_analyze::Rule>::Options;
pub type UseCollapsedIf =
    <lint::nursery::use_collapsed_if::UseCollapsedIf as biome_analyze::Rule>::Options;
pub type UseComponentExportOnlyModules = < lint :: nursery :: use_component_export_only_modules :: UseComponentExportOnlyModules as biome_analyze :: Rule > :: Options ;
pub type UseConsistentArrayType = < lint :: style :: use_consistent_array_type :: UseConsistentArrayType as biome_analyze :: Rule > :: Options ;
pub type UseConsistentBuiltinInstantiation = < lint :: style :: use_consistent_builtin_instantiation :: UseConsistentBuiltinInstantiation as biome_analyze :: Rule > :: Options ;
pub type UseConsistentCurlyBraces = < lint :: nursery :: use_consistent_curly_braces :: UseConsistentCurlyBraces as biome_analyze :: Rule > :: Options ;
pub type UseConsistentMemberAccessibility = < lint :: nursery :: use_consistent_member_accessibility :: UseConsistentMemberAccessibility as biome_analyze :: Rule > :: Options ;
pub type UseConst = <lint::style::use_const::UseConst as biome_analyze::Rule>::Options;
pub type UseDateNow = <lint::complexity::use_date_now::UseDateNow as biome_analyze::Rule>::Options;
pub type UseDefaultParameterLast = < lint :: style :: use_default_parameter_last :: UseDefaultParameterLast as biome_analyze :: Rule > :: Options ;
pub type UseDefaultSwitchClause = < lint :: style :: use_default_switch_clause :: UseDefaultSwitchClause as biome_analyze :: Rule > :: Options ;
pub type UseDefaultSwitchClauseLast = < lint :: suspicious :: use_default_switch_clause_last :: UseDefaultSwitchClauseLast as biome_analyze :: Rule > :: Options ;
pub type UseEnumInitializers =
    <lint::style::use_enum_initializers::UseEnumInitializers as biome_analyze::Rule>::Options;
pub type UseErrorMessage =
    <lint::suspicious::use_error_message::UseErrorMessage as biome_analyze::Rule>::Options;
pub type UseExhaustiveDependencies = < lint :: correctness :: use_exhaustive_dependencies :: UseExhaustiveDependencies as biome_analyze :: Rule > :: Options ;
pub type UseExplicitLengthCheck = < lint :: style :: use_explicit_length_check :: UseExplicitLengthCheck as biome_analyze :: Rule > :: Options ;
pub type UseExplicitType =
    <lint::nursery::use_explicit_type::UseExplicitType as biome_analyze::Rule>::Options;
pub type UseExponentiationOperator = < lint :: style :: use_exponentiation_operator :: UseExponentiationOperator as biome_analyze :: Rule > :: Options ;
pub type UseExportType =
    <lint::style::use_export_type::UseExportType as biome_analyze::Rule>::Options;
pub type UseExportsLast =
    <lint::nursery::use_exports_last::UseExportsLast as biome_analyze::Rule>::Options;
pub type UseFilenamingConvention = < lint :: style :: use_filenaming_convention :: UseFilenamingConvention as biome_analyze :: Rule > :: Options ;
pub type UseFlatMap = <lint::complexity::use_flat_map::UseFlatMap as biome_analyze::Rule>::Options;
pub type UseFocusableInteractive = < lint :: a11y :: use_focusable_interactive :: UseFocusableInteractive as biome_analyze :: Rule > :: Options ;
pub type UseForOf = <lint::style::use_for_of::UseForOf as biome_analyze::Rule>::Options;
pub type UseFragmentSyntax =
    <lint::style::use_fragment_syntax::UseFragmentSyntax as biome_analyze::Rule>::Options;
pub type UseGetterReturn =
    <lint::suspicious::use_getter_return::UseGetterReturn as biome_analyze::Rule>::Options;
pub type UseGoogleFontDisplay =
    <lint::nursery::use_google_font_display::UseGoogleFontDisplay as biome_analyze::Rule>::Options;
pub type UseGoogleFontPreconnect = < lint :: nursery :: use_google_font_preconnect :: UseGoogleFontPreconnect as biome_analyze :: Rule > :: Options ;
pub type UseGuardForIn =
    <lint::nursery::use_guard_for_in::UseGuardForIn as biome_analyze::Rule>::Options;
pub type UseHeadingContent =
    <lint::a11y::use_heading_content::UseHeadingContent as biome_analyze::Rule>::Options;
pub type UseHookAtTopLevel =
    <lint::correctness::use_hook_at_top_level::UseHookAtTopLevel as biome_analyze::Rule>::Options;
pub type UseHtmlLang = <lint::a11y::use_html_lang::UseHtmlLang as biome_analyze::Rule>::Options;
pub type UseIframeTitle =
    <lint::a11y::use_iframe_title::UseIframeTitle as biome_analyze::Rule>::Options;
pub type UseImportExtensions =
    <lint::correctness::use_import_extensions::UseImportExtensions as biome_analyze::Rule>::Options;
pub type UseImportType =
    <lint::style::use_import_type::UseImportType as biome_analyze::Rule>::Options;
pub type UseIsArray = <lint::suspicious::use_is_array::UseIsArray as biome_analyze::Rule>::Options;
pub type UseIsNan = <lint::correctness::use_is_nan::UseIsNan as biome_analyze::Rule>::Options;
pub type UseJsxKeyInIterable = < lint :: correctness :: use_jsx_key_in_iterable :: UseJsxKeyInIterable as biome_analyze :: Rule > :: Options ;
pub type UseKeyWithClickEvents =
    <lint::a11y::use_key_with_click_events::UseKeyWithClickEvents as biome_analyze::Rule>::Options;
pub type UseKeyWithMouseEvents =
    <lint::a11y::use_key_with_mouse_events::UseKeyWithMouseEvents as biome_analyze::Rule>::Options;
pub type UseLiteralEnumMembers =
    <lint::style::use_literal_enum_members::UseLiteralEnumMembers as biome_analyze::Rule>::Options;
pub type UseLiteralKeys =
    <lint::complexity::use_literal_keys::UseLiteralKeys as biome_analyze::Rule>::Options;
pub type UseMediaCaption =
    <lint::a11y::use_media_caption::UseMediaCaption as biome_analyze::Rule>::Options;
pub type UseNamespaceKeyword =
    <lint::suspicious::use_namespace_keyword::UseNamespaceKeyword as biome_analyze::Rule>::Options;
pub type UseNamingConvention =
    <lint::style::use_naming_convention::UseNamingConvention as biome_analyze::Rule>::Options;
pub type UseNodeAssertStrict =
    <lint::style::use_node_assert_strict::UseNodeAssertStrict as biome_analyze::Rule>::Options;
pub type UseNodejsImportProtocol = < lint :: style :: use_nodejs_import_protocol :: UseNodejsImportProtocol as biome_analyze :: Rule > :: Options ;
pub type UseNumberNamespace =
    <lint::style::use_number_namespace::UseNumberNamespace as biome_analyze::Rule>::Options;
pub type UseNumberToFixedDigitsArgument = < lint :: suspicious :: use_number_to_fixed_digits_argument :: UseNumberToFixedDigitsArgument as biome_analyze :: Rule > :: Options ;
pub type UseNumericLiterals =
    <lint::style::use_numeric_literals::UseNumericLiterals as biome_analyze::Rule>::Options;
pub type UseOptionalChain =
    <lint::complexity::use_optional_chain::UseOptionalChain as biome_analyze::Rule>::Options;
pub type UseParseIntRadix =
    <lint::nursery::use_parse_int_radix::UseParseIntRadix as biome_analyze::Rule>::Options;
pub type UseRegexLiterals =
    <lint::complexity::use_regex_literals::UseRegexLiterals as biome_analyze::Rule>::Options;
pub type UseSelfClosingElements = < lint :: style :: use_self_closing_elements :: UseSelfClosingElements as biome_analyze :: Rule > :: Options ;
pub type UseSemanticElements =
    <lint::a11y::use_semantic_elements::UseSemanticElements as biome_analyze::Rule>::Options;
pub type UseShorthandArrayType =
    <lint::style::use_shorthand_array_type::UseShorthandArrayType as biome_analyze::Rule>::Options;
pub type UseShorthandAssign =
    <lint::style::use_shorthand_assign::UseShorthandAssign as biome_analyze::Rule>::Options;
pub type UseShorthandFunctionType = < lint :: style :: use_shorthand_function_type :: UseShorthandFunctionType as biome_analyze :: Rule > :: Options ;
pub type UseSimpleNumberKeys =
    <lint::complexity::use_simple_number_keys::UseSimpleNumberKeys as biome_analyze::Rule>::Options;
pub type UseSimplifiedLogicExpression = < lint :: complexity :: use_simplified_logic_expression :: UseSimplifiedLogicExpression as biome_analyze :: Rule > :: Options ;
pub type UseSingleCaseStatement = < lint :: style :: use_single_case_statement :: UseSingleCaseStatement as biome_analyze :: Rule > :: Options ;
pub type UseSingleVarDeclarator = < lint :: style :: use_single_var_declarator :: UseSingleVarDeclarator as biome_analyze :: Rule > :: Options ;
pub type UseSortedAttributes =
    <assist::source::use_sorted_attributes::UseSortedAttributes as biome_analyze::Rule>::Options;
pub type UseSortedClasses =
    <lint::nursery::use_sorted_classes::UseSortedClasses as biome_analyze::Rule>::Options;
pub type UseStrictMode =
    <lint::nursery::use_strict_mode::UseStrictMode as biome_analyze::Rule>::Options;
pub type UseTemplate = <lint::style::use_template::UseTemplate as biome_analyze::Rule>::Options;
pub type UseThrowNewError =
    <lint::style::use_throw_new_error::UseThrowNewError as biome_analyze::Rule>::Options;
pub type UseThrowOnlyError =
    <lint::style::use_throw_only_error::UseThrowOnlyError as biome_analyze::Rule>::Options;
pub type UseTopLevelRegex =
    <lint::performance::use_top_level_regex::UseTopLevelRegex as biome_analyze::Rule>::Options;
pub type UseTrimStartEnd =
    <lint::nursery::use_trim_start_end::UseTrimStartEnd as biome_analyze::Rule>::Options;
pub type UseValidAnchor =
    <lint::a11y::use_valid_anchor::UseValidAnchor as biome_analyze::Rule>::Options;
pub type UseValidAriaProps =
    <lint::a11y::use_valid_aria_props::UseValidAriaProps as biome_analyze::Rule>::Options;
pub type UseValidAriaRole =
    <lint::a11y::use_valid_aria_role::UseValidAriaRole as biome_analyze::Rule>::Options;
pub type UseValidAriaValues =
    <lint::a11y::use_valid_aria_values::UseValidAriaValues as biome_analyze::Rule>::Options;
pub type UseValidAutocomplete =
    <lint::nursery::use_valid_autocomplete::UseValidAutocomplete as biome_analyze::Rule>::Options;
pub type UseValidForDirection = < lint :: correctness :: use_valid_for_direction :: UseValidForDirection as biome_analyze :: Rule > :: Options ;
pub type UseValidLang = <lint::a11y::use_valid_lang::UseValidLang as biome_analyze::Rule>::Options;
pub type UseValidTypeof =
    <lint::suspicious::use_valid_typeof::UseValidTypeof as biome_analyze::Rule>::Options;
pub type UseWhile = <lint::complexity::use_while::UseWhile as biome_analyze::Rule>::Options;
pub type UseYield = <lint::correctness::use_yield::UseYield as biome_analyze::Rule>::Options;
