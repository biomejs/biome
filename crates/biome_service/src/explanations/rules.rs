//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::{FixKind, RuleMetadata};
pub(super) fn get_rule_metadata(s: &str) -> Option<RuleMetadata> {
    match s {
        "noAccessKey" => Some(RuleMetadata {
            name: "noAccessKey",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_ACCESS_KEY,
        }),
        "noAccumulatingSpread" => Some(RuleMetadata {
            name: "noAccumulatingSpread",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_ACCUMULATING_SPREAD,
        }),
        "noApproximativeNumericConstant" => Some(RuleMetadata {
            name: "noApproximativeNumericConstant",
            version: "1.3.0",
            fix_kind: None,
            recommended: false,
            deprecated: None,
            docs: NO_APPROXIMATIVE_NUMERIC_CONSTANT,
        }),
        "noArguments" => Some(RuleMetadata {
            name: "noArguments",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_ARGUMENTS,
        }),
        "noAriaHiddenOnFocusable" => Some(RuleMetadata {
            name: "noAriaHiddenOnFocusable",
            version: "1.4.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_ARIA_HIDDEN_ON_FOCUSABLE,
        }),
        "noAriaUnsupportedElements" => Some(RuleMetadata {
            name: "noAriaUnsupportedElements",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_ARIA_UNSUPPORTED_ELEMENTS,
        }),
        "noArrayIndexKey" => Some(RuleMetadata {
            name: "noArrayIndexKey",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_ARRAY_INDEX_KEY,
        }),
        "noAssignInExpressions" => Some(RuleMetadata {
            name: "noAssignInExpressions",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_ASSIGN_IN_EXPRESSIONS,
        }),
        "noAsyncPromiseExecutor" => Some(RuleMetadata {
            name: "noAsyncPromiseExecutor",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_ASYNC_PROMISE_EXECUTOR,
        }),
        "noAutofocus" => Some(RuleMetadata {
            name: "noAutofocus",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_AUTOFOCUS,
        }),
        "noBannedTypes" => Some(RuleMetadata {
            name: "noBannedTypes",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: NO_BANNED_TYPES,
        }),
        "noBlankTarget" => Some(RuleMetadata {
            name: "noBlankTarget",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: NO_BLANK_TARGET,
        }),
        "noCatchAssign" => Some(RuleMetadata {
            name: "noCatchAssign",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_CATCH_ASSIGN,
        }),
        "noChildrenProp" => Some(RuleMetadata {
            name: "noChildrenProp",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_CHILDREN_PROP,
        }),
        "noClassAssign" => Some(RuleMetadata {
            name: "noClassAssign",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_CLASS_ASSIGN,
        }),
        "noCommaOperator" => Some(RuleMetadata {
            name: "noCommaOperator",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_COMMA_OPERATOR,
        }),
        "noCommentText" => Some(RuleMetadata {
            name: "noCommentText",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_COMMENT_TEXT,
        }),
        "noCompareNegZero" => Some(RuleMetadata {
            name: "noCompareNegZero",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: NO_COMPARE_NEG_ZERO,
        }),
        "noConfusingLabels" => Some(RuleMetadata {
            name: "noConfusingLabels",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_CONFUSING_LABELS,
        }),
        "noConfusingVoidType" => Some(RuleMetadata {
            name: "noConfusingVoidType",
            version: "1.2.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_CONFUSING_VOID_TYPE,
        }),
        "noConsoleLog" => Some(RuleMetadata {
            name: "noConsoleLog",
            version: "1.0.0",
            fix_kind: None,
            recommended: false,
            deprecated: None,
            docs: NO_CONSOLE_LOG,
        }),
        "noConstAssign" => Some(RuleMetadata {
            name: "noConstAssign",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_CONST_ASSIGN,
        }),
        "noConstEnum" => Some(RuleMetadata {
            name: "noConstEnum",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: NO_CONST_ENUM,
        }),
        "noConstantCondition" => Some(RuleMetadata {
            name: "noConstantCondition",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_CONSTANT_CONDITION,
        }),
        "noConstructorReturn" => Some(RuleMetadata {
            name: "noConstructorReturn",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_CONSTRUCTOR_RETURN,
        }),
        "noControlCharactersInRegex" => Some(RuleMetadata {
            name: "noControlCharactersInRegex",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_CONTROL_CHARACTERS_IN_REGEX,
        }),
        "noDangerouslySetInnerHtml" => Some(RuleMetadata {
            name: "noDangerouslySetInnerHtml",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_DANGEROUSLY_SET_INNER_HTML,
        }),
        "noDangerouslySetInnerHtmlWithChildren" => Some(RuleMetadata {
            name: "noDangerouslySetInnerHtmlWithChildren",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_DANGEROUSLY_SET_INNER_HTML_WITH_CHILDREN,
        }),
        "noDebugger" => Some(RuleMetadata {
            name: "noDebugger",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_DEBUGGER,
        }),
        "noDefaultExport" => Some(RuleMetadata {
            name: "noDefaultExport",
            version: "1.4.0",
            fix_kind: None,
            recommended: false,
            deprecated: None,
            docs: NO_DEFAULT_EXPORT,
        }),
        "noDelete" => Some(RuleMetadata {
            name: "noDelete",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_DELETE,
        }),
        "noDistractingElements" => Some(RuleMetadata {
            name: "noDistractingElements",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_DISTRACTING_ELEMENTS,
        }),
        "noDoubleEquals" => Some(RuleMetadata {
            name: "noDoubleEquals",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_DOUBLE_EQUALS,
        }),
        "noDuplicateCase" => Some(RuleMetadata {
            name: "noDuplicateCase",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_DUPLICATE_CASE,
        }),
        "noDuplicateClassMembers" => Some(RuleMetadata {
            name: "noDuplicateClassMembers",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_DUPLICATE_CLASS_MEMBERS,
        }),
        "noDuplicateJsonKeys" => Some(RuleMetadata {
            name: "noDuplicateJsonKeys",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_DUPLICATE_JSON_KEYS,
        }),
        "noDuplicateJsxProps" => Some(RuleMetadata {
            name: "noDuplicateJsxProps",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_DUPLICATE_JSX_PROPS,
        }),
        "noDuplicateObjectKeys" => Some(RuleMetadata {
            name: "noDuplicateObjectKeys",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_DUPLICATE_OBJECT_KEYS,
        }),
        "noDuplicateParameters" => Some(RuleMetadata {
            name: "noDuplicateParameters",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_DUPLICATE_PARAMETERS,
        }),
        "noDuplicatePrivateClassMembers" => Some(RuleMetadata {
            name: "noDuplicatePrivateClassMembers",
            version: "1.0.0",
            fix_kind: None,
            recommended: false,
            deprecated: None,
            docs: NO_DUPLICATE_PRIVATE_CLASS_MEMBERS,
        }),
        "noEmptyBlockStatements" => Some(RuleMetadata {
            name: "noEmptyBlockStatements",
            version: "1.3.0",
            fix_kind: None,
            recommended: false,
            deprecated: None,
            docs: NO_EMPTY_BLOCK_STATEMENTS,
        }),
        "noEmptyCharacterClassInRegex" => Some(RuleMetadata {
            name: "noEmptyCharacterClassInRegex",
            version: "1.3.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_EMPTY_CHARACTER_CLASS_IN_REGEX,
        }),
        "noEmptyInterface" => Some(RuleMetadata {
            name: "noEmptyInterface",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: NO_EMPTY_INTERFACE,
        }),
        "noEmptyPattern" => Some(RuleMetadata {
            name: "noEmptyPattern",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_EMPTY_PATTERN,
        }),
        "noExcessiveCognitiveComplexity" => Some(RuleMetadata {
            name: "noExcessiveCognitiveComplexity",
            version: "1.0.0",
            fix_kind: None,
            recommended: false,
            deprecated: None,
            docs: NO_EXCESSIVE_COGNITIVE_COMPLEXITY,
        }),
        "noExplicitAny" => Some(RuleMetadata {
            name: "noExplicitAny",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_EXPLICIT_ANY,
        }),
        "noExtraBooleanCast" => Some(RuleMetadata {
            name: "noExtraBooleanCast",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_EXTRA_BOOLEAN_CAST,
        }),
        "noExtraNonNullAssertion" => Some(RuleMetadata {
            name: "noExtraNonNullAssertion",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: NO_EXTRA_NON_NULL_ASSERTION,
        }),
        "noFallthroughSwitchClause" => Some(RuleMetadata {
            name: "noFallthroughSwitchClause",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_FALLTHROUGH_SWITCH_CLAUSE,
        }),
        "noForEach" => Some(RuleMetadata {
            name: "noForEach",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_FOR_EACH,
        }),
        "noFunctionAssign" => Some(RuleMetadata {
            name: "noFunctionAssign",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_FUNCTION_ASSIGN,
        }),
        "noGlobalIsFinite" => Some(RuleMetadata {
            name: "noGlobalIsFinite",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_GLOBAL_IS_FINITE,
        }),
        "noGlobalIsNan" => Some(RuleMetadata {
            name: "noGlobalIsNan",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_GLOBAL_IS_NAN,
        }),
        "noGlobalObjectCalls" => Some(RuleMetadata {
            name: "noGlobalObjectCalls",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_GLOBAL_OBJECT_CALLS,
        }),
        "noHeaderScope" => Some(RuleMetadata {
            name: "noHeaderScope",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_HEADER_SCOPE,
        }),
        "noImplicitAnyLet" => Some(RuleMetadata {
            name: "noImplicitAnyLet",
            version: "1.4.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_IMPLICIT_ANY_LET,
        }),
        "noImplicitBoolean" => Some(RuleMetadata {
            name: "noImplicitBoolean",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: false,
            deprecated: None,
            docs: NO_IMPLICIT_BOOLEAN,
        }),
        "noImportAssign" => Some(RuleMetadata {
            name: "noImportAssign",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_IMPORT_ASSIGN,
        }),
        "noInferrableTypes" => Some(RuleMetadata {
            name: "noInferrableTypes",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: NO_INFERRABLE_TYPES,
        }),
        "noInitializerWithDefinite" => Some(RuleMetadata {
            name: "noInitializerWithDefinite",
            version: "1.4.0",
            fix_kind: None,
            recommended: false,
            deprecated: None,
            docs: NO_INITIALIZER_WITH_DEFINITE,
        }),
        "noInnerDeclarations" => Some(RuleMetadata {
            name: "noInnerDeclarations",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_INNER_DECLARATIONS,
        }),
        "noInteractiveElementToNoninteractiveRole" => Some(RuleMetadata {
            name: "noInteractiveElementToNoninteractiveRole",
            version: "1.3.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_INTERACTIVE_ELEMENT_TO_NONINTERACTIVE_ROLE,
        }),
        "noInvalidConstructorSuper" => Some(RuleMetadata {
            name: "noInvalidConstructorSuper",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_INVALID_CONSTRUCTOR_SUPER,
        }),
        "noInvalidNewBuiltin" => Some(RuleMetadata {
            name: "noInvalidNewBuiltin",
            version: "1.3.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_INVALID_NEW_BUILTIN,
        }),
        "noLabelVar" => Some(RuleMetadata {
            name: "noLabelVar",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_LABEL_VAR,
        }),
        "noMisleadingCharacterClass" => Some(RuleMetadata {
            name: "noMisleadingCharacterClass",
            version: "next",
            fix_kind: Some(FixKind::Safe),
            recommended: false,
            deprecated: None,
            docs: NO_MISLEADING_CHARACTER_CLASS,
        }),
        "noMisleadingInstantiator" => Some(RuleMetadata {
            name: "noMisleadingInstantiator",
            version: "1.3.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_MISLEADING_INSTANTIATOR,
        }),
        "noMisrefactoredShorthandAssign" => Some(RuleMetadata {
            name: "noMisrefactoredShorthandAssign",
            version: "1.3.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: false,
            deprecated: None,
            docs: NO_MISREFACTORED_SHORTHAND_ASSIGN,
        }),
        "noMultipleSpacesInRegularExpressionLiterals" => Some(RuleMetadata {
            name: "noMultipleSpacesInRegularExpressionLiterals",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_MULTIPLE_SPACES_IN_REGULAR_EXPRESSION_LITERALS,
        }),
        "noNamespace" => Some(RuleMetadata {
            name: "noNamespace",
            version: "1.0.0",
            fix_kind: None,
            recommended: false,
            deprecated: None,
            docs: NO_NAMESPACE,
        }),
        "noNegationElse" => Some(RuleMetadata {
            name: "noNegationElse",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: false,
            deprecated: None,
            docs: NO_NEGATION_ELSE,
        }),
        "noNewSymbol" => Some(RuleMetadata {
            name: "noNewSymbol",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: false,
            deprecated: Some("Use `noInvalidNewBuiltin` instead."),
            docs: NO_NEW_SYMBOL,
        }),
        "noNonNullAssertion" => Some(RuleMetadata {
            name: "noNonNullAssertion",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_NON_NULL_ASSERTION,
        }),
        "noNoninteractiveElementToInteractiveRole" => Some(RuleMetadata {
            name: "noNoninteractiveElementToInteractiveRole",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_NONINTERACTIVE_ELEMENT_TO_INTERACTIVE_ROLE,
        }),
        "noNoninteractiveTabindex" => Some(RuleMetadata {
            name: "noNoninteractiveTabindex",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_NONINTERACTIVE_TABINDEX,
        }),
        "noNonoctalDecimalEscape" => Some(RuleMetadata {
            name: "noNonoctalDecimalEscape",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_NONOCTAL_DECIMAL_ESCAPE,
        }),
        "noParameterAssign" => Some(RuleMetadata {
            name: "noParameterAssign",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_PARAMETER_ASSIGN,
        }),
        "noParameterProperties" => Some(RuleMetadata {
            name: "noParameterProperties",
            version: "1.0.0",
            fix_kind: None,
            recommended: false,
            deprecated: None,
            docs: NO_PARAMETER_PROPERTIES,
        }),
        "noPositiveTabindex" => Some(RuleMetadata {
            name: "noPositiveTabindex",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_POSITIVE_TABINDEX,
        }),
        "noPrecisionLoss" => Some(RuleMetadata {
            name: "noPrecisionLoss",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_PRECISION_LOSS,
        }),
        "noPrototypeBuiltins" => Some(RuleMetadata {
            name: "noPrototypeBuiltins",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_PROTOTYPE_BUILTINS,
        }),
        "noRedeclare" => Some(RuleMetadata {
            name: "noRedeclare",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_REDECLARE,
        }),
        "noRedundantAlt" => Some(RuleMetadata {
            name: "noRedundantAlt",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_REDUNDANT_ALT,
        }),
        "noRedundantRoles" => Some(RuleMetadata {
            name: "noRedundantRoles",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_REDUNDANT_ROLES,
        }),
        "noRedundantUseStrict" => Some(RuleMetadata {
            name: "noRedundantUseStrict",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: NO_REDUNDANT_USE_STRICT,
        }),
        "noRenderReturnValue" => Some(RuleMetadata {
            name: "noRenderReturnValue",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_RENDER_RETURN_VALUE,
        }),
        "noRestrictedGlobals" => Some(RuleMetadata {
            name: "noRestrictedGlobals",
            version: "1.0.0",
            fix_kind: None,
            recommended: false,
            deprecated: None,
            docs: NO_RESTRICTED_GLOBALS,
        }),
        "noSelfAssign" => Some(RuleMetadata {
            name: "noSelfAssign",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_SELF_ASSIGN,
        }),
        "noSelfCompare" => Some(RuleMetadata {
            name: "noSelfCompare",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_SELF_COMPARE,
        }),
        "noSetterReturn" => Some(RuleMetadata {
            name: "noSetterReturn",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_SETTER_RETURN,
        }),
        "noShadowRestrictedNames" => Some(RuleMetadata {
            name: "noShadowRestrictedNames",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_SHADOW_RESTRICTED_NAMES,
        }),
        "noShoutyConstants" => Some(RuleMetadata {
            name: "noShoutyConstants",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: false,
            deprecated: None,
            docs: NO_SHOUTY_CONSTANTS,
        }),
        "noSparseArray" => Some(RuleMetadata {
            name: "noSparseArray",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_SPARSE_ARRAY,
        }),
        "noStaticOnlyClass" => Some(RuleMetadata {
            name: "noStaticOnlyClass",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_STATIC_ONLY_CLASS,
        }),
        "noStringCaseMismatch" => Some(RuleMetadata {
            name: "noStringCaseMismatch",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_STRING_CASE_MISMATCH,
        }),
        "noSuperWithoutExtends" => Some(RuleMetadata {
            name: "noSuperWithoutExtends",
            version: "1.0.0",
            fix_kind: None,
            recommended: false,
            deprecated: None,
            docs: NO_SUPER_WITHOUT_EXTENDS,
        }),
        "noSvgWithoutTitle" => Some(RuleMetadata {
            name: "noSvgWithoutTitle",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_SVG_WITHOUT_TITLE,
        }),
        "noSwitchDeclarations" => Some(RuleMetadata {
            name: "noSwitchDeclarations",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_SWITCH_DECLARATIONS,
        }),
        "noThisInStatic" => Some(RuleMetadata {
            name: "noThisInStatic",
            version: "1.3.1",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_THIS_IN_STATIC,
        }),
        "noUndeclaredVariables" => Some(RuleMetadata {
            name: "noUndeclaredVariables",
            version: "1.0.0",
            fix_kind: None,
            recommended: false,
            deprecated: None,
            docs: NO_UNDECLARED_VARIABLES,
        }),
        "noUnnecessaryContinue" => Some(RuleMetadata {
            name: "noUnnecessaryContinue",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_UNNECESSARY_CONTINUE,
        }),
        "noUnreachable" => Some(RuleMetadata {
            name: "noUnreachable",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_UNREACHABLE,
        }),
        "noUnreachableSuper" => Some(RuleMetadata {
            name: "noUnreachableSuper",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_UNREACHABLE_SUPER,
        }),
        "noUnsafeDeclarationMerging" => Some(RuleMetadata {
            name: "noUnsafeDeclarationMerging",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_UNSAFE_DECLARATION_MERGING,
        }),
        "noUnsafeFinally" => Some(RuleMetadata {
            name: "noUnsafeFinally",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_UNSAFE_FINALLY,
        }),
        "noUnsafeNegation" => Some(RuleMetadata {
            name: "noUnsafeNegation",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_UNSAFE_NEGATION,
        }),
        "noUnsafeOptionalChaining" => Some(RuleMetadata {
            name: "noUnsafeOptionalChaining",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_UNSAFE_OPTIONAL_CHAINING,
        }),
        "noUnusedImports" => Some(RuleMetadata {
            name: "noUnusedImports",
            version: "1.3.0",
            fix_kind: Some(FixKind::Safe),
            recommended: false,
            deprecated: None,
            docs: NO_UNUSED_IMPORTS,
        }),
        "noUnusedLabels" => Some(RuleMetadata {
            name: "noUnusedLabels",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_UNUSED_LABELS,
        }),
        "noUnusedPrivateClassMembers" => Some(RuleMetadata {
            name: "noUnusedPrivateClassMembers",
            version: "1.3.3",
            fix_kind: Some(FixKind::Unsafe),
            recommended: false,
            deprecated: None,
            docs: NO_UNUSED_PRIVATE_CLASS_MEMBERS,
        }),
        "noUnusedTemplateLiteral" => Some(RuleMetadata {
            name: "noUnusedTemplateLiteral",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_UNUSED_TEMPLATE_LITERAL,
        }),
        "noUnusedVariables" => Some(RuleMetadata {
            name: "noUnusedVariables",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: false,
            deprecated: None,
            docs: NO_UNUSED_VARIABLES,
        }),
        "noUselessCatch" => Some(RuleMetadata {
            name: "noUselessCatch",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_USELESS_CATCH,
        }),
        "noUselessConstructor" => Some(RuleMetadata {
            name: "noUselessConstructor",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_USELESS_CONSTRUCTOR,
        }),
        "noUselessElse" => Some(RuleMetadata {
            name: "noUselessElse",
            version: "1.3.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_USELESS_ELSE,
        }),
        "noUselessEmptyExport" => Some(RuleMetadata {
            name: "noUselessEmptyExport",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: NO_USELESS_EMPTY_EXPORT,
        }),
        "noUselessFragments" => Some(RuleMetadata {
            name: "noUselessFragments",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_USELESS_FRAGMENTS,
        }),
        "noUselessLabel" => Some(RuleMetadata {
            name: "noUselessLabel",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: NO_USELESS_LABEL,
        }),
        "noUselessLoneBlockStatements" => Some(RuleMetadata {
            name: "noUselessLoneBlockStatements",
            version: "1.3.3",
            fix_kind: Some(FixKind::Unsafe),
            recommended: false,
            deprecated: None,
            docs: NO_USELESS_LONE_BLOCK_STATEMENTS,
        }),
        "noUselessRename" => Some(RuleMetadata {
            name: "noUselessRename",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: NO_USELESS_RENAME,
        }),
        "noUselessSwitchCase" => Some(RuleMetadata {
            name: "noUselessSwitchCase",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_USELESS_SWITCH_CASE,
        }),
        "noUselessThisAlias" => Some(RuleMetadata {
            name: "noUselessThisAlias",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: NO_USELESS_THIS_ALIAS,
        }),
        "noUselessTypeConstraint" => Some(RuleMetadata {
            name: "noUselessTypeConstraint",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: NO_USELESS_TYPE_CONSTRAINT,
        }),
        "noVar" => Some(RuleMetadata {
            name: "noVar",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_VAR,
        }),
        "noVoid" => Some(RuleMetadata {
            name: "noVoid",
            version: "1.0.0",
            fix_kind: None,
            recommended: false,
            deprecated: None,
            docs: NO_VOID,
        }),
        "noVoidElementsWithChildren" => Some(RuleMetadata {
            name: "noVoidElementsWithChildren",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: NO_VOID_ELEMENTS_WITH_CHILDREN,
        }),
        "noVoidTypeReturn" => Some(RuleMetadata {
            name: "noVoidTypeReturn",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_VOID_TYPE_RETURN,
        }),
        "noWith" => Some(RuleMetadata {
            name: "noWith",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: NO_WITH,
        }),
        "organizeImports" => Some(RuleMetadata {
            name: "organizeImports",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: false,
            deprecated: None,
            docs: ORGANIZE_IMPORTS,
        }),
        "useAltText" => Some(RuleMetadata {
            name: "useAltText",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: USE_ALT_TEXT,
        }),
        "useAnchorContent" => Some(RuleMetadata {
            name: "useAnchorContent",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: USE_ANCHOR_CONTENT,
        }),
        "useAriaActivedescendantWithTabindex" => Some(RuleMetadata {
            name: "useAriaActivedescendantWithTabindex",
            version: "1.3.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: USE_ARIA_ACTIVEDESCENDANT_WITH_TABINDEX,
        }),
        "useAriaPropsForRole" => Some(RuleMetadata {
            name: "useAriaPropsForRole",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: USE_ARIA_PROPS_FOR_ROLE,
        }),
        "useArrowFunction" => Some(RuleMetadata {
            name: "useArrowFunction",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: USE_ARROW_FUNCTION,
        }),
        "useAsConstAssertion" => Some(RuleMetadata {
            name: "useAsConstAssertion",
            version: "1.3.0",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: USE_AS_CONST_ASSERTION,
        }),
        "useAwait" => Some(RuleMetadata {
            name: "useAwait",
            version: "1.4.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: USE_AWAIT,
        }),
        "useBlockStatements" => Some(RuleMetadata {
            name: "useBlockStatements",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: false,
            deprecated: None,
            docs: USE_BLOCK_STATEMENTS,
        }),
        "useButtonType" => Some(RuleMetadata {
            name: "useButtonType",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: USE_BUTTON_TYPE,
        }),
        "useCollapsedElseIf" => Some(RuleMetadata {
            name: "useCollapsedElseIf",
            version: "1.1.0",
            fix_kind: Some(FixKind::Safe),
            recommended: false,
            deprecated: None,
            docs: USE_COLLAPSED_ELSE_IF,
        }),
        "useConst" => Some(RuleMetadata {
            name: "useConst",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: USE_CONST,
        }),
        "useDefaultParameterLast" => Some(RuleMetadata {
            name: "useDefaultParameterLast",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: USE_DEFAULT_PARAMETER_LAST,
        }),
        "useDefaultSwitchClauseLast" => Some(RuleMetadata {
            name: "useDefaultSwitchClauseLast",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: USE_DEFAULT_SWITCH_CLAUSE_LAST,
        }),
        "useEnumInitializers" => Some(RuleMetadata {
            name: "useEnumInitializers",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: USE_ENUM_INITIALIZERS,
        }),
        "useExhaustiveDependencies" => Some(RuleMetadata {
            name: "useExhaustiveDependencies",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: USE_EXHAUSTIVE_DEPENDENCIES,
        }),
        "useExponentiationOperator" => Some(RuleMetadata {
            name: "useExponentiationOperator",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: USE_EXPONENTIATION_OPERATOR,
        }),
        "useExportType" => Some(RuleMetadata {
            name: "useExportType",
            version: "next",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: USE_EXPORT_TYPE,
        }),
        "useFlatMap" => Some(RuleMetadata {
            name: "useFlatMap",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: USE_FLAT_MAP,
        }),
        "useForOf" => Some(RuleMetadata {
            name: "useForOf",
            version: "next",
            fix_kind: None,
            recommended: false,
            deprecated: None,
            docs: USE_FOR_OF,
        }),
        "useFragmentSyntax" => Some(RuleMetadata {
            name: "useFragmentSyntax",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: false,
            deprecated: None,
            docs: USE_FRAGMENT_SYNTAX,
        }),
        "useGetterReturn" => Some(RuleMetadata {
            name: "useGetterReturn",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: USE_GETTER_RETURN,
        }),
        "useGroupedTypeImport" => Some(RuleMetadata {
            name: "useGroupedTypeImport",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: USE_GROUPED_TYPE_IMPORT,
        }),
        "useHeadingContent" => Some(RuleMetadata {
            name: "useHeadingContent",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: USE_HEADING_CONTENT,
        }),
        "useHookAtTopLevel" => Some(RuleMetadata {
            name: "useHookAtTopLevel",
            version: "1.0.0",
            fix_kind: None,
            recommended: false,
            deprecated: None,
            docs: USE_HOOK_AT_TOP_LEVEL,
        }),
        "useHtmlLang" => Some(RuleMetadata {
            name: "useHtmlLang",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: USE_HTML_LANG,
        }),
        "useIframeTitle" => Some(RuleMetadata {
            name: "useIframeTitle",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: USE_IFRAME_TITLE,
        }),
        "useImportRestrictions" => Some(RuleMetadata {
            name: "useImportRestrictions",
            version: "1.0.0",
            fix_kind: None,
            recommended: false,
            deprecated: None,
            docs: USE_IMPORT_RESTRICTIONS,
        }),
        "useIsArray" => Some(RuleMetadata {
            name: "useIsArray",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: USE_IS_ARRAY,
        }),
        "useIsNan" => Some(RuleMetadata {
            name: "useIsNan",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: USE_IS_NAN,
        }),
        "useKeyWithClickEvents" => Some(RuleMetadata {
            name: "useKeyWithClickEvents",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: USE_KEY_WITH_CLICK_EVENTS,
        }),
        "useKeyWithMouseEvents" => Some(RuleMetadata {
            name: "useKeyWithMouseEvents",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: USE_KEY_WITH_MOUSE_EVENTS,
        }),
        "useLiteralEnumMembers" => Some(RuleMetadata {
            name: "useLiteralEnumMembers",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: USE_LITERAL_ENUM_MEMBERS,
        }),
        "useLiteralKeys" => Some(RuleMetadata {
            name: "useLiteralKeys",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: USE_LITERAL_KEYS,
        }),
        "useMediaCaption" => Some(RuleMetadata {
            name: "useMediaCaption",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: USE_MEDIA_CAPTION,
        }),
        "useNamespaceKeyword" => Some(RuleMetadata {
            name: "useNamespaceKeyword",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: USE_NAMESPACE_KEYWORD,
        }),
        "useNamingConvention" => Some(RuleMetadata {
            name: "useNamingConvention",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: false,
            deprecated: None,
            docs: USE_NAMING_CONVENTION,
        }),
        "useNumericLiterals" => Some(RuleMetadata {
            name: "useNumericLiterals",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: USE_NUMERIC_LITERALS,
        }),
        "useOptionalChain" => Some(RuleMetadata {
            name: "useOptionalChain",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: USE_OPTIONAL_CHAIN,
        }),
        "useRegexLiterals" => Some(RuleMetadata {
            name: "useRegexLiterals",
            version: "1.3.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: false,
            deprecated: None,
            docs: USE_REGEX_LITERALS,
        }),
        "useSelfClosingElements" => Some(RuleMetadata {
            name: "useSelfClosingElements",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: USE_SELF_CLOSING_ELEMENTS,
        }),
        "useShorthandArrayType" => Some(RuleMetadata {
            name: "useShorthandArrayType",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: false,
            deprecated: None,
            docs: USE_SHORTHAND_ARRAY_TYPE,
        }),
        "useShorthandAssign" => Some(RuleMetadata {
            name: "useShorthandAssign",
            version: "1.3.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: false,
            deprecated: None,
            docs: USE_SHORTHAND_ASSIGN,
        }),
        "useShorthandFunctionType" => Some(RuleMetadata {
            name: "useShorthandFunctionType",
            version: "next",
            fix_kind: Some(FixKind::Safe),
            recommended: false,
            deprecated: None,
            docs: USE_SHORTHAND_FUNCTION_TYPE,
        }),
        "useSimpleNumberKeys" => Some(RuleMetadata {
            name: "useSimpleNumberKeys",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: USE_SIMPLE_NUMBER_KEYS,
        }),
        "useSimplifiedLogicExpression" => Some(RuleMetadata {
            name: "useSimplifiedLogicExpression",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: false,
            deprecated: None,
            docs: USE_SIMPLIFIED_LOGIC_EXPRESSION,
        }),
        "useSingleCaseStatement" => Some(RuleMetadata {
            name: "useSingleCaseStatement",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: false,
            deprecated: None,
            docs: USE_SINGLE_CASE_STATEMENT,
        }),
        "useSingleVarDeclarator" => Some(RuleMetadata {
            name: "useSingleVarDeclarator",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: USE_SINGLE_VAR_DECLARATOR,
        }),
        "useTemplate" => Some(RuleMetadata {
            name: "useTemplate",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: USE_TEMPLATE,
        }),
        "useValidAnchor" => Some(RuleMetadata {
            name: "useValidAnchor",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: USE_VALID_ANCHOR,
        }),
        "useValidAriaProps" => Some(RuleMetadata {
            name: "useValidAriaProps",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: USE_VALID_ARIA_PROPS,
        }),
        "useValidAriaRole" => Some(RuleMetadata {
            name: "useValidAriaRole",
            version: "1.4.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: USE_VALID_ARIA_ROLE,
        }),
        "useValidAriaValues" => Some(RuleMetadata {
            name: "useValidAriaValues",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: USE_VALID_ARIA_VALUES,
        }),
        "useValidForDirection" => Some(RuleMetadata {
            name: "useValidForDirection",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: USE_VALID_FOR_DIRECTION,
        }),
        "useValidLang" => Some(RuleMetadata {
            name: "useValidLang",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: USE_VALID_LANG,
        }),
        "useValidTypeof" => Some(RuleMetadata {
            name: "useValidTypeof",
            version: "1.0.0",
            fix_kind: Some(FixKind::Unsafe),
            recommended: true,
            deprecated: None,
            docs: USE_VALID_TYPEOF,
        }),
        "useWhile" => Some(RuleMetadata {
            name: "useWhile",
            version: "1.0.0",
            fix_kind: Some(FixKind::Safe),
            recommended: true,
            deprecated: None,
            docs: USE_WHILE,
        }),
        "useYield" => Some(RuleMetadata {
            name: "useYield",
            version: "1.0.0",
            fix_kind: None,
            recommended: true,
            deprecated: None,
            docs: USE_YIELD,
        }),
        _ => None,
    }
}
const NO_ACCESS_KEY : & str = "Enforce that the `accessKey` attribute is not used on any HTML element.\n\nThe `accessKey` assigns a keyboard shortcut to the current element. However, the `accessKey` value\ncan conflict with keyboard commands used by screen readers and keyboard-only users, which leads to\ninconsistent keyboard actions across applications. To avoid accessibility complications,\nthis rule suggests users remove the `accessKey` attribute on elements.\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<input type=\"submit\" accessKey=\"s\" value=\"Submit\" />\n```\n\n```jsx,expect_diagnostic\n<a href=\"https://webaim.org/\" accessKey=\"w\">WebAIM.org</a>\n```\n\n```jsx,expect_diagnostic\n<button accessKey=\"n\">Next</button>\n```\n\n## Resources\n\n- [WebAIM: Keyboard Accessibility - Accesskey](https://webaim.org/techniques/keyboard/accesskey#spec)\n- [MDN `accesskey` documentation](https://developer.mozilla.org/docs/Web/HTML/Global_attributes/accesskey)\n" ;
const NO_ACCUMULATING_SPREAD : & str = "Disallow the use of spread (`...`) syntax on accumulators.\n\nSpread syntax allows an iterable to be expanded into its individual elements.\n\nSpread syntax should be avoided on accumulators (like those in `.reduce`)\nbecause it causes a time complexity of `O(n^2)` instead of `O(n)`.\n\nSource: https://prateeksurana.me/blog/why-using-object-spread-with-reduce-bad-idea/\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nvar a = ['a', 'b', 'c'];\na.reduce((acc, val) => [...acc, val], []);\n```\n\n```js,expect_diagnostic\nvar a = ['a', 'b', 'c'];\na.reduce((acc, val) => {return [...acc, val];}, []);\n```\n\n```js,expect_diagnostic\nvar a = ['a', 'b', 'c'];\na.reduce((acc, val) => ({...acc, [val]: val}), {});\n```\n\n## Valid\n\n```js\nvar a = ['a', 'b', 'c'];\na.reduce((acc, val) => {acc.push(val); return acc}, []);\n```\n" ;
const NO_APPROXIMATIVE_NUMERIC_CONSTANT : & str = "Usually, the definition in the standard library is more precise than what people come up with or the used constant exceeds the maximum precision of the number type.\n\nSource: https://rust-lang.github.io/rust-clippy/master/#approx_constant\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nlet x = 3.141;\n```\n```js,expect_diagnostic\nlet x = 2.302;\n```\n\n## Valid\n\n```js\nlet x = Math.PI;\n```\n```js\nlet x = Math.LN10;\n```" ;
const NO_ARGUMENTS : & str = "Disallow the use of ```arguments```\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nfunction f() {\nconsole.log(arguments);\n}\n```\n\n### Valid\n\n```cjs\nfunction f() {\nlet arguments = 1;\nconsole.log(arguments);\n}\n```" ;
const NO_ARIA_HIDDEN_ON_FOCUSABLE : & str = "Enforce that aria-hidden=\"true\" is not set on focusable elements.\n\n`aria-hidden=\"true\"` can be used to hide purely decorative content from screen reader users.\nA focusable element with `aria-hidden=\"true\"` can be reached by keyboard.\nThis can lead to confusion or unexpected behavior for screen reader users.\n\nSource: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-aria-hidden-on-focusable.md\n\n## Example\n\n### Invalid\n\n```js,expect_diagnostic\n<div aria-hidden=\"true\" tabIndex=\"0\" />\n```\n\n```js, expect_diagnostic\n<a href=\"/\" aria-hidden=\"true\" />\n```\n\n## Valid\n\n```js\n<button aria-hidden=\"true\" tabIndex=\"-1\" />\n```\n\n```js\n<div aria-hidden=\"true\"><a href=\"#\"></a></div>\n```\n\n## Resources\n\n- [aria-hidden elements do not contain focusable elements](https://dequeuniversity.com/rules/axe/html/4.4/aria-hidden-focus)\n- [Element with aria-hidden has no content in sequential focus navigation](https://www.w3.org/WAI/standards-guidelines/act/rules/6cfa84/proposed/)\n- [MDN aria-hidden](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden)\n" ;
const NO_ARIA_UNSUPPORTED_ELEMENTS : & str = "Enforce that elements that do not support ARIA roles, states, and properties do not have those attributes.\n\nSource: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/aria-unsupported-elements.md\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<meta charset=\"UTF-8\" role=\"meta\" />\n```\n\n```jsx,expect_diagnostic\n<html aria-required=\"true\" />\n```\n\n## Valid\n\n```jsx\n<meta charset=\"UTF-8\" />\n```\n\n```jsx\n<html></html>\n```\n\n" ;
const NO_ARRAY_INDEX_KEY : & str = "Discourage the usage of Array index in keys.\n\n> We don’t recommend using indexes for keys if the order of items may change.\nThis can negatively impact performance and may cause issues with component state.\nCheck out Robin Pokorny’s article for an\n[in-depth explanation on the negative impacts of using an index as a key](https://robinpokorny.com/blog/index-as-a-key-is-an-anti-pattern/).\nIf you choose not to assign an explicit key to list items then React will default to using indexes as keys.\n\nSource [React documentation](https://reactjs.org/docs/lists-and-keys.html#keys)\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\nsomething.forEach((Element, index) => {\n<Component key={index} >foo</Component>\n});\n```\n\n```jsx,expect_diagnostic\nReact.Children.map(this.props.children, (child, index) => (\nReact.cloneElement(child, { key: index })\n))\n```" ;
const NO_ASSIGN_IN_EXPRESSIONS : & str = "Disallow assignments in expressions.\n\nIn expressions, it is common to mistype a comparison operator (such as `==`) as an assignment operator (such as `=`).\nMoreover, the use of assignments in expressions is confusing.\nIndeed, expressions are often considered as side-effect free.\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\nlet a, b;\na = (b = 1) + 1;\n```\n\n```ts,expect_diagnostic\nlet a;\nif (a = 1) {\n}\n```\n\n```ts,expect_diagnostic\nfunction f(a) {\nreturn a = 1;\n}\n```\n\n### Valid\n\n```ts\nlet a;\na = 1;\n```" ;
const NO_ASYNC_PROMISE_EXECUTOR : & str = "Disallows using an async function as a Promise executor.\n\nThe executor function can also be an async function. However, this is usually a mistake, for a few reasons:\n1. If an async executor function throws an error, the error will be lost and won't cause the newly-constructed `Promise` to reject. This could make it difficult to debug and handle some errors.\n2. If a Promise executor function is using `await`, this is usually a sign that it is not actually necessary to use the `new Promise` constructor, or the scope of the `new Promise` constructor can be reduced.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nnew Promise(async function foo(resolve, reject) {})\n```\n\n```js,expect_diagnostic\nnew Promise(async (resolve, reject) => {})\n```\n\n```js,expect_diagnostic\nnew Promise(((((async () => {})))))\n```\n\n### Valid\n\n```js\nnew Promise((resolve, reject) => {})\nnew Promise((resolve, reject) => {}, async function unrelated() {})\nnew Foo(async (resolve, reject) => {})\nnew Foo((( (resolve, reject) => {} )))\n```" ;
const NO_AUTOFOCUS : & str = "Enforce that autoFocus prop is not used on elements.\n\nAutofocusing elements can cause usability issues for sighted and non-sighted users, alike.\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<input autoFocus />\n```\n\n```jsx,expect_diagnostic\n<input autoFocus=\"true\" />\n```\n\n```jsx,expect_diagnostic\n<input autoFocus={\"false\"} />\n```\n\n```jsx,expect_diagnostic\n<input autoFocus={undefined} />\n```\n\n### Valid\n\n```jsx\n<input />\n```\n\n```jsx\n<div />\n```\n\n```jsx\n<button />\n```\n\n```jsx\n// `autoFocus` prop in user created component is valid\n<MyComponent autoFocus={true} />\n```\n\n## Resources\n\n- [WHATWG HTML Standard, The autofocus attribute](https://html.spec.whatwg.org/multipage/interaction.html#attr-fe-autofocus)\n- [The accessibility of HTML 5 autofocus](https://brucelawson.co.uk/2009/the-accessibility-of-html-5-autofocus/)\n" ;
const NO_BANNED_TYPES : & str = "Disallow primitive type aliases and misleading types.\n\n- Enforce consistent names for primitive types\n\nPrimitive types have aliases.\nFor example, `Number` is an alias of `number`.\nThe rule recommends the lowercase primitive type names.\n\n- Disallow the `Function` type\n\nThe `Function` type is loosely typed and is thus considered dangerous or harmful.\n`Function` is equivalent to the type `(...rest: any[]) => any` that uses the unsafe `any` type.\n\n- Disallow the misleading non-nullable type `{}`\n\nIn TypeScript, the type `{}` doesn't represent an empty object.\nIt represents any value except `null` and `undefined`.\nThe following TypeScript example is perfectly valid:\n\n```ts,expect_diagnostic\nconst n: {} = 0\n```\n\nTo represent an empty object, you should use `{ [k: string]: never }` or `Record<string, never>`.\n\nTo avoid any confusion, the rule forbids the use of the type `{}`, except in two situations:\n\n1. In type constraints to restrict a generic type to non-nullable types:\n\n```ts\nfunction f<T extends {}>(x: T) {\nassert(x != null);\n}\n```\n\n2. In a type intersection to narrow a type to its non-nullable equivalent type:\n\n```ts\ntype NonNullableMyType = MyType & {};\n```\n\nIn this last case, you can also use the `NonNullable` utility type:\n\n```ts\ntype NonNullableMyType = NonNullable<MyType>;\n```\n\nSource: https://typescript-eslint.io/rules/ban-types\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\nlet foo: String = \"bar\";\n```\n\n```ts,expect_diagnostic\nlet bool = true as Boolean;\n```\n\n```ts,expect_diagnostic\nlet invalidTuple: [string, Boolean] = [\"foo\", false];\n```\n\n### Valid\n\n```ts\nlet foo: string = \"bar\";\n```\n\n```ts\nlet tuple: [boolean, string] = [false, \"foo\"];\n```\n" ;
const NO_BLANK_TARGET : & str = "Disallow `target=\"_blank\"` attribute without `rel=\"noreferrer\"`\n\nWhen creating anchor `a` element, there are times when its link has to be opened in a new browser tab\nvia `target=\"_blank\"` attribute. This attribute has to paired with `rel=\"noreferrer\"` or you're incur\nin a security issue.\n\nRefer to [the noreferrer documentation](https://html.spec.whatwg.org/multipage/links.html#link-type-noreferrer)\nand the [the noopener documentation](https://html.spec.whatwg.org/multipage/links.html#link-type-noopener)\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<a href='http://external.link' target='_blank'>child</a>\n```\n\n```jsx,expect_diagnostic\n<a href='http://external.link' target='_blank' rel=\"noopener\">child</a>\n```\n\n```jsx,expect_diagnostic\n<a {...props} href='http://external.link' target='_blank' rel=\"noopener\">child</a>\n```\n\n### Valid\n\n```jsx\n<a href='http://external.link' rel='noreferrer' target='_blank'>child</a>\n```\n\n```jsx\n<a href='http://external.link' target='_blank' rel=\"noopener\" {...props}>child</a>\n```" ;
const NO_CATCH_ASSIGN : & str = "Disallow reassigning exceptions in catch clauses.\n\nAssignment to a `catch` parameter can be misleading and confusing.\nIt is often unintended and indicative of a programmer error.\n\nSource: https://eslint.org/docs/latest/rules/no-ex-assign\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\ntry {\n\n} catch (e) {\ne;\ne = 10;\n}\n```\n\n### Valid\n\n```js\ntry {\n\n} catch (e) {\nlet e = 10;\ne = 100;\n}\n```" ;
const NO_CHILDREN_PROP : & str = "Prevent passing of **children** as props.\n\nWhen using JSX, the children should be nested between the opening and closing tags.\nWhen not using JSX, the children should be passed as additional arguments to `React.createElement`.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\n<FirstComponent children={'foo'} />\n```\n\n```js,expect_diagnostic\nReact.createElement('div', { children: 'foo' });\n```" ;
const NO_CLASS_ASSIGN : & str = "Disallow reassigning class members.\n\nA class declaration creates a variable that we can modify, however, the modification is a mistake in most cases.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nclass A {}\nA = 0;\n```\n\n```js,expect_diagnostic\nA = 0;\nclass A {}\n```\n\n```js,expect_diagnostic\nclass A {\nb() {\nA = 0;\n}\n}\n```\n\n```js,expect_diagnostic\nlet A = class A {\nb() {\nA = 0;\n// `let A` is shadowed by the class name.\n}\n}\n```\n\n### Valid\n\n```js\nlet A = class A {}\nA = 0; // A is a variable.\n```\n\n```js\nlet A = class {\nb() {\nA = 0; // A is a variable.\n}\n}\n```\n\n```js\nclass A {\nb(A) {\nA = 0; // A is a parameter.\n}\n}\n```\n" ;
const NO_COMMA_OPERATOR : & str = "Disallow comma operator.\n\nThe comma operator includes multiple expressions where only one is expected.\nIt evaluates every operand from left to right and returns the value of the last operand.\nIt frequently obscures side effects, and its use is often an accident.\n\nThe use of the comma operator in the initialization and update parts of a `for` is still allowed.\n\nSource: https://eslint.org/docs/latest/rules/no-sequences\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nconst foo = (doSomething(), 0);\n```\n\n```js,expect_diagnostic\nfor (; doSomething(), !!test; ) {}\n```\n\n```js,expect_diagnostic\n// Use a semicolon instead.\nlet a, b;\na = 1, b = 2;\n```\n\n### Valid\n\n```js\nfor(a = 0, b = 0; (a + b) < 10; a++, b += 2) {}\n```\n" ;
const NO_COMMENT_TEXT : & str = "Prevent comments from being inserted as text nodes\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nconst a3 = <div>// comment</div>;\n```\n\n```js,expect_diagnostic\nconst a4 = <div>/* comment */</div>;\n```\n\n```js,expect_diagnostic\nconst a5 = <div>/** comment */</div>;\n```\n\n### Valid\n\n```js\nconst a = <div>{/* comment */}</div>;\nconst a1 = <div>{/** comment */}</div>;\nconst a2 = <div className={\"cls\" /* comment */}></div>;\n```" ;
const NO_COMPARE_NEG_ZERO : & str = "Disallow comparing against `-0`\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\n(1 >= -0)\n```\n\n### Valid\n\n```js\n(1 >= 0)\n```" ;
const NO_CONFUSING_LABELS : & str = "Disallow labeled statements that are not loops.\n\nLabeled statements in JavaScript are used in conjunction with `break` and `continue` to control flow around multiple loops.\nTheir use for other statements is suspicious and unfamiliar.\n\nSource: https://eslint.org/docs/latest/rules/no-labels\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nlabel: f();\n```\n\n```js,expect_diagnostic\nlabel: {\nf();\nbreak label;\n}\n```\n\n```js,expect_diagnostic\nlabel: if (a) {\nf()\nbreak label;\n}\n```\n\n```js,expect_diagnostic\nlabel: switch (a) {\ncase 0:\nbreak label;\n}\n```\n\n### Valid\n\n```js\nouter: while (a) {\nwhile(b) {\nbreak outer;\n}\n}\n```" ;
const NO_CONFUSING_VOID_TYPE : & str = "Disallow `void` type outside of generic or return types.\n\n`void` in TypeScript refers to a function return that is meant to be ignored. Attempting to use a void type outside of a return type or generic type argument is often a sign of programmer error. `void` can also be misleading for other developers even if used correctly.\n\n> The `void` type means cannot be mixed with any other types, other than `never`, which accepts all types.\n> If you think you need this then you probably want the `undefined` type instead.\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\nlet foo: void;\n```\n\n```ts,expect_diagnostic\nfunction logSomething(thing: void) {}\n```\n\n```ts,expect_diagnostic\ninterface Interface {\nprop: void;\n}\n```\n\n```ts,expect_diagnostic\ntype PossibleValues = number | void;\n```\n\n### Valid\n\n```ts\nfunction foo(): void {};\n```\n\n```ts\nfunction doSomething(this: void) {}\n```\n\n```ts\nfunction printArg<T = void>(arg: T) {}\n```" ;
const NO_CONSOLE_LOG : & str = "Disallow the use of `console.log`\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nconsole.log()\n```\n\n## Valid\n\n```js\nconsole.info(\"info\");\nconsole.warn(\"warn\");\nconsole.error(\"error\");\nconsole.assert(true);\nconsole.table([\"foo\", \"bar\"]);\nconst console = { log() {} };\nconsole.log();\n```\n" ;
const NO_CONST_ASSIGN : & str = "Prevents from having `const` variables being re-assigned.\n\nTrying to assign a value to a `const` will cause an `TypeError` when the code is executed.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nconst a = 1;\na = 4;\n```\n\n```js,expect_diagnostic\nconst a = 2;\na += 1;\n```\n\n```js,expect_diagnostic\nconst a = 1;\n++a;\n```\n\n```js,expect_diagnostic\nconst a = 1, b = 2;\n\na = 2;\n```\n\n### Valid\n\n```js\nconst a = 10;\nlet b = 10;\nb = 20;\n```\n" ;
const NO_CONST_ENUM : & str = "Disallow TypeScript `const enum`\n\nConst enums are enums that should be inlined at use sites.\nConst enums are not supported by bundlers and are incompatible with the `isolatedModules` mode.\nTheir use can lead to import nonexistent values (because const enums are erased).\n\nThus, library authors and bundler users should not use const enums.\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\nconst enum Status {\nOpen,\nClose,\n}\n```\n\n### Valid\n\n```ts\nenum Status {\nOpen,\nClose,\n}\n```" ;
const NO_CONSTANT_CONDITION : & str = "Disallow constant expressions in conditions\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nif (false) {\ndoSomethingUnfinished();\n}\n```\n\n```js,expect_diagnostic\nif (Boolean(1)) {\ndoSomethingAlways();\n}\n```\n\n```js,expect_diagnostic\nif (undefined) {\ndoSomethingUnfinished();\n}\n```\n\n```js,expect_diagnostic\nfor (;-2;) {\ndoSomethingForever();\n}\n```\n\n```js,expect_diagnostic\nwhile (typeof x) {\ndoSomethingForever();\n}\n```\n\n```js,expect_diagnostic\nvar result = 0 ? a : b;\n```\n\n### Valid\n\n```js\nif (x === 0) {\ndoSomething();\n}\n\nfor (;;) {\ndoSomethingForever();\n}\n\nwhile (typeof x === \"undefined\") {\ndoSomething();\n}\n\ndo {\ndoSomething();\n} while (x);\n\nvar result = x !== 0 ? a : b;\n\n// Exception\nwhile (true) {\nif (x) { break; }\nx = f();\n}\n```\n" ;
const NO_CONSTRUCTOR_RETURN : & str = "Disallow returning a value from a `constructor`.\n\nReturning a value from a `constructor` of a class is a possible error.\nForbidding this pattern prevents errors resulting from unfamiliarity with JavaScript or a copy-paste error.\n\nOnly returning without a value is allowed, as it’s a control flow statement.\n\nSource: https://eslint.org/docs/latest/rules/no-constructor-return\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nclass A {\nconstructor() {\nreturn 0;\n}\n}\n```\n\n### Valid\n\n```js\nclass A {\nconstructor() {}\n}\n```\n\n```js\nclass B {\nconstructor(x) {\nreturn;\n}\n}\n```\n" ;
const NO_CONTROL_CHARACTERS_IN_REGEX : & str = "Prevents from having control characters and some escape sequences that match control characters in regular expressions.\n\nControl characters are hidden special characters that are numbered from 0 to 31 in the ASCII system.\nThey're not commonly used in JavaScript text. So, if you see them in a pattern (called a regular expression), it's probably a mistake.\n\nThe following elements of regular expression patterns are considered possible errors in typing and are therefore disallowed by this rule:\n\n- Hexadecimal character escapes from `\\x00` to `\\x1F`\n- Unicode character escapes from `\\u0000` to `\\u001F`\n- Unicode code point escapes from `\\u{0}` to `\\u{1F}`\n- Unescaped raw characters from U+0000 to U+001F\n\nControl escapes such as `\\t` and `\\n` are allowed by this rule.\n\nSource: https://eslint.org/docs/latest/rules/no-control-regex\n\n## Examples\n\n### Invalid\n```js,expect_diagnostic\nvar pattern1 = /\\x00/;\n```\n```js,expect_diagnostic\nvar pattern2 = /\\x0C/;\n```\n```js,expect_diagnostic\nvar pattern3 = /\\x1F/;\n```\n```js,expect_diagnostic\nvar pattern4 = /\\u000C/;\n```\n```js,expect_diagnostic\nvar pattern5 = /\\u{C}/u;\n```\n```js,expect_diagnostic\nvar pattern7 = new RegExp(\"\\x0C\");\n```\n```js,expect_diagnostic\nvar pattern7 = new RegExp(\"\\\\x0C\");\n```\n\n### Valid\n```js\nvar pattern1 = /\\x20/;\nvar pattern2 = /\\u0020/;\nvar pattern3 = /\\u{20}/u;\nvar pattern4 = /\\t/;\nvar pattern5 = /\\n/;\nvar pattern6 = new RegExp(\"\\x20\");\n```\n" ;
const NO_DANGEROUSLY_SET_INNER_HTML : & str = "Prevent the usage of dangerous JSX props\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\nfunction createMarkup() {\nreturn { __html: 'child' }\n}\n<div dangerouslySetInnerHTML={createMarkup()}></div>\n```\n\n```js,expect_diagnostic\nReact.createElement('div', {\ndangerouslySetInnerHTML: { __html: 'child' }\n});\n```" ;
const NO_DANGEROUSLY_SET_INNER_HTML_WITH_CHILDREN : & str = "Report when a DOM element or a component uses both `children` and `dangerouslySetInnerHTML` prop.\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\nfunction createMarkup() {\nreturn { __html: 'child' }\n}\n<Component dangerouslySetInnerHTML={createMarkup()}>\"child1\"</Component>\n```\n\n```jsx,expect_diagnostic\nfunction createMarkup() {\nreturn { __html: 'child' }\n}\n<Component dangerouslySetInnerHTML={createMarkup()} children=\"child1\" />\n```\n\n```js,expect_diagnostic\nReact.createElement('div', { dangerouslySetInnerHTML: { __html: 'HTML' } }, 'children')\n```" ;
const NO_DEBUGGER : & str = "Disallow the use of `debugger`\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\ndebugger;\n```\n\n### Valid\n\n```js\nconst test = { debugger: 1 };\ntest.debugger;\n```" ;
const NO_DEFAULT_EXPORT : & str = "Disallow default exports.\n\nDefault exports cannot be easily discovered inside an editor:\nThey cannot be suggested by the editor when the user tries to import a name.\n\nAlso, default exports don't encourage consistency over a code base:\nthe module that imports the default export must choose a name.\nIt is likely that different modules use different names.\n\nMoreover, default exports encourage exporting an object that acts as a namespace.\nThis is a legacy pattern used to mimic CommonJS modules.\n\nFor all these reasons, a team may want to disallow default exports.\n\nNote that this rule disallows only default exports in EcmaScript Module.\nIt ignores CommonJS default exports.\n\nSource: https://github.com/import-js/eslint-plugin-import/blob/main/docs/rules/no-default-export.md\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nexport default function f() {};\n```\n\n```js,expect_diagnostic\nexport default class C {};\n```\n\n```js,expect_diagnostic\nexport default {\nf() {},\ng() {},\n};\n```\n\n```js,expect_diagnostic\nexport { X as default };\n```\n\n## Valid\n\n```js\nexport function f () {};\nexport class C {};\nexport { default as X } from \"mod\";\n```\n\n```cjs\nmodule.exports = class {};\n```\n" ;
const NO_DELETE : & str = "Disallow the use of the `delete` operator.\n\nThe `delete` operator enables the removal of a property from an object.\n\nThe `delete` operator should be avoided because it [can prevent some optimizations of _JavaScript_ engines](https://webkit.org/blog/10298/inline-caching-delete/).\nMoreover, it can lead to unexpected results.\nFor instance, deleting an array element [does not change the length of the array](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/delete#deleting_array_elements).\n\nThe only legitimate use of `delete` is on an object that behaves like a _map_.\nTo allow this pattern, this rule does not report `delete` on computed properties that are not literal values.\nConsider using [Map](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map) instead of an object.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nconst arr = [1, 2, 3];\ndelete arr[0];\n```\n\n```js,expect_diagnostic\nconst obj = {a: {b: {c: 123}}};\ndelete obj.a.b.c;\n```\n\n### Valid\n\n```js\nconst foo = new Set([1,2,3]);\nfoo.delete(1);\n```\n\n```js\nconst map = Object.create(null);\nconst key = \"key\"\nmap[key] = \"value\"\ndelete map[key];\n```\n\n```js\nlet x = 5;\ndelete f(); // uncovered by this rule.\n```\n" ;
const NO_DISTRACTING_ELEMENTS : & str = "Enforces that no distracting elements are used.\n\nElements that can be visually distracting can cause accessibility issues with visually impaired users.\nSuch elements are most likely deprecated, and should be avoided.\nBy default, the following elements are visually distracting: `<marquee>` and `<blink>`.\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<marquee />\n```\n\n```jsx,expect_diagnostic\n<blink />\n```\n\n### Valid\n\n```jsx\n<div />\n```\n\n## Accessibility guidelines\n\n- [WCAG 2.2.2](https://www.w3.org/WAI/WCAG21/Understanding/pause-stop-hide)\n" ;
const NO_DOUBLE_EQUALS : & str = "Require the use of `===` and `!==`\n\nIt is generally bad practice to use `==` for comparison instead of\n`===`. Double operators will triger implicit [type coercion](https://developer.mozilla.org/en-US/docs/Glossary/Type_coercion)\nand are thus not prefered. Using strict equality operators is almost\nalways best practice.\n\nFor ergonomic reasons, this rule makes an exception for `== null` for\ncomparing to both `null` and `undefined`.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nfoo == bar\n```\n\n### Valid\n\n```js\nfoo == null\n```\n\n```js\nfoo != null\n```\n\n```js\nnull == foo\n```\n\n```js\nnull != foo\n```" ;
const NO_DUPLICATE_CASE : & str = "Disallow duplicate case labels.\n\nIf a switch statement has duplicate test expressions in case clauses, it is likely that a programmer copied a case clause but forgot to change the test expression.\n\nSource: https://eslint.org/docs/latest/rules/no-duplicate-case\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nswitch (a) {\ncase 1:\nbreak;\ncase 1:\nbreak;\ndefault:\nbreak;\n}\n```\n\n```js,expect_diagnostic\nswitch (a) {\ncase one:\nbreak;\ncase one:\nbreak;\ndefault:\nbreak;\n}\n```\n\n```js,expect_diagnostic\nswitch (a) {\ncase \"1\":\nbreak;\ncase \"1\":\nbreak;\ndefault:\nbreak;\n}\n```\n\n### Valid\n\n```js\nswitch (a) {\ncase 1:\nbreak;\ncase 2:\nbreak;\ndefault:\nbreak;\n}\n```\n\n```js\nswitch (a) {\ncase one:\nbreak;\ncase two:\nbreak;\ndefault:\nbreak;\n}\n```\n\n```js\nswitch (a) {\ncase \"1\":\nbreak;\ncase \"2\":\nbreak;\ndefault:\nbreak;\n}\n```" ;
const NO_DUPLICATE_CLASS_MEMBERS : & str = "Disallow duplicate class members.\n\nIf there are declarations of the same name among class members,\nthe last declaration overwrites other declarations silently.\nIt can cause unexpected behaviours.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nclass Foo {\nbar() { }\nbar() { }\n}\n```\n\n```js,expect_diagnostic\nclass Foo {\nbar() { }\nget bar() { }\n}\n```\n\n```js,expect_diagnostic\nclass Foo {\nbar;\nbar() { }\n}\n```\n\n```js,expect_diagnostic\nclass Foo {\nstatic bar() { }\nstatic bar() { }\n}\n```\n\n## Valid\n\n```js\nclass Foo {\nbar() { }\nqux() { }\n}\n```\n\n```js\nclass Foo {\nset bar(value) { }\nget bar() { }\n}\n```\n\n```js\nclass Foo {\nbar;\nqux;\n}\n```\n\n```js\nclass Foo {\nbar;\nqux() { }\n}\n```\n\n```js\nclass Foo {\nstatic bar() { }\nbar() { }\n}\n```\n" ;
const NO_DUPLICATE_JSON_KEYS : & str = "Disallow two keys with the same name inside a JSON object.\n\n## Examples\n\n### Invalid\n\n```json,expect_diagnostic\n{\n\"title\": \"New title\",\n\"title\": \"Second title\"\n}\n```\n\n### Valid\n\n```json\n{\n\"title\": \"New title\",\n\"secondTitle\": \"Second title\"\n}\n```" ;
const NO_DUPLICATE_JSX_PROPS : & str = "Prevents JSX properties to be assigned multiple times.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\n<Hello name=\"John\" name=\"John\" />\n```\n\n```js,expect_diagnostic\n<label xml:lang=\"en-US\" xml:lang=\"en-US\"></label>\n```\n\n### Valid\n\n```js\n<Hello firstname=\"John\" lastname=\"Doe\" />\n```\n\n```js\n<label xml:lang=\"en-US\" lang=\"en-US\"></label>\n```" ;
const NO_DUPLICATE_OBJECT_KEYS : & str = "Prevents object literals having more than one property declaration for the same name.\n\nIf an object property with the same name is defined multiple times (except when combining a getter with a setter), only the last definition makes it into the object and previous definitions are ignored, which is likely a mistake.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nconst obj = {\na: 1,\na: 2,\n}\n```\n\n```js,expect_diagnostic\nconst obj = {\nset a(v) {},\na: 2,\n}\n```\n\n### Valid\n\n```js\nconst obj = {\na: 1,\nb: 2,\n}\n```\n\n```js\nconst obj = {\nget a() { return 1; },\nset a(v) {},\n}\n```\n" ;
const NO_DUPLICATE_PARAMETERS : & str = "Disallow duplicate function parameter name.\n\nIf more than one parameter has the same name in a function definition,\nthe last occurrence overrides the preceding occurrences.\nA duplicated name might be a typing error.\n\nSource: https://eslint.org/docs/latest/rules/no-dupe-args\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nvar f = function(a, b, b) {}\n```\n\n```js,expect_diagnostic\nfunction b(a, b, b) {}\n```\n\n### Valid\n\n```js\nfunction i(i, b, c) {}\nvar j = function (j, b, c) {};\nfunction k({ k, b }, { c, d }) {}\nfunction l([, l]) {}\nfunction foo([[a, b], [c, d]]) {}\n```" ;
const NO_DUPLICATE_PRIVATE_CLASS_MEMBERS : & str = "Catch a `SyntaxError` when defining duplicate private class members.\n\n## Examples\n\n```js\nclass A {\n#foo;\n#foo;\n```" ;
const NO_EMPTY_BLOCK_STATEMENTS : & str = "Disallow empty block statements and static blocks.\n\nEmpty static blocks and block statements, while not technically errors, usually occur due to refactoring that wasn’t completed. They can cause confusion when reading code.\n\nThis rule disallows empty block statements and static blocks.\nThis rule ignores block statements or static blocks which contain a comment (for example, in an empty catch or finally block of a try statement to indicate that execution should continue regardless of errors).\n\nSource: https://eslint.org/docs/latest/rules/no-empty-static-block/\nSource: https://eslint.org/docs/latest/rules/no-empty/\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nfunction emptyFunctionBody () {}\n```\n\n```js,expect_diagnostic\ntry {\ndoSomething();\n} catch(ex) {\n\n}\n```\n\n```js,expect_diagnostic\nclass Foo {\nstatic {}\n}\n```\n\n## Valid\n\n```js\nfunction foo () {\ndoSomething();\n}\n```\n\n```js\ntry {\ndoSomething();\n} catch (ex) {\n// continue regardless of error\n}\n```\n" ;
const NO_EMPTY_CHARACTER_CLASS_IN_REGEX : & str = "Disallow empty character classes in regular expression literals.\n\nEmpty character classes don't match anything.\nIn contrast, negated empty classes match any character.\nThey are often the result of a typing mistake.\n\nSource: https://eslint.org/docs/latest/rules/no-empty-character-class/\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\n/^a[]/.test(\"a\"); // false\n```\n\n```js,expect_diagnostic\n/^a[^]/.test(\"ax\"); // true\n```\n\n## Valid\n\n```js\n/^a[xy]/.test(\"ay\"); // true\n```\n\n```js\n/^a[^xy]/.test(\"ab\"); // true\n```\n\n```js\n/^a\\[]/.test(\"a[]\"); // true\n```\n" ;
const NO_EMPTY_INTERFACE : & str = "Disallow the declaration of empty interfaces.\n\nAn empty interface in TypeScript does very little: any non-nullable value is assignable to `{}`.\nUsing an empty interface is often a sign of programmer error, such as misunderstanding the concept of `{}` or forgetting to fill in fields.\n\nSource: https://typescript-eslint.io/rules/no-empty-interface\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\ninterface A {}\n```\n\n```ts,expect_diagnostic\ninterface A extends B {}\n```\n\n### Valid\n\n```ts\ninterface A {\nprop: string;\n}\n\n// Allow empty interfaces that extend at least two types.\ninterface A extends B, C {}\n\ndeclare module \"@external/module\" {\n// Allow empty interfaces that extend at least one type in external module.\ninterface Existing extends A {}\n}\n```" ;
const NO_EMPTY_PATTERN : & str = "Disallows empty destructuring patterns.\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nvar {} = foo;\n```\n\n```js,expect_diagnostic\nvar {a: {}} = foo;\n```\n\n```js,expect_diagnostic\nfunction foo({}) {}\n```\n\n### Valid\nThe following cases are valid because they create new bindings.\n\n```js\nvar {a = {}} = foo;\nvar {a, b = {}} = foo;\nvar {a = []} = foo;\nfunction foo({a = {}}) {}\nfunction foo({a = []}) {}\nvar [a] = foo;\n```" ;
const NO_EXCESSIVE_COGNITIVE_COMPLEXITY : & str = "Disallow functions that exceed a given Cognitive Complexity score.\n\nThe more complexity a function contains, the harder it is to understand\nlater on.\n\nReducing complexity helps to make code more maintenable, both by making\nit easier to understand as well as by reducing chances of accidental\nside-effects when making changes.\n\nThis rule calculates a complexity score for every function and disallows\nthose that exceed a configured complexity threshold (default: 15).\n\nThe complexity score is calculated based on the Cognitive Complexity\nalgorithm: http://redirect.sonarsource.com/doc/cognitive-complexity.html\n\nSource:\n\n* https://github.com/SonarSource/eslint-plugin-sonarjs/blob/HEAD/docs/rules/cognitive-complexity.md\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nfunction tooComplex() {\nfor (let x = 0; x < 10; x++) {\nfor (let y = 0; y < 10; y++) {\nfor (let z = 0; z < 10; z++) {\nif (x % 2 === 0) {\nif (y % 2 === 0) {\nconsole.log(x > y ? `${x} > ${y}` : `${y} > ${x}`);\n}\n}\n}\n}\n}\n}\n```\n\n## Options\n\nAllows to specify the maximum allowed complexity.\n\n```json\n{\n\"//\": \"...\",\n\"options\": {\n\"maxAllowedComplexity\": 15\n}\n}\n```\n\nThe allowed values range from 1 through 254. The default is 15.\n" ;
const NO_EXPLICIT_ANY : & str = "Disallow the `any` type usage.\n\nThe `any` type in TypeScript is a dangerous \"escape hatch\" from the type system.\nUsing `any` disables many type checking rules and is generally best used only as a last resort or when prototyping code.\n\nTypeScript's `--noImplicitAny` compiler option prevents an implied `any`,\nbut doesn't prevent `any` from being explicitly used the way this rule does.\n\nSometimes you can use the type `unknown` instead of the type `any`.\nIt also accepts any value, however it requires to check that a property exists before calling it.\n\nSource: https://typescript-eslint.io/rules/no-explicit-any\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\nlet variable: any = 1;\n```\n\n```ts,expect_diagnostic\nclass SomeClass {\nmessage: Array<Array<any>>;\n}\n```\n\n```ts,expect_diagnostic\nfunction fn(param: Array<any>): void {}\n```\n\n### Valid\n\n```ts\nlet variable: number = 1;\nlet variable2 = 1;\n```\n\n```ts\nclass SomeClass<T extends any> {\nmessage: Array<Array<unknown>>;\n}\n```\n\n```ts\nfunction fn(param: Array<Array<unknown>>): Array<unknown> {}\n```\n" ;
const NO_EXTRA_BOOLEAN_CAST : & str = "Disallow unnecessary boolean casts\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nif (!Boolean(foo)) {\n}\n```\n\n```js,expect_diagnostic\nwhile (!!foo) {}\n```\n\n```js,expect_diagnostic\nlet x = 1;\ndo {\n1 + 1;\n} while (Boolean(x));\n```\n\n```js,expect_diagnostic\nfor (; !!foo; ) {}\n```\n\n```js,expect_diagnostic\nnew Boolean(!!x);\n```\n\n### Valid\n```js\nBoolean(!x);\n!x;\n!!x;\n```" ;
const NO_EXTRA_NON_NULL_ASSERTION : & str = "Prevents the wrong usage of the non-null assertion operator (`!`) in TypeScript files.\n\n> The `!` non-null assertion operator in TypeScript is used to assert that a value's type does not include `null` or `undefined`. Using the operator any more than once on a single value does nothing.\n\nSource: https://typescript-eslint.io/rules/no-extra-non-null-assertion\n\n## Examples\n\n### Invalid\n```ts,expect_diagnostic\nconst bar = foo!!.bar;\n```\n\n```ts,expect_diagnostic\nfunction fn(bar?: { n: number }) {\nreturn bar!?.n;\n}\n```\n\n```ts,expect_diagnostic\nfunction fn(bar?: { n: number }) {\nreturn ((bar!))?.();\n}\n```\n\n### Valid\n```ts\nconst bar = foo!.bar;\n\nobj?.string!.trim();\n\nfunction fn(key: string | null) {\nconst obj = {};\nreturn obj?.[key!];\n}\n```\n" ;
const NO_FALLTHROUGH_SWITCH_CLAUSE : & str = "Disallow fallthrough of `switch` clauses.\n\nSwitch clauses in `switch` statements fall through by default.\nThis can lead to unexpected behavior when forgotten.\n\nSource: https://eslint.org/docs/latest/rules/no-fallthrough\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nswitch (bar) {\ncase 0:\na();\ncase 1:\nb();\n}\n```\n\n## Valid\n\n```js\nswitch (foo) {\ncase 1:\ncase 2:\ndoSomething();\nbreak;\ncase 3: {\nif (cond) {\nbreak;\n} else {\nbreak;\n}\n}\ncase 4:\ndoSomething();\n}\n```\n" ;
const NO_FOR_EACH : & str = "Prefer `for...of` statement instead of `Array.forEach`.\n\nHere's a summary of why `forEach` may be disallowed, and why `for...of` is preferred for almost any use-case of `forEach`:\n- Performance: Using `forEach` can lead to performance issues, especially when working with large arrays.\nWhen more requirements are added on, `forEach` typically gets chained with other methods like `filter` or `map`, causing multiple iterations over the same Array.\nEncouraging for loops discourages chaining and encourages single-iteration logic (e.g. using a continue instead of `filter`).\n\n- Readability: While `forEach` is a simple and concise way to iterate over an array, it can make the code less readable, especially when the callback function is complex.\nIn contrast, using a for loop or a `for...of` loop can make the code more explicit and easier to read.\n\n- Debugging: `forEach` can make debugging more difficult, because it hides the iteration process.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nels.forEach(el => {\nel\n})\n```\n\n```js,expect_diagnostic\nels['forEach'](el => {\nel\n})\n```\n\n## Valid\n\n```js\nfor (const el of els) {\nel\n}\n```\n" ;
const NO_FUNCTION_ASSIGN : & str = "Disallow reassigning function declarations.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nfunction foo() { };\nfoo = bar;\n```\n\n```js,expect_diagnostic\nfunction foo() {\nfoo = bar;\n}\n```\n\n```js,expect_diagnostic\nfoo = bar;\nfunction foo() { };\n```\n\n```js,expect_diagnostic\n[foo] = bar;\nfunction foo() { };\n```\n\n```js,expect_diagnostic\n({ x: foo = 0 } = bar);\nfunction foo() { };\n```\n\n```js,expect_diagnostic\nfunction foo() {\n[foo] = bar;\n}\n```\n```js,expect_diagnostic\n(function () {\n({ x: foo = 0 } = bar);\nfunction foo() { };\n})();\n```\n\n## Valid\n\n```js\nfunction foo() {\nvar foo = bar;\n}\n```\n\n```js\nfunction foo(foo) {\nfoo = bar;\n}\n```\n\n```js\nfunction foo() {\nvar foo;\nfoo = bar;\n}\n```\n\n```js\nvar foo = () => {};\nfoo = bar;\n```\n\n```js\nvar foo = function() {};\nfoo = bar;\n```\n\n```js\nvar foo = function() {\nfoo = bar;\n};\n```\n\n```js\nimport bar from 'bar';\nfunction foo() {\nvar foo = bar;\n}\n```" ;
const NO_GLOBAL_IS_FINITE : & str = "Use `Number.isFinite` instead of global `isFinite`.\n\n`Number.isFinite()` and `isFinite()` [have not the same behavior](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/isFinite#difference_between_number.isfinite_and_global_isfinite).\nWhen the argument to `isFinite()` is not a number, the value is first coerced to a number.\n`Number.isFinite()` does not perform this coercion.\nTherefore, it is a more reliable way to test whether a number is finite.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nisFinite(false); // true\n```\n\n## Valid\n\n```js\nNumber.isFinite(false); // false\n```" ;
const NO_GLOBAL_IS_NAN : & str = "Use `Number.isNaN` instead of global `isNaN`.\n\n`Number.isNaN()` and `isNaN()` [have not the same behavior](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/isNaN#description).\nWhen the argument to `isNaN()` is not a number, the value is first coerced to a number.\n`Number.isNaN()` does not perform this coercion.\nTherefore, it is a more reliable way to test whether a value is `NaN`.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nisNaN({}); // true\n```\n\n## Valid\n\n```js\nNumber.isNaN({}); // false\n```\n" ;
const NO_GLOBAL_OBJECT_CALLS : & str = "Disallow calling global object properties as functions\n\nECMAScript provides several global objects that are intended to be used as-is.\nSome of these objects look as if they could be constructors due their capitalization (such as Math and JSON) but will throw an error if you try to execute them as functions.\n\nThe ECMAScript 5 specification makes it clear that both Math and JSON cannot be invoked:\nThe Math object does not have a [[Call]] internal property; it is not possible to invoke the Math object as a function.\n\nThe ECMAScript 2015 specification makes it clear that Reflect cannot be invoked:\nThe Reflect object also does not have a [[Call]] internal method; it is not possible to invoke the Reflect object as a function.\n\nThe ECMAScript 2017 specification makes it clear that Atomics cannot be invoked:\nThe Atomics object does not have a [[Call]] internal method; it is not possible to invoke the Atomics object as a function.\n\nAnd the ECMAScript Internationalization API Specification makes it clear that Intl cannot be invoked:\nThe Intl object does not have a [[Call]] internal method; it is not possible to invoke the Intl object as a function.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nvar math = Math();\n```\n\n```js,expect_diagnostic\nvar newMath = new Math();\n```\n\n```js,expect_diagnostic\nvar json = JSON();\n```\n\n```js,expect_diagnostic\nvar newJSON = new JSON();\n```\n\n```js,expect_diagnostic\nvar reflect = Reflect();\n```\n\n```js,expect_diagnostic\nvar newReflect = new Reflect();\n```\n\n```js,expect_diagnostic\nvar atomics = Atomics();\n```\n\n```js,expect_diagnostic\nvar newAtomics = new Atomics();\n```\n\n```js,expect_diagnostic\nvar intl = Intl();\n```\n\n```js,expect_diagnostic\nvar newIntl = new Intl();\n```\n\n## Valid\n\n```js\nfunction area(r) {\nreturn Math.PI * r * r;\n}\n\nvar object = JSON.parse(\"{}\");\n\nvar value = Reflect.get({ x: 1, y: 2 }, \"x\");\n\nvar first = Atomics.load(foo, 0);\n\nvar segmenterFr = new Intl.Segmenter(\"fr\", { granularity: \"word\" });\n```\n" ;
const NO_HEADER_SCOPE : & str = "The scope prop should be used only on `<th>` elements.\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<div scope={scope} />\n```\n\n```jsx,expect_diagnostic\n<div scope=\"col\" />\n```\n\n### Valid\n\n```jsx\n<th scope={scope}></th>\n```\n\n```jsx\n<th scope=\"col\"></th>\n```\n\n## Accessibility guidelines\n\n- [WCAG 1.3.1](https://www.w3.org/WAI/WCAG21/Understanding/info-and-relationships)\n- [WCAG 4.1.1](https://www.w3.org/WAI/WCAG21/Understanding/parsing)\n" ;
const NO_IMPLICIT_ANY_LET : & str = "Disallow use of implicit `any` type on variable declarations.\n\nTypeScript variable declaration without any type annotation and initialization have the `any` type.\nThe any type in TypeScript is a dangerous “escape hatch” from the type system.\nUsing any disables many type checking rules and is generally best used only as a last resort or when prototyping code.\nTypeScript’s `--noImplicitAny` compiler option doesn't report this case.\n\n\n\nSource: https://www.typescriptlang.org/tsconfig#noImplicitAny\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\nvar a;\na = 2;\n````\n\n```ts,expect_diagnostic\nlet b;\nb = 1\n```\n\n## Valid\n\n```ts\nvar a = 1;\nlet a:number;\nvar b: number\nvar b =10;\n```\n" ;
const NO_IMPLICIT_BOOLEAN : & str = "Disallow implicit `true` values on JSX boolean attributes\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<input disabled />\n```\n\n### Valid\n\n```jsx\n<input disabled={false} />\n```\n\n```jsx\n<input disabled={''} />\n```\n\n```jsx\n<input disabled={0} />\n```\n\n```jsx\n<input disabled={undefined} />\n```\n\n```jsx\n<input disabled='false' />\n```" ;
const NO_IMPORT_ASSIGN : & str = "Disallow assigning to imported bindings\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nimport x from \"y\";\nx = 1;\n```\n```js,expect_diagnostic\nimport y from \"y\";\n[y] = 1;\n```\n```js,expect_diagnostic\nimport z from \"y\";\n({ z } = 1); /// ```\n```js,expect_diagnostic\nimport a from \"y\";\n[...a] = 1;\n```\n```js,expect_diagnostic\nimport b from \"y\";\n({ ...b } = 1);\n```\n```js,expect_diagnostic\nimport c from \"y\";\nfor (c in y) {};\n```\n\n```js,expect_diagnostic\nimport d from \"y\";\nd += 1;\n```\n```js,expect_diagnostic\nimport * as e from \"y\";\ne = 1;\n```" ;
const NO_INFERRABLE_TYPES : & str = "Disallow type annotations for variables, parameters, and class properties initialized with a literal expression.\n\nTypeScript is able to infer the types of parameters, properties, and variables from their default or initial values.\nThere is no need to use an explicit `:` type annotation for trivially inferred types (boolean, bigint, number, regex, string).\nDoing so adds unnecessary verbosity to code making it harder to read.\n\nIn contrast to ESLint's rule, this rule allows to use a wide type for `const` declarations.\nMoreover, the rule does not recognize `undefined` values, primitive type constructors (String, Number, ...), and `RegExp` type.\nThese global variables could be shadowed by local ones.\n\nSource: https://typescript-eslint.io/rules/no-inferrable-types\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\nconst variable: 1 = 1;\n```\n\n```ts,expect_diagnostic\nlet variable: number = 1;\n```\n\n```ts,expect_diagnostic\nclass SomeClass {\nreadonly field: 1 = 1;\n}\n```\n\n```ts,expect_diagnostic\nclass SomeClass {\nfield: number = 1;\n}\n```\n\n```ts,expect_diagnostic\nfunction f(param: number = 1): void {}\n```\n\n### Valid\n\n```ts\nconst variable: number = 1;\n```\n\n```ts\nlet variable: 1 | 2 = 1;\n```\n\n```ts\nclass SomeClass {\nreadonly field: number = 1;\n}\n```\n\n```ts\n// `undefined` could be shadowed\nconst variable: undefined = undefined;\n```\n\n```ts\n// `RegExp` could be shadowed\nconst variable: RegExp = /a/;\n```\n\n```ts\n// `String` could be shadowed\nlet variable: string = String(5);\n```\n\n```ts\nclass SomeClass {\nfield: 1 | 2 = 1;\n}\n```\n\n```ts\nfunction f(param: 1 | 2 = 1): void {}\n```\n" ;
const NO_INITIALIZER_WITH_DEFINITE : & str = "Disallow initializing a variable with a definite assertion to prevent `SyntaxError`.\n\n## Examples\n\n```js\nlet foo!: string = \"bar\";\n```" ;
const NO_INNER_DECLARATIONS : & str = "Disallow `function` and `var` declarations that are accessible outside their block.\n\nA `var` is accessible in the whole body of the nearest root (function, module, script, static block).\nTo avoid confusion, they should be declared to the nearest root.\n\nPrior to ES2015, `function` declarations were only allowed in the nearest root,\nthough parsers sometimes erroneously accept them elsewhere.\nIn ES2015, inside an _ES module_, a `function` declaration is always block-scoped.\n\nNote that `const` and `let` declarations are block-scoped,\nand therefore they are not affected by this rule.\nMoreover, `function` declarations in nested blocks are allowed inside _ES modules_.\n\nSource: https://eslint.org/docs/rules/no-inner-declarations\n\n## Examples\n\n### Invalid\n\n```cjs,expect_diagnostic\nif (test) {\nfunction f() {}\n}\n```\n\n```js,expect_diagnostic\nif (test) {\nvar x = 1;\n}\n```\n\n```cjs,expect_diagnostic\nfunction f() {\nif (test) {\nfunction g() {}\n}\n}\n```\n\n```js,expect_diagnostic\nfunction f() {\nif (test) {\nvar x = 1;\n}\n}\n```\n\n### Valid\n\n```js\n// inside a module, function declarations are block-scoped and thus allowed.\nif (test) {\nfunction f() {}\n}\nexport {}\n```\n\n```js\nfunction f() { }\n```\n\n```js\nfunction f() {\nfunction g() {}\n}\n```\n\n```js\nfunction f() {\nvar x = 1;\n}\n```\n\n```js\nfunction f() {\nif (test) {\nconst g = function() {};\n}\n}\n```\n" ;
const NO_INTERACTIVE_ELEMENT_TO_NONINTERACTIVE_ROLE : & str = "Enforce that non-interactive ARIA roles are not assigned to interactive HTML elements.\n\nInteractive HTML elements indicate controls in the user interface.\nInteractive elements include `<a href>`, `<button>`, `<input>`, `<select>`, `<textarea>`.\nNon-interactive HTML elements and non-interactive ARIA roles indicate content and containers in the user interface.\nNon-interactive elements include `<main>`, `<area>`, `<h1>` (,`<h2>`, etc), `<img>`, `<li>`, `<ul>` and `<ol>`.\n\n[WAI-ARIA roles](https://www.w3.org/TR/wai-aria-1.1/#usage_intro) should not be used to convert an interactive element to a non-interactive element.\nNon-interactive ARIA roles include `article`, `banner`, `complementary`, `img`, `listitem`, `main`, `region` and `tooltip`.\n\nSource: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-interactive-element-to-noninteractive-role.md\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<input role=\"img\" />;\n```\n\n## Valid\n\n```jsx\n<input role=\"button\" />;\n```\n" ;
const NO_INVALID_CONSTRUCTOR_SUPER : & str = "Prevents the incorrect use of `super()` inside classes. It also checks whether a call `super()` is missing from classes that extends other constructors.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nclass A {\nconstructor() {\nsuper();\n}\n}\n```\n\n```js,expect_diagnostic\nclass A extends undefined {\nconstructor() {\nsuper();\n}\n}\n```\n\n### Valid\n\n```js\nexport default class A extends B {\nconstructor() {\nsuper();\n}\n}\n```\n\n```js\nexport class A {\nconstructor() {}\n}\n```\n" ;
const NO_INVALID_NEW_BUILTIN : & str = "Disallow `new` operators with global non-constructor functions.\n\nSome global functions cannot be called using the new operator and\nwill throw a `TypeError` if you attempt to do so. These functions are:\n\n- [`Symbol`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/Symbol)\n- [`BigInt`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/BigInt/BigInt)\n\nSource: https://eslint.org/docs/latest/rules/no-new-native-nonconstructor/\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nlet foo = new Symbol('foo');\n```\n\n```js,expect_diagnostic\nlet bar = new BigInt(9007199254740991);\n```\n\n## Valid\n\n```js\nlet foo = Symbol('foo');\n\nfunction baz(Symbol) {\nconst qux = new Symbol(\"baz\");\n}\n```\n\n```js\nlet bar = BigInt(9007199254740991);\n\nfunction quux(BigInt) {\nconst corge = new BigInt(9007199254740991);\n}\n```" ;
const NO_LABEL_VAR : & str = "Disallow labels that share a name with a variable\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nconst x1 = \"test\";\nx1: expr;\n```\n\n### Valid\n\n```js\nconst x = \"test\";\nz: expr;\n```" ;
const NO_MISLEADING_CHARACTER_CLASS : & str = "Disallow characters made with multiple code points in character class syntax.\n\nUnicode includes the characters which are made with multiple code points. e.g. A\u{301}, 🇯🇵, 👨\u{200d}👩\u{200d}👦.\nA RegExp character class `/[abc]/` cannot handle characters with multiple code points.\nFor example, the character `❇\u{fe0f}` consists of two code points: `❇` (U+2747) and `VARIATION SELECTOR-16` (U+FE0F).\nIf this character is in a RegExp character class, it will match to either `❇` or `VARIATION SELECTOR-16` rather than `❇\u{fe0f}`.\nThis rule reports the regular expressions which include multiple code point characters in character class syntax.\n\nSource: https://eslint.org/docs/latest/rules/no-misleading-character-class\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\n/^[A\u{301}]$/u;\n```\n\n```js,expect_diagnostic\n/^[❇\u{fe0f}]$/u;\n```\n\n```js,expect_diagnostic\n/^[👶🏻]$/u;\n```\n\n```js,expect_diagnostic\n/^[🇯🇵]$/u;\n```\n\n```js,expect_diagnostic\n/^[👨\u{200d}👩\u{200d}👦]$/u;\n```\n\n```js,expect_diagnostic\n/^[👍]$/; // surrogate pair without u flag\n```\n\n## Valid\n\n```js\n/^[abc]$/;\n/^[👍]$/u;\n/^[\\q{👶🏻}]$/v;\n```\n" ;
const NO_MISLEADING_INSTANTIATOR : & str = "Enforce proper usage of `new` and `constructor`.\n\nIn JavaScript, classes utilize the `constructor` method to initialize a new instance. On the other hand, TypeScript interfaces can describe a class type with a `new()` method signature, though this pattern is not commonly seen in real-world code. Developers, especially those new to JavaScript or TypeScript, might occasionally confuse the use of `constructor` with `new`.\nThis rule triggers warnings in the following scenarios:\n- When a class has a method named `new`.\n- When an interface defines a method named `constructor` or `new` that returns the interface type.\n- When a type alias has a `constructor` method.\n\nYou should not use this rule if you intentionally want a class with a `new` method, and you're confident nobody working in your code will mistake it with an `constructor`.\n\nSource: https://typescript-eslint.io/rules/no-misused-new/\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\ninterface I {\nnew (): I;\nconstructor(): void;\n}\n```\n\n```ts,expect_diagnostic\nclass C {\nnew(): C;\n}\n```\n\n### Valid\n\n```ts\ndeclare class C {\nconstructor();\n}\n\ninterface I {\nnew (): C;\n}\n```" ;
const NO_MISREFACTORED_SHORTHAND_ASSIGN : & str = "Disallow shorthand assign when variable appears on both sides.\n\nThis rule helps to avoid potential bugs related to incorrect assignments or unintended\nside effects that may occur during refactoring.\n\nSource: https://rust-lang.github.io/rust-clippy/master/#/misrefactored_assign_op\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\na += a + b\n```\n\n```js,expect_diagnostic\na -= a - b\n```\n\n```js,expect_diagnostic\na *= a * b\n```\n\n## Valid\n\n```js\na += b\n```\n\n```js\na = a + b\n```\n\n```js\na = a - b\n```" ;
const NO_MULTIPLE_SPACES_IN_REGULAR_EXPRESSION_LITERALS : & str = "Disallow unclear usage of consecutive space characters in regular expression literals\n\nSource: https://eslint.org/docs/latest/rules/no-regex-spaces/\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\n/   /\n```\n\n```js,expect_diagnostic\n/foo  */\n```\n\n```js,expect_diagnostic\n/foo  {2,}bar   {3,5}baz/\n```\n\n```js,expect_diagnostic\n/foo [ba]r  b(a|z)/\n```\n\n### Valid\n\n```js\n/foo {2}bar/\n```\n\n```js\n/ foo bar baz /\n```\n\n```js\n/foo bar\tbaz/\n```" ;
const NO_NAMESPACE : & str = "Disallow the use of TypeScript's `namespace`s.\n\nNamespaces are an old way to organize your code in TypeScript.\nThey are not recommended anymore and should be replaced by ES6 modules\n(the `import`/`export` syntax).\n\nSource: https://typescript-eslint.io/rules/no-namespace\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\nmodule foo {}\n```\n\n```ts,expect_diagnostic\ndeclare module foo {}\n```\n\n```ts,expect_diagnostic\nnamespace foo {}\n```\n\n```ts,expect_diagnostic\ndeclare namespace foo {}\n```\n\n## Valid\n\n```ts\nimport foo from 'foo';\nexport { bar };\n```\n\n```ts\ndeclare global {}\n```\n\n```ts\ndeclare module 'foo' {}\n```\n" ;
const NO_NEGATION_ELSE : & str = "Disallow negation in the condition of an `if` statement if it has an `else` clause.\n\nSource: https://eslint.org/docs/latest/rules/no-negated-condition/\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nif (!cond) { f();} else { g();}\n```\n\n```js,expect_diagnostic\n!cond ? 0 : 1\n```\n\n### Valid\n\n```js\nif (!cond) { f(); }\n```\n\n```js\ncond ? 1 : 0\n```\n\n```js\nif (!cond) { f(); }\n```\n\n```js\nif (!!val) { f(); } else { g(); }\n```" ;
const NO_NEW_SYMBOL : & str = "Disallow `new` operators with the `Symbol` object.\n\n`Symbol` cannot be instantiated. This results in throwing a `TypeError`.\n\nSource: https://eslint.org/docs/latest/rules/no-new-symbol\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nvar foo = new Symbol('foo');\n```\n\n### Valid\n\n```js\nvar bar = Symbol('bar');\nfunction baz() {\nfunction Symbol() { }\nnew Symbol();\n}\n```" ;
const NO_NON_NULL_ASSERTION : & str = "Disallow non-null assertions using the `!` postfix operator.\n\nTypeScript's `!` non-null assertion operator asserts to the type system that an expression is non-nullable, as\nin not `null` or `undefined`. Using assertions to tell the type system new information is often a sign that\ncode is not fully type-safe. It's generally better to structure program logic so that TypeScript understands\nwhen values may be nullable.\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\ninterface Example {\nproperty?: string;\n}\ndeclare const example: Example;\nconst includesBaz = foo.property!.includes('baz');\n```\n```ts,expect_diagnostic\n(b!! as number) = \"test\";\n```\n\n### Valid\n\n```ts\ninterface Example {\nproperty?: string;\n}\n\ndeclare const example: Example;\nconst includesBaz = foo.property?.includes('baz') ?? false;\n```\n" ;
const NO_NONINTERACTIVE_ELEMENT_TO_INTERACTIVE_ROLE : & str = "Enforce that interactive ARIA roles are not assigned to non-interactive HTML elements.\n\nNon-interactive HTML elements indicate _content_ and _containers_ in the user interface.\nNon-interactive elements include `<main>`, `<area>`, `<h1>` (,`<h2>`, etc), `<img>`, `<li>`, `<ul>` and `<ol>`.\n\nInteractive HTML elements indicate _controls_ in the user interface.\nInteractive elements include `<a href>`, `<button>`, `<input>`, `<select>`, `<textarea>`.\n\n[WAI-ARIA roles](https://www.w3.org/TR/wai-aria-1.1/#usage_intro) should not be used to convert a non-interactive element to an interactive element.\nInteractive ARIA roles include `button`, `link`, `checkbox`, `menuitem`, `menuitemcheckbox`, `menuitemradio`, `option`, `radio`, `searchbox`, `switch` and `textbox`.\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<h1 role=\"button\">Some text</h1>\n```\n\n### Valid\n\n\n```jsx\n<span role=\"button\">Some text</span>\n```\n\n## Accessibility guidelines\n\n- [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)\n\n### Resources\n\n- [WAI-ARIA roles](https://www.w3.org/TR/wai-aria-1.1/#usage_intro)\n- [WAI-ARIA Authoring Practices Guide - Design Patterns and Widgets](https://www.w3.org/TR/wai-aria-practices-1.1/#aria_ex)\n- [Fundamental Keyboard Navigation Conventions](https://www.w3.org/TR/wai-aria-practices-1.1/#kbd_generalnav)\n- [Mozilla Developer Network - ARIA Techniques](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/ARIA_Techniques/Using_the_button_role#Keyboard_and_focus)\n" ;
const NO_NONINTERACTIVE_TABINDEX : & str = "Enforce that `tabIndex` is not assigned to non-interactive HTML elements.\n\nWhen using the tab key to navigate a webpage, limit it to interactive elements.\nYou don't need to add tabindex to items in an unordered list as assistive technology can navigate through the HTML.\nKeep the tab ring small, which is the order of elements when tabbing, for a more efficient and accessible browsing experience.\n\nESLint (eslint-plugin-jsx-a11y) Equivalent: [no-noninteractive-tabindex](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-noninteractive-tabindex.md)\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<div tabIndex=\"0\" />\n```\n\n```jsx,expect_diagnostic\n<div role=\"article\" tabIndex=\"0\" />\n```\n\n```jsx,expect_diagnostic\n<article tabIndex=\"0\" />\n```\n\n## Valid\n\n```jsx\n<div />\n```\n\n```jsx\n<MyButton tabIndex={0} />\n```\n\n```jsx\n<article tabIndex=\"-1\" />\n```\n" ;
const NO_NONOCTAL_DECIMAL_ESCAPE : & str = "Disallow `\\8` and `\\9` escape sequences in string literals.\n\nSince ECMAScript 2021, the escape sequences \\8 and \\9 have been defined as non-octal decimal escape sequences.\nHowever, most JavaScript engines consider them to be \"useless\" escapes. For example:\n\n```js,ignore\n\"\\8\" === \"8\"; // true\n\"\\9\" === \"9\"; // true\n```\n\nAlthough this syntax is deprecated, it is still supported for compatibility reasons.\nIf the ECMAScript host is not a web browser, this syntax is optional.\nHowever, web browsers are still required to support it, but only in non-strict mode.\nRegardless of your targeted environment, it is recommended to avoid using these escape sequences in new code.\n\nSource: https://eslint.org/docs/latest/rules/no-nonoctal-decimal-escape\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nconst x = \"\\8\";\n```\n\n```js,expect_diagnostic\nconst x = \"Don't use \\8 escape.\";\n```\n\n```js,expect_diagnostic\nconst x = \"Don't use \\9 escape.\";\n```\n\n## Valid\n\n```js\nconst x = \"8\";\n```\n\n```js\nconst x = \"Don't use \\\\8 and \\\\9 escapes.\";\n```\n" ;
const NO_PARAMETER_ASSIGN : & str = "Disallow reassigning `function` parameters.\n\nAssignment to a `function` parameters can be misleading and confusing,\nas modifying parameters will also mutate the `arguments` object.\nIt is often unintended and indicative of a programmer error.\n\nSource: https://eslint.org/docs/latest/rules/no-param-reassign\n\nIn contrast to the _ESLint_ rule, this rule cannot be configured to report\nassignments to a property of a parameter.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nfunction f(param) {\nparam = 13;\n}\n```\n\n```js,expect_diagnostic\nfunction f(param) {\nparam++;\n}\n```\n\n```js,expect_diagnostic\nfunction f(param) {\nfor (param of arr) {}\n}\n```\n\n```ts,expect_diagnostic\nclass C {\nconstructor(readonly prop: number) {\nprop++\n}\n}\n```\n\n## Valid\n\n```js\nfunction f(param) {\nlet local = param;\n}\n```\n" ;
const NO_PARAMETER_PROPERTIES : & str = "Disallow the use of parameter properties in class constructors.\n\nTypeScript includes a \"parameter properties\" shorthand for declaring a class constructor parameter and class property in one location.\nParameter properties can confuse those new to TypeScript as they are less explicit than other ways of declaring and initializing class members.\nMoreover, private class properties, starting with `#`, cannot be turned into \"parameter properties\".\nThis questions the future of this feature.\n\nSource: https://typescript-eslint.io/rules/parameter-properties\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\nclass A {\nconstructor(readonly name: string) {}\n}\n```\n\n### Valid\n\n```ts\nclass A {\nconstructor(name: string) {}\n}\n```\n" ;
const NO_POSITIVE_TABINDEX : & str = "Prevent the usage of positive integers on `tabIndex` property\n\nAvoid positive `tabIndex` property values to synchronize the flow of the page with keyboard tab order.\n## Accessibility guidelines\n\n[WCAG 2.4.3](https://www.w3.org/WAI/WCAG21/Understanding/focus-order)\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<div tabIndex={1}>foo</div>\n```\n\n```jsx,expect_diagnostic\n<div tabIndex={\"1\"} />\n```\n\n```js,expect_diagnostic\nReact.createElement(\"div\", { tabIndex: 1 })\n```\n\n### Valid\n\n```jsx\n<div tabIndex=\"0\" />\n```\n\n```js\nReact.createElement(\"div\", { tabIndex: -1 })\n```" ;
const NO_PRECISION_LOSS : & str = "Disallow literal numbers that lose precision\n\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nconst x = 9007199254740993\n```\n\n```js,expect_diagnostic\nconst x = 5123000000000000000000000000001\n```\n\n```js,expect_diagnostic\nconst x = 1230000000000000000000000.0\n```\n\n```js,expect_diagnostic\nconst x = .1230000000000000000000000\n```\n\n```js,expect_diagnostic\nconst x = 0X20000000000001\n```\n\n```js,expect_diagnostic\nconst x = 0X2_000000000_0001;\n```\n\n### Valid\n\n```js\nconst x = 12345\nconst x = 123.456\nconst x = 123e34\nconst x = 12300000000000000000000000\nconst x = 0x1FFFFFFFFFFFFF\nconst x = 9007199254740991\nconst x = 9007_1992547409_91\n```\n" ;
const NO_PROTOTYPE_BUILTINS : & str = "Disallow direct use of `Object.prototype` builtins.\n\nECMAScript 5.1 added `Object.create` which allows the creation of an object with a custom prototype.\nThis pattern is often used for objects used as Maps. However, this pattern can lead to errors\nif something else relies on prototype properties/methods.\nMoreover, the methods could be shadowed, this can lead to random bugs and denial of service\nvulnerabilities. For example, calling `hasOwnProperty` directly on parsed JSON like `{\"hasOwnProperty\": 1}` could lead to vulnerabilities.\nTo avoid subtle bugs like this, you should call these methods from `Object.prototype`.\nFor example, `foo.isPrototypeof(bar)` should be replaced with `Object.prototype.isPrototypeof.call(foo, \"bar\")`\nAs for the `hasOwn` method, `foo.hasOwn(\"bar\")` should be replaced with `Object.hasOwn(foo, \"bar\")`.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nvar invalid = foo.hasOwnProperty(\"bar\");\n```\n\n```js,expect_diagnostic\nvar invalid = foo.isPrototypeOf(bar);\n```\n\n```js,expect_diagnostic\nvar invalid = foo.propertyIsEnumerable(\"bar\");\n```\n\n## Valid\n\n```js\nvar valid = Object.hasOwn(foo, \"bar\");\nvar valid = Object.prototype.isPrototypeOf.call(foo, bar);\nvar valid = {}.propertyIsEnumerable.call(foo, \"bar\");\n```\n" ;
const NO_REDECLARE : & str = "Disallow variable, function, class, and type redeclarations in the same scope.\n\nSource: https://typescript-eslint.io/rules/no-redeclare\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nvar a = 3;\nvar a = 10;\n```\n\n```js,expect_diagnostic\nlet a = 3;\nlet a = 10;\n```\n\n```js,expect_diagnostic\nfunction f() {}\nfunction f() {}\n```\n\n```js,expect_diagnostic\nclass C {\nstatic {\nvar c = 3;\nvar c = 10;\n}\n}\n```\n\n```ts,expect_diagnostic\ntype Person = { name: string; }\nclass Person { name: string; }\n```\n\n### Valid\n\n```js\nvar a = 3;\na = 10;\n```\n\n```ts\nclass Foo {\nbar(a: A);\nbar(a: A, b: B);\nbar(a: A, b: B) {}\n}\n```" ;
const NO_REDUNDANT_ALT : & str = "Enforce `img` alt prop does not contain the word \"image\", \"picture\", or \"photo\".\n\nThe rule will first check if `aria-hidden` is truthy to determine whether to enforce the rule. If the image is\nhidden, then the rule will always succeed.\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<img src=\"src\" alt=\"photo content\" />;\n```\n\n```jsx,expect_diagnostic\n<img alt={`picture doing ${things}`} {...this.props} />;\n```\n\n```jsx,expect_diagnostic\n<img alt=\"picture of cool person\" aria-hidden={false} />;\n```\n\n### Valid\n\n```jsx\n<>\n<img src=\"src\" alt=\"alt\" />\n<img src=\"src\" alt={photo} />\n<img src=\"bar\" aria-hidden alt=\"Picture of me taking a photo of an image\" />\n</>\n```\n" ;
const NO_REDUNDANT_ROLES : & str = "Enforce explicit `role` property is not the same as implicit/default role property on an element.\n\nESLint (eslint-plugin-jsx-a11y) Equivalent: [no-redundant-roles](https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-redundant-roles.md)\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<article role='article'></article>\n```\n\n```jsx,expect_diagnostic\n<button role='button'></button>\n```\n\n```jsx,expect_diagnostic\n<h1 role='heading' aria-level='1'>title</h1>\n```\n\n## Valid\n\n```jsx\n<article role='presentation'></article>\n```\n\n```jsx\n<Button role='button'></Button>\n```\n\n```jsx\n<span></span>\n```\n" ;
const NO_REDUNDANT_USE_STRICT : & str = "Prevents from having redundant `\"use strict\"`.\n\n## Examples\n\n### Invalid\n```cjs,expect_diagnostic\n\"use strict\";\nfunction foo() {\n\"use strict\";\n}\n```\n```cjs,expect_diagnostic\n\"use strict\";\n\"use strict\";\n\nfunction foo() {\n\n}\n```\n```cjs,expect_diagnostic\nfunction foo() {\n\"use strict\";\n\"use strict\";\n}\n```\n```cjs,expect_diagnostic\nclass C1 {\ntest() {\n\"use strict\";\n}\n}\n```\n```cjs,expect_diagnostic\nconst C2 = class {\ntest() {\n\"use strict\";\n}\n};\n\n```\n### Valid\n```cjs\nfunction foo() {\n\n}\n```\n```cjs\nfunction foo() {\n\"use strict\";\n}\nfunction bar() {\n\"use strict\";\n}\n```\n" ;
const NO_RENDER_RETURN_VALUE : & str = "Prevent the usage of the return value of `React.render`.\n\n> `ReactDOM.render()` currently returns a reference to the root `ReactComponent` instance. However, using this return value is legacy\nand should be avoided because future versions of React may render components asynchronously in some cases.\nIf you need a reference to the root `ReactComponent` instance, the preferred solution is to attach a [callback ref](https://reactjs.org/docs/refs-and-the-dom.html#callback-refs)\nto the root element.\n\nSource: [ReactDOM documentation](https://facebook.github.io/react/docs/react-dom.html#render)\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\nconst foo = ReactDOM.render(<div />, document.body);\n```\n\n### Valid\n\n```jsx\nReactDOM.render(<div />, document.body);\n```" ;
const NO_RESTRICTED_GLOBALS : & str = "This rule allows you to specify global variable names that you don’t want to use in your application.\n\n> Disallowing usage of specific global variables can be useful if you want to allow a set of\nglobal variables by enabling an environment, but still want to disallow some of those.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nconsole.log(event)\n```\n\n### Valid\n```js\nfunction f(event) {\nconsole.log(event)\n}\n```\n## Options\n\nUse the options to specify additional globals that you want to restrict in your\nsource code.\n\n```json\n{\n\"//\": \"...\",\n\"options\": {\n\"deniedGlobals\": [\"$\", \"MooTools\"]\n}\n}\n```\n\nIn the example above, the rule will emit a diagnostics if tried to use `$` or `MooTools` without\ncreating a local variable.\n" ;
const NO_SELF_ASSIGN : & str = "Disallow assignments where both sides are exactly the same.\n\nSelf assignments have no effect, so probably those are an error due to incomplete refactoring.\n\nSource: https://eslint.org/docs/latest/rules/no-self-assign\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\na = a;\n```\n\n```js,expect_diagnostic\n[a] = [a];\n```\n\n```js,expect_diagnostic\n({a: b} = {a: b});\n```\n\n```js,expect_diagnostic\na.b = a.b;\n```\n\n```js,expect_diagnostic\na[b] = a[b];\n```\n\n```js,expect_diagnostic\na[b].foo = a[b].foo;\n```\n\n```js,expect_diagnostic\na['b'].foo = a['b'].foo;\n```\n\n## Valid\n\n```js\na &= a;\nvar a = a;\nlet a = a;\nconst a = a;\n[a, b] = [b, a];\n```\n" ;
const NO_SELF_COMPARE : & str = "Disallow comparisons where both sides are exactly the same.\n\n> Comparing a variable against itself is usually an error, either a typo or refactoring error. It is confusing to the reader and may potentially introduce a runtime error.\n\n> The only time you would compare a variable against itself is when you are testing for `NaN`.\nHowever, it is far more appropriate to use `typeof x === 'number' && Number.isNaN(x)` for that use case rather than leaving the reader of the code to determine the intent of self comparison.\n\nSource: [no-self-compare](https://eslint.org/docs/latest/rules/no-self-compare).\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nif (x === x) {}\n```\n\n```js,expect_diagnostic\nif (a.b.c() !== a.b .c()) {}\n```\n" ;
const NO_SETTER_RETURN : & str = "Disallow returning a value from a setter\n\nWhile returning a value from a setter does not produce an error, the returned value is being ignored. Therefore, returning a value from a setter is either unnecessary or a possible error.\n\nOnly returning without a value is allowed, as it’s a control flow statement.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nclass A {\nset foo(x) {\nreturn x;\n}\n}\n```\n\n```js,expect_diagnostic\nconst b = {\nset foo(x) {\nreturn x;\n},\n};\n```\n\n```js,expect_diagnostic\nconst c = {\nset foo(x) {\nif (x) {\nreturn x;\n}\n},\n};\n```\n\n### Valid\n\n```js\n// early-return\nclass A {\nset foo(x) {\nif (x) {\nreturn;\n}\n}\n}\n```\n\n```js\n// not a setter\nclass B {\nset(x) {\nreturn x;\n}\n}\n```" ;
const NO_SHADOW_RESTRICTED_NAMES : & str = "Disallow identifiers from shadowing restricted names.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nfunction NaN() {}\n```\n\n```js,expect_diagnostic\nlet Set;\n```\n\n```js,expect_diagnostic\ntry {\t} catch(Object) {}\n```\n\n```js,expect_diagnostic\nfunction Array() {}\n```\n\n```js,expect_diagnostic\nfunction test(JSON) {console.log(JSON)}\n```" ;
const NO_SHOUTY_CONSTANTS : & str = "Disallow the use of constants which its value is the upper-case version of its name.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nconst FOO = \"FOO\";\nconsole.log(FOO);\n```\n\n### Valid\n\n```js\nlet FOO = \"FOO\";\nconsole.log(FOO);\n```\n\n```js\nexport const FOO = \"FOO\";\nconsole.log(FOO);\n```\n\n```js\nfunction f(FOO = \"FOO\") {\nreturn FOO;\n}\n```\n" ;
const NO_SPARSE_ARRAY: &str =
    "Disallow sparse arrays\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\n[1,,2]\n```";
const NO_STATIC_ONLY_CLASS : & str = "This rule reports when a class has no non-static members, such as for a class used exclusively as a static namespace.\n\nUsers who come from a [OOP](https://en.wikipedia.org/wiki/Object-oriented_programming) paradigm may wrap their utility functions in an extra class,\ninstead of putting them at the top level of an ECMAScript module. Doing so is generally unnecessary in JavaScript and TypeScript projects.\n\n- Wrapper classes add extra cognitive complexity to code without adding any structural improvements\n- Whatever would be put on them, such as utility functions, are already organized by virtue of being in a module.\n- As an alternative, you can import * as ... the module to get all of them in a single object.\n- IDEs can't provide as good suggestions for static class or namespace imported properties when you start typing property names\n- It's more difficult to statically analyze code for unused variables, etc. when they're all on the class (see: Finding dead code (and dead types) in TypeScript).\n\nSource: https://typescript-eslint.io/rules/no-extraneous-class\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nclass X {\nstatic foo = false;\nstatic bar() {};\n}\n```\n```js,expect_diagnostic\nclass StaticConstants {\nstatic readonly version = 42;\n\nstatic isProduction() {\nreturn process.env.NODE_ENV === 'production';\n}\n}\n```\n\n## Valid\n\n```js\nconst X = {\nfoo: false,\nbar() {}\n};\n```\n```js\nexport const version = 42;\n\nexport function isProduction() {\nreturn process.env.NODE_ENV === 'production';\n}\n\nfunction logHelloWorld() {\nconsole.log('Hello, world!');\n}\n```\n```js\nclass Empty {}\n```\n\n## Notes on Mutating Variables\nOne case you need to be careful of is exporting mutable variables. While class properties can be mutated externally, exported variables are always constant. This means that importers can only ever read the first value they are assigned and cannot write to the variables.\n\nNeeding to write to an exported variable is very rare and is generally considered a code smell. If you do need it you can accomplish it using getter and setter functions:\n```js,expect_diagnostic\nexport class Utilities {\nstatic mutableCount = 1;\nstatic incrementCount() {\nUtilities.mutableCount += 1;\n}\n}\n```\n\nDo this instead:\n```js\nlet mutableCount = 1;\n\nexport function getMutableCount() {\nreturn mutableField;\n}\n\nexport function incrementCount() {\nmutableField += 1;\n}\n```" ;
const NO_STRING_CASE_MISMATCH : & str = "Disallow comparison of expressions modifying the string case with non-compliant value.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nif (s.toUpperCase() === \"Abc\") {}\n```\n\n```js,expect_diagnostic\nwhile (s.toLowerCase() === \"Abc\") {}\n```\n### Valid\n\n```js\nif (s.toUpperCase() === \"ABC\") {}\nwhile (s.toLowerCase() === \"abc\") {}\nfor (;s.toLocaleLowerCase() === \"ABC\";) {}\nwhile (s.toLocaleUpperCase() === \"abc\") {}\nfor (let s = \"abc\"; s === \"abc\"; s = s.toUpperCase()) {}\n```" ;
const NO_SUPER_WITHOUT_EXTENDS : & str = "Catch a `SyntaxError` when writing calling `super()` on a class that doesn't extends any class\n\n## Examples\n\n```js\nclass A {\n```" ;
const NO_SVG_WITHOUT_TITLE : & str = "Enforces the usage of the `title` element for the `svg` element.\n\nIt is not possible to specify the `alt` attribute for the `svg` as for the `img`.\nTo make svg accessible, the following methods are available:\n- provide the `title` element as the first child to `svg`\n- provide `role=\"img\"` and `aria-label` or `aria-labelledby` to `svg`\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\n<svg>foo</svg>\n```\n\n```js,expect_diagnostic\n<svg>\n<title></title>\n<circle />\n</svg>\n```\n\n```js,expect_diagnostic\n<svg>foo</svg>\n```\n\n```js\n<svg role=\"img\" aria-label=\"\">\n<span id=\"\">Pass</span>\n</svg>\n```\n\n## Valid\n\n```js\n<svg>\n<rect />\n<rect />\n<g>\n<circle />\n<circle />\n<g>\n<title>Pass</title>\n<circle />\n<circle />\n</g>\n</g>\n</svg>\n```\n\n```js\n<svg>\n<title>Pass</title>\n<circle />\n</svg>\n```\n\n```js\n<svg role=\"img\" aria-labelledby=\"title\">\n<span id=\"title\">Pass</span>\n</svg>\n```\n\n```js\n<svg role=\"img\" aria-label=\"title\">\n<span id=\"title\">Pass</span>\n</svg>\n```\n\n## Accessibility guidelines\n[Document Structure – SVG 1.1 (Second Edition)](https://www.w3.org/TR/SVG11/struct.html#DescriptionAndTitleElements)\n[ARIA: img role - Accessibility | MDN](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles/img_role)\n[Accessible SVGs | CSS-Tricks - CSS-Tricks](https://css-tricks.com/accessible-svgs/)\n[Contextually Marking up accessible images and SVGs | scottohara.me](https://www.scottohara.me/blog/2019/05/22/contextual-images-svgs-and-a11y.html)\n" ;
const NO_SWITCH_DECLARATIONS : & str = "Disallow lexical declarations in `switch` clauses.\n\nLexical declarations in `switch` clauses are accessible in the entire `switch`.\nHowever, it only gets initialized when it is assigned, which will only happen if the `switch` clause where it is defined is reached.\n\nTo ensure that the lexical declarations only apply to the current `switch` clause wrap your declarations in a block.\n\nSource: https://eslint.org/docs/latest/rules/no-case-declarations\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nswitch (foo) {\ncase 0:\nconst x = 1;\nbreak;\ncase 2:\nx; // `x` can be used while it is not initialized\nbreak;\n}\n```\n\n```js,expect_diagnostic\nswitch (foo) {\ncase 0:\nfunction f() {}\nbreak;\ncase 2:\nf(); // `f` can be called here\nbreak;\n}\n```\n\n```js,expect_diagnostic\nswitch (foo) {\ncase 0:\nclass A {}\nbreak;\ndefault:\nnew A(); // `A` can be instantiated here\nbreak;\n}\n```\n\n### Valid\n\n```js\nswitch (foo) {\ncase 0: {\nconst x = 1;\nbreak;\n}\ncase 1:\n// `x` is not visible here\nbreak;\n}\n```\n" ;
const NO_THIS_IN_STATIC : & str = "Disallow `this` and `super` in `static` contexts.\n\nIn JavaScript, the `this` keyword in static contexts refers to the class (the constructor) instance,\nnot an instance of the class. This can be confusing for developers coming from other languages where\n`this` typically refers to an instance of the class, not the class itself.\n\nSimilarly, `super` in static contexts refers to the parent class, not an instance of the class.\nThis can lead to unexpected behavior if not properly understood.\n\nThis rule enforces the use of the class name itself to access static methods,\nwhich can make the code clearer and less prone to errors. It helps to prevent\nmisunderstandings and bugs that can arise from the unique behavior of `this` and `super` in static contexts.\n\nSource: https://github.com/mysticatea/eslint-plugin/blob/master/docs/rules/no-this-in-static.md\n\n## Example\n\n### Invalid\n\n```js,expect_diagnostic\nclass A {\nstatic CONSTANT = 0;\n\nstatic foo() {\nthis.CONSTANT;\n}\n}\n```\n\n```js,expect_diagnostic\nclass B extends A {\nstatic bar() {\nsuper.CONSTANT;\n}\n}\n```\n\n### Valid\n\n```js\nclass B extends A {\nstatic ANOTHER_CONSTANT = A.CONSTANT + 1;\n\nstatic foo() {\nA.CONSTANT;\nB.ANOTHER_CONSTANT;\n}\n\nbar() {\nthis.property;\n}\n}\n```\n\n```js\nclass A {\nstatic foo() {\ndoSomething()\n}\n\nbar() {\nA.foo()\n}\n}\n```\n" ;
const NO_UNDECLARED_VARIABLES : & str = "Prevents the usage of variables that haven't been declared inside the document.\n\nIf you need to allow-list some global bindings, you can use the [`javascript.globals`](/reference/configuration/#javascriptglobals) configuration.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nfoobar;\n```\n\n```js,expect_diagnostic\n// throw diagnostic for JavaScript files\nPromiseLike;\n```\n### Valid\n\n```ts\ntype B<T> = PromiseLike<T>\n```" ;
const NO_UNNECESSARY_CONTINUE : & str = "Avoid using unnecessary `continue`.\n\n## Examples\n\n### Invalid\n```js,expect_diagnostic\nloop: for (let i = 0; i < 5; i++) {\ncontinue loop;\n}\n```\n```js,expect_diagnostic\nwhile (i--) {\ncontinue;\n}\n```\n```js,expect_diagnostic\nwhile (1) {\ncontinue;\n}\n```\n```js,expect_diagnostic\nfor (let i = 0; i < 10; i++) {\nif (i > 5) {\nconsole.log(\"foo\");\ncontinue;\n} else if (i >= 5 && i < 8) {\nconsole.log(\"test\");\n} else {\nconsole.log(\"test\");\n}\n}\n```\n```js,expect_diagnostic\nfor (let i = 0; i < 9; i++) {\ncontinue;\n}\n```\n\n```js, expect_diagnostic\ntest2: do {\ncontinue test2;\n} while (true);\n```\n\n### Valid\n```js\nwhile (i) {\nif (i > 5) {\ncontinue;\n}\nconsole.log(i);\ni--;\n}\n\nloop: while (1) {\nforLoop: for (let i = 0; i < 5; i++) {\nif (someCondition) {\ncontinue loop;\n}\n}\n}\n```" ;
const NO_UNREACHABLE : & str = "Disallow unreachable code\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nfunction example() {\nreturn;\nneverCalled();\n}\n```\n\n```js,expect_diagnostic\nfunction example() {\nfor(let i = 0; i < 10; ++i) {\nbreak;\n}\n}\n```\n\n```js,expect_diagnostic\nfunction example() {\nfor(const key in value) {\ncontinue;\nneverCalled();\n}\n}\n```" ;
const NO_UNREACHABLE_SUPER : & str = "Ensures the `super()` constructor is called exactly once on every code  path in a class constructor before `this` is accessed if the class has a superclass\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nclass A extends B {\nconstructor() {}\n}\n```\n\n```js,expect_diagnostic\nclass A extends B {\nconstructor(value) {\nthis.prop = value;\nsuper();\n}\n}\n```\n\n```js,expect_diagnostic\nclass A extends B {\nconstructor(cond) {\nif(cond) {\nsuper();\n}\n}\n}\n```\n\n### Valid\n\n```js\nexport default class A extends B {\nconstructor() {\nsuper();\n}\n}\n```\n\n```js\nexport class A {\nconstructor() {}\n}\n```\n" ;
const NO_UNSAFE_DECLARATION_MERGING : & str = "Disallow unsafe declaration merging between interfaces and classes.\n\n_TypeScript_'s [declaration merging](https://www.typescriptlang.org/docs/handbook/declaration-merging.html) supports merging separate declarations with the same name.\n\n_Declaration merging_ between classes and interfaces is unsafe.\nThe _TypeScript Compiler_ doesn't check whether properties defined in the interface are initialized in the class.\nThis can cause lead to _TypeScript_ not detecting code that will cause runtime errors.\n\nSource: https://typescript-eslint.io/rules/no-unsafe-declaration-merging/\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\ninterface Foo {\nf(): void\n}\n\nclass Foo {}\n\nconst foo = new Foo();\nfoo.f(); // Runtime Error: Cannot read properties of undefined.\n```\n\n## Valid\n\n```ts\ninterface Foo {}\nclass Bar implements Foo {}\n```\n\n```ts\nnamespace Baz {}\nnamespace Baz {}\nenum Baz {}\n```" ;
const NO_UNSAFE_FINALLY : & str = "Disallow control flow statements in finally blocks.\n\nJavaScript suspends the control flow statements of `try` and `catch` blocks until\nthe execution of finally block finishes. So, when `return`, `throw`, `break` or `continue`\nis used in finally, control flow statements inside `try` and `catch` are overwritten,\nwhich is considered as unexpected behavior.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\n(() => {\ntry {\nreturn 1; // 1 is returned but suspended until finally block ends\n} catch(err) {\nreturn 2;\n} finally {\nreturn 3; // 3 is returned before 1, which we did not expect\n}\n})();\n```\n\n```js,expect_diagnostic\n(() => {\ntry {\nthrow new Error(\"Try\"); // error is thrown but suspended until finally block ends\n} finally {\nreturn 3; // 3 is returned before the error is thrown, which we did not expect\n}\n})();\n```\n\n```js,expect_diagnostic\n(() => {\ntry {\nthrow new Error(\"Try\")\n} catch(err) {\nthrow err; // The error thrown from try block is caught and re-thrown\n} finally {\nthrow new Error(\"Finally\"); // Finally(...) is thrown, which we did not expect\n}\n})();\n```\n\n```js,expect_diagnostic\n(() => {\nlabel: try {\nreturn 0; // 0 is returned but suspended until finally block ends\n} finally {\nbreak label; // It breaks out the try-finally block, before 0 is returned.\n}\nreturn 1;\n})();\n```\n\n```js,expect_diagnostic\nfunction a() {\nswitch (condition) {\ncase 'a': {\ntry {\nconsole.log('a');\nreturn;\n} finally {\nbreak;\n}\n}\ncase 'b': {\nconsole.log('b');\n}\n}\n}\n```\n\n### Valid\n\n```js\nlet foo = function() {\ntry {\nreturn 1;\n} catch(err) {\nreturn 2;\n} finally {\nconsole.log(\"hola!\");\n}\n};\n```\n\n```js\nlet foo = function() {\ntry {\nreturn 1;\n} catch(err) {\nreturn 2;\n} finally {\nlet a = function() {\nreturn \"hola!\";\n}\n}\n};\n```\n\n```js\nlet foo = function(a) {\ntry {\nreturn 1;\n} catch(err) {\nreturn 2;\n} finally {\nswitch(a) {\ncase 1: {\nconsole.log(\"hola!\")\nbreak;\n}\n}\n}\n};\n```\n" ;
const NO_UNSAFE_NEGATION : & str = "Disallow using unsafe negation.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\n!1 in [1,2];\n```\n\n```js,expect_diagnostic\n/**test*/!/** test*/1 instanceof [1,2];\n```\n\n### Valid\n```js\n-1 in [1,2];\n~1 in [1,2];\ntypeof 1 in [1,2];\nvoid 1 in [1,2];\ndelete 1 in [1,2];\n+1 instanceof [1,2];\n```" ;
const NO_UNSAFE_OPTIONAL_CHAINING : & str = "Disallow the use of optional chaining in contexts where the undefined value is not allowed.\n\nThe optional chaining (?.) expression can short-circuit with a return value of undefined.\nTherefore, treating an evaluated optional chaining expression as a function, object, number, etc., can cause TypeError or unexpected results.\nAlso, parentheses limit the scope of short-circuiting in chains.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\n1 in obj?.foo;\n```\n\n```cjs,expect_diagnostic\nwith (obj?.foo);\n```\n\n```js,expect_diagnostic\nfor (bar of obj?.foo);\n```\n\n```js,expect_diagnostic\nbar instanceof obj?.foo;\n```\n\n```js,expect_diagnostic\nconst { bar } = obj?.foo;\n```\n\n```js,expect_diagnostic\n(obj?.foo)();\n```\n\n```js,expect_diagnostic\n(baz?.bar).foo;\n```\n\n## Valid\n\n```js\n(obj?.foo)?.();\nobj?.foo();\n(obj?.foo ?? bar)();\nobj?.foo.bar;\nobj.foo?.bar;\nfoo?.()?.bar;\n```\n" ;
const NO_UNUSED_IMPORTS : & str = "Disallow unused imports.\n\nUnused imports might be the result of an incomplete refactoring.\nThe code fix can remove comments associated with an `import`.\nSee the last invalid example.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nimport A from 'mod';\n```\n\n```js,expect_diagnostic\nimport * as A from 'mod';\n```\n\n```ts,expect_diagnostic\nimport { type A, B } from 'mod';\n\nexport { B }\n```\n\n```js,expect_diagnostic\n// Header comment\nimport /*inner comment */ A from 'mod'; // Associated comment\n\n// Another header comment\nimport {\n// A's header comment\ntype A, // A's comment\n// B's header comment\nB,\n} from 'mod';\n\nexport { B }\n```\n\n## Valid\n\n```ts\nimport { A, type B } from 'mod';\n\nfunction f(arg: B): A {\nreturn new A(arg);\n}\n```" ;
const NO_UNUSED_LABELS : & str = "Disallow unused labels.\n\nLabels that are declared and never used are most likely an error due to incomplete refactoring.\n\nSource: https://eslint.org/docs/latest/rules/no-unused-labels\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nLOOP: for (const x of xs) {\nif (x > 0) {\nbreak;\n}\nf(x);\n}\n```\n\n### Valid\n\n```js\nLOOP: for (const x of xs) {\nif (x > 0) {\nbreak LOOP;\n}\nf(x);\n}\n```\n\n```js\nfunction nonNegative(n) {\nDEV: assert(n >= 0);\nreturn n;\n}\n```" ;
const NO_UNUSED_PRIVATE_CLASS_MEMBERS : & str = "Disallow unused private class members\n\nPrivate class members that are declared and not used anywhere in the code are most likely an error due to incomplete refactoring.\nSuch class members take up space in the code and can lead to confusion by readers.\n\nSource: https://eslint.org/docs/latest/rules/no-unused-private-class-members/\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nclass OnlyWrite {\n#usedOnlyInWrite = 5;\n\nmethod() {\nthis.#usedOnlyInWrite = 212;\n}\n}\n```\n\n```ts,expect_diagnostic\nclass TsBioo {\nprivate unusedProperty = 5;\n}\n```\n\n```ts,expect_diagnostic\nclass TsBioo {\nprivate unusedMethod() {}\n}\n```\n\n## Valid\n\n```js\nclass UsedMember {\n#usedMember = 42;\n\nmethod() {\nreturn this.#usedMember;\n}\n}\n```\n" ;
const NO_UNUSED_TEMPLATE_LITERAL : & str = "Disallow template literals if interpolation and special-character handling are not needed\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nconst foo = `bar`\n```\n\n```js,expect_diagnostic\nconst foo = `bar `\n```\n\n### Valid\n\n```js\nconst foo = `bar\nhas newline`;\n```\n\n```js\nconst foo = `\"bar\"`\n```\n\n```js\nconst foo = `'bar'`\n```" ;
const NO_UNUSED_VARIABLES : & str = "Disallow unused variables.\n\nThere are two exceptions to this rule:\n1. variables that starts with underscore, ex: `let _something;`\n2. the `React` variable;\n\nThe pattern of having an underscore as prefix of a name of variable is a very diffuse\npattern among programmers, and Biome decided to follow it.\n\nImporting the `React` variable was a mandatory pattern until some time ago:\n\nFor the time being this rule will ignore it, but this **might change in the future releases**.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nlet a = 4;\na++;\n```\n\n```js,expect_diagnostic\nfunction foo() {}\n```\n\n```js,expect_diagnostic\nexport function foo(myVar) {\nconsole.log('foo');\n}\n```\n\n```js,expect_diagnostic\nfunction foo() {\nfoo();\n}\n```\n\n```js,expect_diagnostic\nconst foo = () => {\nfoo();\n};\n```\n\n```ts,expect_diagnostic\nexport function f<T>() {}\n```\n\n# Valid\n\n```js\nfunction foo(b) {\nconsole.log(b)\n};\nfoo();\n```\n\n```js\nexport function foo(_unused) {}\n```\n\n```jsx\nimport React from 'react';\nfunction foo() {\nreturn <div />;\n};\nfoo();\n```\n\n```ts\nfunction used_overloaded(): number;\nfunction used_overloaded(s: string): string;\nfunction used_overloaded(s?: string) {\nreturn s;\n}\nused_overloaded();\n```" ;
const NO_USELESS_CATCH : & str = "Disallow unnecessary `catch` clauses.\n\nA `catch` clause that only rethrows the original error is redundant,\nand has no effect on the runtime behavior of the program.\nThese redundant clauses can be a source of confusion and code bloat,\nso it’s better to disallow these unnecessary `catch` clauses.\n\nSource: https://eslint.org/docs/latest/rules/no-useless-catch\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\ntry {\ndoSomething();\n} catch(e) {\nthrow e;\n}\n```\n```js,expect_diagnostic\ntry {\ndoSomething();\n} catch(e) {\nthrow e;\n} finally {\ndoCleanUp();\n}\n```\n## Valid\n\n```js\ntry {\ndoSomething();\n} catch(e) {\ndoSomethingWhenCatch();\nthrow e;\n}\n```\n\n```js\ntry {\ndoSomething();\n} catch(e) {\nhandleError(e);\n}\n```\n" ;
const NO_USELESS_CONSTRUCTOR : & str = "Disallow unnecessary constructors.\n\n_ES2015_ provides a default class constructor if one is not specified.\nAs such, providing an empty constructor or one that delegates into its parent is unnecessary.\n\nThe rule ignores:\n\n- decorated classes;\n- constructors with at least one [parameter property](https://www.typescriptlang.org/docs/handbook/classes.html#parameter-properties);\n- `private` and `protected` constructors.\n\nSource: https://typescript-eslint.io/rules/no-useless-constructor\n\n## Caveat\n\nThis rule reports on constructors whose sole purpose is to make a parent constructor public.\nSee the last invalid example.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nclass A {\nconstructor (a) {}\n}\n```\n\n```ts,expect_diagnostic\nclass B extends A {\nconstructor (a) {\nsuper(a);\n}\n}\n```\n\n```js,expect_diagnostic\nclass C {\n/**\n* Documented constructor.\n*/\nconstructor () {}\n}\n```\n\n```js,expect_diagnostic\nclass A {\nprotected constructor() {\nthis.prop = 1;\n}\n}\n\nclass B extends A {\n// Make the parent constructor public.\nconstructor () {\nsuper();\n}\n}\n```\n\n## Valid\n\n```js\nclass A {\nconstructor (prop) {\nthis.prop = prop;\n}\n}\n```\n\n```js\nclass B extends A {\nconstructor () {\nsuper(5);\n}\n}\n```\n\n```ts\nclass C {\n// Empty constructor with parameter properties are allowed.\nconstructor (private prop: number) {}\n}\n```\n\n```ts\n@Decorator\nclass C {\nconstructor (prop: number) {}\n}\n```" ;
const NO_USELESS_ELSE : & str = "Disallow `else` block when the `if` block breaks early.\n\nIf an `if` block breaks early using a breaking statement (`return`, `break`, `continue`, or `throw`),\nthen the `else` block becomes useless.\nIts contents can be placed outside of the block.\n\nIf an `if` block breaks early using a breaking statement (`return`, `break`, `continue`, or `throw`),\nthen the `else` block becomes unnecessary.\nThis is because the content of the `else` block will never be executed in conjunction with the `if` block,\nas the breaking statement ensures the control flow exits the `if` block immediately.\nTherefore, the `else` block is redundant, and its content can be placed outside of the block,\nreducing the indentation level by one.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nwhile (x > 0) {\nif (f(x)) {\nbreak;\n} else {\nx++\n}\n}\n```\n\n```js,expect_diagnostic\nfunction f(x) {\nif (x < 0) {\nreturn 0;\n} else {\nreturn x;\n}\n}\n```\n\n```js,expect_diagnostic\nfunction f(x) {\nif (x < 0) {\nthrow new RangeError();\n} else {\nreturn x;\n}\n}\n```\n\n## Valid\n\n```js\nfunction f(x) {\nif (x < 0) {\nreturn 0;\n}\nreturn x;\n}\n```\n\n```js\nfunction f(x) {\nif (x < 0) {\nconsole.info(\"negative number\");\n} else if (x > 0) {\nreturn x;\n} else {\nconsole.info(\"number 0\");\n}\n}\n```" ;
const NO_USELESS_EMPTY_EXPORT : & str = "Disallow empty exports that don't change anything in a module file.\n\nAn empty `export {}` is sometimes useful to turn a file that would otherwise be a script into a module.\nPer the [TypeScript Handbook Modules page](https://www.typescriptlang.org/docs/handbook/modules.html):\n\n> In TypeScript, just as in ECMAScript 2015,\n> any file containing a top-level import or export is considered a module.\n> Conversely, a file without any top-level import or export declarations is treated as a script\n> whose contents are available in the global scope.\n\nHowever, an `export {}` statement does nothing if there are any other top-level import or export in the file.\n\nSource: https://typescript-eslint.io/rules/no-useless-empty-export/\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nimport { A } from \"module\";\nexport {};\n```\n\n```js,expect_diagnostic\nexport const A = 0;\nexport {};\n```\n\n## Valid\n\n```js\nexport {};\n```\n" ;
const NO_USELESS_FRAGMENTS : & str = "Disallow unnecessary fragments\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<>\nfoo\n</>\n```\n\n```jsx,expect_diagnostic\n<React.Fragment>\nfoo\n</React.Fragment>\n```\n\n```jsx,expect_diagnostic\n<>\n<>foo</>\n<SomeComponent />\n</>\n```\n\n```jsx,expect_diagnostic\n<></>\n```" ;
const NO_USELESS_LABEL : & str = "Disallow unnecessary labels.\n\nIf a loop contains no nested loops or switches, labeling the loop is unnecessary.\n\nSource: https://eslint.org/docs/latest/rules/no-extra-label\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nloop: while(a) {\nbreak loop;\n}\n```\n\n### Valid\n\n```js\nouter: while(a) {\nwhile(b) {\nbreak outer;\n}\n}\n```\n" ;
const NO_USELESS_LONE_BLOCK_STATEMENTS : & str = "Disallow unnecessary nested block statements.\n\n> In JavaScript, prior to ES6, standalone code blocks delimited by curly braces do not create a new scope and have no use.\n> In ES6, code blocks may create a new scope if a block-level binding (let and const), a class declaration or a function declaration (in strict mode) are present. A block is not considered redundant in these cases.\n\nSource: https://eslint.org/docs/latest/rules/no-lone-blocks\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\n{}\n```\n\n```js,expect_diagnostic\nif (foo) {\nbar();\n{\nbaz();\n}\n}\n```\n\n## Valid\n\n```js\nwhile (foo) {\nbar();\n}\n```\n" ;
const NO_USELESS_RENAME : & str = "Disallow renaming import, export, and destructured assignments to the same name.\n\nES2015 allows for the renaming of references in import and export statements as well as destructuring assignments.\nThis gives programmers a concise syntax for performing these operations while renaming these references:\n\n```js\nimport { foo as bar } from \"baz\";\nexport { foo as bar };\nlet { foo: bar } = baz;\n```\n\nWith this syntax, it is possible to rename a reference to the same name.\nThis is a completely redundant operation, as this is the same as not renaming at all.\n\nSource: https://eslint.org/docs/latest/rules/no-useless-rename\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nimport { foo as foo } from \"bar\";\n```\n\n```js,expect_diagnostic\nexport { foo as foo };\n```\n\n```js,expect_diagnostic\nlet { foo: foo } = bar;\n```\n\n### Valid\n\n```js\nimport { foo as bar } from \"baz\";\n```\n\n```js\nexport { foo as bar };\n```\n\n```js\nlet { foo: bar } = baz;\n```\n" ;
const NO_USELESS_SWITCH_CASE : & str = "Disallow useless `case` in `switch` statements.\n\nA `switch` statement can optionally have a `default` clause.\n\nThe `default` clause will be still executed only if there is no match in the `case` clauses.\nAn empty `case` clause that precedes the `default` clause is thus useless.\n\nSource: https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/no-useless-switch-case.md\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nswitch (foo) {\ncase 0:\ndefault:\nbreak;\ncase 1:\nbreak;\n}\n```\n\n```js,expect_diagnostic\nswitch (foo) {\ndefault:\ncase 0:\nbreak;\ncase 1:\nbreak;\n}\n```\n\n### Valid\n\n```js\nswitch (foo) {\ncase 0:\nbreak;\ndefault:\nbreak;\n}\n```\n\n```js\nswitch (foo) {\ncase 0:\nbreak;\n}\n```\n" ;
const NO_USELESS_THIS_ALIAS : & str = "Disallow useless `this` aliasing.\n\nArrow functions inherits `this` from their enclosing scope;\nthis makes `this` aliasing useless in this situation.\n\nCredits: https://typescript-eslint.io/rules/no-this-alias/\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nclass A {\nmethod() {\nconst self = this;\nreturn () => {\nreturn self;\n}\n}\n}\n```\n\n## Valid\n\n```js\nclass A {\nmethod() {\nconst self = this;\nreturn function() {\nthis.g();\nreturn self;\n}\n}\n}\n```\n" ;
const NO_USELESS_TYPE_CONSTRAINT : & str = "Disallow using `any` or `unknown` as type constraint.\n\nGeneric type parameters (`<T>`) in TypeScript may be **constrained** with [`extends`](https://www.typescriptlang.org/docs/handbook/generics.html#generic-constraints).\nA supplied type must then be a subtype of the supplied constraint.\nAll types are subtypes of `any` and `unknown`.\nIt is thus useless to extend from `any` or `unknown`.\n\nSource: https://typescript-eslint.io/rules/no-unnecessary-type-constraint/\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\ninterface FooAny<T extends any> {}\n```\n```ts,expect_diagnostic\ntype BarAny<T extends any> = {};\n```\n```ts,expect_diagnostic\nclass BazAny<T extends any> {\n}\n```\n```ts,expect_diagnostic\nclass BazAny {\nquxAny<U extends any>() {}\n}\n```\n```ts,expect_diagnostic\nconst QuuxAny = <T extends any>() => {};\n```\n```ts,expect_diagnostic\nfunction QuuzAny<T extends any>() {}\n```\n\n```ts,expect_diagnostic\ninterface FooUnknown<T extends unknown> {}\n```\n```ts,expect_diagnostic\ntype BarUnknown<T extends unknown> = {};\n```\n```ts,expect_diagnostic\nclass BazUnknown<T extends unknown> {\n}\n```ts,expect_diagnostic\nclass BazUnknown {\nquxUnknown<U extends unknown>() {}\n}\n```\n```ts,expect_diagnostic\nconst QuuxUnknown = <T extends unknown>() => {};\n```\n```ts,expect_diagnostic\nfunction QuuzUnknown<T extends unknown>() {}\n```\n\n### Valid\n\n```ts\ninterface Foo<T> {}\n\ntype Bar<T> = {};\n```" ;
const NO_VAR : & str = "Disallow the use of `var`\n\nECMAScript 6 allows programmers to create variables with block scope instead of function scope using the let and const keywords.\n\nBlock scope is common in many other programming languages and helps programmers avoid mistakes.\n\nSource: https://eslint.org/docs/latest/rules/no-var\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nvar foo = 1;\n```\n\n### Valid\n\n```js\nconst foo = 1;\nlet bar = 1;\n```" ;
const NO_VOID : & str = "Disallow the use of `void` operators, which is not a familiar operator.\n\n> The `void` operator is often used merely to obtain the undefined primitive value,\n> usually using `void(0)` (which is equivalent to `void 0`). In these cases, the global variable `undefined` can be used.\n\nSource: https://eslint.org/docs/latest/rules/no-void\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nvoid 0;\n```\n" ;
const NO_VOID_ELEMENTS_WITH_CHILDREN : & str = "This rules prevents void elements (AKA self-closing elements) from having children.\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<br>invalid child</br>\n```\n\n```jsx,expect_diagnostic\n<img alt=\"some text\" children={\"some child\"} />\n```\n\n```js,expect_diagnostic\nReact.createElement('img', {}, 'child')\n```" ;
const NO_VOID_TYPE_RETURN : & str = "Disallow returning a value from a function with the return type 'void'\n\n'void' signals the absence of value. The returned value is likely to be ignored by the caller.\nThus, returning a value when the return type of function is 'void', is undoubtedly an error.\n\nOnly returning without a value is allowed, as it’s a control flow statement.\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\nclass A {\nf(): void {\nreturn undefined;\n}\n}\n```\n\n```ts,expect_diagnostic\nconst a = {\nf(): void {\nreturn undefined;\n}\n}\n```\n\n```ts,expect_diagnostic\nfunction f(): void {\nreturn undefined;\n}\n```\n\n```ts,expect_diagnostic\nexport default function(): void {\nreturn undefined;\n}\n```\n\n```ts,expect_diagnostic\nconst g = (): void => {\nreturn undefined;\n};\n```\n\n```ts,expect_diagnostic\nconst h = function(): void {\nreturn undefined;\n};\n```\n\n### Valid\n\n```js\nclass A {\nf() {\nreturn undefined;\n}\n}\n```\n\n```ts\nclass B {\nf(): void {}\n}\n```\n\n```ts\nfunction f(): void {\nreturn;\n}\n```\n" ;
const NO_WITH : & str = "Disallow `with` statements in non-strict contexts.\n\nThe `with` statement is potentially problematic because it adds members of an object to the current\nscope, making it impossible to tell what a variable inside the block actually refers to.\n\n## Examples\n\n### Invalid\n\n```cjs,expect_diagnostic\nfunction f() {\nwith (point) {\nr = Math.sqrt(x * x + y * y); // is r a member of point?\n}\n}\n```" ;
const ORGANIZE_IMPORTS : & str = "Provides a whole-source code action to sort the imports in the file\nusing import groups and natural ordering.\n\n## Examples\n\n```js\nimport React, {\nFC,\nuseEffect,\nuseRef,\nChangeEvent,\nKeyboardEvent,\n} from 'react';\nimport { logger } from '@core/logger';\nimport { reduce, debounce } from 'lodash';\nimport { Message } from '../Message';\nimport { createServer } from '@server/node';\nimport { Alert } from '@ui/Alert';\nimport { repeat, filter, add } from '../utils';\nimport { initializeApp } from '@core/app';\nimport { Popup } from '@ui/Popup';\nimport { createConnection } from '@server/database';\n```" ;
const USE_ALT_TEXT : & str = "Enforce that all elements that require alternative text have meaningful information to relay back to the end user.\n\nThis is a critical component of accessibility for screen reader users in order for them to understand the content's purpose on the page.\nBy default, this rule checks for alternative text on the following elements: `<img>`, `<area>`, `<input type=\"image\">`, and `<object>`.\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<img src=\"image.png\" />\n```\n\n```jsx,expect_diagnostic\n<input type=\"image\" src=\"image.png\" />\n```\n\n### Valid\n\n```jsx\n<img src=\"image.png\" alt=\"image alt\" />\n```\n\n```jsx\n<input type=\"image\" src=\"image.png\" alt=\"alt text\" />\n```\n\n```jsx\n<input type=\"image\" src=\"image.png\" aria-label=\"alt text\" />\n```\n\n```jsx\n<input type=\"image\" src=\"image.png\" aria-labelledby=\"someId\" />\n```\n\n## Accessibility guidelines\n\n- [WCAG 1.1.1](https://www.w3.org/WAI/WCAG21/Understanding/non-text-content.html)\n" ;
const USE_ANCHOR_CONTENT : & str = "Enforce that anchors have content and that the content is accessible to screen readers.\n\nAccessible means the content is not hidden using the `aria-hidden` attribute.\nRefer to the references to learn about why this is important.\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<a />\n```\n\n```jsx,expect_diagnostic\n<a></a>\n```\n\n```jsx,expect_diagnostic\n<a>    </a>\n```\n\n```jsx,expect_diagnostic\n<a aria-hidden>content</a>\n```\n\n```jsx,expect_diagnostic\n<a><span aria-hidden=\"true\">content</span></a>\n```\n\n## Valid\n\n```jsx\n<a>content</a>\n```\n\n```jsx\nfunction html() {\nreturn { __html: \"foo\" }\n}\n<a dangerouslySetInnerHTML={html()} />\n```\n\n```jsx\n<a><TextWrapper aria-hidden={true} />content</a>\n```\n\n```jsx\n<a><div aria-hidden=\"true\"></div>content</a>\n```\n\n## Accessibility guidelines\n\n- [WCAG 2.4.4](https://www.w3.org/WAI/WCAG21/Understanding/link-purpose-in-context)\n- [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)\n" ;
const USE_ARIA_ACTIVEDESCENDANT_WITH_TABINDEX : & str = "Enforce that `tabIndex` is assigned to non-interactive HTML elements with `aria-activedescendant`.\n\n`aria-activedescendant` is used to manage to focus within a [composite widget](https://www.w3.org/TR/wai-aria/#composite).\nThe element with the attribute `aria-activedescendant` retains the active document focus.\n\nIt indicates which of its child elements has a secondary focus by assigning the ID of that\nelement to the value of `aria-activedescendant`. This pattern is used to build a widget\nlike a search typeahead select list. The search input box retains document focus\nso that the user can type in the input. If the down arrow key is pressed and\na search suggestion is highlighted, the ID of the suggestion element will be applied\nas the value of `aria-activedescendant` on the input element.\n\nBecause an element with `aria-activedescendant` must be tabbable,\nit must either have an inherent tabIndex of zero or declare a tabIndex attribute.\n\nSource: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/aria-activedescendant-has-tabindex.md\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<div aria-activedescendant={someID} />\n```\n\n## Valid\n\n```jsx\n<div aria-activedescendant={someID} tabIndex={0} />\n```\n\n```jsx\n<input aria-activedescendant={someID} />\n```\n" ;
const USE_ARIA_PROPS_FOR_ROLE : & str = "Enforce that elements with ARIA roles must have all required ARIA attributes for that role.\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<span role=\"checkbox\"></span>\n```\n\n```jsx,expect_diagnostic\n<span role=\"heading\"></span>\n```\n\n### Valid\n\n```jsx\n<span role=\"checkbox\" aria-checked=\"true\"></span>\n```\n\n```jsx\n<span role=\"heading\" aria-level=\"1\"></span>\n```\n\n\n## Accessibility guidelines\n- [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)\n\n### Resources\n- [ARIA Spec, Roles](https://www.w3.org/TR/wai-aria/#roles)\n- [Chrome Audit Rules, AX_ARIA_03](https://github.com/GoogleChrome/accessibility-developer-tools/wiki/Audit-Rules#ax_aria_03)" ;
const USE_ARROW_FUNCTION : & str = "Use arrow functions over function expressions.\n\nAn arrow function expression is a compact alternative to a regular function expression,\nwith an important distinction:\n`this` is not bound to the arrow function. It inherits `this` from its parent scope.\n\nThis rule proposes turning all function expressions that are not generators (`function*`) and don't use `this` into arrow functions.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nconst z = function() {\nreturn 0;\n}\n```\n\n```js,expect_diagnostic\nconst delegatedFetch = async function(url) {\nreturn await fetch(url);\n}\n```\n\n## Valid\n\n```js\nconst f = function() {\nreturn this.prop;\n}\n```\n\nNamed function expressions are ignored:\n\n```js\nconst z = function z() {\nreturn 0;\n}\n```\n\nFunction expressions that declare the type of `this` are  also ignored:\n\n```ts\nconst z = function(this: A): number {\nreturn 0;\n}\n```" ;
const USE_AS_CONST_ASSERTION : & str = "Enforce the use of `as const` over literal type and type annotation.\n\nIn TypeScript, there are three common ways to specify that a value is of a specific type such as `2` and not a general type such as `number`:\n\n1. `as const`: telling TypeScript to infer the literal type automatically\n2. `as <literal>`: explicitly telling the literal type to TypeScript\n3. type annotation: explicitly telling the literal type to TypeScript when declare variables\n\nThe rule suggests to use `as const` when you're using `as` with a literal type or type annotation, since `as const` is simpler and doesn't require retyping the value.\n\nSource: https://typescript-eslint.io/rules/prefer-as-const/\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\nlet bar: 2 = 2;\n```\n\n```ts,expect_diagnostic\nlet foo = { bar: 'baz' as 'baz' };\n```\n\n## Valid\n\n```ts\nlet foo = 'bar';\nlet foo = 'bar' as const;\nlet foo: 'bar' = 'bar' as const;\nlet bar = 'bar' as string;\nlet foo = { bar: 'baz' };\n```" ;
const USE_AWAIT : & str = "Ensure `async` functions utilize `await`.\n\nThis rule reports `async` functions that lack an `await` expression. As `async`\nfunctions return a promise, the use of `await` is often necessary to capture the\nresolved value and handle the asynchronous operation appropriately. Without `await`,\nthe function operates synchronously and might not leverage the advantages of async\nfunctions.\n\nSource: [require-await](https://eslint.org/docs/latest/rules/require-await)\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nasync function fetchData() {\n// Missing `await` for the promise returned by `fetch`\nreturn fetch('/data');\n}\n```\n\n### Valid\n\n```js\nasync function fetchData() {\nconst response = await fetch('/data');\nconst data = await response.json();\nreturn data;\n}\n\n// This rule does not warn about non-async functions\nfunction processData() {\nreturn compute(data);\n}\n\n// Nor does it warn about empty `async` functions\nasync function noop() { }\n```" ;
const USE_BLOCK_STATEMENTS : & str = "Requires following curly brace conventions.\n\nJavaScript allows the omission of curly braces when a block contains only one statement. However, it is considered by many to be best practice to never omit curly braces around blocks, even when they are optional, because it can lead to bugs and reduces code clarity.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nif (x) x;\n```\n\n```js,expect_diagnostic\nif (x) {\nx;\n} else y;\n```\n\n```js,expect_diagnostic\nif (x) {\nx;\n} else if (y) y;\n```\n\n```js,expect_diagnostic\nfor (;;);\n```\n\n```js,expect_diagnostic\nfor (p in obj);\n```\n\n```js,expect_diagnostic\nfor (x of xs);\n```\n\n```js,expect_diagnostic\ndo;\nwhile (x);\n```\n\n```js,expect_diagnostic\nwhile (x);\n```\n\n```js,expect_diagnostic\nwith (x);\n```" ;
const USE_BUTTON_TYPE : & str = "Enforces the usage of the attribute `type` for the element `button`\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<button>Do something</button>\n```\n\n```jsx,expect_diagnostic\n<button type=\"incorrectType\">Do something</button>\n```\n\n```js,expect_diagnostic\nReact.createElement('button');\n```\n\n## Valid\n\n```jsx\n<>\n<button type=\"button\">Do something</button>\n<button type={buttonType}>Do something</button>\n</>\n```" ;
const USE_COLLAPSED_ELSE_IF : & str = "Enforce using `else if` instead of nested `if` in `else` clauses.\n\nIf an `if` statement is the only statement in the `else` block, it is often clearer to use an `else if` form.\n\nSource: https://eslint.org/docs/latest/rules/no-lonely-if\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nif (condition) {\n// ...\n} else {\nif (anotherCondition) {\n// ...\n}\n}\n```\n\n```js,expect_diagnostic\nif (condition) {\n// ...\n} else {\nif (anotherCondition) {\n// ...\n} else {\n// ...\n}\n}\n```\n\n```js,expect_diagnostic\nif (condition) {\n// ...\n} else {\n// Comment\nif (anotherCondition) {\n// ...\n}\n}\n```\n\n### Valid\n\n```js\nif (condition) {\n// ...\n} else if (anotherCondition) {\n// ...\n}\n```\n\n```js\nif (condition) {\n// ...\n} else if (anotherCondition) {\n// ...\n} else {\n// ...\n}\n```\n\n```js\nif (condition) {\n// ...\n} else {\nif (anotherCondition) {\n// ...\n}\ndoSomething();\n}\n```\n" ;
const USE_CONST : & str = "Require `const` declarations for variables that are never reassigned after declared.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nlet a = 3;\nconsole.log(a);\n```\n\n```js,expect_diagnostic\n// `a` is redefined (not reassigned) on each loop step.\nfor (let a of [1, 2, 3]) {\nconsole.log(a);\n}\n```\n\n```js,expect_diagnostic\n// `a` is redefined (not reassigned) on each loop step.\nfor (let a in [1, 2, 3]) {\nconsole.log(a);\n}\n```\n\n```js,expect_diagnostic\nlet a = 3;\n{\nlet a = 4;\na = 2;\n}\n```\n\n## Valid\n\n```js\nlet a = 2;\na = 3;\nconsole.log(a);\n```\n\n```js\nlet a = 1, b = 2;\nb = 3;\n```" ;
const USE_DEFAULT_PARAMETER_LAST : & str = "Enforce default function parameters and optional function parameters to be last.\n\nDefault and optional parameters that precede a required parameter cannot be omitted at call site.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nfunction f(a = 0, b) {}\n```\n\n```js,expect_diagnostic\nfunction f(a, b = 0, c) {}\n```\n\n```ts,expect_diagnostic\nfunction f(a: number, b?: number, c: number) {}\n```\n\n```ts,expect_diagnostic\nclass Foo {\nconstructor(readonly a = 10, readonly b: number) {}\n}\n```\n\n### Valid\n\n```js\nfunction f(a, b = 0) {}\n```\n\n```ts\nfunction f(a: number, b?: number, c = 0) {}\n```\n\n```ts\nfunction f(a: number, b = 0, c?: number) {}\n```\n" ;
const USE_DEFAULT_SWITCH_CLAUSE_LAST : & str = "Enforce default clauses in switch statements to be last\n\nA switch statement can optionally have a default clause.\n\nIf present, it’s usually the last clause, but it doesn’t need to be. It is also allowed to put the default clause before all case clauses, or anywhere between.\nThe behavior is mostly the same as if it was the last clause.\n\nThe default block will be still executed only if there is no match in the case clauses (including those defined after the default),\nbut there is also the ability to “fall through” from the default clause to the following clause in the list.\nHowever, such flow is not common and it would be confusing to the readers.\n\nEven if there is no \"fall through\" logic, it’s still unexpected to see the default clause before or between the case clauses. By convention, it is expected to be the last clause.\n\nSource: https://eslint.org/docs/latest/rules/default-case-last\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nswitch (foo) {\ndefault:\nbreak;\ncase 0:\nbreak;\n}\n```\n\n```js,expect_diagnostic\nswitch (foo) {\ndefault:\nf();\ncase 0:\nbreak;\n}\n```\n\n```js,expect_diagnostic\nswitch (foo) {\ncase 0:\nbreak;\ndefault:\ncase 1:\nbreak;\n}\n```\n\n### Valid\n\n```js\nswitch (foo) {\ncase 0:\nbreak;\ncase 1:\ndefault:\nbreak;\n}\n```\n\n```js\nswitch (foo) {\ncase 0:\nbreak;\n}\n```\n" ;
const USE_ENUM_INITIALIZERS : & str = "Require that each enum member value be explicitly initialized.\n\n_TypeScript_ enums are a practical way to organize semantically related constant values.\nMembers of enums that don't have explicit values are by default given sequentially increasing numbers.\n\nWhen the value of enum members are important,\nallowing implicit values for enum members can cause bugs if enum declarations are modified over time.\n\nSource: https://typescript-eslint.io/rules/prefer-enum-initializers\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\nenum Version {\nV1,\n}\n```\n\n```ts,expect_diagnostic\nenum Status {\nOpen = 1,\nClose,\n}\n```\n\n```ts,expect_diagnostic\nenum Color {\nRed = \"Red\",\nGreen = \"Green\",\nBlue,\n}\n```\n\n### Valid\n\n```ts\nenum Status {\nOpen = 1,\nClose = 2,\n}\n```\n\n```ts\nenum Color {\nRed = \"Red\",\nGreen = \"Green\",\nBlue = \"Blue\",\n}\n```\n\n```ts\ndeclare enum Weather {\nRainy,\nSunny,\n}\n```" ;
const USE_EXHAUSTIVE_DEPENDENCIES : & str = "Enforce all dependencies are correctly specified in a React hook.\n\nThis rule is a port of the rule [react-hooks/exhaustive-deps](https://legacy.reactjs.org/docs/hooks-rules.html#eslint-plugin), and it's meant to target projects that uses React.\n\nIf your project _doesn't_ use React, **you shouldn't use this rule**.\n\nThe rule will inspect the following **known** hooks:\n\n- `useEffect`\n- `useLayoutEffect`\n- `useInsertionEffect`\n- `useCallback`\n- `useMemo`\n- `useImperativeHandle`\n- `useState`\n- `useReducer`\n- `useRef`\n- `useDebugValue`\n- `useDeferredValue`\n- `useTransition`\n\nIf you want to add more hooks to the rule, check the [#options](options).\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nimport { useEffect } from \"react\";\n\nfunction component() {\nlet a = 1;\nuseEffect(() => {\nconsole.log(a);\n}, []);\n}\n```\n\n```js,expect_diagnostic\nimport { useEffect } from \"react\";\n\nfunction component() {\nlet b = 1;\nuseEffect(() => {\n}, [b]);\n}\n```\n\n```js,expect_diagnostic\nimport { useEffect, useState } from \"react\";\n\nfunction component() {\nconst [name, setName] = useState();\nuseEffect(() => {\nconsole.log(name);\nsetName(\"\");\n}, [name, setName]);\n}\n```\n\n```js,expect_diagnostic\nimport { useEffect } from \"react\";\n\nfunction component() {\nlet a = 1;\nconst b = a + 1;\nuseEffect(() => {\nconsole.log(b);\n}, []);\n}\n```\n\n## Valid\n\n```js\nimport { useEffect } from \"react\";\n\nfunction component() {\nlet a = 1;\nuseEffect(() => {\nconsole.log(a);\n}, [a]);\n}\n```\n\n```js\nimport { useEffect } from \"react\";\n\nfunction component() {\nconst a = 1;\nuseEffect(() => {\nconsole.log(a);\n});\n}\n```\n\n```js\nimport { useEffect, useState } from \"react\";\n\nfunction component() {\nconst [name, setName] = useState();\nuseEffect(() => {\nconsole.log(name);\nsetName(\"\");\n}, [name]);\n}\n```\n\n```js\nimport { useEffect } from \"react\";\nlet outer = false;\nfunction component() {\nuseEffect(() => {\nouter = true;\n}, []);\n}\n```\n\n## Options\n\nAllows to specify custom hooks - from libraries or internal projects - that can be considered stable.\n\n```json\n{\n\"//\": \"...\",\n\"options\": {\n\"hooks\": [\n{ \"name\": \"useLocation\", \"closureIndex\": 0, \"dependenciesIndex\": 1},\n{ \"name\": \"useQuery\", \"closureIndex\": 1, \"dependenciesIndex\": 0}\n]\n}\n}\n```\n\nGiven the previous example, your hooks be used like this:\n\n```js\nfunction Foo() {\nconst location = useLocation(() => {}, []);\nconst query = useQuery([], () => {});\n}\n```\n" ;
const USE_EXPONENTIATION_OPERATOR : & str = "Disallow the use of `Math.pow` in favor of the `**` operator.\n\nIntroduced in ES2016, the infix exponentiation operator `**` is an alternative for the standard `Math.pow` function.\nInfix notation is considered to be more readable and thus more preferable than the function notation.\n\nSource: https://eslint.org/docs/latest/rules/prefer-exponentiation-operator\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nconst foo = Math.pow(2, 8);\n```\n\n```js,expect_diagnostic\nconst bar = Math.pow(a, b);\n```\n\n```js,expect_diagnostic\nlet baz = Math.pow(a + b, c + d);\n```\n\n```js,expect_diagnostic\nlet quux = Math.pow(-1, n);\n```\n\n### Valid\n\n```js\nconst foo = 2 ** 8;\n\nconst bar = a ** b;\n\nlet baz = (a + b) ** (c + d);\n\nlet quux = (-1) ** n;\n```\n" ;
const USE_EXPORT_TYPE : & str = "Promotes the use of `export type` for types.\n\n_TypeScript_ allows specifying a `type` marker on an `export` to indicate that the `export` doesn't exist at runtime.\nThis allows transpilers to safely drop exports of types without looking for their definition.\n\nThe rule ensures that types are exported using a type-only `export`.\nIt also groups inline type exports into a grouped `export type`.\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\ninterface I {}\nexport { I };\n```\n\n```ts,expect_diagnostic\ntype T = number;\nexport { T };\n```\n\n```ts,expect_diagnostic\nimport type { T } from \"./mod.js\";\nexport { T };\n```\n\n```ts,expect_diagnostic\nexport { type X, type Y };\n```\n\n## Valid\n\n```js\nclass C {}\nfunction f() {}\nexport { C, f };\n```\n\nThis rules checks only the identifiers that are defined in a file.\nIt doesn't warn against a type exported as a value in a re-export clause such as:\n\n```ts,ignore\nexport { TypeA } from \"./mod.ts\"\n```" ;
const USE_FLAT_MAP : & str = "Promotes the use of `.flatMap()` when `map().flat()` are used together.\n\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nconst array = [\"split\", \"the text\", \"into words\"];\narray.map(sentence => sentence.split(' ')).flat();\n```\n\n```js,expect_diagnostic\nconst array = [\"split\", \"the text\", \"into words\"];\narray.map(sentence => sentence.split(' ')).flat(1);\n```\n\n### Valid\n\n```js\nconst array = [\"split\", \"the text\", \"into words\"];\narray.map(sentence => sentence.split(' ')).flat(2);\n```\n" ;
const USE_FOR_OF : & str = "This rule recommends a `for-of` loop when in a `for` loop, the index used to extract an item from the iterated array.\n\n\nSource: https://typescript-eslint.io/rules/prefer-for-of/\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nfor (let i = 0; i < array.length; i++) {\nconsole.log(array[i]);\n}\n```\n\n## Valid\n\n```js\nfor (let i = 0; i < array.length; i++) {\nconsole.log(i, array[i]);\n}\n```\n\n```js\nfor (let i = 0, j = 0; i < array.length; i++) {\nconsole.log(i, array[i]);\n}\n```\n" ;
const USE_FRAGMENT_SYNTAX : & str = "This rule enforces the use of `<>...</>` over `<Fragment>...</Fragment>`.\n\nThe shorthand fragment syntax saves keystrokes and is only inapplicable when keys are required.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\n<Fragment>child</Fragment>\n```\n\n```js,expect_diagnostic\n<React.Fragment>child</React.Fragment>\n```" ;
const USE_GETTER_RETURN : & str = "Enforce `get` methods to always return a value.\n\nSource: https://eslint.org/docs/latest/rules/getter-return\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nclass Person {\nget firstName() {}\n}\n```\n\n```js,expect_diagnostic\nconst obj = {\nget firstName() {\nreturn;\n}\n}\n```\n\n```js,expect_diagnostic\nclass Option {\nget value() {\nif (this.hasValue) {\nlog();\n} else {\nreturn null;\n}\n}\n}\n```\n\n## Valid\n\n```js\nclass Person {\nget firstName() {\nreturn this.fullname.split(\" \")[0];\n}\n}\n```\n\n```js\nconst obj = {\nget firstName() {\nreturn this.fullname.split(\" \")[0];\n}\n}\n```\n" ;
const USE_GROUPED_TYPE_IMPORT : & str = "Enforce the use of `import type` when an `import` only has specifiers with `type` qualifier.\n\nThe [`--verbatimModuleSyntax`](https://www.typescriptlang.org/tsconfig#verbatimModuleSyntax) _TypeScript_'s compiler option causes _TypeScript_ to do simple and predictable transpilation on `import` declarations.\nNamely, it completely removes `import type` and any imported names with the `type` qualifier.\n\nFor instance, the following code:\n\n```ts,expect_diagnostic\nimport { type A, type B } from \"mod-1\";\nimport type { C, D } from \"mod-2\";\n```\n\nis transpiled to:\n\n```ts\nimport \"mod-1\";\n```\n\nNote that, an `import` that includes only names qualified with `type` is transpiled to a [side-effect `import`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import#import_a_module_for_its_side_effects_only).\nThis can be a surprising behavior: most of developers could expect the deletion of the `import`.\n\nThis behavior may still be desirable for applying the potential side-effects of the imported module.\nIn most cases you will not want to leave behind an unnecessary side effect `import`.\nIn teh remaining cases, it is often preferable to explicitly use a side-effect `import` to apply the side-effects of a module:\n\n```ts\nimport \"mod\"; // side-effect import\nimport type { A, B } from \"mod\";\n```\n\nSource: https://typescript-eslint.io/rules/no-import-type-side-effects/\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\nimport { type A } from \"mod\";\n```\n\n## Valid\n\n```ts\nimport type { A, B } from \"mod\";\n```\n\n```ts\nimport { A, type B } from \"mod\";\n```" ;
const USE_HEADING_CONTENT : & str = "Enforce that heading elements (h1, h2, etc.) have content and that the content is accessible to screen readers. Accessible means that it is not hidden using the aria-hidden prop.\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<h1 />\n```\n\n```jsx,expect_diagnostic\n<h1><div aria-hidden /></h1>\n```\n\n```jsx,expect_diagnostic\n<h1></h1>\n```\n\n## Valid\n\n```jsx\n<h1>heading</h1>\n```\n\n```jsx\n<h1><div aria-hidden=\"true\"></div>visible content</h1>\n```\n\n```jsx\n<h1 dangerouslySetInnerHTML={{ __html: \"heading\" }} />\n```\n\n```jsx\n<h1><div aria-hidden />visible content</h1>\n```\n\n## Accessibility guidelines\n\n- [WCAG 2.4.6](https://www.w3.org/TR/UNDERSTANDING-WCAG20/navigation-mechanisms-descriptive.html)\n" ;
const USE_HOOK_AT_TOP_LEVEL : & str = "Enforce that all React hooks are being called from the Top Level component functions.\n\nTo understand why this required see https://reactjs.org/docs/hooks-rules.html#only-call-hooks-at-the-top-level\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nfunction Component1({ a }) {\nif (a == 1) {\nuseEffect();\n}\n}\n```\n\n```js,expect_diagnostic\nfunction Component1({ a }) {\nif (a != 1) {\nreturn;\n}\n\nuseEffect();\n}\n```\n\n## Valid\n\n```js\nfunction Component1() {\nuseEffect();\n}\n```\n\n## Options\n\nAllows to specify custom hooks - from libraries or internal projects - that can be considered stable.\n\n```json\n{\n\"//\": \"...\",\n\"options\": {\n\"hooks\": [\n{ \"name\": \"useLocation\", \"closureIndex\": 0, \"dependenciesIndex\": 1},\n{ \"name\": \"useQuery\", \"closureIndex\": 1, \"dependenciesIndex\": 0}\n]\n}\n}\n```\n\nGiven the previous example, your hooks be used like this:\n\n```js\nfunction Foo() {\nconst location = useLocation(() => {}, []);\nconst query = useQuery([], () => {});\n}\n```\n" ;
const USE_HTML_LANG : & str = "Enforce that `html` element has `lang` attribute.\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<html></html>\n```\n\n```jsx,expect_diagnostic\n<html lang={\"\"}></html>\n```\n\n```jsx,expect_diagnostic\n<html lang={null}></html>\n```\n\n```jsx,expect_diagnostic\n<html lang={undefined}></html>\n```\n\n```jsx,expect_diagnostic\n<html lang={true}></html>\n```\n\n### Valid\n\n```jsx\n<html lang=\"en\"></html>\n```\n\n```jsx\n<html lang={language}></html>\n```\n\n```jsx\n<html {...props}></html>\n```\n\n```jsx\n<html lang={\"\"} {...props}></html>\n```\n\n## Accessibility guidelines\n\n- [WCAG 3.1.1](https://www.w3.org/WAI/WCAG21/Understanding/language-of-page)\n" ;
const USE_IFRAME_TITLE : & str = "Enforces the usage of the attribute `title` for the element `iframe`.\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<iframe />\n```\n\n```jsx,expect_diagnostic\n<iframe></iframe>\n```\n\n```jsx,expect_diagnostic\n<iframe title=\"\" />\n```\n\n```jsx,expect_diagnostic\n<iframe title={\"\"} />\n```\n\n```jsx,expect_diagnostic\n<iframe title={undefined} />\n```\n\n```jsx,expect_diagnostic\n<iframe title={false} />\n```\n\n```jsx,expect_diagnostic\n<iframe title={true} />\n```\n\n```jsx,expect_diagnostic\n<iframe title={42} />\n```\n\n\n### Valid\n\n```jsx\n<>\n<iframe title=\"This is a unique title\" />\n<iframe title={uniqueTitle} />\n<iframe {...props} />\n</>\n```\n\n## Accessibility guidelines\n\n- [WCAG 2.4.1](https://www.w3.org/WAI/WCAG21/Understanding/bypass-blocks)\n- [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)\n" ;
const USE_IMPORT_RESTRICTIONS : & str = "Disallows package private imports.\n\nThis rules enforces the following restrictions:\n\n## Package private visibility\n\nAll exported symbols, such as types, functions or other things that may be exported, are\nconsidered to be \"package private\". This means that modules that reside in the same\ndirectory, as well as submodules of those \"sibling\" modules, are allowed to import them,\nwhile any other modules that are further away in the file system are restricted from\nimporting them. A symbol's visibility may be extended by re-exporting from an index file.\n\nNotes:\n\n* This rule only applies to relative imports. External dependencies are exempted.\n* This rule only applies to imports for JavaScript and TypeScript files. Imports for\nresources such as images or CSS files are exempted.\n\nSource: https://github.com/uhyo/eslint-plugin-import-access\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\n// Attempt to import from `foo.js` from outside its `sub` module.\nimport { fooPackageVariable } from \"./sub/foo.js\";\n```\n```js,expect_diagnostic\n// Attempt to import from `bar.ts` from outside its `aunt` module.\nimport { barPackageVariable } from \"../aunt/bar.ts\";\n```\n\n```js,expect_diagnostic\n// Assumed to resolve to a JS/TS file.\nimport { fooPackageVariable } from \"./sub/foo\";\n```\n\n```js,expect_diagnostic\n// If the `sub/foo` module is inaccessible, so is its index file.\nimport { fooPackageVariable } from \"./sub/foo/index.js\";\n```\n\n### Valid\n\n```js\n// Imports within the same module are always allowed.\nimport { fooPackageVariable } from \"./foo.js\";\n\n// Resources (anything other than JS/TS files) are exempt.\nimport { barResource } from \"../aunt/bar.png\";\n\n// A parent index file is accessible like other modules.\nimport { internal } from \"../../index.js\";\n\n// If the `sub` module is accessible, so is its index file.\nimport { subPackageVariable } from \"./sub/index.js\";\n\n// Library imports are exempt.\nimport useAsync from \"react-use/lib/useAsync\";\n```\n" ;
const USE_IS_ARRAY : & str = "Use `Array.isArray()` instead of `instanceof Array`.\n\nIn _JavaScript_ some array-like objects such as _arguments_ are not instances of the `Array` class.    ///\nMoreover, the global `Array` class can be different between two execution contexts.\nFor instance, two frames in a web browser have a distinct `Array` class.\nPassing arrays across these contexts, results in arrays that are not instances of the contextual global `Array` class.\nTo avoid these issues, use `Array.isArray()` instead of `instanceof Array`.\nSee the [MDN docs](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/isArray) for more details.\n\nSource: https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/no-instanceof-array.md\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nconst xs = [];\nif (xs instanceof Array) {}\n```\n\n## Valid\n\n```js\nconst xs = [];\nif (Array.isArray(xs)) {}\n```\n" ;
const USE_IS_NAN : & str = "Require calls to `isNaN()` when checking for `NaN`.\n\nIn JavaScript, `NaN` is a special value of the `Number` type.\nIt’s used to represent any of the \"not-a-number\" values represented by the double-precision 64-bit format as specified by the IEEE Standard for Binary Floating-Point Arithmetic.\n\nBecause `NaN` is unique in JavaScript by not being equal to anything, including itself, the results of comparisons to `NaN` are confusing:\n- `NaN` === `NaN` or `NaN` == `NaN` evaluate to false\n- `NaN` !== `NaN` or `NaN` != `NaN` evaluate to true\n\nTherefore, use `Number.isNaN()` or global `isNaN()` functions to test whether a value is `NaN`.\n\nNote that `Number.isNaN()` and `isNaN()` [have not the same behavior](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/isNaN#description).\nWhen the argument to `isNaN()` is not a number, the value is first coerced to a number.\n`Number.isNaN()` does not perform this coercion.\nTherefore, it is a more reliable way to test whether a value is `NaN`.\n\nSource: [use-isnan](https://eslint.org/docs/latest/rules/use-isnan).\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\n123 == NaN\n```\n\n```js,expect_diagnostic\n123 != NaN\n```\n\n```js,expect_diagnostic\nswitch(foo) { case (NaN): break; }\n```\n\n```js,expect_diagnostic\nNumber.NaN == \"abc\"\n```\n\n### Valid\n\n```js\nif (Number.isNaN(123) !== true) {}\n\nfoo(Number.NaN / 2)\n\nswitch(foo) {}\n```\n" ;
const USE_KEY_WITH_CLICK_EVENTS : & str = "Enforce onClick is accompanied by at least one of the following: `onKeyUp`, `onKeyDown`, `onKeyPress`.\n\nCoding for the keyboard is important for users with physical disabilities who cannot use a mouse, AT compatibility, and screenreader users.\nThis does not apply for interactive or hidden elements.\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<div onClick={() => {}} />\n```\n\n```jsx,expect_diagnostic\n<div onClick={() => {}} ></div>\n```\n\n### Valid\n\n```jsx\n<div onClick={() => {}} onKeyDown={handleKeyDown} />\n```\n\n```jsx\n<div onClick={() => {}} onKeyUp={handleKeyUp} />\n```\n\n```jsx\n<div onClick={() => {}} onKeyPress={handleKeyPress} />\n```\n\n```jsx\n// this rule doesn't apply to user created component\n<MyComponent onClick={() => {}} />\n```\n\n```jsx,\n<div onClick={() => {}} {...spread}></div>\n```\n\n```jsx\n<div {...spread} onClick={() => {}} ></div>\n```\n\n```jsx\n<button onClick={() => console.log(\"test\")}>Submit</button>\n```\n\n## Accessibility guidelines\n\n- [WCAG 2.1.1](https://www.w3.org/WAI/WCAG21/Understanding/keyboard)\n" ;
const USE_KEY_WITH_MOUSE_EVENTS : & str = "Enforce `onMouseOver` / `onMouseOut` are accompanied by `onFocus` / `onBlur`.\n\nCoding for the keyboard is important for users with physical disabilities who cannot use a mouse, AT compatibility, and screenreader users.\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<div onMouseOver={() => {}} />\n```\n\n```jsx,expect_diagnostic\n<div onMouseOut={() => {}} />\n```\n\n### Valid\n\n```jsx\n<>\n<div onMouseOver={() => {}} onFocus={() => {}} />\n<div onMouseOut={() => {}} onBlur={() => {}} />\n<div onMouseOver={() => {}} {...otherProps} />\n<div onMouseOut={() => {}} {...otherProps} />\n<div onMouseOver={() => {}} onFocus={() => {}} {...otherProps} />\n<div onMouseOut={() => {}} onBlur={() => {}} {...otherProps} />\n</>\n```\n\n## Accessibility guidelines\n\n- [WCAG 2.1.1](https://www.w3.org/WAI/WCAG21/Understanding/keyboard)\n" ;
const USE_LITERAL_ENUM_MEMBERS : & str = "Require all enum members to be literal values.\n\nUsually, an enum member is initialized with a literal number or a literal string.\nHowever, _TypeScript_ allows the value of an enum member to be many different kinds of expressions.\nUsing a computed enum member is often error-prone and confusing.\nThis rule requires the initialization of enum members with constant expressions.\nIt allows numeric and bitwise expressions for supporting [enum flags](https://stackoverflow.com/questions/39359740/what-are-enum-flags-in-typescript/39359953#39359953).\nIt also allows referencing previous enum members.\n\nSource: https://typescript-eslint.io/rules/prefer-literal-enum-member/\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\nconst x = 2;\nenum Computed {\nA,\nB = x,\n}\n```\n\n## Valid\n\n```ts\nenum Direction {\nLeft,\nRight,\n}\n```\n\n```ts\nenum Order {\nLess = -1,\nEqual = 0,\nGreater = 1,\n}\n```\n\n```ts\nenum State {\nOpen = \"Open\",\nClose = \"Close\",\n}\n```\n\n```ts\nenum FileAccess {\nNone = 0,\nRead = 1,\nWrite = 1 << 1,\nAll = Read | Write\n}\n```" ;
const USE_LITERAL_KEYS : & str = "Enforce the usage of a literal access to properties over computed property access.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\na.b[\"c\"];\n```\n\n```js,expect_diagnostic\na.c[`d`]\n```\n\n```js,expect_diagnostic\na.c[`d`] = \"something\"\n```\n\n```js,expect_diagnostic\na = {\n['b']: d\n}\n```\n\n## Valid\n\n```js\na[\"c\" + \"d\"];\na[d.c];\n```\n" ;
const USE_MEDIA_CAPTION : & str = "Enforces that `audio` and `video` elements must have a `track` for captions.\n\n**ESLint Equivalent:** [media-has-caption](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/media-has-caption.md)\n\n## Examples\n\n### Invalid\n```jsx,expect_diagnostic\n<video />\n```\n\n```jsx,expect_diagnostic\n<audio>child</audio>\n```\n\n### Valid\n\n```jsx\n<audio>\n<track kind=\"captions\" {...props} />\n</audio>\n```\n\n```jsx\n<video muted {...props}></video>\n```" ;
const USE_NAMESPACE_KEYWORD : & str = "Require using the `namespace` keyword over the `module` keyword to declare TypeScript namespaces.\n\nTypeScript historically allowed a code organization called _namespace_.\n[_ECMAScript modules_ are preferred](https://www.typescriptlang.org/docs/handbook/2/modules.html#typescript-namespaces) (`import` / `export`).\n\nFor projects still using _namespaces_, it's preferred to use the `namespace` keyword instead of the `module` keyword.\nThe `module` keyword is deprecated to avoid any confusion with the _ECMAScript modules_ which are often called _modules_.\n\nNote that TypeScript `module` declarations to describe external APIs (`declare module \"foo\" {}`) are still allowed.\n\nSource: https://typescript-eslint.io/rules/prefer-namespace-keyword\n\nSee also: https://www.typescriptlang.org/docs/handbook/namespaces-and-modules.html\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\nmodule Example {}\n```\n\n## Valid\n\n```ts\nnamespace Example {}\n```\n\n```ts\ndeclare module \"foo\" {}\n```\n" ;
const USE_NAMING_CONVENTION : & str = "Enforce naming conventions for everything across a codebase.\n\nEnforcing [naming conventions](https://en.wikipedia.org/wiki/Naming_convention_(programming)) helps to keep the codebase consistent,\nand reduces overhead when thinking about the name [case] of a variable.\n\n## Naming conventions\n\nAll names can be prefixed and suffixed by underscores `_` and dollar signs `$`.\n\n### Variable names\n\nAll variables, including function parameters and catch parameters, are in [`camelCase`].\n\nAdditionally, top-level variables declared as `const` or `var` may be in [`CONSTANT_CASE`] or [`PascalCase`].\nTop-level variables are declared at module or script level.\nVariables declared in a TypeScript `module` or `namespace` are also considered top-level.\n\n```js\nfunction f(param, _unusedParam) {\nlet localValue = 0;\ntry {\n/* ... */\n} catch (customError) {\n/* ... */\n}\n}\n\nexport const A_CONSTANT = 5;\n\nexport const Person = class {}\n\nlet aVariable = 0;\n\nexport namespace ns {\nexport const ANOTHER_CONSTANT = \"\";\n}\n```\n\nExamples of incorrect names:\n\n```js,expect_diagnostic\nlet a_value = 0;\n```\n\n```js,expect_diagnostic\nconst fooYPosition = 0;\n```\n\n```js,expect_diagnostic\nfunction f(FirstParam) {}\n```\n\n### Function names\n\nA `function` name is in [`camelCase`] or [`PascalCase`].\n\n```jsx\nfunction trimString(s) { /*...*/ }\n\nfunction Component() {\nreturn <div></div>;\n}\n```\n\n### TypeScript `enum` names\n\nA _TypeScript_ `enum` name is in [`PascalCase`].\n\n`enum` members are by default in [`PascalCase`].\nHowever, you can configure the [case] of `enum` members.\nSee [options](#options) for more details.\n\n```ts\nenum Status {\nOpen,\nClose,\n}\n```\n\n### Classes\n\n- A class name is in [`PascalCase`].\n\n- Static property and static getter names are in [`camelCase`] or [`CONSTANT_CASE`].\n\n- Class property and method names are in [`camelCase`].\n\n```js\nclass Person {\nstatic MAX_FRIEND_COUNT = 256;\n\nstatic get SPECIAL_PERSON_INSTANCE() { /*...*/ }\n\ninitializedProperty = 0;\n\nspecialMethod() {}\n}\n```\n\n### TypeScript `type` aliases and `interface`\n\n- A `type` alias or an interface name are in [`PascalCase`].\n\n- Property and method names in a type are in [`camelCase`].\n\n- `readonly` property and getter names can also be in [`CONSTANT_CASE`].\n\n```ts\ntype Named = {\nreadonly fullName: string;\n\nspecialMethod(): void;\n};\n\ninterface Named {\nreadonly fullName: string;\n\nspecialMethod(): void;\n}\n\ninterface PersonConstructor {\nreadonly MAX_FRIEND_COUNT: number;\n\nget SPECIAL_PERSON_INSTANCE(): Person;\n\nnew(): Person;\n}\n```\n\nExamples of an incorrect type alias:\n\n```ts,expect_diagnostic\ntype person = { fullName: string };\n```\n\n### Literal object property and method names\n\nLiteral object property and method names are in [`camelCase`].\n\n```js\nconst alice = {\nfullName: \"Alice\",\n}\n```\n\nExample of an incorrect name:\n\n```js,expect_diagnostic\nconst alice = {\nFULL_NAME: \"Alice\",\n}\n```\n\n### Imported and exported module aliases\n\nImported and exported module aliases are in [`camelCase`] or [`PascalCase`].\n\n```js\nimport * as myLib from \"my-lib\";\nimport * as Framework from \"framework\";\n\nexport * as myLib from \"my-lib\";\nexport * as Framework from \"framework\";\n```\n\n`import` and `export` aliases are in [`camelCase`], [`PascalCase`], or [`CONSTANT_CASE`]:\n\n```js\nimport assert, {\ndeepStrictEqual as deepEqual,\nAssertionError as AssertError\n} from \"node:assert\";\n```\n\nExamples of an incorrect name:\n\n```ts,expect_diagnostic\nimport * as MY_LIB from \"my-lib\";\n```\n\n### TypeScript type parameter names\n\nA _TypeScript_ type parameter name is in [`PascalCase`].\n\n```ts\nfunction id<Val>(value: Val): Val { /* ... */}\n```\n\n### TypeScript `namespace` names\n\nA _TypeScript_ `namespace` name is in [`camelCase`] or in [`PascalCase`].\n\n```ts\nnamespace mathExtra {\n/*...*/\n}\n\nnamespace MathExtra {\n/*...*/\n}\n```\n\n## Options\n\nThe rule provides two options that are detailed in the following subsections.\n\n```json\n{\n\"//\": \"...\",\n\"options\": {\n\"strictCase\": false,\n\"enumMemberCase\": \"CONSTANT_CASE\"\n}\n}\n```\n\n### strictCase\n\nWhen this option is set to `true`, it forbids consecutive uppercase characters in [`camelCase`] and [`PascalCase`].\nFor instance,  when the option is set to `true`, `HTTPServer` or `aHTTPServer` will throw an error.\nThese names should be renamed to `HttpServer` and `aHttpServer`\n\nWhen the option is set to `false`, consecutive uppercase characters are allowed.\n`HTTPServer` and `aHTTPServer` are so valid.\n\nDefault: `true`\n\n### enumMemberCase\n\nBy default, the rule enforces the naming convention followed by the [TypeScript Compiler team](https://www.typescriptlang.org/docs/handbook/enums.html):\nan `enum` member is in [`PascalCase`].\n\nYou can enforce another convention by setting `enumMemberCase` option.\nThe supported cases are: [`PascalCase`], [`CONSTANT_CASE`], and [`camelCase`].\n\n[case]: https://en.wikipedia.org/wiki/Naming_convention_(programming)#Examples_of_multiple-word_identifier_formats\n[`camelCase`]: https://en.wikipedia.org/wiki/Camel_case\n[`PascalCase`]: https://en.wikipedia.org/wiki/Camel_case\n[`CONSTANT_CASE`]: https://en.wikipedia.org/wiki/Snake_case" ;
const USE_NUMERIC_LITERALS : & str = "Disallow `parseInt()` and `Number.parseInt()` in favor of binary, octal, and hexadecimal literals\n\n_JavaScript_ provides literal forms for binary, octal, and hexadecimal numbers.\nFor example: `0b11`, `0o77`, and `0xff`.\nUsing the literal forms enable static code analysis and avoid unnecessary computations.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nparseInt(\"111110111\", 2);\n```\n\n```js,expect_diagnostic\nNumber.parseInt(\"767\", 8);\n```\n\n```js,expect_diagnostic\nNumber.parseInt(\"-1f7\", 16);\n```\n\n### Valid\n\n```js\nparseInt(1);\nparseInt(1, 3);\nNumber.parseInt(1);\nNumber.parseInt(1, 3);\n\n0b111110111 === 503;\n0o767 === 503;\n0x1F7 === 503;\n\na[parseInt](1,2);\n\nparseInt(foo);\nparseInt(foo, 2);\nNumber.parseInt(foo);\nNumber.parseInt(foo, 2);\n```" ;
const USE_OPTIONAL_CHAIN : & str = "Enforce using concise optional chain instead of chained logical expressions.\n\nTypeScript 3.7 added support for the optional chain operator.\nThis operator allows you to safely access properties and methods on objects when they are potentially `null` or `undefined`.\nThe optional chain operator only chains when the property value is `null` or `undefined`.\nIt is much safer than relying upon logical operator chaining; which chains on any truthy value.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nfoo && foo.bar && foo.bar.baz && foo.bar.baz.buzz\n```\n\n```js,expect_diagnostic\nfoo.bar && foo.bar.baz.buzz\n```\n\n```js,expect_diagnostic\nfoo !== undefined && foo.bar != undefined && foo.bar.baz !== null && foo.bar.baz.buzz\n```\n\n```js,expect_diagnostic\n((foo || {}).bar || {}).baz;\n```\n\n```js,expect_diagnostic\n(await (foo1 || {}).foo2 || {}).foo3;\n```\n\n```ts,expect_diagnostic\n(((typeof x) as string) || {}).bar;\n```\n\n### Valid\n\n```js\nfoo && bar;\n```\n```js\nfoo || {};\n```\n\n```js\n(foo = 2 || {}).bar;\n```\n\n```js\nfoo || foo.bar;\n```\n\n```js\nfoo[\"some long\"] && foo[\"some long string\"].baz\n```\n" ;
const USE_REGEX_LITERALS : & str = "Enforce the use of the regular expression literals instead of the RegExp constructor if possible.\n\nThere are two ways to create a regular expression:\n- Regular expression literals, e.g., `/abc/u`.\n- The RegExp constructor function, e.g., `new RegExp(\"abc\", \"u\")` .\n\nThe constructor function is particularly useful when you want to dynamically generate the pattern,\nbecause it takes string arguments.\n\nUsing regular expression literals avoids some escaping required in a string literal,\nand are easier to analyze statically.\n\nSource: https://eslint.org/docs/latest/rules/prefer-regex-literals/\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nnew RegExp(\"abc\", \"u\");\n```\n\n## Valid\n\n```js\n/abc/u;\n\nnew RegExp(\"abc\", flags);\n```\n" ;
const USE_SELF_CLOSING_ELEMENTS : & str = "Prevent extra closing tags for components without children\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\n<div></div>\n```\n\n```js,expect_diagnostic\n<Component></Component>\n```\n\n```js,expect_diagnostic\n<Foo.bar></Foo.bar>\n```\n\n### Valid\n\n```js\n<div />\n```\n\n```js\n<div>child</div>\n```\n\n```js\n<Component />\n```\n\n```js\n<Component>child</Component>\n```\n\n```js\n<Foo.bar />\n```\n\n```js\n<Foo.bar>child</Foo.bar>\n```" ;
const USE_SHORTHAND_ARRAY_TYPE : & str = "When expressing array types, this rule promotes the usage of `T[]` shorthand instead of `Array<T>`.\n\nESLint (typescript-eslint) equivalent: [array-type/array-simple](https://typescript-eslint.io/rules/array-type/#array-simple)\n\n## Examples\n\n### Invalid\n```ts,expect_diagnostic\nlet invalid: Array<foo>;\n```\n\n```ts,expect_diagnostic\nlet invalid: Promise<Array<string>>;\n```\n\n```ts,expect_diagnostic\nlet invalid: Array<Foo<Bar>>;\n```\n\n```ts,expect_diagnostic\nlet invalid: Array<[number, number]>;\n```\n\n```ts,expect_diagnostic\nlet invalid: Array<[number, number]>;\n```\n\n```ts,expect_diagnostic\nlet invalid: ReadonlyArray<string>;\n```\n\n### Valid\n\n```ts\nlet valid: Array<Foo | Bar>;\nlet valid: Array<keyof Bar>;\nlet valid: Array<foo | bar>;\n```" ;
const USE_SHORTHAND_ASSIGN : & str = "Require assignment operator shorthand where possible.\n\nJavaScript provides shorthand operators combining a variable assignment and simple mathematical operation.\n\nSource: https://eslint.org/docs/latest/rules/operator-assignment/\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\na = a + 1;\n```\n\n```js,expect_diagnostic\na = a - 1;\n```\n\n```js,expect_diagnostic\na = a * 1;\n```\n\n## Valid\n\n```js\na += 1;\n```\n\n```js\na -= 1;\n```\n\n```js\na *= 1;\n```" ;
const USE_SHORTHAND_FUNCTION_TYPE : & str = "Enforce using function types instead of object type with call signatures.\n\nTypeScript allows for two common ways to declare a type for a function:\n\n- Function type: `() => string`\n- Object type with a signature: `{ (): string }`\n\nThe function type form is generally preferred when possible for being more succinct.\n\nThis rule suggests using a function type instead of an interface or object type literal with a single call signature.\n\nSource: https://typescript-eslint.io/rules/prefer-function-type/\n\n## Examples\n\n### Invalid\n\n```ts,expect_diagnostic\ninterface Example {\n(): string;\n}\n```\n\n```ts,expect_diagnostic\nfunction foo(example: { (): number }): number {\nreturn example();\n}\n```\n\n## Valid\n\n```ts\ntype Example = () => string;\n```\n\n```ts\nfunction foo(example: () => number): number {\nreturn bar();\n}\n```\n\n```ts\n// returns the function itself, not the `this` argument.\ntype ReturnsSelf2 = (arg: string) => ReturnsSelf;\n```\n\n```ts\ninterface Foo {\nbar: string;\n}\ninterface Bar extends Foo {\n(): void;\n}\n```\n\n```ts\n// multiple call signatures (overloads) is allowed:\ninterface Overloaded {\n(data: string): number;\n(id: number): string;\n}\n// this is equivalent to Overloaded interface.\ntype Intersection = ((data: string) => number) & ((id: number) => string);\n```\n" ;
const USE_SIMPLE_NUMBER_KEYS : & str = "Disallow number literal object member names which are not base10 or uses underscore as separator\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\n({ 0x1: 1 });\n```\n```js,expect_diagnostic\n({ 11_1.11: \"ee\" });\n```\n```js,expect_diagnostic\n({ 0o1: 1 });\n```\n```js,expect_diagnostic\n({ 1n: 1 });\n```\n```js,expect_diagnostic\n({ 11_1.11: \"ee\" });\n```\n\n## Valid\n\n```js\n({ 0: \"zero\" });\n({ 122: \"integer\" });\n({ 1.22: \"floating point\" });\n({ 3.1e12: \"floating point with e\" });\n```\n" ;
const USE_SIMPLIFIED_LOGIC_EXPRESSION : & str = "Discard redundant terms from logical expressions.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nconst boolExp = true;\nconst r = true && boolExp;\n```\n\n```js,expect_diagnostic\nconst boolExp2 = true;\nconst r2 = boolExp || true;\n```\n\n```js,expect_diagnostic\nconst nonNullExp = 123;\nconst r3 = null ?? nonNullExp;\n```\n\n```js,expect_diagnostic\nconst boolExpr1 = true;\nconst boolExpr2 = false;\nconst r4 = !boolExpr1 || !boolExpr2;\n```\n\n### Valid\n```js\nconst boolExpr3 = true;\nconst boolExpr4 = false;\nconst r5 = !(boolExpr1 && boolExpr2);\nconst boolExpr5 = true;\nconst boolExpr6 = false;\n```\n" ;
const USE_SINGLE_CASE_STATEMENT : & str = "Enforces switch clauses have a single statement, emits a quick fix wrapping the statements in a block.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nswitch (foo) {\ncase true:\ncase false:\nlet foo = '';\nfoo;\n}\n```\n\n### Valid\n\n```js\nswitch (foo) {\ncase true:\ncase false: {\nlet foo = '';\nfoo;\n}\n}\n```" ;
const USE_SINGLE_VAR_DECLARATOR : & str = "Disallow multiple variable declarations in the same variable statement\n\nIn JavaScript, multiple variables can be declared within a single `var`, `const` or `let` declaration.\nIt is often considered a best practice to declare every variable separately.\nThat is what this rule enforces.\n\nSource: https://eslint.org/docs/latest/rules/one-var\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nlet foo = 0, bar, baz;\n```\n\n### Valid\n\n```js\nconst foo = 0;\nlet bar;\nlet baz;\n```\n\n```js\nfor (let i = 0, x = 1; i < arr.length; i++) {}\n```" ;
const USE_TEMPLATE : & str = "Prefer template literals over string concatenation.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nconst s = foo + \"baz\";\n```\n\n```js,expect_diagnostic\nconst s = 1 + 2 + \"foo\" + 3;\n```\n\n```js,expect_diagnostic\nconst s = 1 * 2 + \"foo\";\n```\n\n```js,expect_diagnostic\nconst s = 1 + \"foo\" + 2 + \"bar\" + \"baz\" + 3;\n```\n\n### Valid\n\n```js\nlet s = \"foo\" + \"bar\" + `baz`;\n```\n\n```js\nlet s = `value: ${1}`;\n```" ;
const USE_VALID_ANCHOR : & str = "Enforce that all anchors are valid, and they are navigable elements.\n\nThe anchor element (`<a></a>`) - also called **hyperlink** - is an important element\nthat allows users to navigate pages, in the same page, same website or on another website.\n\nWhile before it was possible to attach logic to an anchor element, with the advent of JSX libraries,\nit's now  easier to attach logic to any HTML element, anchors included.\n\nThis rule is designed to prevent users to attach logic at the click of anchors, and also makes\nsure that the `href` provided to the anchor element is valid. If the anchor has logic attached to it,\nthe rules suggests to turn it to a `button`, because that's likely what the user wants.\n\nAnchor `<a></a>` elements should be used for navigation, while `<button></button>` should be\nused for user interaction.\n\nThere are **many reasons** why an anchor should not have a logic and have a correct `href` attribute:\n- it can disrupt the correct flow of the user navigation e.g. a user that wants to open the link\nin another tab, but the default \"click\" behaviour is prevented\n- it can source of invalid links, and crawlers can't navigate the website, risking to penalise\nSEO ranking\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<a href={null}>navigate here</a>\n```\n```jsx,expect_diagnostic\n<a href={undefined}>navigate here</a>\n```\n```jsx,expect_diagnostic\n<a href>navigate here</a>\n```\n```jsx,expect_diagnostic\n<a href=\"javascript:void(0)\">navigate here</a>\n```\n```jsx,expect_diagnostic\n<a href=\"https://example.com\" onClick={something}>navigate here</a>\n```\n### Valid\n\n```jsx\n<a href={`https://www.javascript.com`}>navigate here</a>\n```\n\n```jsx\n<a href={somewhere}>navigate here</a>\n```\n\n```jsx\n<a {...spread}>navigate here</a>\n```\n\n## Accessibility guidelines\n\n- [WCAG 2.1.1](https://www.w3.org/WAI/WCAG21/Understanding/keyboard)\n" ;
const USE_VALID_ARIA_PROPS : & str = "Ensures that ARIA properties `aria-*` are all valid.\n\n## Examples\n\n### Invalid\n\n```jsx, expect_diagnostic\n<input className=\"\" aria-labell=\"\" />\n```\n\n```jsx,expect_diagnostic\n<div aria-lorem=\"foobar\" />;\n```\n\n## Accessibility guidelines\n- [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)" ;
const USE_VALID_ARIA_ROLE : & str = "Elements with ARIA roles must use a valid, non-abstract ARIA role.\n\nSource: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/aria-role.md\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\n<div role=\"datepicker\"></div>\n```\n\n```js,expect_diagnostic\n<div role=\"range\"></div>\n```\n\n```js,expect_diagnostic\n<div role=\"\"></div>\n```\n\n```js,expect_diagnostic\n<Foo role=\"foo\"></Foo>\n```\n\n### Valid\n\n```js\n<>\n<div role=\"button\"></div>\n<div role={role}></div>\n<div></div>\n</>\n```\n\n### Options\n\n```json\n{\n\"//\": \"...\",\n\"options\": {\n\"allowInvalidRoles\": [\"invalid-role\", \"text\"],\n\"nonIgnoreDom\": true\n}\n}\n```\n\n## Accessibility guidelines\n\n- [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)\n\n## Resources\n\n- [Chrome Audit Rules, AX_ARIA_01](https://github.com/GoogleChrome/accessibility-developer-tools/wiki/Audit-Rules#ax_aria_01)\n- [DPUB-ARIA roles](https://www.w3.org/TR/dpub-aria-1.0/)\n- [MDN: Using ARIA: Roles, states, and properties](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/ARIA_Techniques)\n" ;
const USE_VALID_ARIA_VALUES : & str = "Enforce that ARIA state and property values are valid.\n\n\n## Examples\n\n### Invalid\n\n```jsx, expect_diagnostic\n<span role=\"checkbox\" aria-checked=\"test\">some text</span>\n```\n\n```jsx, expect_diagnostic\n<span aria-labelledby=\"\">some text</span>\n```\n\n```jsx, expect_diagnostic\n<span aria-valuemax=\"hey\">some text</span>\n```\n\n```jsx, expect_diagnostic\n<span aria-orientation=\"hey\">some text</span>\n```\n\n### Valid\n\n```jsx\n<>\n<span role=\"checkbox\" aria-checked={checked} >some text</span>\n<span aria-labelledby=\"fooId barId\" >some text</span>\n</>\n```\n\n## Accessibility guidelines\n- [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)\n\n### Resources\n- [ARIA Spec, States and Properties](https://www.w3.org/TR/wai-aria/#states_and_properties)\n- [Chrome Audit Rules, AX_ARIA_04](https://github.com/GoogleChrome/accessibility-developer-tools/wiki/Audit-Rules#ax_aria_04)" ;
const USE_VALID_FOR_DIRECTION : & str = "Enforce \"for\" loop update clause moving the counter in the right direction.\n\nA for loop with a stop condition that can never be reached,\nsuch as one with a counter that moves in the wrong direction, will run infinitely.\nWhile there are occasions when an infinite loop is intended, the convention is to construct such loops as while loops.\nMore typically, an infinite for loop is a bug.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nfor (var i = 0; i < 10; i--) {\n}\n```\n\n```js,expect_diagnostic\nfor (var i = 10; i >= 0; i++) {\n}\n```\n\n```js,expect_diagnostic\nfor (var i = 0; i > 10; i++) {\n}\n```\n\n### Valid\n\n```js\nfor (var i = 0; i < 10; i++) {\n}\n```" ;
const USE_VALID_LANG : & str = "Ensure that the attribute passed to the `lang` attribute is a correct ISO language and/or country.\n\n## Examples\n\n### Invalid\n\n```jsx,expect_diagnostic\n<html lang=\"lorem\" />\n```\n\n```jsx,expect_diagnostic\n<html lang=\"en-babab\" />\n```\n\n```jsx,expect_diagnostic\n<html lang=\"en-GB-typo\" />\n```\n\n### Valid\n\n```jsx\n<Html lang=\"en-babab\" />\n```" ;
const USE_VALID_TYPEOF : & str = "This rule verifies the result of `typeof $expr` unary expressions is being compared to valid values, either string literals containing valid type names or other `typeof` expressions\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\ntypeof foo === \"strnig\"\n```\n\n```js,expect_diagnostic\ntypeof foo == \"undefimed\"\n```\n\n```js,expect_diagnostic\ntypeof bar != \"nunber\"\n```\n\n```js,expect_diagnostic\ntypeof bar !== \"fucntion\"\n```\n\n```js,expect_diagnostic\ntypeof foo === undefined\n```\n\n```js,expect_diagnostic\ntypeof bar == Object\n```\n\n```js,expect_diagnostic\ntypeof foo === baz\n```\n\n```js,expect_diagnostic\ntypeof foo == 5\n```\n\n```js,expect_diagnostic\ntypeof foo == -5\n```\n\n### Valid\n\n```js\ntypeof foo === \"string\"\n```\n\n```js\ntypeof bar == \"undefined\"\n```\n\n```js\ntypeof bar === typeof qux\n```" ;
const USE_WHILE : & str = "Enforce the use of `while` loops instead of `for` loops when the initializer and update expressions are not needed.\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nfor (; x.running;) {\nx.step();\n}\n```\n\n### Valid\n\n```js\nfor(let x = 0; x < 10; i++) {}\n```\n\n```js\nlet x = 0\nfor(; x < 10; i++) {}\n```\n\n```js\nfor(let x = 0; x < 10;) {\ni++\n}\n```" ;
const USE_YIELD : & str = "Require generator functions to contain `yield`.\n\nThis rule generates warnings for generator functions that do not have the `yield` keyword.\n\nSource: [require-yield](https://eslint.org/docs/latest/rules/require-yield).\n\n## Examples\n\n### Invalid\n\n```js,expect_diagnostic\nfunction* foo() {\nreturn 10;\n}\n```\n\n### Valid\n```js\nfunction* foo() {\nyield 5;\nreturn 10;\n}\n\nfunction foo() {\nreturn 10;\n}\n\n// This rule does not warn on empty generator functions.\nfunction* foo() { }\n```" ;
