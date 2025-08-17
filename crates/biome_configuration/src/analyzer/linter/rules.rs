//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::analyzer::{
    GroupPlainConfiguration, RuleConfiguration, RuleFixConfiguration, RuleGroupExt,
    RulePlainConfiguration, SeverityOrGroup,
};
use biome_analyze::{RuleFilter, options::RuleOptions};
use biome_deserialize_macros::{Deserializable, Merge};
use biome_diagnostics::{Category, Severity};
use rustc_hash::FxHashSet;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(
    Clone,
    Copy,
    Debug,
    Deserializable,
    Eq,
    Hash,
    Merge,
    Ord,
    PartialEq,
    PartialOrd,
    serde :: Deserialize,
    serde :: Serialize,
)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum RuleGroup {
    A11y,
    Complexity,
    Correctness,
    Nursery,
    Performance,
    Security,
    Style,
    Suspicious,
}
impl RuleGroup {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::A11y => A11y::GROUP_NAME,
            Self::Complexity => Complexity::GROUP_NAME,
            Self::Correctness => Correctness::GROUP_NAME,
            Self::Nursery => Nursery::GROUP_NAME,
            Self::Performance => Performance::GROUP_NAME,
            Self::Security => Security::GROUP_NAME,
            Self::Style => Style::GROUP_NAME,
            Self::Suspicious => Suspicious::GROUP_NAME,
        }
    }
}
impl std::str::FromStr for RuleGroup {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            A11y::GROUP_NAME => Ok(Self::A11y),
            Complexity::GROUP_NAME => Ok(Self::Complexity),
            Correctness::GROUP_NAME => Ok(Self::Correctness),
            Nursery::GROUP_NAME => Ok(Self::Nursery),
            Performance::GROUP_NAME => Ok(Self::Performance),
            Security::GROUP_NAME => Ok(Self::Security),
            Style::GROUP_NAME => Ok(Self::Style),
            Suspicious::GROUP_NAME => Ok(Self::Suspicious),
            _ => Err("This rule group doesn't exist."),
        }
    }
}
impl std::fmt::Display for RuleGroup {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(self.as_str())
    }
}
#[derive(
    Clone,
    Copy,
    Debug,
    Deserializable,
    Eq,
    Hash,
    Merge,
    Ord,
    PartialEq,
    PartialOrd,
    serde :: Deserialize,
    serde :: Serialize,
)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum RuleName {
    NoAccessKey,
    NoAccumulatingSpread,
    NoAdjacentSpacesInRegex,
    NoAlert,
    NoApproximativeNumericConstant,
    NoArguments,
    NoAriaHiddenOnFocusable,
    NoAriaUnsupportedElements,
    NoArrayIndexKey,
    NoAssignInExpressions,
    NoAsyncPromiseExecutor,
    NoAutofocus,
    NoAwaitInLoops,
    NoBannedTypes,
    NoBarrelFile,
    NoBiomeFirstException,
    NoBitwiseOperators,
    NoBlankTarget,
    NoCatchAssign,
    NoChildrenProp,
    NoClassAssign,
    NoCommaOperator,
    NoCommentText,
    NoCommonJs,
    NoCompareNegZero,
    NoConfusingLabels,
    NoConfusingVoidType,
    NoConsole,
    NoConstAssign,
    NoConstEnum,
    NoConstantBinaryExpressions,
    NoConstantCondition,
    NoConstantMathMinMaxClamp,
    NoConstructorReturn,
    NoControlCharactersInRegex,
    NoDangerouslySetInnerHtml,
    NoDangerouslySetInnerHtmlWithChildren,
    NoDebugger,
    NoDefaultExport,
    NoDelete,
    NoDescendingSpecificity,
    NoDistractingElements,
    NoDocumentCookie,
    NoDocumentImportInPage,
    NoDoneCallback,
    NoDoubleEquals,
    NoDuplicateAtImportRules,
    NoDuplicateCase,
    NoDuplicateClassMembers,
    NoDuplicateCustomProperties,
    NoDuplicateElseIf,
    NoDuplicateFields,
    NoDuplicateFontNames,
    NoDuplicateJsxProps,
    NoDuplicateObjectKeys,
    NoDuplicateParameters,
    NoDuplicateProperties,
    NoDuplicateSelectorsKeyframeBlock,
    NoDuplicateTestHooks,
    NoDynamicNamespaceImportAccess,
    NoEmptyBlock,
    NoEmptyBlockStatements,
    NoEmptyCharacterClassInRegex,
    NoEmptyInterface,
    NoEmptyPattern,
    NoEmptyTypeParameters,
    NoEnum,
    NoEvolvingTypes,
    NoExcessiveCognitiveComplexity,
    NoExcessiveLinesPerFunction,
    NoExcessiveNestedTestSuites,
    NoExplicitAny,
    NoExportedImports,
    NoExportsInTest,
    NoExtraBooleanCast,
    NoExtraNonNullAssertion,
    NoFallthroughSwitchClause,
    NoFlatMapIdentity,
    NoFloatingPromises,
    NoFocusedTests,
    NoForEach,
    NoFunctionAssign,
    NoGlobalAssign,
    NoGlobalDirnameFilename,
    NoGlobalEval,
    NoGlobalIsFinite,
    NoGlobalIsNan,
    NoGlobalObjectCalls,
    NoHeadElement,
    NoHeadImportInDocument,
    NoHeaderScope,
    NoImgElement,
    NoImplicitAnyLet,
    NoImplicitBoolean,
    NoImplicitCoercions,
    NoImportAssign,
    NoImportCycles,
    NoImportantInKeyframe,
    NoImportantStyles,
    NoInferrableTypes,
    NoInnerDeclarations,
    NoInteractiveElementToNoninteractiveRole,
    NoInvalidBuiltinInstantiation,
    NoInvalidConstructorSuper,
    NoInvalidDirectionInLinearGradient,
    NoInvalidGridAreas,
    NoInvalidPositionAtImportRule,
    NoInvalidUseBeforeDeclaration,
    NoIrregularWhitespace,
    NoJsxLiterals,
    NoLabelVar,
    NoLabelWithoutControl,
    NoMagicNumbers,
    NoMisleadingCharacterClass,
    NoMisleadingInstantiator,
    NoMisplacedAssertion,
    NoMisrefactoredShorthandAssign,
    NoMissingVarFunction,
    NoMisusedPromises,
    NoNamespace,
    NoNamespaceImport,
    NoNegationElse,
    NoNestedComponentDefinitions,
    NoNestedTernary,
    NoNextAsyncClientComponent,
    NoNodejsModules,
    NoNonNullAssertedOptionalChain,
    NoNonNullAssertion,
    NoNoninteractiveElementInteractions,
    NoNoninteractiveElementToInteractiveRole,
    NoNoninteractiveTabindex,
    NoNonoctalDecimalEscape,
    NoOctalEscape,
    NoParameterAssign,
    NoParameterProperties,
    NoPositiveTabindex,
    NoPrecisionLoss,
    NoPrivateImports,
    NoProcessEnv,
    NoProcessGlobal,
    NoPrototypeBuiltins,
    NoQuickfixBiome,
    NoQwikUseVisibleTask,
    NoReExportAll,
    NoReactPropAssignments,
    NoReactSpecificProps,
    NoRedeclare,
    NoRedundantAlt,
    NoRedundantRoles,
    NoRedundantUseStrict,
    NoRenderReturnValue,
    NoRestrictedElements,
    NoRestrictedGlobals,
    NoRestrictedImports,
    NoRestrictedTypes,
    NoSecrets,
    NoSelfAssign,
    NoSelfCompare,
    NoSetterReturn,
    NoShadow,
    NoShadowRestrictedNames,
    NoShorthandPropertyOverrides,
    NoShoutyConstants,
    NoSkippedTests,
    NoSolidDestructuredProps,
    NoSparseArray,
    NoStaticElementInteractions,
    NoStaticOnlyClass,
    NoStringCaseMismatch,
    NoSubstr,
    NoSuspiciousSemicolonInJsx,
    NoSvgWithoutTitle,
    NoSwitchDeclarations,
    NoTemplateCurlyInString,
    NoThenProperty,
    NoThisInStatic,
    NoTsIgnore,
    NoUnassignedVariables,
    NoUndeclaredDependencies,
    NoUndeclaredVariables,
    NoUnknownAtRules,
    NoUnknownFunction,
    NoUnknownMediaFeatureName,
    NoUnknownProperty,
    NoUnknownPseudoClass,
    NoUnknownPseudoElement,
    NoUnknownTypeSelector,
    NoUnknownUnit,
    NoUnmatchableAnbSelector,
    NoUnnecessaryConditions,
    NoUnreachable,
    NoUnreachableSuper,
    NoUnresolvedImports,
    NoUnsafeDeclarationMerging,
    NoUnsafeFinally,
    NoUnsafeNegation,
    NoUnsafeOptionalChaining,
    NoUnusedFunctionParameters,
    NoUnusedImports,
    NoUnusedLabels,
    NoUnusedPrivateClassMembers,
    NoUnusedTemplateLiteral,
    NoUnusedVariables,
    NoUnwantedPolyfillio,
    NoUselessCatch,
    NoUselessConstructor,
    NoUselessContinue,
    NoUselessElse,
    NoUselessEmptyExport,
    NoUselessEscapeInRegex,
    NoUselessEscapeInString,
    NoUselessFragments,
    NoUselessLabel,
    NoUselessLoneBlockStatements,
    NoUselessRegexBackrefs,
    NoUselessRename,
    NoUselessStringConcat,
    NoUselessStringRaw,
    NoUselessSwitchCase,
    NoUselessTernary,
    NoUselessThisAlias,
    NoUselessTypeConstraint,
    NoUselessUndefined,
    NoUselessUndefinedInitialization,
    NoValueAtRule,
    NoVar,
    NoVoid,
    NoVoidElementsWithChildren,
    NoVoidTypeReturn,
    NoVueDataObjectDeclaration,
    NoVueReservedKeys,
    NoVueReservedProps,
    NoWith,
    NoYodaExpression,
    UseAdjacentOverloadSignatures,
    UseAltText,
    UseAnchorContent,
    UseAnchorHref,
    UseAriaActivedescendantWithTabindex,
    UseAriaPropsForRole,
    UseAriaPropsSupportedByRole,
    UseArrayLiterals,
    UseArrowFunction,
    UseAsConstAssertion,
    UseAtIndex,
    UseAwait,
    UseBiomeIgnoreFolder,
    UseBlockStatements,
    UseButtonType,
    UseCollapsedElseIf,
    UseCollapsedIf,
    UseComponentExportOnlyModules,
    UseConsistentArrayType,
    UseConsistentBuiltinInstantiation,
    UseConsistentCurlyBraces,
    UseConsistentMemberAccessibility,
    UseConsistentObjectDefinitions,
    UseConsistentTypeDefinitions,
    UseConst,
    UseDateNow,
    UseDefaultParameterLast,
    UseDefaultSwitchClause,
    UseDefaultSwitchClauseLast,
    UseDeprecatedReason,
    UseEnumInitializers,
    UseErrorMessage,
    UseExhaustiveDependencies,
    UseExhaustiveSwitchCases,
    UseExplicitLengthCheck,
    UseExplicitType,
    UseExponentiationOperator,
    UseExportType,
    UseExportsLast,
    UseFilenamingConvention,
    UseFlatMap,
    UseFocusableInteractive,
    UseForOf,
    UseFragmentSyntax,
    UseGenericFontNames,
    UseGetterReturn,
    UseGoogleFontDisplay,
    UseGoogleFontPreconnect,
    UseGraphqlNamedOperations,
    UseGraphqlNamingConvention,
    UseGroupedAccessorPairs,
    UseGuardForIn,
    UseHeadingContent,
    UseHookAtTopLevel,
    UseHtmlLang,
    UseIframeTitle,
    UseImageSize,
    UseImportExtensions,
    UseImportType,
    UseIndexOf,
    UseIsArray,
    UseIsNan,
    UseIterableCallbackReturn,
    UseJsonImportAttributes,
    UseJsxKeyInIterable,
    UseKeyWithClickEvents,
    UseKeyWithMouseEvents,
    UseLiteralEnumMembers,
    UseLiteralKeys,
    UseMaxParams,
    UseMediaCaption,
    UseNamespaceKeyword,
    UseNamingConvention,
    UseNodeAssertStrict,
    UseNodejsImportProtocol,
    UseNumberNamespace,
    UseNumberToFixedDigitsArgument,
    UseNumericLiterals,
    UseNumericSeparators,
    UseObjectSpread,
    UseOptionalChain,
    UseParseIntRadix,
    UseQwikClasslist,
    UseReactFunctionComponents,
    UseReadonlyClassProperties,
    UseRegexLiterals,
    UseSelfClosingElements,
    UseSemanticElements,
    UseShorthandAssign,
    UseShorthandFunctionType,
    UseSimpleNumberKeys,
    UseSimplifiedLogicExpression,
    UseSingleJsDocAsterisk,
    UseSingleVarDeclarator,
    UseSolidForComponent,
    UseSortedClasses,
    UseStaticResponseMethods,
    UseStrictMode,
    UseSymbolDescription,
    UseTemplate,
    UseThrowNewError,
    UseThrowOnlyError,
    UseTopLevelRegex,
    UseTrimStartEnd,
    UseUnifiedTypeSignatures,
    UseUniqueElementIds,
    UseValidAnchor,
    UseValidAriaProps,
    UseValidAriaRole,
    UseValidAriaValues,
    UseValidAutocomplete,
    UseValidForDirection,
    UseValidLang,
    UseValidTypeof,
    UseWhile,
    UseYield,
}
impl RuleName {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAccessKey => "noAccessKey",
            Self::NoAccumulatingSpread => "noAccumulatingSpread",
            Self::NoAdjacentSpacesInRegex => "noAdjacentSpacesInRegex",
            Self::NoAlert => "noAlert",
            Self::NoApproximativeNumericConstant => "noApproximativeNumericConstant",
            Self::NoArguments => "noArguments",
            Self::NoAriaHiddenOnFocusable => "noAriaHiddenOnFocusable",
            Self::NoAriaUnsupportedElements => "noAriaUnsupportedElements",
            Self::NoArrayIndexKey => "noArrayIndexKey",
            Self::NoAssignInExpressions => "noAssignInExpressions",
            Self::NoAsyncPromiseExecutor => "noAsyncPromiseExecutor",
            Self::NoAutofocus => "noAutofocus",
            Self::NoAwaitInLoops => "noAwaitInLoops",
            Self::NoBannedTypes => "noBannedTypes",
            Self::NoBarrelFile => "noBarrelFile",
            Self::NoBiomeFirstException => "noBiomeFirstException",
            Self::NoBitwiseOperators => "noBitwiseOperators",
            Self::NoBlankTarget => "noBlankTarget",
            Self::NoCatchAssign => "noCatchAssign",
            Self::NoChildrenProp => "noChildrenProp",
            Self::NoClassAssign => "noClassAssign",
            Self::NoCommaOperator => "noCommaOperator",
            Self::NoCommentText => "noCommentText",
            Self::NoCommonJs => "noCommonJs",
            Self::NoCompareNegZero => "noCompareNegZero",
            Self::NoConfusingLabels => "noConfusingLabels",
            Self::NoConfusingVoidType => "noConfusingVoidType",
            Self::NoConsole => "noConsole",
            Self::NoConstAssign => "noConstAssign",
            Self::NoConstEnum => "noConstEnum",
            Self::NoConstantBinaryExpressions => "noConstantBinaryExpressions",
            Self::NoConstantCondition => "noConstantCondition",
            Self::NoConstantMathMinMaxClamp => "noConstantMathMinMaxClamp",
            Self::NoConstructorReturn => "noConstructorReturn",
            Self::NoControlCharactersInRegex => "noControlCharactersInRegex",
            Self::NoDangerouslySetInnerHtml => "noDangerouslySetInnerHtml",
            Self::NoDangerouslySetInnerHtmlWithChildren => "noDangerouslySetInnerHtmlWithChildren",
            Self::NoDebugger => "noDebugger",
            Self::NoDefaultExport => "noDefaultExport",
            Self::NoDelete => "noDelete",
            Self::NoDescendingSpecificity => "noDescendingSpecificity",
            Self::NoDistractingElements => "noDistractingElements",
            Self::NoDocumentCookie => "noDocumentCookie",
            Self::NoDocumentImportInPage => "noDocumentImportInPage",
            Self::NoDoneCallback => "noDoneCallback",
            Self::NoDoubleEquals => "noDoubleEquals",
            Self::NoDuplicateAtImportRules => "noDuplicateAtImportRules",
            Self::NoDuplicateCase => "noDuplicateCase",
            Self::NoDuplicateClassMembers => "noDuplicateClassMembers",
            Self::NoDuplicateCustomProperties => "noDuplicateCustomProperties",
            Self::NoDuplicateElseIf => "noDuplicateElseIf",
            Self::NoDuplicateFields => "noDuplicateFields",
            Self::NoDuplicateFontNames => "noDuplicateFontNames",
            Self::NoDuplicateJsxProps => "noDuplicateJsxProps",
            Self::NoDuplicateObjectKeys => "noDuplicateObjectKeys",
            Self::NoDuplicateParameters => "noDuplicateParameters",
            Self::NoDuplicateProperties => "noDuplicateProperties",
            Self::NoDuplicateSelectorsKeyframeBlock => "noDuplicateSelectorsKeyframeBlock",
            Self::NoDuplicateTestHooks => "noDuplicateTestHooks",
            Self::NoDynamicNamespaceImportAccess => "noDynamicNamespaceImportAccess",
            Self::NoEmptyBlock => "noEmptyBlock",
            Self::NoEmptyBlockStatements => "noEmptyBlockStatements",
            Self::NoEmptyCharacterClassInRegex => "noEmptyCharacterClassInRegex",
            Self::NoEmptyInterface => "noEmptyInterface",
            Self::NoEmptyPattern => "noEmptyPattern",
            Self::NoEmptyTypeParameters => "noEmptyTypeParameters",
            Self::NoEnum => "noEnum",
            Self::NoEvolvingTypes => "noEvolvingTypes",
            Self::NoExcessiveCognitiveComplexity => "noExcessiveCognitiveComplexity",
            Self::NoExcessiveLinesPerFunction => "noExcessiveLinesPerFunction",
            Self::NoExcessiveNestedTestSuites => "noExcessiveNestedTestSuites",
            Self::NoExplicitAny => "noExplicitAny",
            Self::NoExportedImports => "noExportedImports",
            Self::NoExportsInTest => "noExportsInTest",
            Self::NoExtraBooleanCast => "noExtraBooleanCast",
            Self::NoExtraNonNullAssertion => "noExtraNonNullAssertion",
            Self::NoFallthroughSwitchClause => "noFallthroughSwitchClause",
            Self::NoFlatMapIdentity => "noFlatMapIdentity",
            Self::NoFloatingPromises => "noFloatingPromises",
            Self::NoFocusedTests => "noFocusedTests",
            Self::NoForEach => "noForEach",
            Self::NoFunctionAssign => "noFunctionAssign",
            Self::NoGlobalAssign => "noGlobalAssign",
            Self::NoGlobalDirnameFilename => "noGlobalDirnameFilename",
            Self::NoGlobalEval => "noGlobalEval",
            Self::NoGlobalIsFinite => "noGlobalIsFinite",
            Self::NoGlobalIsNan => "noGlobalIsNan",
            Self::NoGlobalObjectCalls => "noGlobalObjectCalls",
            Self::NoHeadElement => "noHeadElement",
            Self::NoHeadImportInDocument => "noHeadImportInDocument",
            Self::NoHeaderScope => "noHeaderScope",
            Self::NoImgElement => "noImgElement",
            Self::NoImplicitAnyLet => "noImplicitAnyLet",
            Self::NoImplicitBoolean => "noImplicitBoolean",
            Self::NoImplicitCoercions => "noImplicitCoercions",
            Self::NoImportAssign => "noImportAssign",
            Self::NoImportCycles => "noImportCycles",
            Self::NoImportantInKeyframe => "noImportantInKeyframe",
            Self::NoImportantStyles => "noImportantStyles",
            Self::NoInferrableTypes => "noInferrableTypes",
            Self::NoInnerDeclarations => "noInnerDeclarations",
            Self::NoInteractiveElementToNoninteractiveRole => {
                "noInteractiveElementToNoninteractiveRole"
            }
            Self::NoInvalidBuiltinInstantiation => "noInvalidBuiltinInstantiation",
            Self::NoInvalidConstructorSuper => "noInvalidConstructorSuper",
            Self::NoInvalidDirectionInLinearGradient => "noInvalidDirectionInLinearGradient",
            Self::NoInvalidGridAreas => "noInvalidGridAreas",
            Self::NoInvalidPositionAtImportRule => "noInvalidPositionAtImportRule",
            Self::NoInvalidUseBeforeDeclaration => "noInvalidUseBeforeDeclaration",
            Self::NoIrregularWhitespace => "noIrregularWhitespace",
            Self::NoJsxLiterals => "noJsxLiterals",
            Self::NoLabelVar => "noLabelVar",
            Self::NoLabelWithoutControl => "noLabelWithoutControl",
            Self::NoMagicNumbers => "noMagicNumbers",
            Self::NoMisleadingCharacterClass => "noMisleadingCharacterClass",
            Self::NoMisleadingInstantiator => "noMisleadingInstantiator",
            Self::NoMisplacedAssertion => "noMisplacedAssertion",
            Self::NoMisrefactoredShorthandAssign => "noMisrefactoredShorthandAssign",
            Self::NoMissingVarFunction => "noMissingVarFunction",
            Self::NoMisusedPromises => "noMisusedPromises",
            Self::NoNamespace => "noNamespace",
            Self::NoNamespaceImport => "noNamespaceImport",
            Self::NoNegationElse => "noNegationElse",
            Self::NoNestedComponentDefinitions => "noNestedComponentDefinitions",
            Self::NoNestedTernary => "noNestedTernary",
            Self::NoNextAsyncClientComponent => "noNextAsyncClientComponent",
            Self::NoNodejsModules => "noNodejsModules",
            Self::NoNonNullAssertedOptionalChain => "noNonNullAssertedOptionalChain",
            Self::NoNonNullAssertion => "noNonNullAssertion",
            Self::NoNoninteractiveElementInteractions => "noNoninteractiveElementInteractions",
            Self::NoNoninteractiveElementToInteractiveRole => {
                "noNoninteractiveElementToInteractiveRole"
            }
            Self::NoNoninteractiveTabindex => "noNoninteractiveTabindex",
            Self::NoNonoctalDecimalEscape => "noNonoctalDecimalEscape",
            Self::NoOctalEscape => "noOctalEscape",
            Self::NoParameterAssign => "noParameterAssign",
            Self::NoParameterProperties => "noParameterProperties",
            Self::NoPositiveTabindex => "noPositiveTabindex",
            Self::NoPrecisionLoss => "noPrecisionLoss",
            Self::NoPrivateImports => "noPrivateImports",
            Self::NoProcessEnv => "noProcessEnv",
            Self::NoProcessGlobal => "noProcessGlobal",
            Self::NoPrototypeBuiltins => "noPrototypeBuiltins",
            Self::NoQuickfixBiome => "noQuickfixBiome",
            Self::NoQwikUseVisibleTask => "noQwikUseVisibleTask",
            Self::NoReExportAll => "noReExportAll",
            Self::NoReactPropAssignments => "noReactPropAssignments",
            Self::NoReactSpecificProps => "noReactSpecificProps",
            Self::NoRedeclare => "noRedeclare",
            Self::NoRedundantAlt => "noRedundantAlt",
            Self::NoRedundantRoles => "noRedundantRoles",
            Self::NoRedundantUseStrict => "noRedundantUseStrict",
            Self::NoRenderReturnValue => "noRenderReturnValue",
            Self::NoRestrictedElements => "noRestrictedElements",
            Self::NoRestrictedGlobals => "noRestrictedGlobals",
            Self::NoRestrictedImports => "noRestrictedImports",
            Self::NoRestrictedTypes => "noRestrictedTypes",
            Self::NoSecrets => "noSecrets",
            Self::NoSelfAssign => "noSelfAssign",
            Self::NoSelfCompare => "noSelfCompare",
            Self::NoSetterReturn => "noSetterReturn",
            Self::NoShadow => "noShadow",
            Self::NoShadowRestrictedNames => "noShadowRestrictedNames",
            Self::NoShorthandPropertyOverrides => "noShorthandPropertyOverrides",
            Self::NoShoutyConstants => "noShoutyConstants",
            Self::NoSkippedTests => "noSkippedTests",
            Self::NoSolidDestructuredProps => "noSolidDestructuredProps",
            Self::NoSparseArray => "noSparseArray",
            Self::NoStaticElementInteractions => "noStaticElementInteractions",
            Self::NoStaticOnlyClass => "noStaticOnlyClass",
            Self::NoStringCaseMismatch => "noStringCaseMismatch",
            Self::NoSubstr => "noSubstr",
            Self::NoSuspiciousSemicolonInJsx => "noSuspiciousSemicolonInJsx",
            Self::NoSvgWithoutTitle => "noSvgWithoutTitle",
            Self::NoSwitchDeclarations => "noSwitchDeclarations",
            Self::NoTemplateCurlyInString => "noTemplateCurlyInString",
            Self::NoThenProperty => "noThenProperty",
            Self::NoThisInStatic => "noThisInStatic",
            Self::NoTsIgnore => "noTsIgnore",
            Self::NoUnassignedVariables => "noUnassignedVariables",
            Self::NoUndeclaredDependencies => "noUndeclaredDependencies",
            Self::NoUndeclaredVariables => "noUndeclaredVariables",
            Self::NoUnknownAtRules => "noUnknownAtRules",
            Self::NoUnknownFunction => "noUnknownFunction",
            Self::NoUnknownMediaFeatureName => "noUnknownMediaFeatureName",
            Self::NoUnknownProperty => "noUnknownProperty",
            Self::NoUnknownPseudoClass => "noUnknownPseudoClass",
            Self::NoUnknownPseudoElement => "noUnknownPseudoElement",
            Self::NoUnknownTypeSelector => "noUnknownTypeSelector",
            Self::NoUnknownUnit => "noUnknownUnit",
            Self::NoUnmatchableAnbSelector => "noUnmatchableAnbSelector",
            Self::NoUnnecessaryConditions => "noUnnecessaryConditions",
            Self::NoUnreachable => "noUnreachable",
            Self::NoUnreachableSuper => "noUnreachableSuper",
            Self::NoUnresolvedImports => "noUnresolvedImports",
            Self::NoUnsafeDeclarationMerging => "noUnsafeDeclarationMerging",
            Self::NoUnsafeFinally => "noUnsafeFinally",
            Self::NoUnsafeNegation => "noUnsafeNegation",
            Self::NoUnsafeOptionalChaining => "noUnsafeOptionalChaining",
            Self::NoUnusedFunctionParameters => "noUnusedFunctionParameters",
            Self::NoUnusedImports => "noUnusedImports",
            Self::NoUnusedLabels => "noUnusedLabels",
            Self::NoUnusedPrivateClassMembers => "noUnusedPrivateClassMembers",
            Self::NoUnusedTemplateLiteral => "noUnusedTemplateLiteral",
            Self::NoUnusedVariables => "noUnusedVariables",
            Self::NoUnwantedPolyfillio => "noUnwantedPolyfillio",
            Self::NoUselessCatch => "noUselessCatch",
            Self::NoUselessConstructor => "noUselessConstructor",
            Self::NoUselessContinue => "noUselessContinue",
            Self::NoUselessElse => "noUselessElse",
            Self::NoUselessEmptyExport => "noUselessEmptyExport",
            Self::NoUselessEscapeInRegex => "noUselessEscapeInRegex",
            Self::NoUselessEscapeInString => "noUselessEscapeInString",
            Self::NoUselessFragments => "noUselessFragments",
            Self::NoUselessLabel => "noUselessLabel",
            Self::NoUselessLoneBlockStatements => "noUselessLoneBlockStatements",
            Self::NoUselessRegexBackrefs => "noUselessRegexBackrefs",
            Self::NoUselessRename => "noUselessRename",
            Self::NoUselessStringConcat => "noUselessStringConcat",
            Self::NoUselessStringRaw => "noUselessStringRaw",
            Self::NoUselessSwitchCase => "noUselessSwitchCase",
            Self::NoUselessTernary => "noUselessTernary",
            Self::NoUselessThisAlias => "noUselessThisAlias",
            Self::NoUselessTypeConstraint => "noUselessTypeConstraint",
            Self::NoUselessUndefined => "noUselessUndefined",
            Self::NoUselessUndefinedInitialization => "noUselessUndefinedInitialization",
            Self::NoValueAtRule => "noValueAtRule",
            Self::NoVar => "noVar",
            Self::NoVoid => "noVoid",
            Self::NoVoidElementsWithChildren => "noVoidElementsWithChildren",
            Self::NoVoidTypeReturn => "noVoidTypeReturn",
            Self::NoVueDataObjectDeclaration => "noVueDataObjectDeclaration",
            Self::NoVueReservedKeys => "noVueReservedKeys",
            Self::NoVueReservedProps => "noVueReservedProps",
            Self::NoWith => "noWith",
            Self::NoYodaExpression => "noYodaExpression",
            Self::UseAdjacentOverloadSignatures => "useAdjacentOverloadSignatures",
            Self::UseAltText => "useAltText",
            Self::UseAnchorContent => "useAnchorContent",
            Self::UseAnchorHref => "useAnchorHref",
            Self::UseAriaActivedescendantWithTabindex => "useAriaActivedescendantWithTabindex",
            Self::UseAriaPropsForRole => "useAriaPropsForRole",
            Self::UseAriaPropsSupportedByRole => "useAriaPropsSupportedByRole",
            Self::UseArrayLiterals => "useArrayLiterals",
            Self::UseArrowFunction => "useArrowFunction",
            Self::UseAsConstAssertion => "useAsConstAssertion",
            Self::UseAtIndex => "useAtIndex",
            Self::UseAwait => "useAwait",
            Self::UseBiomeIgnoreFolder => "useBiomeIgnoreFolder",
            Self::UseBlockStatements => "useBlockStatements",
            Self::UseButtonType => "useButtonType",
            Self::UseCollapsedElseIf => "useCollapsedElseIf",
            Self::UseCollapsedIf => "useCollapsedIf",
            Self::UseComponentExportOnlyModules => "useComponentExportOnlyModules",
            Self::UseConsistentArrayType => "useConsistentArrayType",
            Self::UseConsistentBuiltinInstantiation => "useConsistentBuiltinInstantiation",
            Self::UseConsistentCurlyBraces => "useConsistentCurlyBraces",
            Self::UseConsistentMemberAccessibility => "useConsistentMemberAccessibility",
            Self::UseConsistentObjectDefinitions => "useConsistentObjectDefinitions",
            Self::UseConsistentTypeDefinitions => "useConsistentTypeDefinitions",
            Self::UseConst => "useConst",
            Self::UseDateNow => "useDateNow",
            Self::UseDefaultParameterLast => "useDefaultParameterLast",
            Self::UseDefaultSwitchClause => "useDefaultSwitchClause",
            Self::UseDefaultSwitchClauseLast => "useDefaultSwitchClauseLast",
            Self::UseDeprecatedReason => "useDeprecatedReason",
            Self::UseEnumInitializers => "useEnumInitializers",
            Self::UseErrorMessage => "useErrorMessage",
            Self::UseExhaustiveDependencies => "useExhaustiveDependencies",
            Self::UseExhaustiveSwitchCases => "useExhaustiveSwitchCases",
            Self::UseExplicitLengthCheck => "useExplicitLengthCheck",
            Self::UseExplicitType => "useExplicitType",
            Self::UseExponentiationOperator => "useExponentiationOperator",
            Self::UseExportType => "useExportType",
            Self::UseExportsLast => "useExportsLast",
            Self::UseFilenamingConvention => "useFilenamingConvention",
            Self::UseFlatMap => "useFlatMap",
            Self::UseFocusableInteractive => "useFocusableInteractive",
            Self::UseForOf => "useForOf",
            Self::UseFragmentSyntax => "useFragmentSyntax",
            Self::UseGenericFontNames => "useGenericFontNames",
            Self::UseGetterReturn => "useGetterReturn",
            Self::UseGoogleFontDisplay => "useGoogleFontDisplay",
            Self::UseGoogleFontPreconnect => "useGoogleFontPreconnect",
            Self::UseGraphqlNamedOperations => "useGraphqlNamedOperations",
            Self::UseGraphqlNamingConvention => "useGraphqlNamingConvention",
            Self::UseGroupedAccessorPairs => "useGroupedAccessorPairs",
            Self::UseGuardForIn => "useGuardForIn",
            Self::UseHeadingContent => "useHeadingContent",
            Self::UseHookAtTopLevel => "useHookAtTopLevel",
            Self::UseHtmlLang => "useHtmlLang",
            Self::UseIframeTitle => "useIframeTitle",
            Self::UseImageSize => "useImageSize",
            Self::UseImportExtensions => "useImportExtensions",
            Self::UseImportType => "useImportType",
            Self::UseIndexOf => "useIndexOf",
            Self::UseIsArray => "useIsArray",
            Self::UseIsNan => "useIsNan",
            Self::UseIterableCallbackReturn => "useIterableCallbackReturn",
            Self::UseJsonImportAttributes => "useJsonImportAttributes",
            Self::UseJsxKeyInIterable => "useJsxKeyInIterable",
            Self::UseKeyWithClickEvents => "useKeyWithClickEvents",
            Self::UseKeyWithMouseEvents => "useKeyWithMouseEvents",
            Self::UseLiteralEnumMembers => "useLiteralEnumMembers",
            Self::UseLiteralKeys => "useLiteralKeys",
            Self::UseMaxParams => "useMaxParams",
            Self::UseMediaCaption => "useMediaCaption",
            Self::UseNamespaceKeyword => "useNamespaceKeyword",
            Self::UseNamingConvention => "useNamingConvention",
            Self::UseNodeAssertStrict => "useNodeAssertStrict",
            Self::UseNodejsImportProtocol => "useNodejsImportProtocol",
            Self::UseNumberNamespace => "useNumberNamespace",
            Self::UseNumberToFixedDigitsArgument => "useNumberToFixedDigitsArgument",
            Self::UseNumericLiterals => "useNumericLiterals",
            Self::UseNumericSeparators => "useNumericSeparators",
            Self::UseObjectSpread => "useObjectSpread",
            Self::UseOptionalChain => "useOptionalChain",
            Self::UseParseIntRadix => "useParseIntRadix",
            Self::UseQwikClasslist => "useQwikClasslist",
            Self::UseReactFunctionComponents => "useReactFunctionComponents",
            Self::UseReadonlyClassProperties => "useReadonlyClassProperties",
            Self::UseRegexLiterals => "useRegexLiterals",
            Self::UseSelfClosingElements => "useSelfClosingElements",
            Self::UseSemanticElements => "useSemanticElements",
            Self::UseShorthandAssign => "useShorthandAssign",
            Self::UseShorthandFunctionType => "useShorthandFunctionType",
            Self::UseSimpleNumberKeys => "useSimpleNumberKeys",
            Self::UseSimplifiedLogicExpression => "useSimplifiedLogicExpression",
            Self::UseSingleJsDocAsterisk => "useSingleJsDocAsterisk",
            Self::UseSingleVarDeclarator => "useSingleVarDeclarator",
            Self::UseSolidForComponent => "useSolidForComponent",
            Self::UseSortedClasses => "useSortedClasses",
            Self::UseStaticResponseMethods => "useStaticResponseMethods",
            Self::UseStrictMode => "useStrictMode",
            Self::UseSymbolDescription => "useSymbolDescription",
            Self::UseTemplate => "useTemplate",
            Self::UseThrowNewError => "useThrowNewError",
            Self::UseThrowOnlyError => "useThrowOnlyError",
            Self::UseTopLevelRegex => "useTopLevelRegex",
            Self::UseTrimStartEnd => "useTrimStartEnd",
            Self::UseUnifiedTypeSignatures => "useUnifiedTypeSignatures",
            Self::UseUniqueElementIds => "useUniqueElementIds",
            Self::UseValidAnchor => "useValidAnchor",
            Self::UseValidAriaProps => "useValidAriaProps",
            Self::UseValidAriaRole => "useValidAriaRole",
            Self::UseValidAriaValues => "useValidAriaValues",
            Self::UseValidAutocomplete => "useValidAutocomplete",
            Self::UseValidForDirection => "useValidForDirection",
            Self::UseValidLang => "useValidLang",
            Self::UseValidTypeof => "useValidTypeof",
            Self::UseWhile => "useWhile",
            Self::UseYield => "useYield",
        }
    }
    pub const fn group(self) -> RuleGroup {
        match self {
            Self::NoAccessKey => RuleGroup::A11y,
            Self::NoAccumulatingSpread => RuleGroup::Performance,
            Self::NoAdjacentSpacesInRegex => RuleGroup::Complexity,
            Self::NoAlert => RuleGroup::Suspicious,
            Self::NoApproximativeNumericConstant => RuleGroup::Suspicious,
            Self::NoArguments => RuleGroup::Complexity,
            Self::NoAriaHiddenOnFocusable => RuleGroup::A11y,
            Self::NoAriaUnsupportedElements => RuleGroup::A11y,
            Self::NoArrayIndexKey => RuleGroup::Suspicious,
            Self::NoAssignInExpressions => RuleGroup::Suspicious,
            Self::NoAsyncPromiseExecutor => RuleGroup::Suspicious,
            Self::NoAutofocus => RuleGroup::A11y,
            Self::NoAwaitInLoops => RuleGroup::Performance,
            Self::NoBannedTypes => RuleGroup::Complexity,
            Self::NoBarrelFile => RuleGroup::Performance,
            Self::NoBiomeFirstException => RuleGroup::Suspicious,
            Self::NoBitwiseOperators => RuleGroup::Suspicious,
            Self::NoBlankTarget => RuleGroup::Security,
            Self::NoCatchAssign => RuleGroup::Suspicious,
            Self::NoChildrenProp => RuleGroup::Correctness,
            Self::NoClassAssign => RuleGroup::Suspicious,
            Self::NoCommaOperator => RuleGroup::Complexity,
            Self::NoCommentText => RuleGroup::Suspicious,
            Self::NoCommonJs => RuleGroup::Style,
            Self::NoCompareNegZero => RuleGroup::Suspicious,
            Self::NoConfusingLabels => RuleGroup::Suspicious,
            Self::NoConfusingVoidType => RuleGroup::Suspicious,
            Self::NoConsole => RuleGroup::Suspicious,
            Self::NoConstAssign => RuleGroup::Correctness,
            Self::NoConstEnum => RuleGroup::Suspicious,
            Self::NoConstantBinaryExpressions => RuleGroup::Suspicious,
            Self::NoConstantCondition => RuleGroup::Correctness,
            Self::NoConstantMathMinMaxClamp => RuleGroup::Correctness,
            Self::NoConstructorReturn => RuleGroup::Correctness,
            Self::NoControlCharactersInRegex => RuleGroup::Suspicious,
            Self::NoDangerouslySetInnerHtml => RuleGroup::Security,
            Self::NoDangerouslySetInnerHtmlWithChildren => RuleGroup::Security,
            Self::NoDebugger => RuleGroup::Suspicious,
            Self::NoDefaultExport => RuleGroup::Style,
            Self::NoDelete => RuleGroup::Performance,
            Self::NoDescendingSpecificity => RuleGroup::Style,
            Self::NoDistractingElements => RuleGroup::A11y,
            Self::NoDocumentCookie => RuleGroup::Suspicious,
            Self::NoDocumentImportInPage => RuleGroup::Suspicious,
            Self::NoDoneCallback => RuleGroup::Style,
            Self::NoDoubleEquals => RuleGroup::Suspicious,
            Self::NoDuplicateAtImportRules => RuleGroup::Suspicious,
            Self::NoDuplicateCase => RuleGroup::Suspicious,
            Self::NoDuplicateClassMembers => RuleGroup::Suspicious,
            Self::NoDuplicateCustomProperties => RuleGroup::Suspicious,
            Self::NoDuplicateElseIf => RuleGroup::Suspicious,
            Self::NoDuplicateFields => RuleGroup::Suspicious,
            Self::NoDuplicateFontNames => RuleGroup::Suspicious,
            Self::NoDuplicateJsxProps => RuleGroup::Suspicious,
            Self::NoDuplicateObjectKeys => RuleGroup::Suspicious,
            Self::NoDuplicateParameters => RuleGroup::Suspicious,
            Self::NoDuplicateProperties => RuleGroup::Suspicious,
            Self::NoDuplicateSelectorsKeyframeBlock => RuleGroup::Suspicious,
            Self::NoDuplicateTestHooks => RuleGroup::Suspicious,
            Self::NoDynamicNamespaceImportAccess => RuleGroup::Performance,
            Self::NoEmptyBlock => RuleGroup::Suspicious,
            Self::NoEmptyBlockStatements => RuleGroup::Suspicious,
            Self::NoEmptyCharacterClassInRegex => RuleGroup::Correctness,
            Self::NoEmptyInterface => RuleGroup::Suspicious,
            Self::NoEmptyPattern => RuleGroup::Correctness,
            Self::NoEmptyTypeParameters => RuleGroup::Complexity,
            Self::NoEnum => RuleGroup::Style,
            Self::NoEvolvingTypes => RuleGroup::Suspicious,
            Self::NoExcessiveCognitiveComplexity => RuleGroup::Complexity,
            Self::NoExcessiveLinesPerFunction => RuleGroup::Complexity,
            Self::NoExcessiveNestedTestSuites => RuleGroup::Complexity,
            Self::NoExplicitAny => RuleGroup::Suspicious,
            Self::NoExportedImports => RuleGroup::Style,
            Self::NoExportsInTest => RuleGroup::Suspicious,
            Self::NoExtraBooleanCast => RuleGroup::Complexity,
            Self::NoExtraNonNullAssertion => RuleGroup::Suspicious,
            Self::NoFallthroughSwitchClause => RuleGroup::Suspicious,
            Self::NoFlatMapIdentity => RuleGroup::Complexity,
            Self::NoFloatingPromises => RuleGroup::Nursery,
            Self::NoFocusedTests => RuleGroup::Suspicious,
            Self::NoForEach => RuleGroup::Complexity,
            Self::NoFunctionAssign => RuleGroup::Suspicious,
            Self::NoGlobalAssign => RuleGroup::Suspicious,
            Self::NoGlobalDirnameFilename => RuleGroup::Correctness,
            Self::NoGlobalEval => RuleGroup::Security,
            Self::NoGlobalIsFinite => RuleGroup::Suspicious,
            Self::NoGlobalIsNan => RuleGroup::Suspicious,
            Self::NoGlobalObjectCalls => RuleGroup::Correctness,
            Self::NoHeadElement => RuleGroup::Style,
            Self::NoHeadImportInDocument => RuleGroup::Suspicious,
            Self::NoHeaderScope => RuleGroup::A11y,
            Self::NoImgElement => RuleGroup::Performance,
            Self::NoImplicitAnyLet => RuleGroup::Suspicious,
            Self::NoImplicitBoolean => RuleGroup::Style,
            Self::NoImplicitCoercions => RuleGroup::Complexity,
            Self::NoImportAssign => RuleGroup::Suspicious,
            Self::NoImportCycles => RuleGroup::Nursery,
            Self::NoImportantInKeyframe => RuleGroup::Suspicious,
            Self::NoImportantStyles => RuleGroup::Complexity,
            Self::NoInferrableTypes => RuleGroup::Style,
            Self::NoInnerDeclarations => RuleGroup::Correctness,
            Self::NoInteractiveElementToNoninteractiveRole => RuleGroup::A11y,
            Self::NoInvalidBuiltinInstantiation => RuleGroup::Correctness,
            Self::NoInvalidConstructorSuper => RuleGroup::Correctness,
            Self::NoInvalidDirectionInLinearGradient => RuleGroup::Correctness,
            Self::NoInvalidGridAreas => RuleGroup::Correctness,
            Self::NoInvalidPositionAtImportRule => RuleGroup::Correctness,
            Self::NoInvalidUseBeforeDeclaration => RuleGroup::Correctness,
            Self::NoIrregularWhitespace => RuleGroup::Suspicious,
            Self::NoJsxLiterals => RuleGroup::Nursery,
            Self::NoLabelVar => RuleGroup::Suspicious,
            Self::NoLabelWithoutControl => RuleGroup::A11y,
            Self::NoMagicNumbers => RuleGroup::Style,
            Self::NoMisleadingCharacterClass => RuleGroup::Suspicious,
            Self::NoMisleadingInstantiator => RuleGroup::Suspicious,
            Self::NoMisplacedAssertion => RuleGroup::Suspicious,
            Self::NoMisrefactoredShorthandAssign => RuleGroup::Suspicious,
            Self::NoMissingVarFunction => RuleGroup::Correctness,
            Self::NoMisusedPromises => RuleGroup::Nursery,
            Self::NoNamespace => RuleGroup::Style,
            Self::NoNamespaceImport => RuleGroup::Performance,
            Self::NoNegationElse => RuleGroup::Style,
            Self::NoNestedComponentDefinitions => RuleGroup::Correctness,
            Self::NoNestedTernary => RuleGroup::Style,
            Self::NoNextAsyncClientComponent => RuleGroup::Nursery,
            Self::NoNodejsModules => RuleGroup::Correctness,
            Self::NoNonNullAssertedOptionalChain => RuleGroup::Nursery,
            Self::NoNonNullAssertion => RuleGroup::Style,
            Self::NoNoninteractiveElementInteractions => RuleGroup::A11y,
            Self::NoNoninteractiveElementToInteractiveRole => RuleGroup::A11y,
            Self::NoNoninteractiveTabindex => RuleGroup::A11y,
            Self::NoNonoctalDecimalEscape => RuleGroup::Correctness,
            Self::NoOctalEscape => RuleGroup::Suspicious,
            Self::NoParameterAssign => RuleGroup::Style,
            Self::NoParameterProperties => RuleGroup::Style,
            Self::NoPositiveTabindex => RuleGroup::A11y,
            Self::NoPrecisionLoss => RuleGroup::Correctness,
            Self::NoPrivateImports => RuleGroup::Correctness,
            Self::NoProcessEnv => RuleGroup::Style,
            Self::NoProcessGlobal => RuleGroup::Correctness,
            Self::NoPrototypeBuiltins => RuleGroup::Suspicious,
            Self::NoQuickfixBiome => RuleGroup::Suspicious,
            Self::NoQwikUseVisibleTask => RuleGroup::Nursery,
            Self::NoReExportAll => RuleGroup::Performance,
            Self::NoReactPropAssignments => RuleGroup::Correctness,
            Self::NoReactSpecificProps => RuleGroup::Suspicious,
            Self::NoRedeclare => RuleGroup::Suspicious,
            Self::NoRedundantAlt => RuleGroup::A11y,
            Self::NoRedundantRoles => RuleGroup::A11y,
            Self::NoRedundantUseStrict => RuleGroup::Suspicious,
            Self::NoRenderReturnValue => RuleGroup::Correctness,
            Self::NoRestrictedElements => RuleGroup::Correctness,
            Self::NoRestrictedGlobals => RuleGroup::Style,
            Self::NoRestrictedImports => RuleGroup::Style,
            Self::NoRestrictedTypes => RuleGroup::Style,
            Self::NoSecrets => RuleGroup::Nursery,
            Self::NoSelfAssign => RuleGroup::Correctness,
            Self::NoSelfCompare => RuleGroup::Suspicious,
            Self::NoSetterReturn => RuleGroup::Correctness,
            Self::NoShadow => RuleGroup::Nursery,
            Self::NoShadowRestrictedNames => RuleGroup::Suspicious,
            Self::NoShorthandPropertyOverrides => RuleGroup::Suspicious,
            Self::NoShoutyConstants => RuleGroup::Style,
            Self::NoSkippedTests => RuleGroup::Suspicious,
            Self::NoSolidDestructuredProps => RuleGroup::Correctness,
            Self::NoSparseArray => RuleGroup::Suspicious,
            Self::NoStaticElementInteractions => RuleGroup::A11y,
            Self::NoStaticOnlyClass => RuleGroup::Complexity,
            Self::NoStringCaseMismatch => RuleGroup::Correctness,
            Self::NoSubstr => RuleGroup::Style,
            Self::NoSuspiciousSemicolonInJsx => RuleGroup::Suspicious,
            Self::NoSvgWithoutTitle => RuleGroup::A11y,
            Self::NoSwitchDeclarations => RuleGroup::Correctness,
            Self::NoTemplateCurlyInString => RuleGroup::Suspicious,
            Self::NoThenProperty => RuleGroup::Suspicious,
            Self::NoThisInStatic => RuleGroup::Complexity,
            Self::NoTsIgnore => RuleGroup::Suspicious,
            Self::NoUnassignedVariables => RuleGroup::Suspicious,
            Self::NoUndeclaredDependencies => RuleGroup::Correctness,
            Self::NoUndeclaredVariables => RuleGroup::Correctness,
            Self::NoUnknownAtRules => RuleGroup::Suspicious,
            Self::NoUnknownFunction => RuleGroup::Correctness,
            Self::NoUnknownMediaFeatureName => RuleGroup::Correctness,
            Self::NoUnknownProperty => RuleGroup::Correctness,
            Self::NoUnknownPseudoClass => RuleGroup::Correctness,
            Self::NoUnknownPseudoElement => RuleGroup::Correctness,
            Self::NoUnknownTypeSelector => RuleGroup::Correctness,
            Self::NoUnknownUnit => RuleGroup::Correctness,
            Self::NoUnmatchableAnbSelector => RuleGroup::Correctness,
            Self::NoUnnecessaryConditions => RuleGroup::Nursery,
            Self::NoUnreachable => RuleGroup::Correctness,
            Self::NoUnreachableSuper => RuleGroup::Correctness,
            Self::NoUnresolvedImports => RuleGroup::Nursery,
            Self::NoUnsafeDeclarationMerging => RuleGroup::Suspicious,
            Self::NoUnsafeFinally => RuleGroup::Correctness,
            Self::NoUnsafeNegation => RuleGroup::Suspicious,
            Self::NoUnsafeOptionalChaining => RuleGroup::Correctness,
            Self::NoUnusedFunctionParameters => RuleGroup::Correctness,
            Self::NoUnusedImports => RuleGroup::Correctness,
            Self::NoUnusedLabels => RuleGroup::Correctness,
            Self::NoUnusedPrivateClassMembers => RuleGroup::Correctness,
            Self::NoUnusedTemplateLiteral => RuleGroup::Style,
            Self::NoUnusedVariables => RuleGroup::Correctness,
            Self::NoUnwantedPolyfillio => RuleGroup::Performance,
            Self::NoUselessCatch => RuleGroup::Complexity,
            Self::NoUselessConstructor => RuleGroup::Complexity,
            Self::NoUselessContinue => RuleGroup::Complexity,
            Self::NoUselessElse => RuleGroup::Style,
            Self::NoUselessEmptyExport => RuleGroup::Complexity,
            Self::NoUselessEscapeInRegex => RuleGroup::Complexity,
            Self::NoUselessEscapeInString => RuleGroup::Suspicious,
            Self::NoUselessFragments => RuleGroup::Complexity,
            Self::NoUselessLabel => RuleGroup::Complexity,
            Self::NoUselessLoneBlockStatements => RuleGroup::Complexity,
            Self::NoUselessRegexBackrefs => RuleGroup::Suspicious,
            Self::NoUselessRename => RuleGroup::Complexity,
            Self::NoUselessStringConcat => RuleGroup::Complexity,
            Self::NoUselessStringRaw => RuleGroup::Complexity,
            Self::NoUselessSwitchCase => RuleGroup::Complexity,
            Self::NoUselessTernary => RuleGroup::Complexity,
            Self::NoUselessThisAlias => RuleGroup::Complexity,
            Self::NoUselessTypeConstraint => RuleGroup::Complexity,
            Self::NoUselessUndefined => RuleGroup::Nursery,
            Self::NoUselessUndefinedInitialization => RuleGroup::Complexity,
            Self::NoValueAtRule => RuleGroup::Style,
            Self::NoVar => RuleGroup::Suspicious,
            Self::NoVoid => RuleGroup::Complexity,
            Self::NoVoidElementsWithChildren => RuleGroup::Correctness,
            Self::NoVoidTypeReturn => RuleGroup::Correctness,
            Self::NoVueDataObjectDeclaration => RuleGroup::Nursery,
            Self::NoVueReservedKeys => RuleGroup::Nursery,
            Self::NoVueReservedProps => RuleGroup::Nursery,
            Self::NoWith => RuleGroup::Suspicious,
            Self::NoYodaExpression => RuleGroup::Style,
            Self::UseAdjacentOverloadSignatures => RuleGroup::Suspicious,
            Self::UseAltText => RuleGroup::A11y,
            Self::UseAnchorContent => RuleGroup::A11y,
            Self::UseAnchorHref => RuleGroup::Nursery,
            Self::UseAriaActivedescendantWithTabindex => RuleGroup::A11y,
            Self::UseAriaPropsForRole => RuleGroup::A11y,
            Self::UseAriaPropsSupportedByRole => RuleGroup::A11y,
            Self::UseArrayLiterals => RuleGroup::Style,
            Self::UseArrowFunction => RuleGroup::Complexity,
            Self::UseAsConstAssertion => RuleGroup::Style,
            Self::UseAtIndex => RuleGroup::Style,
            Self::UseAwait => RuleGroup::Suspicious,
            Self::UseBiomeIgnoreFolder => RuleGroup::Suspicious,
            Self::UseBlockStatements => RuleGroup::Style,
            Self::UseButtonType => RuleGroup::A11y,
            Self::UseCollapsedElseIf => RuleGroup::Style,
            Self::UseCollapsedIf => RuleGroup::Style,
            Self::UseComponentExportOnlyModules => RuleGroup::Style,
            Self::UseConsistentArrayType => RuleGroup::Style,
            Self::UseConsistentBuiltinInstantiation => RuleGroup::Style,
            Self::UseConsistentCurlyBraces => RuleGroup::Style,
            Self::UseConsistentMemberAccessibility => RuleGroup::Style,
            Self::UseConsistentObjectDefinitions => RuleGroup::Style,
            Self::UseConsistentTypeDefinitions => RuleGroup::Nursery,
            Self::UseConst => RuleGroup::Style,
            Self::UseDateNow => RuleGroup::Complexity,
            Self::UseDefaultParameterLast => RuleGroup::Style,
            Self::UseDefaultSwitchClause => RuleGroup::Style,
            Self::UseDefaultSwitchClauseLast => RuleGroup::Suspicious,
            Self::UseDeprecatedReason => RuleGroup::Style,
            Self::UseEnumInitializers => RuleGroup::Style,
            Self::UseErrorMessage => RuleGroup::Suspicious,
            Self::UseExhaustiveDependencies => RuleGroup::Correctness,
            Self::UseExhaustiveSwitchCases => RuleGroup::Nursery,
            Self::UseExplicitLengthCheck => RuleGroup::Style,
            Self::UseExplicitType => RuleGroup::Nursery,
            Self::UseExponentiationOperator => RuleGroup::Style,
            Self::UseExportType => RuleGroup::Style,
            Self::UseExportsLast => RuleGroup::Style,
            Self::UseFilenamingConvention => RuleGroup::Style,
            Self::UseFlatMap => RuleGroup::Complexity,
            Self::UseFocusableInteractive => RuleGroup::A11y,
            Self::UseForOf => RuleGroup::Style,
            Self::UseFragmentSyntax => RuleGroup::Style,
            Self::UseGenericFontNames => RuleGroup::A11y,
            Self::UseGetterReturn => RuleGroup::Suspicious,
            Self::UseGoogleFontDisplay => RuleGroup::Suspicious,
            Self::UseGoogleFontPreconnect => RuleGroup::Performance,
            Self::UseGraphqlNamedOperations => RuleGroup::Correctness,
            Self::UseGraphqlNamingConvention => RuleGroup::Style,
            Self::UseGroupedAccessorPairs => RuleGroup::Style,
            Self::UseGuardForIn => RuleGroup::Suspicious,
            Self::UseHeadingContent => RuleGroup::A11y,
            Self::UseHookAtTopLevel => RuleGroup::Correctness,
            Self::UseHtmlLang => RuleGroup::A11y,
            Self::UseIframeTitle => RuleGroup::A11y,
            Self::UseImageSize => RuleGroup::Nursery,
            Self::UseImportExtensions => RuleGroup::Correctness,
            Self::UseImportType => RuleGroup::Style,
            Self::UseIndexOf => RuleGroup::Complexity,
            Self::UseIsArray => RuleGroup::Suspicious,
            Self::UseIsNan => RuleGroup::Correctness,
            Self::UseIterableCallbackReturn => RuleGroup::Suspicious,
            Self::UseJsonImportAttributes => RuleGroup::Correctness,
            Self::UseJsxKeyInIterable => RuleGroup::Correctness,
            Self::UseKeyWithClickEvents => RuleGroup::A11y,
            Self::UseKeyWithMouseEvents => RuleGroup::A11y,
            Self::UseLiteralEnumMembers => RuleGroup::Style,
            Self::UseLiteralKeys => RuleGroup::Complexity,
            Self::UseMaxParams => RuleGroup::Nursery,
            Self::UseMediaCaption => RuleGroup::A11y,
            Self::UseNamespaceKeyword => RuleGroup::Suspicious,
            Self::UseNamingConvention => RuleGroup::Style,
            Self::UseNodeAssertStrict => RuleGroup::Style,
            Self::UseNodejsImportProtocol => RuleGroup::Style,
            Self::UseNumberNamespace => RuleGroup::Style,
            Self::UseNumberToFixedDigitsArgument => RuleGroup::Suspicious,
            Self::UseNumericLiterals => RuleGroup::Complexity,
            Self::UseNumericSeparators => RuleGroup::Style,
            Self::UseObjectSpread => RuleGroup::Style,
            Self::UseOptionalChain => RuleGroup::Complexity,
            Self::UseParseIntRadix => RuleGroup::Correctness,
            Self::UseQwikClasslist => RuleGroup::Nursery,
            Self::UseReactFunctionComponents => RuleGroup::Nursery,
            Self::UseReadonlyClassProperties => RuleGroup::Style,
            Self::UseRegexLiterals => RuleGroup::Complexity,
            Self::UseSelfClosingElements => RuleGroup::Style,
            Self::UseSemanticElements => RuleGroup::A11y,
            Self::UseShorthandAssign => RuleGroup::Style,
            Self::UseShorthandFunctionType => RuleGroup::Style,
            Self::UseSimpleNumberKeys => RuleGroup::Complexity,
            Self::UseSimplifiedLogicExpression => RuleGroup::Complexity,
            Self::UseSingleJsDocAsterisk => RuleGroup::Correctness,
            Self::UseSingleVarDeclarator => RuleGroup::Style,
            Self::UseSolidForComponent => RuleGroup::Performance,
            Self::UseSortedClasses => RuleGroup::Nursery,
            Self::UseStaticResponseMethods => RuleGroup::Suspicious,
            Self::UseStrictMode => RuleGroup::Suspicious,
            Self::UseSymbolDescription => RuleGroup::Style,
            Self::UseTemplate => RuleGroup::Style,
            Self::UseThrowNewError => RuleGroup::Style,
            Self::UseThrowOnlyError => RuleGroup::Style,
            Self::UseTopLevelRegex => RuleGroup::Performance,
            Self::UseTrimStartEnd => RuleGroup::Style,
            Self::UseUnifiedTypeSignatures => RuleGroup::Style,
            Self::UseUniqueElementIds => RuleGroup::Correctness,
            Self::UseValidAnchor => RuleGroup::A11y,
            Self::UseValidAriaProps => RuleGroup::A11y,
            Self::UseValidAriaRole => RuleGroup::A11y,
            Self::UseValidAriaValues => RuleGroup::A11y,
            Self::UseValidAutocomplete => RuleGroup::A11y,
            Self::UseValidForDirection => RuleGroup::Correctness,
            Self::UseValidLang => RuleGroup::A11y,
            Self::UseValidTypeof => RuleGroup::Correctness,
            Self::UseWhile => RuleGroup::Complexity,
            Self::UseYield => RuleGroup::Correctness,
        }
    }
}
impl std::str::FromStr for RuleName {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noAccessKey" => Ok(Self::NoAccessKey),
            "noAccumulatingSpread" => Ok(Self::NoAccumulatingSpread),
            "noAdjacentSpacesInRegex" => Ok(Self::NoAdjacentSpacesInRegex),
            "noAlert" => Ok(Self::NoAlert),
            "noApproximativeNumericConstant" => Ok(Self::NoApproximativeNumericConstant),
            "noArguments" => Ok(Self::NoArguments),
            "noAriaHiddenOnFocusable" => Ok(Self::NoAriaHiddenOnFocusable),
            "noAriaUnsupportedElements" => Ok(Self::NoAriaUnsupportedElements),
            "noArrayIndexKey" => Ok(Self::NoArrayIndexKey),
            "noAssignInExpressions" => Ok(Self::NoAssignInExpressions),
            "noAsyncPromiseExecutor" => Ok(Self::NoAsyncPromiseExecutor),
            "noAutofocus" => Ok(Self::NoAutofocus),
            "noAwaitInLoops" => Ok(Self::NoAwaitInLoops),
            "noBannedTypes" => Ok(Self::NoBannedTypes),
            "noBarrelFile" => Ok(Self::NoBarrelFile),
            "noBiomeFirstException" => Ok(Self::NoBiomeFirstException),
            "noBitwiseOperators" => Ok(Self::NoBitwiseOperators),
            "noBlankTarget" => Ok(Self::NoBlankTarget),
            "noCatchAssign" => Ok(Self::NoCatchAssign),
            "noChildrenProp" => Ok(Self::NoChildrenProp),
            "noClassAssign" => Ok(Self::NoClassAssign),
            "noCommaOperator" => Ok(Self::NoCommaOperator),
            "noCommentText" => Ok(Self::NoCommentText),
            "noCommonJs" => Ok(Self::NoCommonJs),
            "noCompareNegZero" => Ok(Self::NoCompareNegZero),
            "noConfusingLabels" => Ok(Self::NoConfusingLabels),
            "noConfusingVoidType" => Ok(Self::NoConfusingVoidType),
            "noConsole" => Ok(Self::NoConsole),
            "noConstAssign" => Ok(Self::NoConstAssign),
            "noConstEnum" => Ok(Self::NoConstEnum),
            "noConstantBinaryExpressions" => Ok(Self::NoConstantBinaryExpressions),
            "noConstantCondition" => Ok(Self::NoConstantCondition),
            "noConstantMathMinMaxClamp" => Ok(Self::NoConstantMathMinMaxClamp),
            "noConstructorReturn" => Ok(Self::NoConstructorReturn),
            "noControlCharactersInRegex" => Ok(Self::NoControlCharactersInRegex),
            "noDangerouslySetInnerHtml" => Ok(Self::NoDangerouslySetInnerHtml),
            "noDangerouslySetInnerHtmlWithChildren" => {
                Ok(Self::NoDangerouslySetInnerHtmlWithChildren)
            }
            "noDebugger" => Ok(Self::NoDebugger),
            "noDefaultExport" => Ok(Self::NoDefaultExport),
            "noDelete" => Ok(Self::NoDelete),
            "noDescendingSpecificity" => Ok(Self::NoDescendingSpecificity),
            "noDistractingElements" => Ok(Self::NoDistractingElements),
            "noDocumentCookie" => Ok(Self::NoDocumentCookie),
            "noDocumentImportInPage" => Ok(Self::NoDocumentImportInPage),
            "noDoneCallback" => Ok(Self::NoDoneCallback),
            "noDoubleEquals" => Ok(Self::NoDoubleEquals),
            "noDuplicateAtImportRules" => Ok(Self::NoDuplicateAtImportRules),
            "noDuplicateCase" => Ok(Self::NoDuplicateCase),
            "noDuplicateClassMembers" => Ok(Self::NoDuplicateClassMembers),
            "noDuplicateCustomProperties" => Ok(Self::NoDuplicateCustomProperties),
            "noDuplicateElseIf" => Ok(Self::NoDuplicateElseIf),
            "noDuplicateFields" => Ok(Self::NoDuplicateFields),
            "noDuplicateFontNames" => Ok(Self::NoDuplicateFontNames),
            "noDuplicateJsxProps" => Ok(Self::NoDuplicateJsxProps),
            "noDuplicateObjectKeys" => Ok(Self::NoDuplicateObjectKeys),
            "noDuplicateParameters" => Ok(Self::NoDuplicateParameters),
            "noDuplicateProperties" => Ok(Self::NoDuplicateProperties),
            "noDuplicateSelectorsKeyframeBlock" => Ok(Self::NoDuplicateSelectorsKeyframeBlock),
            "noDuplicateTestHooks" => Ok(Self::NoDuplicateTestHooks),
            "noDynamicNamespaceImportAccess" => Ok(Self::NoDynamicNamespaceImportAccess),
            "noEmptyBlock" => Ok(Self::NoEmptyBlock),
            "noEmptyBlockStatements" => Ok(Self::NoEmptyBlockStatements),
            "noEmptyCharacterClassInRegex" => Ok(Self::NoEmptyCharacterClassInRegex),
            "noEmptyInterface" => Ok(Self::NoEmptyInterface),
            "noEmptyPattern" => Ok(Self::NoEmptyPattern),
            "noEmptyTypeParameters" => Ok(Self::NoEmptyTypeParameters),
            "noEnum" => Ok(Self::NoEnum),
            "noEvolvingTypes" => Ok(Self::NoEvolvingTypes),
            "noExcessiveCognitiveComplexity" => Ok(Self::NoExcessiveCognitiveComplexity),
            "noExcessiveLinesPerFunction" => Ok(Self::NoExcessiveLinesPerFunction),
            "noExcessiveNestedTestSuites" => Ok(Self::NoExcessiveNestedTestSuites),
            "noExplicitAny" => Ok(Self::NoExplicitAny),
            "noExportedImports" => Ok(Self::NoExportedImports),
            "noExportsInTest" => Ok(Self::NoExportsInTest),
            "noExtraBooleanCast" => Ok(Self::NoExtraBooleanCast),
            "noExtraNonNullAssertion" => Ok(Self::NoExtraNonNullAssertion),
            "noFallthroughSwitchClause" => Ok(Self::NoFallthroughSwitchClause),
            "noFlatMapIdentity" => Ok(Self::NoFlatMapIdentity),
            "noFloatingPromises" => Ok(Self::NoFloatingPromises),
            "noFocusedTests" => Ok(Self::NoFocusedTests),
            "noForEach" => Ok(Self::NoForEach),
            "noFunctionAssign" => Ok(Self::NoFunctionAssign),
            "noGlobalAssign" => Ok(Self::NoGlobalAssign),
            "noGlobalDirnameFilename" => Ok(Self::NoGlobalDirnameFilename),
            "noGlobalEval" => Ok(Self::NoGlobalEval),
            "noGlobalIsFinite" => Ok(Self::NoGlobalIsFinite),
            "noGlobalIsNan" => Ok(Self::NoGlobalIsNan),
            "noGlobalObjectCalls" => Ok(Self::NoGlobalObjectCalls),
            "noHeadElement" => Ok(Self::NoHeadElement),
            "noHeadImportInDocument" => Ok(Self::NoHeadImportInDocument),
            "noHeaderScope" => Ok(Self::NoHeaderScope),
            "noImgElement" => Ok(Self::NoImgElement),
            "noImplicitAnyLet" => Ok(Self::NoImplicitAnyLet),
            "noImplicitBoolean" => Ok(Self::NoImplicitBoolean),
            "noImplicitCoercions" => Ok(Self::NoImplicitCoercions),
            "noImportAssign" => Ok(Self::NoImportAssign),
            "noImportCycles" => Ok(Self::NoImportCycles),
            "noImportantInKeyframe" => Ok(Self::NoImportantInKeyframe),
            "noImportantStyles" => Ok(Self::NoImportantStyles),
            "noInferrableTypes" => Ok(Self::NoInferrableTypes),
            "noInnerDeclarations" => Ok(Self::NoInnerDeclarations),
            "noInteractiveElementToNoninteractiveRole" => {
                Ok(Self::NoInteractiveElementToNoninteractiveRole)
            }
            "noInvalidBuiltinInstantiation" => Ok(Self::NoInvalidBuiltinInstantiation),
            "noInvalidConstructorSuper" => Ok(Self::NoInvalidConstructorSuper),
            "noInvalidDirectionInLinearGradient" => Ok(Self::NoInvalidDirectionInLinearGradient),
            "noInvalidGridAreas" => Ok(Self::NoInvalidGridAreas),
            "noInvalidPositionAtImportRule" => Ok(Self::NoInvalidPositionAtImportRule),
            "noInvalidUseBeforeDeclaration" => Ok(Self::NoInvalidUseBeforeDeclaration),
            "noIrregularWhitespace" => Ok(Self::NoIrregularWhitespace),
            "noJsxLiterals" => Ok(Self::NoJsxLiterals),
            "noLabelVar" => Ok(Self::NoLabelVar),
            "noLabelWithoutControl" => Ok(Self::NoLabelWithoutControl),
            "noMagicNumbers" => Ok(Self::NoMagicNumbers),
            "noMisleadingCharacterClass" => Ok(Self::NoMisleadingCharacterClass),
            "noMisleadingInstantiator" => Ok(Self::NoMisleadingInstantiator),
            "noMisplacedAssertion" => Ok(Self::NoMisplacedAssertion),
            "noMisrefactoredShorthandAssign" => Ok(Self::NoMisrefactoredShorthandAssign),
            "noMissingVarFunction" => Ok(Self::NoMissingVarFunction),
            "noMisusedPromises" => Ok(Self::NoMisusedPromises),
            "noNamespace" => Ok(Self::NoNamespace),
            "noNamespaceImport" => Ok(Self::NoNamespaceImport),
            "noNegationElse" => Ok(Self::NoNegationElse),
            "noNestedComponentDefinitions" => Ok(Self::NoNestedComponentDefinitions),
            "noNestedTernary" => Ok(Self::NoNestedTernary),
            "noNextAsyncClientComponent" => Ok(Self::NoNextAsyncClientComponent),
            "noNodejsModules" => Ok(Self::NoNodejsModules),
            "noNonNullAssertedOptionalChain" => Ok(Self::NoNonNullAssertedOptionalChain),
            "noNonNullAssertion" => Ok(Self::NoNonNullAssertion),
            "noNoninteractiveElementInteractions" => Ok(Self::NoNoninteractiveElementInteractions),
            "noNoninteractiveElementToInteractiveRole" => {
                Ok(Self::NoNoninteractiveElementToInteractiveRole)
            }
            "noNoninteractiveTabindex" => Ok(Self::NoNoninteractiveTabindex),
            "noNonoctalDecimalEscape" => Ok(Self::NoNonoctalDecimalEscape),
            "noOctalEscape" => Ok(Self::NoOctalEscape),
            "noParameterAssign" => Ok(Self::NoParameterAssign),
            "noParameterProperties" => Ok(Self::NoParameterProperties),
            "noPositiveTabindex" => Ok(Self::NoPositiveTabindex),
            "noPrecisionLoss" => Ok(Self::NoPrecisionLoss),
            "noPrivateImports" => Ok(Self::NoPrivateImports),
            "noProcessEnv" => Ok(Self::NoProcessEnv),
            "noProcessGlobal" => Ok(Self::NoProcessGlobal),
            "noPrototypeBuiltins" => Ok(Self::NoPrototypeBuiltins),
            "noQuickfixBiome" => Ok(Self::NoQuickfixBiome),
            "noQwikUseVisibleTask" => Ok(Self::NoQwikUseVisibleTask),
            "noReExportAll" => Ok(Self::NoReExportAll),
            "noReactPropAssignments" => Ok(Self::NoReactPropAssignments),
            "noReactSpecificProps" => Ok(Self::NoReactSpecificProps),
            "noRedeclare" => Ok(Self::NoRedeclare),
            "noRedundantAlt" => Ok(Self::NoRedundantAlt),
            "noRedundantRoles" => Ok(Self::NoRedundantRoles),
            "noRedundantUseStrict" => Ok(Self::NoRedundantUseStrict),
            "noRenderReturnValue" => Ok(Self::NoRenderReturnValue),
            "noRestrictedElements" => Ok(Self::NoRestrictedElements),
            "noRestrictedGlobals" => Ok(Self::NoRestrictedGlobals),
            "noRestrictedImports" => Ok(Self::NoRestrictedImports),
            "noRestrictedTypes" => Ok(Self::NoRestrictedTypes),
            "noSecrets" => Ok(Self::NoSecrets),
            "noSelfAssign" => Ok(Self::NoSelfAssign),
            "noSelfCompare" => Ok(Self::NoSelfCompare),
            "noSetterReturn" => Ok(Self::NoSetterReturn),
            "noShadow" => Ok(Self::NoShadow),
            "noShadowRestrictedNames" => Ok(Self::NoShadowRestrictedNames),
            "noShorthandPropertyOverrides" => Ok(Self::NoShorthandPropertyOverrides),
            "noShoutyConstants" => Ok(Self::NoShoutyConstants),
            "noSkippedTests" => Ok(Self::NoSkippedTests),
            "noSolidDestructuredProps" => Ok(Self::NoSolidDestructuredProps),
            "noSparseArray" => Ok(Self::NoSparseArray),
            "noStaticElementInteractions" => Ok(Self::NoStaticElementInteractions),
            "noStaticOnlyClass" => Ok(Self::NoStaticOnlyClass),
            "noStringCaseMismatch" => Ok(Self::NoStringCaseMismatch),
            "noSubstr" => Ok(Self::NoSubstr),
            "noSuspiciousSemicolonInJsx" => Ok(Self::NoSuspiciousSemicolonInJsx),
            "noSvgWithoutTitle" => Ok(Self::NoSvgWithoutTitle),
            "noSwitchDeclarations" => Ok(Self::NoSwitchDeclarations),
            "noTemplateCurlyInString" => Ok(Self::NoTemplateCurlyInString),
            "noThenProperty" => Ok(Self::NoThenProperty),
            "noThisInStatic" => Ok(Self::NoThisInStatic),
            "noTsIgnore" => Ok(Self::NoTsIgnore),
            "noUnassignedVariables" => Ok(Self::NoUnassignedVariables),
            "noUndeclaredDependencies" => Ok(Self::NoUndeclaredDependencies),
            "noUndeclaredVariables" => Ok(Self::NoUndeclaredVariables),
            "noUnknownAtRules" => Ok(Self::NoUnknownAtRules),
            "noUnknownFunction" => Ok(Self::NoUnknownFunction),
            "noUnknownMediaFeatureName" => Ok(Self::NoUnknownMediaFeatureName),
            "noUnknownProperty" => Ok(Self::NoUnknownProperty),
            "noUnknownPseudoClass" => Ok(Self::NoUnknownPseudoClass),
            "noUnknownPseudoElement" => Ok(Self::NoUnknownPseudoElement),
            "noUnknownTypeSelector" => Ok(Self::NoUnknownTypeSelector),
            "noUnknownUnit" => Ok(Self::NoUnknownUnit),
            "noUnmatchableAnbSelector" => Ok(Self::NoUnmatchableAnbSelector),
            "noUnnecessaryConditions" => Ok(Self::NoUnnecessaryConditions),
            "noUnreachable" => Ok(Self::NoUnreachable),
            "noUnreachableSuper" => Ok(Self::NoUnreachableSuper),
            "noUnresolvedImports" => Ok(Self::NoUnresolvedImports),
            "noUnsafeDeclarationMerging" => Ok(Self::NoUnsafeDeclarationMerging),
            "noUnsafeFinally" => Ok(Self::NoUnsafeFinally),
            "noUnsafeNegation" => Ok(Self::NoUnsafeNegation),
            "noUnsafeOptionalChaining" => Ok(Self::NoUnsafeOptionalChaining),
            "noUnusedFunctionParameters" => Ok(Self::NoUnusedFunctionParameters),
            "noUnusedImports" => Ok(Self::NoUnusedImports),
            "noUnusedLabels" => Ok(Self::NoUnusedLabels),
            "noUnusedPrivateClassMembers" => Ok(Self::NoUnusedPrivateClassMembers),
            "noUnusedTemplateLiteral" => Ok(Self::NoUnusedTemplateLiteral),
            "noUnusedVariables" => Ok(Self::NoUnusedVariables),
            "noUnwantedPolyfillio" => Ok(Self::NoUnwantedPolyfillio),
            "noUselessCatch" => Ok(Self::NoUselessCatch),
            "noUselessConstructor" => Ok(Self::NoUselessConstructor),
            "noUselessContinue" => Ok(Self::NoUselessContinue),
            "noUselessElse" => Ok(Self::NoUselessElse),
            "noUselessEmptyExport" => Ok(Self::NoUselessEmptyExport),
            "noUselessEscapeInRegex" => Ok(Self::NoUselessEscapeInRegex),
            "noUselessEscapeInString" => Ok(Self::NoUselessEscapeInString),
            "noUselessFragments" => Ok(Self::NoUselessFragments),
            "noUselessLabel" => Ok(Self::NoUselessLabel),
            "noUselessLoneBlockStatements" => Ok(Self::NoUselessLoneBlockStatements),
            "noUselessRegexBackrefs" => Ok(Self::NoUselessRegexBackrefs),
            "noUselessRename" => Ok(Self::NoUselessRename),
            "noUselessStringConcat" => Ok(Self::NoUselessStringConcat),
            "noUselessStringRaw" => Ok(Self::NoUselessStringRaw),
            "noUselessSwitchCase" => Ok(Self::NoUselessSwitchCase),
            "noUselessTernary" => Ok(Self::NoUselessTernary),
            "noUselessThisAlias" => Ok(Self::NoUselessThisAlias),
            "noUselessTypeConstraint" => Ok(Self::NoUselessTypeConstraint),
            "noUselessUndefined" => Ok(Self::NoUselessUndefined),
            "noUselessUndefinedInitialization" => Ok(Self::NoUselessUndefinedInitialization),
            "noValueAtRule" => Ok(Self::NoValueAtRule),
            "noVar" => Ok(Self::NoVar),
            "noVoid" => Ok(Self::NoVoid),
            "noVoidElementsWithChildren" => Ok(Self::NoVoidElementsWithChildren),
            "noVoidTypeReturn" => Ok(Self::NoVoidTypeReturn),
            "noVueDataObjectDeclaration" => Ok(Self::NoVueDataObjectDeclaration),
            "noVueReservedKeys" => Ok(Self::NoVueReservedKeys),
            "noVueReservedProps" => Ok(Self::NoVueReservedProps),
            "noWith" => Ok(Self::NoWith),
            "noYodaExpression" => Ok(Self::NoYodaExpression),
            "useAdjacentOverloadSignatures" => Ok(Self::UseAdjacentOverloadSignatures),
            "useAltText" => Ok(Self::UseAltText),
            "useAnchorContent" => Ok(Self::UseAnchorContent),
            "useAnchorHref" => Ok(Self::UseAnchorHref),
            "useAriaActivedescendantWithTabindex" => Ok(Self::UseAriaActivedescendantWithTabindex),
            "useAriaPropsForRole" => Ok(Self::UseAriaPropsForRole),
            "useAriaPropsSupportedByRole" => Ok(Self::UseAriaPropsSupportedByRole),
            "useArrayLiterals" => Ok(Self::UseArrayLiterals),
            "useArrowFunction" => Ok(Self::UseArrowFunction),
            "useAsConstAssertion" => Ok(Self::UseAsConstAssertion),
            "useAtIndex" => Ok(Self::UseAtIndex),
            "useAwait" => Ok(Self::UseAwait),
            "useBiomeIgnoreFolder" => Ok(Self::UseBiomeIgnoreFolder),
            "useBlockStatements" => Ok(Self::UseBlockStatements),
            "useButtonType" => Ok(Self::UseButtonType),
            "useCollapsedElseIf" => Ok(Self::UseCollapsedElseIf),
            "useCollapsedIf" => Ok(Self::UseCollapsedIf),
            "useComponentExportOnlyModules" => Ok(Self::UseComponentExportOnlyModules),
            "useConsistentArrayType" => Ok(Self::UseConsistentArrayType),
            "useConsistentBuiltinInstantiation" => Ok(Self::UseConsistentBuiltinInstantiation),
            "useConsistentCurlyBraces" => Ok(Self::UseConsistentCurlyBraces),
            "useConsistentMemberAccessibility" => Ok(Self::UseConsistentMemberAccessibility),
            "useConsistentObjectDefinitions" => Ok(Self::UseConsistentObjectDefinitions),
            "useConsistentTypeDefinitions" => Ok(Self::UseConsistentTypeDefinitions),
            "useConst" => Ok(Self::UseConst),
            "useDateNow" => Ok(Self::UseDateNow),
            "useDefaultParameterLast" => Ok(Self::UseDefaultParameterLast),
            "useDefaultSwitchClause" => Ok(Self::UseDefaultSwitchClause),
            "useDefaultSwitchClauseLast" => Ok(Self::UseDefaultSwitchClauseLast),
            "useDeprecatedReason" => Ok(Self::UseDeprecatedReason),
            "useEnumInitializers" => Ok(Self::UseEnumInitializers),
            "useErrorMessage" => Ok(Self::UseErrorMessage),
            "useExhaustiveDependencies" => Ok(Self::UseExhaustiveDependencies),
            "useExhaustiveSwitchCases" => Ok(Self::UseExhaustiveSwitchCases),
            "useExplicitLengthCheck" => Ok(Self::UseExplicitLengthCheck),
            "useExplicitType" => Ok(Self::UseExplicitType),
            "useExponentiationOperator" => Ok(Self::UseExponentiationOperator),
            "useExportType" => Ok(Self::UseExportType),
            "useExportsLast" => Ok(Self::UseExportsLast),
            "useFilenamingConvention" => Ok(Self::UseFilenamingConvention),
            "useFlatMap" => Ok(Self::UseFlatMap),
            "useFocusableInteractive" => Ok(Self::UseFocusableInteractive),
            "useForOf" => Ok(Self::UseForOf),
            "useFragmentSyntax" => Ok(Self::UseFragmentSyntax),
            "useGenericFontNames" => Ok(Self::UseGenericFontNames),
            "useGetterReturn" => Ok(Self::UseGetterReturn),
            "useGoogleFontDisplay" => Ok(Self::UseGoogleFontDisplay),
            "useGoogleFontPreconnect" => Ok(Self::UseGoogleFontPreconnect),
            "useGraphqlNamedOperations" => Ok(Self::UseGraphqlNamedOperations),
            "useGraphqlNamingConvention" => Ok(Self::UseGraphqlNamingConvention),
            "useGroupedAccessorPairs" => Ok(Self::UseGroupedAccessorPairs),
            "useGuardForIn" => Ok(Self::UseGuardForIn),
            "useHeadingContent" => Ok(Self::UseHeadingContent),
            "useHookAtTopLevel" => Ok(Self::UseHookAtTopLevel),
            "useHtmlLang" => Ok(Self::UseHtmlLang),
            "useIframeTitle" => Ok(Self::UseIframeTitle),
            "useImageSize" => Ok(Self::UseImageSize),
            "useImportExtensions" => Ok(Self::UseImportExtensions),
            "useImportType" => Ok(Self::UseImportType),
            "useIndexOf" => Ok(Self::UseIndexOf),
            "useIsArray" => Ok(Self::UseIsArray),
            "useIsNan" => Ok(Self::UseIsNan),
            "useIterableCallbackReturn" => Ok(Self::UseIterableCallbackReturn),
            "useJsonImportAttributes" => Ok(Self::UseJsonImportAttributes),
            "useJsxKeyInIterable" => Ok(Self::UseJsxKeyInIterable),
            "useKeyWithClickEvents" => Ok(Self::UseKeyWithClickEvents),
            "useKeyWithMouseEvents" => Ok(Self::UseKeyWithMouseEvents),
            "useLiteralEnumMembers" => Ok(Self::UseLiteralEnumMembers),
            "useLiteralKeys" => Ok(Self::UseLiteralKeys),
            "useMaxParams" => Ok(Self::UseMaxParams),
            "useMediaCaption" => Ok(Self::UseMediaCaption),
            "useNamespaceKeyword" => Ok(Self::UseNamespaceKeyword),
            "useNamingConvention" => Ok(Self::UseNamingConvention),
            "useNodeAssertStrict" => Ok(Self::UseNodeAssertStrict),
            "useNodejsImportProtocol" => Ok(Self::UseNodejsImportProtocol),
            "useNumberNamespace" => Ok(Self::UseNumberNamespace),
            "useNumberToFixedDigitsArgument" => Ok(Self::UseNumberToFixedDigitsArgument),
            "useNumericLiterals" => Ok(Self::UseNumericLiterals),
            "useNumericSeparators" => Ok(Self::UseNumericSeparators),
            "useObjectSpread" => Ok(Self::UseObjectSpread),
            "useOptionalChain" => Ok(Self::UseOptionalChain),
            "useParseIntRadix" => Ok(Self::UseParseIntRadix),
            "useQwikClasslist" => Ok(Self::UseQwikClasslist),
            "useReactFunctionComponents" => Ok(Self::UseReactFunctionComponents),
            "useReadonlyClassProperties" => Ok(Self::UseReadonlyClassProperties),
            "useRegexLiterals" => Ok(Self::UseRegexLiterals),
            "useSelfClosingElements" => Ok(Self::UseSelfClosingElements),
            "useSemanticElements" => Ok(Self::UseSemanticElements),
            "useShorthandAssign" => Ok(Self::UseShorthandAssign),
            "useShorthandFunctionType" => Ok(Self::UseShorthandFunctionType),
            "useSimpleNumberKeys" => Ok(Self::UseSimpleNumberKeys),
            "useSimplifiedLogicExpression" => Ok(Self::UseSimplifiedLogicExpression),
            "useSingleJsDocAsterisk" => Ok(Self::UseSingleJsDocAsterisk),
            "useSingleVarDeclarator" => Ok(Self::UseSingleVarDeclarator),
            "useSolidForComponent" => Ok(Self::UseSolidForComponent),
            "useSortedClasses" => Ok(Self::UseSortedClasses),
            "useStaticResponseMethods" => Ok(Self::UseStaticResponseMethods),
            "useStrictMode" => Ok(Self::UseStrictMode),
            "useSymbolDescription" => Ok(Self::UseSymbolDescription),
            "useTemplate" => Ok(Self::UseTemplate),
            "useThrowNewError" => Ok(Self::UseThrowNewError),
            "useThrowOnlyError" => Ok(Self::UseThrowOnlyError),
            "useTopLevelRegex" => Ok(Self::UseTopLevelRegex),
            "useTrimStartEnd" => Ok(Self::UseTrimStartEnd),
            "useUnifiedTypeSignatures" => Ok(Self::UseUnifiedTypeSignatures),
            "useUniqueElementIds" => Ok(Self::UseUniqueElementIds),
            "useValidAnchor" => Ok(Self::UseValidAnchor),
            "useValidAriaProps" => Ok(Self::UseValidAriaProps),
            "useValidAriaRole" => Ok(Self::UseValidAriaRole),
            "useValidAriaValues" => Ok(Self::UseValidAriaValues),
            "useValidAutocomplete" => Ok(Self::UseValidAutocomplete),
            "useValidForDirection" => Ok(Self::UseValidForDirection),
            "useValidLang" => Ok(Self::UseValidLang),
            "useValidTypeof" => Ok(Self::UseValidTypeof),
            "useWhile" => Ok(Self::UseWhile),
            "useYield" => Ok(Self::UseYield),
            _ => Err("This rule name doesn't exist."),
        }
    }
}
impl std::fmt::Display for RuleName {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(self.as_str())
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Rules {
    #[doc = r" It enables the lint rules recommended by Biome. `true` by default."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[deserializable(rename = "a11y")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a11y: Option<SeverityOrGroup<A11y>>,
    #[deserializable(rename = "complexity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complexity: Option<SeverityOrGroup<Complexity>>,
    #[deserializable(rename = "correctness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correctness: Option<SeverityOrGroup<Correctness>>,
    #[deserializable(rename = "nursery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nursery: Option<SeverityOrGroup<Nursery>>,
    #[deserializable(rename = "performance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance: Option<SeverityOrGroup<Performance>>,
    #[deserializable(rename = "security")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<SeverityOrGroup<Security>>,
    #[deserializable(rename = "style")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<SeverityOrGroup<Style>>,
    #[deserializable(rename = "suspicious")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspicious: Option<SeverityOrGroup<Suspicious>>,
}
impl Rules {
    #[doc = r" Checks if the code coming from [biome_diagnostics::Diagnostic] corresponds to a rule."]
    #[doc = r" Usually the code is built like {group}/{rule_name}"]
    pub fn has_rule(group: RuleGroup, rule_name: &str) -> Option<&'static str> {
        match group {
            RuleGroup::A11y => A11y::has_rule(rule_name),
            RuleGroup::Complexity => Complexity::has_rule(rule_name),
            RuleGroup::Correctness => Correctness::has_rule(rule_name),
            RuleGroup::Nursery => Nursery::has_rule(rule_name),
            RuleGroup::Performance => Performance::has_rule(rule_name),
            RuleGroup::Security => Security::has_rule(rule_name),
            RuleGroup::Style => Style::has_rule(rule_name),
            RuleGroup::Suspicious => Suspicious::has_rule(rule_name),
        }
    }
    #[doc = r" Given a category coming from [Diagnostic](biome_diagnostics::Diagnostic), this function returns"]
    #[doc = r" the [Severity](biome_diagnostics::Severity) associated to the rule, if the configuration changed it."]
    #[doc = r" If the severity is off or not set, then the function returns the default severity of the rule:"]
    #[doc = r" [Severity::Error] for recommended rules and [Severity::Warning] for other rules."]
    #[doc = r""]
    #[doc = r" If not, the function returns [None]."]
    pub fn get_severity_from_category(
        &self,
        category: &Category,
        rule_severity: Severity,
    ) -> Option<Severity> {
        let mut split_code = category.name().split('/');
        let _lint = split_code.next();
        debug_assert_eq!(_lint, Some("lint"));
        let group = <RuleGroup as std::str::FromStr>::from_str(split_code.next()?).ok()?;
        let rule_name = split_code.next()?;
        let rule_name = Self::has_rule(group, rule_name)?;
        match group {
            RuleGroup::A11y => self
                .a11y
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .and_then(|(level, _)| match level {
                    RulePlainConfiguration::Off => None,
                    RulePlainConfiguration::On => Some(rule_severity),
                    RulePlainConfiguration::Info
                    | RulePlainConfiguration::Warn
                    | RulePlainConfiguration::Error => Some(Severity::from(level)),
                }),
            RuleGroup::Complexity => self
                .complexity
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .and_then(|(level, _)| match level {
                    RulePlainConfiguration::Off => None,
                    RulePlainConfiguration::On => Some(rule_severity),
                    RulePlainConfiguration::Info
                    | RulePlainConfiguration::Warn
                    | RulePlainConfiguration::Error => Some(Severity::from(level)),
                }),
            RuleGroup::Correctness => self
                .correctness
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .and_then(|(level, _)| match level {
                    RulePlainConfiguration::Off => None,
                    RulePlainConfiguration::On => Some(rule_severity),
                    RulePlainConfiguration::Info
                    | RulePlainConfiguration::Warn
                    | RulePlainConfiguration::Error => Some(Severity::from(level)),
                }),
            RuleGroup::Nursery => self
                .nursery
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .and_then(|(level, _)| match level {
                    RulePlainConfiguration::Off => None,
                    RulePlainConfiguration::On => Some(rule_severity),
                    RulePlainConfiguration::Info
                    | RulePlainConfiguration::Warn
                    | RulePlainConfiguration::Error => Some(Severity::from(level)),
                }),
            RuleGroup::Performance => self
                .performance
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .and_then(|(level, _)| match level {
                    RulePlainConfiguration::Off => None,
                    RulePlainConfiguration::On => Some(rule_severity),
                    RulePlainConfiguration::Info
                    | RulePlainConfiguration::Warn
                    | RulePlainConfiguration::Error => Some(Severity::from(level)),
                }),
            RuleGroup::Security => self
                .security
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .and_then(|(level, _)| match level {
                    RulePlainConfiguration::Off => None,
                    RulePlainConfiguration::On => Some(rule_severity),
                    RulePlainConfiguration::Info
                    | RulePlainConfiguration::Warn
                    | RulePlainConfiguration::Error => Some(Severity::from(level)),
                }),
            RuleGroup::Style => self
                .style
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .and_then(|(level, _)| match level {
                    RulePlainConfiguration::Off => None,
                    RulePlainConfiguration::On => Some(rule_severity),
                    RulePlainConfiguration::Info
                    | RulePlainConfiguration::Warn
                    | RulePlainConfiguration::Error => Some(Severity::from(level)),
                }),
            RuleGroup::Suspicious => self
                .suspicious
                .as_ref()
                .and_then(|group| group.get_rule_configuration(rule_name))
                .and_then(|(level, _)| match level {
                    RulePlainConfiguration::Off => None,
                    RulePlainConfiguration::On => Some(rule_severity),
                    RulePlainConfiguration::Info
                    | RulePlainConfiguration::Warn
                    | RulePlainConfiguration::Error => Some(Severity::from(level)),
                }),
        }
    }
    #[doc = r" Ensure that `recommended` is set to `true` or implied."]
    pub fn set_recommended(&mut self) {
        if self.recommended == Some(false) {
            self.recommended = Some(true)
        }
        if let Some(group) = &mut self.a11y {
            group.set_recommended(None);
        }
        if let Some(group) = &mut self.complexity {
            group.set_recommended(None);
        }
        if let Some(group) = &mut self.correctness {
            group.set_recommended(None);
        }
        if let Some(group) = &mut self.nursery {
            group.set_recommended(None);
        }
        if let Some(group) = &mut self.performance {
            group.set_recommended(None);
        }
        if let Some(group) = &mut self.security {
            group.set_recommended(None);
        }
        if let Some(group) = &mut self.style {
            group.set_recommended(None);
        }
        if let Some(group) = &mut self.suspicious {
            group.set_recommended(None);
        }
    }
    pub(crate) const fn is_recommended_false(&self) -> bool {
        matches!(self.recommended, Some(false))
    }
    #[doc = r" It returns the enabled rules by default."]
    #[doc = r""]
    #[doc = r" The enabled rules are calculated from the difference with the disabled rules."]
    pub fn as_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut enabled_rules = FxHashSet::default();
        let mut disabled_rules = FxHashSet::default();
        if let Some(group) = self.a11y.as_ref() {
            group.collect_preset_rules(!self.is_recommended_false(), &mut enabled_rules);
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(A11y::recommended_rules_as_filters());
        }
        if let Some(group) = self.complexity.as_ref() {
            group.collect_preset_rules(!self.is_recommended_false(), &mut enabled_rules);
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(Complexity::recommended_rules_as_filters());
        }
        if let Some(group) = self.correctness.as_ref() {
            group.collect_preset_rules(!self.is_recommended_false(), &mut enabled_rules);
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(Correctness::recommended_rules_as_filters());
        }
        if let Some(group) = self.nursery.as_ref() {
            group.collect_preset_rules(
                !self.is_recommended_false() && biome_flags::is_unstable(),
                &mut enabled_rules,
            );
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if !self.is_recommended_false() && biome_flags::is_unstable() {
            enabled_rules.extend(Nursery::recommended_rules_as_filters());
        }
        if let Some(group) = self.performance.as_ref() {
            group.collect_preset_rules(!self.is_recommended_false(), &mut enabled_rules);
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(Performance::recommended_rules_as_filters());
        }
        if let Some(group) = self.security.as_ref() {
            group.collect_preset_rules(!self.is_recommended_false(), &mut enabled_rules);
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(Security::recommended_rules_as_filters());
        }
        if let Some(group) = self.style.as_ref() {
            group.collect_preset_rules(!self.is_recommended_false(), &mut enabled_rules);
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(Style::recommended_rules_as_filters());
        }
        if let Some(group) = self.suspicious.as_ref() {
            group.collect_preset_rules(!self.is_recommended_false(), &mut enabled_rules);
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if !self.is_recommended_false() {
            enabled_rules.extend(Suspicious::recommended_rules_as_filters());
        }
        enabled_rules.difference(&disabled_rules).copied().collect()
    }
    #[doc = r" It returns the disabled rules by configuration"]
    pub fn as_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut disabled_rules = FxHashSet::default();
        if let Some(group) = self.a11y.as_ref() {
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if let Some(group) = self.complexity.as_ref() {
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if let Some(group) = self.correctness.as_ref() {
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if let Some(group) = self.nursery.as_ref() {
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if let Some(group) = self.performance.as_ref() {
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if let Some(group) = self.security.as_ref() {
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if let Some(group) = self.style.as_ref() {
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if let Some(group) = self.suspicious.as_ref() {
            disabled_rules.extend(&group.get_disabled_rules());
        }
        disabled_rules
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct A11y { # [doc = r" It enables the recommended rules for this group"] # [serde (skip_serializing_if = "Option::is_none")] pub recommended : Option < bool > , # [doc = "Enforce that the accessKey attribute is not used on any HTML element."] # [serde (skip_serializing_if = "Option::is_none")] pub no_access_key : Option < RuleFixConfiguration < biome_rule_options :: no_access_key :: NoAccessKeyOptions >> , # [doc = "Enforce that aria-hidden=\"true\" is not set on focusable elements."] # [serde (skip_serializing_if = "Option::is_none")] pub no_aria_hidden_on_focusable : Option < RuleFixConfiguration < biome_rule_options :: no_aria_hidden_on_focusable :: NoAriaHiddenOnFocusableOptions >> , # [doc = "Enforce that elements that do not support ARIA roles, states, and properties do not have those attributes."] # [serde (skip_serializing_if = "Option::is_none")] pub no_aria_unsupported_elements : Option < RuleFixConfiguration < biome_rule_options :: no_aria_unsupported_elements :: NoAriaUnsupportedElementsOptions >> , # [doc = "Enforce that autoFocus prop is not used on elements."] # [serde (skip_serializing_if = "Option::is_none")] pub no_autofocus : Option < RuleFixConfiguration < biome_rule_options :: no_autofocus :: NoAutofocusOptions >> , # [doc = "Enforces that no distracting elements are used."] # [serde (skip_serializing_if = "Option::is_none")] pub no_distracting_elements : Option < RuleFixConfiguration < biome_rule_options :: no_distracting_elements :: NoDistractingElementsOptions >> , # [doc = "The scope prop should be used only on \\<th> elements."] # [serde (skip_serializing_if = "Option::is_none")] pub no_header_scope : Option < RuleFixConfiguration < biome_rule_options :: no_header_scope :: NoHeaderScopeOptions >> , # [doc = "Enforce that non-interactive ARIA roles are not assigned to interactive HTML elements."] # [serde (skip_serializing_if = "Option::is_none")] pub no_interactive_element_to_noninteractive_role : Option < RuleFixConfiguration < biome_rule_options :: no_interactive_element_to_noninteractive_role :: NoInteractiveElementToNoninteractiveRoleOptions >> , # [doc = "Enforce that a label element or component has a text label and an associated input."] # [serde (skip_serializing_if = "Option::is_none")] pub no_label_without_control : Option < RuleConfiguration < biome_rule_options :: no_label_without_control :: NoLabelWithoutControlOptions >> , # [doc = "Disallow use event handlers on non-interactive elements."] # [serde (skip_serializing_if = "Option::is_none")] pub no_noninteractive_element_interactions : Option < RuleConfiguration < biome_rule_options :: no_noninteractive_element_interactions :: NoNoninteractiveElementInteractionsOptions >> , # [doc = "Enforce that interactive ARIA roles are not assigned to non-interactive HTML elements."] # [serde (skip_serializing_if = "Option::is_none")] pub no_noninteractive_element_to_interactive_role : Option < RuleFixConfiguration < biome_rule_options :: no_noninteractive_element_to_interactive_role :: NoNoninteractiveElementToInteractiveRoleOptions >> , # [doc = "Enforce that tabIndex is not assigned to non-interactive HTML elements."] # [serde (skip_serializing_if = "Option::is_none")] pub no_noninteractive_tabindex : Option < RuleFixConfiguration < biome_rule_options :: no_noninteractive_tabindex :: NoNoninteractiveTabindexOptions >> , # [doc = "Prevent the usage of positive integers on tabIndex property"] # [serde (skip_serializing_if = "Option::is_none")] pub no_positive_tabindex : Option < RuleFixConfiguration < biome_rule_options :: no_positive_tabindex :: NoPositiveTabindexOptions >> , # [doc = "Enforce img alt prop does not contain the word \"image\", \"picture\", or \"photo\"."] # [serde (skip_serializing_if = "Option::is_none")] pub no_redundant_alt : Option < RuleConfiguration < biome_rule_options :: no_redundant_alt :: NoRedundantAltOptions >> , # [doc = "Enforce explicit role property is not the same as implicit/default role property on an element."] # [serde (skip_serializing_if = "Option::is_none")] pub no_redundant_roles : Option < RuleFixConfiguration < biome_rule_options :: no_redundant_roles :: NoRedundantRolesOptions >> , # [doc = "Enforce that static, visible elements (such as \\<div>) that have click handlers use the valid role attribute."] # [serde (skip_serializing_if = "Option::is_none")] pub no_static_element_interactions : Option < RuleConfiguration < biome_rule_options :: no_static_element_interactions :: NoStaticElementInteractionsOptions >> , # [doc = "Enforces the usage of the title element for the svg element."] # [serde (skip_serializing_if = "Option::is_none")] pub no_svg_without_title : Option < RuleConfiguration < biome_rule_options :: no_svg_without_title :: NoSvgWithoutTitleOptions >> , # [doc = "Enforce that all elements that require alternative text have meaningful information to relay back to the end user."] # [serde (skip_serializing_if = "Option::is_none")] pub use_alt_text : Option < RuleConfiguration < biome_rule_options :: use_alt_text :: UseAltTextOptions >> , # [doc = "Enforce that anchors have content and that the content is accessible to screen readers."] # [serde (skip_serializing_if = "Option::is_none")] pub use_anchor_content : Option < RuleFixConfiguration < biome_rule_options :: use_anchor_content :: UseAnchorContentOptions >> , # [doc = "Enforce that tabIndex is assigned to non-interactive HTML elements with aria-activedescendant."] # [serde (skip_serializing_if = "Option::is_none")] pub use_aria_activedescendant_with_tabindex : Option < RuleFixConfiguration < biome_rule_options :: use_aria_activedescendant_with_tabindex :: UseAriaActivedescendantWithTabindexOptions >> , # [doc = "Enforce that elements with ARIA roles must have all required ARIA attributes for that role."] # [serde (skip_serializing_if = "Option::is_none")] pub use_aria_props_for_role : Option < RuleConfiguration < biome_rule_options :: use_aria_props_for_role :: UseAriaPropsForRoleOptions >> , # [doc = "Enforce that ARIA properties are valid for the roles that are supported by the element."] # [serde (skip_serializing_if = "Option::is_none")] pub use_aria_props_supported_by_role : Option < RuleConfiguration < biome_rule_options :: use_aria_props_supported_by_role :: UseAriaPropsSupportedByRoleOptions >> , # [doc = "Enforces the usage of the attribute type for the element button"] # [serde (skip_serializing_if = "Option::is_none")] pub use_button_type : Option < RuleConfiguration < biome_rule_options :: use_button_type :: UseButtonTypeOptions >> , # [doc = "Elements with an interactive role and interaction handlers must be focusable."] # [serde (skip_serializing_if = "Option::is_none")] pub use_focusable_interactive : Option < RuleConfiguration < biome_rule_options :: use_focusable_interactive :: UseFocusableInteractiveOptions >> , # [doc = "Disallow a missing generic family keyword within font families."] # [serde (skip_serializing_if = "Option::is_none")] pub use_generic_font_names : Option < RuleConfiguration < biome_rule_options :: use_generic_font_names :: UseGenericFontNamesOptions >> , # [doc = "Enforce that heading elements (h1, h2, etc.) have content and that the content is accessible to screen readers. Accessible means that it is not hidden using the aria-hidden prop."] # [serde (skip_serializing_if = "Option::is_none")] pub use_heading_content : Option < RuleConfiguration < biome_rule_options :: use_heading_content :: UseHeadingContentOptions >> , # [doc = "Enforce that html element has lang attribute."] # [serde (skip_serializing_if = "Option::is_none")] pub use_html_lang : Option < RuleConfiguration < biome_rule_options :: use_html_lang :: UseHtmlLangOptions >> , # [doc = "Enforces the usage of the attribute title for the element iframe."] # [serde (skip_serializing_if = "Option::is_none")] pub use_iframe_title : Option < RuleConfiguration < biome_rule_options :: use_iframe_title :: UseIframeTitleOptions >> , # [doc = "Enforce onClick is accompanied by at least one of the following: onKeyUp, onKeyDown, onKeyPress."] # [serde (skip_serializing_if = "Option::is_none")] pub use_key_with_click_events : Option < RuleConfiguration < biome_rule_options :: use_key_with_click_events :: UseKeyWithClickEventsOptions >> , # [doc = "Enforce onMouseOver / onMouseOut are accompanied by onFocus / onBlur."] # [serde (skip_serializing_if = "Option::is_none")] pub use_key_with_mouse_events : Option < RuleConfiguration < biome_rule_options :: use_key_with_mouse_events :: UseKeyWithMouseEventsOptions >> , # [doc = "Enforces that audio and video elements must have a track for captions."] # [serde (skip_serializing_if = "Option::is_none")] pub use_media_caption : Option < RuleConfiguration < biome_rule_options :: use_media_caption :: UseMediaCaptionOptions >> , # [doc = "It detects the use of role attributes in JSX elements and suggests using semantic elements instead."] # [serde (skip_serializing_if = "Option::is_none")] pub use_semantic_elements : Option < RuleConfiguration < biome_rule_options :: use_semantic_elements :: UseSemanticElementsOptions >> , # [doc = "Enforce that all anchors are valid, and they are navigable elements."] # [serde (skip_serializing_if = "Option::is_none")] pub use_valid_anchor : Option < RuleConfiguration < biome_rule_options :: use_valid_anchor :: UseValidAnchorOptions >> , # [doc = "Ensures that ARIA properties aria-* are all valid."] # [serde (skip_serializing_if = "Option::is_none")] pub use_valid_aria_props : Option < RuleFixConfiguration < biome_rule_options :: use_valid_aria_props :: UseValidAriaPropsOptions >> , # [doc = "Elements with ARIA roles must use a valid, non-abstract ARIA role."] # [serde (skip_serializing_if = "Option::is_none")] pub use_valid_aria_role : Option < RuleFixConfiguration < biome_rule_options :: use_valid_aria_role :: UseValidAriaRoleOptions >> , # [doc = "Enforce that ARIA state and property values are valid."] # [serde (skip_serializing_if = "Option::is_none")] pub use_valid_aria_values : Option < RuleConfiguration < biome_rule_options :: use_valid_aria_values :: UseValidAriaValuesOptions >> , # [doc = "Use valid values for the autocomplete attribute on input elements."] # [serde (skip_serializing_if = "Option::is_none")] pub use_valid_autocomplete : Option < RuleConfiguration < biome_rule_options :: use_valid_autocomplete :: UseValidAutocompleteOptions >> , # [doc = "Ensure that the attribute passed to the lang attribute is a correct ISO language and/or country."] # [serde (skip_serializing_if = "Option::is_none")] pub use_valid_lang : Option < RuleConfiguration < biome_rule_options :: use_valid_lang :: UseValidLangOptions >> }
impl A11y {
    const GROUP_NAME: &'static str = "a11y";
    pub(crate) const GROUP_RULES: &'static [&'static str] = &[
        "noAccessKey",
        "noAriaHiddenOnFocusable",
        "noAriaUnsupportedElements",
        "noAutofocus",
        "noDistractingElements",
        "noHeaderScope",
        "noInteractiveElementToNoninteractiveRole",
        "noLabelWithoutControl",
        "noNoninteractiveElementInteractions",
        "noNoninteractiveElementToInteractiveRole",
        "noNoninteractiveTabindex",
        "noPositiveTabindex",
        "noRedundantAlt",
        "noRedundantRoles",
        "noStaticElementInteractions",
        "noSvgWithoutTitle",
        "useAltText",
        "useAnchorContent",
        "useAriaActivedescendantWithTabindex",
        "useAriaPropsForRole",
        "useAriaPropsSupportedByRole",
        "useButtonType",
        "useFocusableInteractive",
        "useGenericFontNames",
        "useHeadingContent",
        "useHtmlLang",
        "useIframeTitle",
        "useKeyWithClickEvents",
        "useKeyWithMouseEvents",
        "useMediaCaption",
        "useSemanticElements",
        "useValidAnchor",
        "useValidAriaProps",
        "useValidAriaRole",
        "useValidAriaValues",
        "useValidAutocomplete",
        "useValidLang",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]),
    ];
    const ALL_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]),
    ];
}
impl RuleGroupExt for A11y {
    fn is_recommended_true(&self) -> bool {
        matches!(self.recommended, Some(true))
    }
    fn is_recommended_unset(&self) -> bool {
        self.recommended.is_none()
    }
    fn get_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_access_key.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
        }
        if let Some(rule) = self.no_aria_hidden_on_focusable.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
        }
        if let Some(rule) = self.no_aria_unsupported_elements.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
        }
        if let Some(rule) = self.no_autofocus.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
        }
        if let Some(rule) = self.no_distracting_elements.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
        }
        if let Some(rule) = self.no_header_scope.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
        }
        if let Some(rule) = self.no_interactive_element_to_noninteractive_role.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
        }
        if let Some(rule) = self.no_label_without_control.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
        }
        if let Some(rule) = self.no_noninteractive_element_interactions.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
        }
        if let Some(rule) = self.no_noninteractive_element_to_interactive_role.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
        }
        if let Some(rule) = self.no_noninteractive_tabindex.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
        }
        if let Some(rule) = self.no_positive_tabindex.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
        }
        if let Some(rule) = self.no_redundant_alt.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
        }
        if let Some(rule) = self.no_redundant_roles.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
        }
        if let Some(rule) = self.no_static_element_interactions.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
        }
        if let Some(rule) = self.no_svg_without_title.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
        }
        if let Some(rule) = self.use_alt_text.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
        }
        if let Some(rule) = self.use_anchor_content.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
        }
        if let Some(rule) = self.use_aria_activedescendant_with_tabindex.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
        }
        if let Some(rule) = self.use_aria_props_for_role.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
        }
        if let Some(rule) = self.use_aria_props_supported_by_role.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
        }
        if let Some(rule) = self.use_button_type.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
        }
        if let Some(rule) = self.use_focusable_interactive.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
        }
        if let Some(rule) = self.use_generic_font_names.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
        }
        if let Some(rule) = self.use_heading_content.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
        }
        if let Some(rule) = self.use_html_lang.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
        }
        if let Some(rule) = self.use_iframe_title.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
        }
        if let Some(rule) = self.use_key_with_click_events.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
        }
        if let Some(rule) = self.use_key_with_mouse_events.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
        }
        if let Some(rule) = self.use_media_caption.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
        }
        if let Some(rule) = self.use_semantic_elements.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
        }
        if let Some(rule) = self.use_valid_anchor.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
        }
        if let Some(rule) = self.use_valid_aria_props.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
        }
        if let Some(rule) = self.use_valid_aria_role.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
        }
        if let Some(rule) = self.use_valid_aria_values.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
        }
        if let Some(rule) = self.use_valid_autocomplete.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
        }
        if let Some(rule) = self.use_valid_lang.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
        }
        index_set
    }
    fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_access_key.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
        }
        if let Some(rule) = self.no_aria_hidden_on_focusable.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
        }
        if let Some(rule) = self.no_aria_unsupported_elements.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
        }
        if let Some(rule) = self.no_autofocus.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
        }
        if let Some(rule) = self.no_distracting_elements.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
        }
        if let Some(rule) = self.no_header_scope.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
        }
        if let Some(rule) = self.no_interactive_element_to_noninteractive_role.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
        }
        if let Some(rule) = self.no_label_without_control.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
        }
        if let Some(rule) = self.no_noninteractive_element_interactions.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
        }
        if let Some(rule) = self.no_noninteractive_element_to_interactive_role.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
        }
        if let Some(rule) = self.no_noninteractive_tabindex.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
        }
        if let Some(rule) = self.no_positive_tabindex.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
        }
        if let Some(rule) = self.no_redundant_alt.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
        }
        if let Some(rule) = self.no_redundant_roles.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
        }
        if let Some(rule) = self.no_static_element_interactions.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
        }
        if let Some(rule) = self.no_svg_without_title.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
        }
        if let Some(rule) = self.use_alt_text.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
        }
        if let Some(rule) = self.use_anchor_content.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
        }
        if let Some(rule) = self.use_aria_activedescendant_with_tabindex.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
        }
        if let Some(rule) = self.use_aria_props_for_role.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
        }
        if let Some(rule) = self.use_aria_props_supported_by_role.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
        }
        if let Some(rule) = self.use_button_type.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
        }
        if let Some(rule) = self.use_focusable_interactive.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
        }
        if let Some(rule) = self.use_generic_font_names.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
        }
        if let Some(rule) = self.use_heading_content.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
        }
        if let Some(rule) = self.use_html_lang.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
        }
        if let Some(rule) = self.use_iframe_title.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
        }
        if let Some(rule) = self.use_key_with_click_events.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
        }
        if let Some(rule) = self.use_key_with_mouse_events.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
        }
        if let Some(rule) = self.use_media_caption.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
        }
        if let Some(rule) = self.use_semantic_elements.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
        }
        if let Some(rule) = self.use_valid_anchor.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
        }
        if let Some(rule) = self.use_valid_aria_props.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
        }
        if let Some(rule) = self.use_valid_aria_role.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
        }
        if let Some(rule) = self.use_valid_aria_values.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
        }
        if let Some(rule) = self.use_valid_autocomplete.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
        }
        if let Some(rule) = self.use_valid_lang.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
        }
        index_set
    }
    #[doc = r" Checks if, given a rule name, matches one of the rules contained in this category"]
    fn has_rule(rule_name: &str) -> Option<&'static str> {
        Some(Self::GROUP_RULES[Self::GROUP_RULES.binary_search(&rule_name).ok()?])
    }
    fn recommended_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
    fn all_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::ALL_RULES_AS_FILTERS
    }
    #[doc = r" Select preset rules"]
    fn collect_preset_rules(
        &self,
        parent_is_recommended: bool,
        enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
    ) {
        if self.is_recommended_true() || self.is_recommended_unset() && parent_is_recommended {
            enabled_rules.extend(Self::recommended_rules_as_filters());
        }
    }
    fn set_recommended(&mut self, value: Option<bool>) {
        self.recommended = value;
    }
    fn get_rule_configuration(
        &self,
        rule_name: &str,
    ) -> Option<(RulePlainConfiguration, Option<RuleOptions>)> {
        match rule_name {
            "noAccessKey" => self
                .no_access_key
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noAriaHiddenOnFocusable" => self
                .no_aria_hidden_on_focusable
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noAriaUnsupportedElements" => self
                .no_aria_unsupported_elements
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noAutofocus" => self
                .no_autofocus
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDistractingElements" => self
                .no_distracting_elements
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noHeaderScope" => self
                .no_header_scope
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInteractiveElementToNoninteractiveRole" => self
                .no_interactive_element_to_noninteractive_role
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noLabelWithoutControl" => self
                .no_label_without_control
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNoninteractiveElementInteractions" => self
                .no_noninteractive_element_interactions
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNoninteractiveElementToInteractiveRole" => self
                .no_noninteractive_element_to_interactive_role
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNoninteractiveTabindex" => self
                .no_noninteractive_tabindex
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noPositiveTabindex" => self
                .no_positive_tabindex
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noRedundantAlt" => self
                .no_redundant_alt
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noRedundantRoles" => self
                .no_redundant_roles
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noStaticElementInteractions" => self
                .no_static_element_interactions
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSvgWithoutTitle" => self
                .no_svg_without_title
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useAltText" => self
                .use_alt_text
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useAnchorContent" => self
                .use_anchor_content
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useAriaActivedescendantWithTabindex" => self
                .use_aria_activedescendant_with_tabindex
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useAriaPropsForRole" => self
                .use_aria_props_for_role
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useAriaPropsSupportedByRole" => self
                .use_aria_props_supported_by_role
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useButtonType" => self
                .use_button_type
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useFocusableInteractive" => self
                .use_focusable_interactive
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useGenericFontNames" => self
                .use_generic_font_names
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useHeadingContent" => self
                .use_heading_content
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useHtmlLang" => self
                .use_html_lang
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useIframeTitle" => self
                .use_iframe_title
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useKeyWithClickEvents" => self
                .use_key_with_click_events
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useKeyWithMouseEvents" => self
                .use_key_with_mouse_events
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useMediaCaption" => self
                .use_media_caption
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useSemanticElements" => self
                .use_semantic_elements
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useValidAnchor" => self
                .use_valid_anchor
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useValidAriaProps" => self
                .use_valid_aria_props
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useValidAriaRole" => self
                .use_valid_aria_role
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useValidAriaValues" => self
                .use_valid_aria_values
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useValidAutocomplete" => self
                .use_valid_autocomplete
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useValidLang" => self
                .use_valid_lang
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            _ => None,
        }
    }
}
impl From<GroupPlainConfiguration> for A11y {
    fn from(value: GroupPlainConfiguration) -> Self {
        Self {
            recommended: None,
            no_access_key: Some(value.into()),
            no_aria_hidden_on_focusable: Some(value.into()),
            no_aria_unsupported_elements: Some(value.into()),
            no_autofocus: Some(value.into()),
            no_distracting_elements: Some(value.into()),
            no_header_scope: Some(value.into()),
            no_interactive_element_to_noninteractive_role: Some(value.into()),
            no_label_without_control: Some(value.into()),
            no_noninteractive_element_interactions: Some(value.into()),
            no_noninteractive_element_to_interactive_role: Some(value.into()),
            no_noninteractive_tabindex: Some(value.into()),
            no_positive_tabindex: Some(value.into()),
            no_redundant_alt: Some(value.into()),
            no_redundant_roles: Some(value.into()),
            no_static_element_interactions: Some(value.into()),
            no_svg_without_title: Some(value.into()),
            use_alt_text: Some(value.into()),
            use_anchor_content: Some(value.into()),
            use_aria_activedescendant_with_tabindex: Some(value.into()),
            use_aria_props_for_role: Some(value.into()),
            use_aria_props_supported_by_role: Some(value.into()),
            use_button_type: Some(value.into()),
            use_focusable_interactive: Some(value.into()),
            use_generic_font_names: Some(value.into()),
            use_heading_content: Some(value.into()),
            use_html_lang: Some(value.into()),
            use_iframe_title: Some(value.into()),
            use_key_with_click_events: Some(value.into()),
            use_key_with_mouse_events: Some(value.into()),
            use_media_caption: Some(value.into()),
            use_semantic_elements: Some(value.into()),
            use_valid_anchor: Some(value.into()),
            use_valid_aria_props: Some(value.into()),
            use_valid_aria_role: Some(value.into()),
            use_valid_aria_values: Some(value.into()),
            use_valid_autocomplete: Some(value.into()),
            use_valid_lang: Some(value.into()),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Complexity { # [doc = r" It enables the recommended rules for this group"] # [serde (skip_serializing_if = "Option::is_none")] pub recommended : Option < bool > , # [doc = "Disallow unclear usage of consecutive space characters in regular expression literals"] # [serde (skip_serializing_if = "Option::is_none")] pub no_adjacent_spaces_in_regex : Option < RuleFixConfiguration < biome_rule_options :: no_adjacent_spaces_in_regex :: NoAdjacentSpacesInRegexOptions >> , # [doc = "Disallow the use of arguments."] # [serde (skip_serializing_if = "Option::is_none")] pub no_arguments : Option < RuleConfiguration < biome_rule_options :: no_arguments :: NoArgumentsOptions >> , # [doc = "Disallow primitive type aliases and misleading types."] # [serde (skip_serializing_if = "Option::is_none")] pub no_banned_types : Option < RuleFixConfiguration < biome_rule_options :: no_banned_types :: NoBannedTypesOptions >> , # [doc = "Disallow comma operator."] # [serde (skip_serializing_if = "Option::is_none")] pub no_comma_operator : Option < RuleConfiguration < biome_rule_options :: no_comma_operator :: NoCommaOperatorOptions >> , # [doc = "Disallow empty type parameters in type aliases and interfaces."] # [serde (skip_serializing_if = "Option::is_none")] pub no_empty_type_parameters : Option < RuleConfiguration < biome_rule_options :: no_empty_type_parameters :: NoEmptyTypeParametersOptions >> , # [doc = "Disallow functions that exceed a given Cognitive Complexity score."] # [serde (skip_serializing_if = "Option::is_none")] pub no_excessive_cognitive_complexity : Option < RuleConfiguration < biome_rule_options :: no_excessive_cognitive_complexity :: NoExcessiveCognitiveComplexityOptions >> , # [doc = "Restrict the number of lines of code in a function."] # [serde (skip_serializing_if = "Option::is_none")] pub no_excessive_lines_per_function : Option < RuleConfiguration < biome_rule_options :: no_excessive_lines_per_function :: NoExcessiveLinesPerFunctionOptions >> , # [doc = "This rule enforces a maximum depth to nested describe() in test files."] # [serde (skip_serializing_if = "Option::is_none")] pub no_excessive_nested_test_suites : Option < RuleConfiguration < biome_rule_options :: no_excessive_nested_test_suites :: NoExcessiveNestedTestSuitesOptions >> , # [doc = "Disallow unnecessary boolean casts"] # [serde (skip_serializing_if = "Option::is_none")] pub no_extra_boolean_cast : Option < RuleFixConfiguration < biome_rule_options :: no_extra_boolean_cast :: NoExtraBooleanCastOptions >> , # [doc = "Disallow to use unnecessary callback on flatMap."] # [serde (skip_serializing_if = "Option::is_none")] pub no_flat_map_identity : Option < RuleFixConfiguration < biome_rule_options :: no_flat_map_identity :: NoFlatMapIdentityOptions >> , # [doc = "Prefer for...of statement instead of Array.forEach."] # [serde (skip_serializing_if = "Option::is_none")] pub no_for_each : Option < RuleConfiguration < biome_rule_options :: no_for_each :: NoForEachOptions >> , # [doc = "Disallow shorthand type conversions."] # [serde (skip_serializing_if = "Option::is_none")] pub no_implicit_coercions : Option < RuleFixConfiguration < biome_rule_options :: no_implicit_coercions :: NoImplicitCoercionsOptions >> , # [doc = "Disallow the use of the !important style."] # [serde (skip_serializing_if = "Option::is_none")] pub no_important_styles : Option < RuleFixConfiguration < biome_rule_options :: no_important_styles :: NoImportantStylesOptions >> , # [doc = "This rule reports when a class has no non-static members, such as for a class used exclusively as a static namespace."] # [serde (skip_serializing_if = "Option::is_none")] pub no_static_only_class : Option < RuleConfiguration < biome_rule_options :: no_static_only_class :: NoStaticOnlyClassOptions >> , # [doc = "Disallow this and super in static contexts."] # [serde (skip_serializing_if = "Option::is_none")] pub no_this_in_static : Option < RuleFixConfiguration < biome_rule_options :: no_this_in_static :: NoThisInStaticOptions >> , # [doc = "Disallow unnecessary catch clauses."] # [serde (skip_serializing_if = "Option::is_none")] pub no_useless_catch : Option < RuleFixConfiguration < biome_rule_options :: no_useless_catch :: NoUselessCatchOptions >> , # [doc = "Disallow unnecessary constructors."] # [serde (skip_serializing_if = "Option::is_none")] pub no_useless_constructor : Option < RuleFixConfiguration < biome_rule_options :: no_useless_constructor :: NoUselessConstructorOptions >> , # [doc = "Avoid using unnecessary continue."] # [serde (skip_serializing_if = "Option::is_none")] pub no_useless_continue : Option < RuleFixConfiguration < biome_rule_options :: no_useless_continue :: NoUselessContinueOptions >> , # [doc = "Disallow empty exports that don't change anything in a module file."] # [serde (skip_serializing_if = "Option::is_none")] pub no_useless_empty_export : Option < RuleFixConfiguration < biome_rule_options :: no_useless_empty_export :: NoUselessEmptyExportOptions >> , # [doc = "Disallow unnecessary escape sequence in regular expression literals."] # [serde (skip_serializing_if = "Option::is_none")] pub no_useless_escape_in_regex : Option < RuleFixConfiguration < biome_rule_options :: no_useless_escape_in_regex :: NoUselessEscapeInRegexOptions >> , # [doc = "Disallow unnecessary fragments"] # [serde (skip_serializing_if = "Option::is_none")] pub no_useless_fragments : Option < RuleFixConfiguration < biome_rule_options :: no_useless_fragments :: NoUselessFragmentsOptions >> , # [doc = "Disallow unnecessary labels."] # [serde (skip_serializing_if = "Option::is_none")] pub no_useless_label : Option < RuleFixConfiguration < biome_rule_options :: no_useless_label :: NoUselessLabelOptions >> , # [doc = "Disallow unnecessary nested block statements."] # [serde (skip_serializing_if = "Option::is_none")] pub no_useless_lone_block_statements : Option < RuleFixConfiguration < biome_rule_options :: no_useless_lone_block_statements :: NoUselessLoneBlockStatementsOptions >> , # [doc = "Disallow renaming import, export, and destructured assignments to the same name."] # [serde (skip_serializing_if = "Option::is_none")] pub no_useless_rename : Option < RuleFixConfiguration < biome_rule_options :: no_useless_rename :: NoUselessRenameOptions >> , # [doc = "Disallow unnecessary concatenation of string or template literals."] # [serde (skip_serializing_if = "Option::is_none")] pub no_useless_string_concat : Option < RuleFixConfiguration < biome_rule_options :: no_useless_string_concat :: NoUselessStringConcatOptions >> , # [doc = "Disallow unnecessary String.raw function in template string literals without any escape sequence."] # [serde (skip_serializing_if = "Option::is_none")] pub no_useless_string_raw : Option < RuleConfiguration < biome_rule_options :: no_useless_string_raw :: NoUselessStringRawOptions >> , # [doc = "Disallow useless case in switch statements."] # [serde (skip_serializing_if = "Option::is_none")] pub no_useless_switch_case : Option < RuleFixConfiguration < biome_rule_options :: no_useless_switch_case :: NoUselessSwitchCaseOptions >> , # [doc = "Disallow ternary operators when simpler alternatives exist."] # [serde (skip_serializing_if = "Option::is_none")] pub no_useless_ternary : Option < RuleFixConfiguration < biome_rule_options :: no_useless_ternary :: NoUselessTernaryOptions >> , # [doc = "Disallow useless this aliasing."] # [serde (skip_serializing_if = "Option::is_none")] pub no_useless_this_alias : Option < RuleFixConfiguration < biome_rule_options :: no_useless_this_alias :: NoUselessThisAliasOptions >> , # [doc = "Disallow using any or unknown as type constraint."] # [serde (skip_serializing_if = "Option::is_none")] pub no_useless_type_constraint : Option < RuleFixConfiguration < biome_rule_options :: no_useless_type_constraint :: NoUselessTypeConstraintOptions >> , # [doc = "Disallow initializing variables to undefined."] # [serde (skip_serializing_if = "Option::is_none")] pub no_useless_undefined_initialization : Option < RuleFixConfiguration < biome_rule_options :: no_useless_undefined_initialization :: NoUselessUndefinedInitializationOptions >> , # [doc = "Disallow the use of void operators, which is not a familiar operator."] # [serde (skip_serializing_if = "Option::is_none")] pub no_void : Option < RuleConfiguration < biome_rule_options :: no_void :: NoVoidOptions >> , # [doc = "Use arrow functions over function expressions."] # [serde (skip_serializing_if = "Option::is_none")] pub use_arrow_function : Option < RuleFixConfiguration < biome_rule_options :: use_arrow_function :: UseArrowFunctionOptions >> , # [doc = "Use Date.now() to get the number of milliseconds since the Unix Epoch."] # [serde (skip_serializing_if = "Option::is_none")] pub use_date_now : Option < RuleFixConfiguration < biome_rule_options :: use_date_now :: UseDateNowOptions >> , # [doc = "Promotes the use of .flatMap() when map().flat() are used together."] # [serde (skip_serializing_if = "Option::is_none")] pub use_flat_map : Option < RuleFixConfiguration < biome_rule_options :: use_flat_map :: UseFlatMapOptions >> , # [doc = "Prefer Array#{indexOf,lastIndexOf}() over Array#{findIndex,findLastIndex}() when looking for the index of an item."] # [serde (skip_serializing_if = "Option::is_none")] pub use_index_of : Option < RuleFixConfiguration < biome_rule_options :: use_index_of :: UseIndexOfOptions >> , # [doc = "Enforce the usage of a literal access to properties over computed property access."] # [serde (skip_serializing_if = "Option::is_none")] pub use_literal_keys : Option < RuleFixConfiguration < biome_rule_options :: use_literal_keys :: UseLiteralKeysOptions >> , # [doc = "Disallow parseInt() and Number.parseInt() in favor of binary, octal, and hexadecimal literals"] # [serde (skip_serializing_if = "Option::is_none")] pub use_numeric_literals : Option < RuleFixConfiguration < biome_rule_options :: use_numeric_literals :: UseNumericLiteralsOptions >> , # [doc = "Enforce using concise optional chain instead of chained logical expressions."] # [serde (skip_serializing_if = "Option::is_none")] pub use_optional_chain : Option < RuleFixConfiguration < biome_rule_options :: use_optional_chain :: UseOptionalChainOptions >> , # [doc = "Enforce the use of the regular expression literals instead of the RegExp constructor if possible."] # [serde (skip_serializing_if = "Option::is_none")] pub use_regex_literals : Option < RuleFixConfiguration < biome_rule_options :: use_regex_literals :: UseRegexLiteralsOptions >> , # [doc = "Disallow number literal object member names which are not base 10 or use underscore as separator."] # [serde (skip_serializing_if = "Option::is_none")] pub use_simple_number_keys : Option < RuleFixConfiguration < biome_rule_options :: use_simple_number_keys :: UseSimpleNumberKeysOptions >> , # [doc = "Discard redundant terms from logical expressions."] # [serde (skip_serializing_if = "Option::is_none")] pub use_simplified_logic_expression : Option < RuleFixConfiguration < biome_rule_options :: use_simplified_logic_expression :: UseSimplifiedLogicExpressionOptions >> , # [doc = "Enforce the use of while loops instead of for loops when the initializer and update expressions are not needed."] # [serde (skip_serializing_if = "Option::is_none")] pub use_while : Option < RuleFixConfiguration < biome_rule_options :: use_while :: UseWhileOptions >> }
impl Complexity {
    const GROUP_NAME: &'static str = "complexity";
    pub(crate) const GROUP_RULES: &'static [&'static str] = &[
        "noAdjacentSpacesInRegex",
        "noArguments",
        "noBannedTypes",
        "noCommaOperator",
        "noEmptyTypeParameters",
        "noExcessiveCognitiveComplexity",
        "noExcessiveLinesPerFunction",
        "noExcessiveNestedTestSuites",
        "noExtraBooleanCast",
        "noFlatMapIdentity",
        "noForEach",
        "noImplicitCoercions",
        "noImportantStyles",
        "noStaticOnlyClass",
        "noThisInStatic",
        "noUselessCatch",
        "noUselessConstructor",
        "noUselessContinue",
        "noUselessEmptyExport",
        "noUselessEscapeInRegex",
        "noUselessFragments",
        "noUselessLabel",
        "noUselessLoneBlockStatements",
        "noUselessRename",
        "noUselessStringConcat",
        "noUselessStringRaw",
        "noUselessSwitchCase",
        "noUselessTernary",
        "noUselessThisAlias",
        "noUselessTypeConstraint",
        "noUselessUndefinedInitialization",
        "noVoid",
        "useArrowFunction",
        "useDateNow",
        "useFlatMap",
        "useIndexOf",
        "useLiteralKeys",
        "useNumericLiterals",
        "useOptionalChain",
        "useRegexLiterals",
        "useSimpleNumberKeys",
        "useSimplifiedLogicExpression",
        "useWhile",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]),
    ];
    const ALL_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]),
    ];
}
impl RuleGroupExt for Complexity {
    fn is_recommended_true(&self) -> bool {
        matches!(self.recommended, Some(true))
    }
    fn is_recommended_unset(&self) -> bool {
        self.recommended.is_none()
    }
    fn get_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_adjacent_spaces_in_regex.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
        }
        if let Some(rule) = self.no_arguments.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
        }
        if let Some(rule) = self.no_banned_types.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
        }
        if let Some(rule) = self.no_comma_operator.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
        }
        if let Some(rule) = self.no_empty_type_parameters.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
        }
        if let Some(rule) = self.no_excessive_cognitive_complexity.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
        }
        if let Some(rule) = self.no_excessive_lines_per_function.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
        }
        if let Some(rule) = self.no_excessive_nested_test_suites.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
        }
        if let Some(rule) = self.no_extra_boolean_cast.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
        }
        if let Some(rule) = self.no_flat_map_identity.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
        }
        if let Some(rule) = self.no_for_each.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
        }
        if let Some(rule) = self.no_implicit_coercions.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
        }
        if let Some(rule) = self.no_important_styles.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
        }
        if let Some(rule) = self.no_static_only_class.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
        }
        if let Some(rule) = self.no_this_in_static.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
        }
        if let Some(rule) = self.no_useless_catch.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
        }
        if let Some(rule) = self.no_useless_constructor.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
        }
        if let Some(rule) = self.no_useless_continue.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
        }
        if let Some(rule) = self.no_useless_empty_export.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
        }
        if let Some(rule) = self.no_useless_escape_in_regex.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
        }
        if let Some(rule) = self.no_useless_fragments.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
        }
        if let Some(rule) = self.no_useless_label.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
        }
        if let Some(rule) = self.no_useless_lone_block_statements.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
        }
        if let Some(rule) = self.no_useless_rename.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
        }
        if let Some(rule) = self.no_useless_string_concat.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
        }
        if let Some(rule) = self.no_useless_string_raw.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
        }
        if let Some(rule) = self.no_useless_switch_case.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
        }
        if let Some(rule) = self.no_useless_ternary.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
        }
        if let Some(rule) = self.no_useless_this_alias.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
        }
        if let Some(rule) = self.no_useless_type_constraint.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
        }
        if let Some(rule) = self.no_useless_undefined_initialization.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
        }
        if let Some(rule) = self.no_void.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
        }
        if let Some(rule) = self.use_arrow_function.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
        }
        if let Some(rule) = self.use_date_now.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
        }
        if let Some(rule) = self.use_flat_map.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
        }
        if let Some(rule) = self.use_index_of.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
        }
        if let Some(rule) = self.use_literal_keys.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
        }
        if let Some(rule) = self.use_numeric_literals.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
        }
        if let Some(rule) = self.use_optional_chain.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
        }
        if let Some(rule) = self.use_regex_literals.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
        }
        if let Some(rule) = self.use_simple_number_keys.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
        }
        if let Some(rule) = self.use_simplified_logic_expression.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
        }
        if let Some(rule) = self.use_while.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
        }
        index_set
    }
    fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_adjacent_spaces_in_regex.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
        }
        if let Some(rule) = self.no_arguments.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
        }
        if let Some(rule) = self.no_banned_types.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
        }
        if let Some(rule) = self.no_comma_operator.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
        }
        if let Some(rule) = self.no_empty_type_parameters.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
        }
        if let Some(rule) = self.no_excessive_cognitive_complexity.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
        }
        if let Some(rule) = self.no_excessive_lines_per_function.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
        }
        if let Some(rule) = self.no_excessive_nested_test_suites.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
        }
        if let Some(rule) = self.no_extra_boolean_cast.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
        }
        if let Some(rule) = self.no_flat_map_identity.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
        }
        if let Some(rule) = self.no_for_each.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
        }
        if let Some(rule) = self.no_implicit_coercions.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
        }
        if let Some(rule) = self.no_important_styles.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
        }
        if let Some(rule) = self.no_static_only_class.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
        }
        if let Some(rule) = self.no_this_in_static.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
        }
        if let Some(rule) = self.no_useless_catch.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
        }
        if let Some(rule) = self.no_useless_constructor.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
        }
        if let Some(rule) = self.no_useless_continue.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
        }
        if let Some(rule) = self.no_useless_empty_export.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
        }
        if let Some(rule) = self.no_useless_escape_in_regex.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
        }
        if let Some(rule) = self.no_useless_fragments.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
        }
        if let Some(rule) = self.no_useless_label.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
        }
        if let Some(rule) = self.no_useless_lone_block_statements.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
        }
        if let Some(rule) = self.no_useless_rename.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
        }
        if let Some(rule) = self.no_useless_string_concat.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
        }
        if let Some(rule) = self.no_useless_string_raw.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
        }
        if let Some(rule) = self.no_useless_switch_case.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
        }
        if let Some(rule) = self.no_useless_ternary.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
        }
        if let Some(rule) = self.no_useless_this_alias.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
        }
        if let Some(rule) = self.no_useless_type_constraint.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
        }
        if let Some(rule) = self.no_useless_undefined_initialization.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
        }
        if let Some(rule) = self.no_void.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
        }
        if let Some(rule) = self.use_arrow_function.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
        }
        if let Some(rule) = self.use_date_now.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
        }
        if let Some(rule) = self.use_flat_map.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
        }
        if let Some(rule) = self.use_index_of.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
        }
        if let Some(rule) = self.use_literal_keys.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
        }
        if let Some(rule) = self.use_numeric_literals.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
        }
        if let Some(rule) = self.use_optional_chain.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
        }
        if let Some(rule) = self.use_regex_literals.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
        }
        if let Some(rule) = self.use_simple_number_keys.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
        }
        if let Some(rule) = self.use_simplified_logic_expression.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
        }
        if let Some(rule) = self.use_while.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
        }
        index_set
    }
    #[doc = r" Checks if, given a rule name, matches one of the rules contained in this category"]
    fn has_rule(rule_name: &str) -> Option<&'static str> {
        Some(Self::GROUP_RULES[Self::GROUP_RULES.binary_search(&rule_name).ok()?])
    }
    fn recommended_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
    fn all_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::ALL_RULES_AS_FILTERS
    }
    #[doc = r" Select preset rules"]
    fn collect_preset_rules(
        &self,
        parent_is_recommended: bool,
        enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
    ) {
        if self.is_recommended_true() || self.is_recommended_unset() && parent_is_recommended {
            enabled_rules.extend(Self::recommended_rules_as_filters());
        }
    }
    fn set_recommended(&mut self, value: Option<bool>) {
        self.recommended = value;
    }
    fn get_rule_configuration(
        &self,
        rule_name: &str,
    ) -> Option<(RulePlainConfiguration, Option<RuleOptions>)> {
        match rule_name {
            "noAdjacentSpacesInRegex" => self
                .no_adjacent_spaces_in_regex
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noArguments" => self
                .no_arguments
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noBannedTypes" => self
                .no_banned_types
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noCommaOperator" => self
                .no_comma_operator
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noEmptyTypeParameters" => self
                .no_empty_type_parameters
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noExcessiveCognitiveComplexity" => self
                .no_excessive_cognitive_complexity
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noExcessiveLinesPerFunction" => self
                .no_excessive_lines_per_function
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noExcessiveNestedTestSuites" => self
                .no_excessive_nested_test_suites
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noExtraBooleanCast" => self
                .no_extra_boolean_cast
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noFlatMapIdentity" => self
                .no_flat_map_identity
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noForEach" => self
                .no_for_each
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noImplicitCoercions" => self
                .no_implicit_coercions
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noImportantStyles" => self
                .no_important_styles
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noStaticOnlyClass" => self
                .no_static_only_class
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noThisInStatic" => self
                .no_this_in_static
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessCatch" => self
                .no_useless_catch
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessConstructor" => self
                .no_useless_constructor
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessContinue" => self
                .no_useless_continue
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessEmptyExport" => self
                .no_useless_empty_export
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessEscapeInRegex" => self
                .no_useless_escape_in_regex
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessFragments" => self
                .no_useless_fragments
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessLabel" => self
                .no_useless_label
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessLoneBlockStatements" => self
                .no_useless_lone_block_statements
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessRename" => self
                .no_useless_rename
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessStringConcat" => self
                .no_useless_string_concat
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessStringRaw" => self
                .no_useless_string_raw
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessSwitchCase" => self
                .no_useless_switch_case
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessTernary" => self
                .no_useless_ternary
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessThisAlias" => self
                .no_useless_this_alias
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessTypeConstraint" => self
                .no_useless_type_constraint
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessUndefinedInitialization" => self
                .no_useless_undefined_initialization
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noVoid" => self
                .no_void
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useArrowFunction" => self
                .use_arrow_function
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useDateNow" => self
                .use_date_now
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useFlatMap" => self
                .use_flat_map
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useIndexOf" => self
                .use_index_of
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useLiteralKeys" => self
                .use_literal_keys
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useNumericLiterals" => self
                .use_numeric_literals
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useOptionalChain" => self
                .use_optional_chain
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useRegexLiterals" => self
                .use_regex_literals
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useSimpleNumberKeys" => self
                .use_simple_number_keys
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useSimplifiedLogicExpression" => self
                .use_simplified_logic_expression
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useWhile" => self
                .use_while
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            _ => None,
        }
    }
}
impl From<GroupPlainConfiguration> for Complexity {
    fn from(value: GroupPlainConfiguration) -> Self {
        Self {
            recommended: None,
            no_adjacent_spaces_in_regex: Some(value.into()),
            no_arguments: Some(value.into()),
            no_banned_types: Some(value.into()),
            no_comma_operator: Some(value.into()),
            no_empty_type_parameters: Some(value.into()),
            no_excessive_cognitive_complexity: Some(value.into()),
            no_excessive_lines_per_function: Some(value.into()),
            no_excessive_nested_test_suites: Some(value.into()),
            no_extra_boolean_cast: Some(value.into()),
            no_flat_map_identity: Some(value.into()),
            no_for_each: Some(value.into()),
            no_implicit_coercions: Some(value.into()),
            no_important_styles: Some(value.into()),
            no_static_only_class: Some(value.into()),
            no_this_in_static: Some(value.into()),
            no_useless_catch: Some(value.into()),
            no_useless_constructor: Some(value.into()),
            no_useless_continue: Some(value.into()),
            no_useless_empty_export: Some(value.into()),
            no_useless_escape_in_regex: Some(value.into()),
            no_useless_fragments: Some(value.into()),
            no_useless_label: Some(value.into()),
            no_useless_lone_block_statements: Some(value.into()),
            no_useless_rename: Some(value.into()),
            no_useless_string_concat: Some(value.into()),
            no_useless_string_raw: Some(value.into()),
            no_useless_switch_case: Some(value.into()),
            no_useless_ternary: Some(value.into()),
            no_useless_this_alias: Some(value.into()),
            no_useless_type_constraint: Some(value.into()),
            no_useless_undefined_initialization: Some(value.into()),
            no_void: Some(value.into()),
            use_arrow_function: Some(value.into()),
            use_date_now: Some(value.into()),
            use_flat_map: Some(value.into()),
            use_index_of: Some(value.into()),
            use_literal_keys: Some(value.into()),
            use_numeric_literals: Some(value.into()),
            use_optional_chain: Some(value.into()),
            use_regex_literals: Some(value.into()),
            use_simple_number_keys: Some(value.into()),
            use_simplified_logic_expression: Some(value.into()),
            use_while: Some(value.into()),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Correctness { # [doc = r" It enables the recommended rules for this group"] # [serde (skip_serializing_if = "Option::is_none")] pub recommended : Option < bool > , # [doc = "Prevent passing of children as props."] # [serde (skip_serializing_if = "Option::is_none")] pub no_children_prop : Option < RuleConfiguration < biome_rule_options :: no_children_prop :: NoChildrenPropOptions >> , # [doc = "Prevents from having const variables being re-assigned."] # [serde (skip_serializing_if = "Option::is_none")] pub no_const_assign : Option < RuleFixConfiguration < biome_rule_options :: no_const_assign :: NoConstAssignOptions >> , # [doc = "Disallow constant expressions in conditions"] # [serde (skip_serializing_if = "Option::is_none")] pub no_constant_condition : Option < RuleConfiguration < biome_rule_options :: no_constant_condition :: NoConstantConditionOptions >> , # [doc = "Disallow the use of Math.min and Math.max to clamp a value where the result itself is constant."] # [serde (skip_serializing_if = "Option::is_none")] pub no_constant_math_min_max_clamp : Option < RuleFixConfiguration < biome_rule_options :: no_constant_math_min_max_clamp :: NoConstantMathMinMaxClampOptions >> , # [doc = "Disallow returning a value from a constructor."] # [serde (skip_serializing_if = "Option::is_none")] pub no_constructor_return : Option < RuleConfiguration < biome_rule_options :: no_constructor_return :: NoConstructorReturnOptions >> , # [doc = "Disallow empty character classes in regular expression literals."] # [serde (skip_serializing_if = "Option::is_none")] pub no_empty_character_class_in_regex : Option < RuleConfiguration < biome_rule_options :: no_empty_character_class_in_regex :: NoEmptyCharacterClassInRegexOptions >> , # [doc = "Disallows empty destructuring patterns."] # [serde (skip_serializing_if = "Option::is_none")] pub no_empty_pattern : Option < RuleConfiguration < biome_rule_options :: no_empty_pattern :: NoEmptyPatternOptions >> , # [doc = "Disallow the use of __dirname and __filename in the global scope."] # [serde (skip_serializing_if = "Option::is_none")] pub no_global_dirname_filename : Option < RuleFixConfiguration < biome_rule_options :: no_global_dirname_filename :: NoGlobalDirnameFilenameOptions >> , # [doc = "Disallow calling global object properties as functions"] # [serde (skip_serializing_if = "Option::is_none")] pub no_global_object_calls : Option < RuleConfiguration < biome_rule_options :: no_global_object_calls :: NoGlobalObjectCallsOptions >> , # [doc = "Disallow function and var declarations that are accessible outside their block."] # [serde (skip_serializing_if = "Option::is_none")] pub no_inner_declarations : Option < RuleConfiguration < biome_rule_options :: no_inner_declarations :: NoInnerDeclarationsOptions >> , # [doc = "Ensure that builtins are correctly instantiated."] # [serde (skip_serializing_if = "Option::is_none")] pub no_invalid_builtin_instantiation : Option < RuleFixConfiguration < biome_rule_options :: no_invalid_builtin_instantiation :: NoInvalidBuiltinInstantiationOptions >> , # [doc = "Prevents the incorrect use of super() inside classes. It also checks whether a call super() is missing from classes that extends other constructors."] # [serde (skip_serializing_if = "Option::is_none")] pub no_invalid_constructor_super : Option < RuleConfiguration < biome_rule_options :: no_invalid_constructor_super :: NoInvalidConstructorSuperOptions >> , # [doc = "Disallow non-standard direction values for linear gradient functions."] # [serde (skip_serializing_if = "Option::is_none")] pub no_invalid_direction_in_linear_gradient : Option < RuleConfiguration < biome_rule_options :: no_invalid_direction_in_linear_gradient :: NoInvalidDirectionInLinearGradientOptions >> , # [doc = "Disallows invalid named grid areas in CSS Grid Layouts."] # [serde (skip_serializing_if = "Option::is_none")] pub no_invalid_grid_areas : Option < RuleConfiguration < biome_rule_options :: no_invalid_grid_areas :: NoInvalidGridAreasOptions >> , # [doc = "Disallow the use of @import at-rules in invalid positions."] # [serde (skip_serializing_if = "Option::is_none")] pub no_invalid_position_at_import_rule : Option < RuleConfiguration < biome_rule_options :: no_invalid_position_at_import_rule :: NoInvalidPositionAtImportRuleOptions >> , # [doc = "Disallow the use of variables and function parameters before their declaration"] # [serde (skip_serializing_if = "Option::is_none")] pub no_invalid_use_before_declaration : Option < RuleConfiguration < biome_rule_options :: no_invalid_use_before_declaration :: NoInvalidUseBeforeDeclarationOptions >> , # [doc = "Disallow missing var function for css variables."] # [serde (skip_serializing_if = "Option::is_none")] pub no_missing_var_function : Option < RuleConfiguration < biome_rule_options :: no_missing_var_function :: NoMissingVarFunctionOptions >> , # [doc = "Disallows defining React components inside other components."] # [serde (skip_serializing_if = "Option::is_none")] pub no_nested_component_definitions : Option < RuleConfiguration < biome_rule_options :: no_nested_component_definitions :: NoNestedComponentDefinitionsOptions >> , # [doc = "Forbid the use of Node.js builtin modules."] # [serde (skip_serializing_if = "Option::is_none")] pub no_nodejs_modules : Option < RuleConfiguration < biome_rule_options :: no_nodejs_modules :: NoNodejsModulesOptions >> , # [doc = "Disallow \\8 and \\9 escape sequences in string literals."] # [serde (skip_serializing_if = "Option::is_none")] pub no_nonoctal_decimal_escape : Option < RuleFixConfiguration < biome_rule_options :: no_nonoctal_decimal_escape :: NoNonoctalDecimalEscapeOptions >> , # [doc = "Disallow literal numbers that lose precision"] # [serde (skip_serializing_if = "Option::is_none")] pub no_precision_loss : Option < RuleConfiguration < biome_rule_options :: no_precision_loss :: NoPrecisionLossOptions >> , # [doc = "Restrict imports of private exports."] # [serde (skip_serializing_if = "Option::is_none")] pub no_private_imports : Option < RuleConfiguration < biome_rule_options :: no_private_imports :: NoPrivateImportsOptions >> , # [doc = "Disallow the use of process global."] # [serde (skip_serializing_if = "Option::is_none")] pub no_process_global : Option < RuleFixConfiguration < biome_rule_options :: no_process_global :: NoProcessGlobalOptions >> , # [doc = "Disallow assigning to React component props."] # [serde (skip_serializing_if = "Option::is_none")] pub no_react_prop_assignments : Option < RuleConfiguration < biome_rule_options :: no_react_prop_assignments :: NoReactPropAssignmentsOptions >> , # [doc = "Prevent the usage of the return value of React.render."] # [serde (skip_serializing_if = "Option::is_none")] pub no_render_return_value : Option < RuleConfiguration < biome_rule_options :: no_render_return_value :: NoRenderReturnValueOptions >> , # [doc = "Disallow the use of configured elements."] # [serde (skip_serializing_if = "Option::is_none")] pub no_restricted_elements : Option < RuleConfiguration < biome_rule_options :: no_restricted_elements :: NoRestrictedElementsOptions >> , # [doc = "Disallow assignments where both sides are exactly the same."] # [serde (skip_serializing_if = "Option::is_none")] pub no_self_assign : Option < RuleConfiguration < biome_rule_options :: no_self_assign :: NoSelfAssignOptions >> , # [doc = "Disallow returning a value from a setter"] # [serde (skip_serializing_if = "Option::is_none")] pub no_setter_return : Option < RuleConfiguration < biome_rule_options :: no_setter_return :: NoSetterReturnOptions >> , # [doc = "Disallow destructuring props inside JSX components in Solid projects."] # [serde (skip_serializing_if = "Option::is_none")] pub no_solid_destructured_props : Option < RuleConfiguration < biome_rule_options :: no_solid_destructured_props :: NoSolidDestructuredPropsOptions >> , # [doc = "Disallow comparison of expressions modifying the string case with non-compliant value."] # [serde (skip_serializing_if = "Option::is_none")] pub no_string_case_mismatch : Option < RuleFixConfiguration < biome_rule_options :: no_string_case_mismatch :: NoStringCaseMismatchOptions >> , # [doc = "Disallow lexical declarations in switch clauses."] # [serde (skip_serializing_if = "Option::is_none")] pub no_switch_declarations : Option < RuleFixConfiguration < biome_rule_options :: no_switch_declarations :: NoSwitchDeclarationsOptions >> , # [doc = "Disallow the use of dependencies that aren't specified in the package.json."] # [serde (skip_serializing_if = "Option::is_none")] pub no_undeclared_dependencies : Option < RuleConfiguration < biome_rule_options :: no_undeclared_dependencies :: NoUndeclaredDependenciesOptions >> , # [doc = "Prevents the usage of variables that haven't been declared inside the document."] # [serde (skip_serializing_if = "Option::is_none")] pub no_undeclared_variables : Option < RuleConfiguration < biome_rule_options :: no_undeclared_variables :: NoUndeclaredVariablesOptions >> , # [doc = "Disallow unknown CSS value functions."] # [serde (skip_serializing_if = "Option::is_none")] pub no_unknown_function : Option < RuleConfiguration < biome_rule_options :: no_unknown_function :: NoUnknownFunctionOptions >> , # [doc = "Disallow unknown media feature names."] # [serde (skip_serializing_if = "Option::is_none")] pub no_unknown_media_feature_name : Option < RuleConfiguration < biome_rule_options :: no_unknown_media_feature_name :: NoUnknownMediaFeatureNameOptions >> , # [doc = "Disallow unknown properties."] # [serde (skip_serializing_if = "Option::is_none")] pub no_unknown_property : Option < RuleConfiguration < biome_rule_options :: no_unknown_property :: NoUnknownPropertyOptions >> , # [doc = "Disallow unknown pseudo-class selectors."] # [serde (skip_serializing_if = "Option::is_none")] pub no_unknown_pseudo_class : Option < RuleConfiguration < biome_rule_options :: no_unknown_pseudo_class :: NoUnknownPseudoClassOptions >> , # [doc = "Disallow unknown pseudo-element selectors."] # [serde (skip_serializing_if = "Option::is_none")] pub no_unknown_pseudo_element : Option < RuleConfiguration < biome_rule_options :: no_unknown_pseudo_element :: NoUnknownPseudoElementOptions >> , # [doc = "Disallow unknown type selectors."] # [serde (skip_serializing_if = "Option::is_none")] pub no_unknown_type_selector : Option < RuleConfiguration < biome_rule_options :: no_unknown_type_selector :: NoUnknownTypeSelectorOptions >> , # [doc = "Disallow unknown CSS units."] # [serde (skip_serializing_if = "Option::is_none")] pub no_unknown_unit : Option < RuleConfiguration < biome_rule_options :: no_unknown_unit :: NoUnknownUnitOptions >> , # [doc = "Disallow unmatchable An+B selectors."] # [serde (skip_serializing_if = "Option::is_none")] pub no_unmatchable_anb_selector : Option < RuleConfiguration < biome_rule_options :: no_unmatchable_anb_selector :: NoUnmatchableAnbSelectorOptions >> , # [doc = "Disallow unreachable code"] # [serde (skip_serializing_if = "Option::is_none")] pub no_unreachable : Option < RuleConfiguration < biome_rule_options :: no_unreachable :: NoUnreachableOptions >> , # [doc = "Ensures the super() constructor is called exactly once on every code  path in a class constructor before this is accessed if the class has a superclass"] # [serde (skip_serializing_if = "Option::is_none")] pub no_unreachable_super : Option < RuleConfiguration < biome_rule_options :: no_unreachable_super :: NoUnreachableSuperOptions >> , # [doc = "Disallow control flow statements in finally blocks."] # [serde (skip_serializing_if = "Option::is_none")] pub no_unsafe_finally : Option < RuleConfiguration < biome_rule_options :: no_unsafe_finally :: NoUnsafeFinallyOptions >> , # [doc = "Disallow the use of optional chaining in contexts where the undefined value is not allowed."] # [serde (skip_serializing_if = "Option::is_none")] pub no_unsafe_optional_chaining : Option < RuleConfiguration < biome_rule_options :: no_unsafe_optional_chaining :: NoUnsafeOptionalChainingOptions >> , # [doc = "Disallow unused function parameters."] # [serde (skip_serializing_if = "Option::is_none")] pub no_unused_function_parameters : Option < RuleFixConfiguration < biome_rule_options :: no_unused_function_parameters :: NoUnusedFunctionParametersOptions >> , # [doc = "Disallow unused imports."] # [serde (skip_serializing_if = "Option::is_none")] pub no_unused_imports : Option < RuleFixConfiguration < biome_rule_options :: no_unused_imports :: NoUnusedImportsOptions >> , # [doc = "Disallow unused labels."] # [serde (skip_serializing_if = "Option::is_none")] pub no_unused_labels : Option < RuleFixConfiguration < biome_rule_options :: no_unused_labels :: NoUnusedLabelsOptions >> , # [doc = "Disallow unused private class members"] # [serde (skip_serializing_if = "Option::is_none")] pub no_unused_private_class_members : Option < RuleFixConfiguration < biome_rule_options :: no_unused_private_class_members :: NoUnusedPrivateClassMembersOptions >> , # [doc = "Disallow unused variables."] # [serde (skip_serializing_if = "Option::is_none")] pub no_unused_variables : Option < RuleFixConfiguration < biome_rule_options :: no_unused_variables :: NoUnusedVariablesOptions >> , # [doc = "This rules prevents void elements (AKA self-closing elements) from having children."] # [serde (skip_serializing_if = "Option::is_none")] pub no_void_elements_with_children : Option < RuleFixConfiguration < biome_rule_options :: no_void_elements_with_children :: NoVoidElementsWithChildrenOptions >> , # [doc = "Disallow returning a value from a function with the return type 'void'"] # [serde (skip_serializing_if = "Option::is_none")] pub no_void_type_return : Option < RuleConfiguration < biome_rule_options :: no_void_type_return :: NoVoidTypeReturnOptions >> , # [doc = "Enforce all dependencies are correctly specified in a React hook."] # [serde (skip_serializing_if = "Option::is_none")] pub use_exhaustive_dependencies : Option < RuleFixConfiguration < biome_rule_options :: use_exhaustive_dependencies :: UseExhaustiveDependenciesOptions >> , # [doc = "Enforce specifying the name of GraphQL operations."] # [serde (skip_serializing_if = "Option::is_none")] pub use_graphql_named_operations : Option < RuleFixConfiguration < biome_rule_options :: use_graphql_named_operations :: UseGraphqlNamedOperationsOptions >> , # [doc = "Enforce that all React hooks are being called from the Top Level component functions."] # [serde (skip_serializing_if = "Option::is_none")] pub use_hook_at_top_level : Option < RuleConfiguration < biome_rule_options :: use_hook_at_top_level :: UseHookAtTopLevelOptions >> , # [doc = "Enforce file extensions for relative imports."] # [serde (skip_serializing_if = "Option::is_none")] pub use_import_extensions : Option < RuleFixConfiguration < biome_rule_options :: use_import_extensions :: UseImportExtensionsOptions >> , # [doc = "Require calls to isNaN() when checking for NaN."] # [serde (skip_serializing_if = "Option::is_none")] pub use_is_nan : Option < RuleFixConfiguration < biome_rule_options :: use_is_nan :: UseIsNanOptions >> , # [doc = "Enforces the use of with { type: \"json\" } for JSON module imports."] # [serde (skip_serializing_if = "Option::is_none")] pub use_json_import_attributes : Option < RuleFixConfiguration < biome_rule_options :: use_json_import_attributes :: UseJsonImportAttributesOptions >> , # [doc = "Disallow missing key props in iterators/collection literals."] # [serde (skip_serializing_if = "Option::is_none")] pub use_jsx_key_in_iterable : Option < RuleConfiguration < biome_rule_options :: use_jsx_key_in_iterable :: UseJsxKeyInIterableOptions >> , # [doc = "Enforce the consistent use of the radix argument when using parseInt()."] # [serde (skip_serializing_if = "Option::is_none")] pub use_parse_int_radix : Option < RuleFixConfiguration < biome_rule_options :: use_parse_int_radix :: UseParseIntRadixOptions >> , # [doc = "Enforce JSDoc comment lines to start with a single asterisk, except for the first one."] # [serde (skip_serializing_if = "Option::is_none")] pub use_single_js_doc_asterisk : Option < RuleFixConfiguration < biome_rule_options :: use_single_js_doc_asterisk :: UseSingleJsDocAsteriskOptions >> , # [doc = "Prevent the usage of static string literal id attribute on elements."] # [serde (skip_serializing_if = "Option::is_none")] pub use_unique_element_ids : Option < RuleConfiguration < biome_rule_options :: use_unique_element_ids :: UseUniqueElementIdsOptions >> , # [doc = "Enforce \"for\" loop update clause moving the counter in the right direction."] # [serde (skip_serializing_if = "Option::is_none")] pub use_valid_for_direction : Option < RuleConfiguration < biome_rule_options :: use_valid_for_direction :: UseValidForDirectionOptions >> , # [doc = "This rule checks that the result of a typeof expression is compared to a valid value."] # [serde (skip_serializing_if = "Option::is_none")] pub use_valid_typeof : Option < RuleFixConfiguration < biome_rule_options :: use_valid_typeof :: UseValidTypeofOptions >> , # [doc = "Require generator functions to contain yield."] # [serde (skip_serializing_if = "Option::is_none")] pub use_yield : Option < RuleConfiguration < biome_rule_options :: use_yield :: UseYieldOptions >> }
impl Correctness {
    const GROUP_NAME: &'static str = "correctness";
    pub(crate) const GROUP_RULES: &'static [&'static str] = &[
        "noChildrenProp",
        "noConstAssign",
        "noConstantCondition",
        "noConstantMathMinMaxClamp",
        "noConstructorReturn",
        "noEmptyCharacterClassInRegex",
        "noEmptyPattern",
        "noGlobalDirnameFilename",
        "noGlobalObjectCalls",
        "noInnerDeclarations",
        "noInvalidBuiltinInstantiation",
        "noInvalidConstructorSuper",
        "noInvalidDirectionInLinearGradient",
        "noInvalidGridAreas",
        "noInvalidPositionAtImportRule",
        "noInvalidUseBeforeDeclaration",
        "noMissingVarFunction",
        "noNestedComponentDefinitions",
        "noNodejsModules",
        "noNonoctalDecimalEscape",
        "noPrecisionLoss",
        "noPrivateImports",
        "noProcessGlobal",
        "noReactPropAssignments",
        "noRenderReturnValue",
        "noRestrictedElements",
        "noSelfAssign",
        "noSetterReturn",
        "noSolidDestructuredProps",
        "noStringCaseMismatch",
        "noSwitchDeclarations",
        "noUndeclaredDependencies",
        "noUndeclaredVariables",
        "noUnknownFunction",
        "noUnknownMediaFeatureName",
        "noUnknownProperty",
        "noUnknownPseudoClass",
        "noUnknownPseudoElement",
        "noUnknownTypeSelector",
        "noUnknownUnit",
        "noUnmatchableAnbSelector",
        "noUnreachable",
        "noUnreachableSuper",
        "noUnsafeFinally",
        "noUnsafeOptionalChaining",
        "noUnusedFunctionParameters",
        "noUnusedImports",
        "noUnusedLabels",
        "noUnusedPrivateClassMembers",
        "noUnusedVariables",
        "noVoidElementsWithChildren",
        "noVoidTypeReturn",
        "useExhaustiveDependencies",
        "useGraphqlNamedOperations",
        "useHookAtTopLevel",
        "useImportExtensions",
        "useIsNan",
        "useJsonImportAttributes",
        "useJsxKeyInIterable",
        "useParseIntRadix",
        "useSingleJsDocAsterisk",
        "useUniqueElementIds",
        "useValidForDirection",
        "useValidTypeof",
        "useYield",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[53]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[56]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[59]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[62]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[63]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[64]),
    ];
    const ALL_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[53]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[54]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[55]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[56]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[57]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[58]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[59]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[60]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[61]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[62]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[63]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[64]),
    ];
}
impl RuleGroupExt for Correctness {
    fn is_recommended_true(&self) -> bool {
        matches!(self.recommended, Some(true))
    }
    fn is_recommended_unset(&self) -> bool {
        self.recommended.is_none()
    }
    fn get_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_children_prop.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
        }
        if let Some(rule) = self.no_const_assign.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
        }
        if let Some(rule) = self.no_constant_condition.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
        }
        if let Some(rule) = self.no_constant_math_min_max_clamp.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
        }
        if let Some(rule) = self.no_constructor_return.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
        }
        if let Some(rule) = self.no_empty_character_class_in_regex.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
        }
        if let Some(rule) = self.no_empty_pattern.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
        }
        if let Some(rule) = self.no_global_dirname_filename.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
        }
        if let Some(rule) = self.no_global_object_calls.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
        }
        if let Some(rule) = self.no_inner_declarations.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
        }
        if let Some(rule) = self.no_invalid_builtin_instantiation.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
        }
        if let Some(rule) = self.no_invalid_constructor_super.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
        }
        if let Some(rule) = self.no_invalid_direction_in_linear_gradient.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
        }
        if let Some(rule) = self.no_invalid_grid_areas.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
        }
        if let Some(rule) = self.no_invalid_position_at_import_rule.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
        }
        if let Some(rule) = self.no_invalid_use_before_declaration.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
        }
        if let Some(rule) = self.no_missing_var_function.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
        }
        if let Some(rule) = self.no_nested_component_definitions.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
        }
        if let Some(rule) = self.no_nodejs_modules.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
        }
        if let Some(rule) = self.no_nonoctal_decimal_escape.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
        }
        if let Some(rule) = self.no_precision_loss.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
        }
        if let Some(rule) = self.no_private_imports.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
        }
        if let Some(rule) = self.no_process_global.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
        }
        if let Some(rule) = self.no_react_prop_assignments.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
        }
        if let Some(rule) = self.no_render_return_value.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
        }
        if let Some(rule) = self.no_restricted_elements.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
        }
        if let Some(rule) = self.no_self_assign.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
        }
        if let Some(rule) = self.no_setter_return.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
        }
        if let Some(rule) = self.no_solid_destructured_props.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
        }
        if let Some(rule) = self.no_string_case_mismatch.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
        }
        if let Some(rule) = self.no_switch_declarations.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
        }
        if let Some(rule) = self.no_undeclared_dependencies.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
        }
        if let Some(rule) = self.no_undeclared_variables.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
        }
        if let Some(rule) = self.no_unknown_function.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
        }
        if let Some(rule) = self.no_unknown_media_feature_name.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
        }
        if let Some(rule) = self.no_unknown_property.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
        }
        if let Some(rule) = self.no_unknown_pseudo_class.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
        }
        if let Some(rule) = self.no_unknown_pseudo_element.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
        }
        if let Some(rule) = self.no_unknown_type_selector.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
        }
        if let Some(rule) = self.no_unknown_unit.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
        }
        if let Some(rule) = self.no_unmatchable_anb_selector.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
        }
        if let Some(rule) = self.no_unreachable.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
        }
        if let Some(rule) = self.no_unreachable_super.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
        }
        if let Some(rule) = self.no_unsafe_finally.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]));
        }
        if let Some(rule) = self.no_unsafe_optional_chaining.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]));
        }
        if let Some(rule) = self.no_unused_function_parameters.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]));
        }
        if let Some(rule) = self.no_unused_imports.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]));
        }
        if let Some(rule) = self.no_unused_labels.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]));
        }
        if let Some(rule) = self.no_unused_private_class_members.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]));
        }
        if let Some(rule) = self.no_unused_variables.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]));
        }
        if let Some(rule) = self.no_void_elements_with_children.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]));
        }
        if let Some(rule) = self.no_void_type_return.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]));
        }
        if let Some(rule) = self.use_exhaustive_dependencies.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]));
        }
        if let Some(rule) = self.use_graphql_named_operations.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[53]));
        }
        if let Some(rule) = self.use_hook_at_top_level.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[54]));
        }
        if let Some(rule) = self.use_import_extensions.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[55]));
        }
        if let Some(rule) = self.use_is_nan.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[56]));
        }
        if let Some(rule) = self.use_json_import_attributes.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[57]));
        }
        if let Some(rule) = self.use_jsx_key_in_iterable.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[58]));
        }
        if let Some(rule) = self.use_parse_int_radix.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[59]));
        }
        if let Some(rule) = self.use_single_js_doc_asterisk.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[60]));
        }
        if let Some(rule) = self.use_unique_element_ids.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[61]));
        }
        if let Some(rule) = self.use_valid_for_direction.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[62]));
        }
        if let Some(rule) = self.use_valid_typeof.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[63]));
        }
        if let Some(rule) = self.use_yield.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[64]));
        }
        index_set
    }
    fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_children_prop.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
        }
        if let Some(rule) = self.no_const_assign.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
        }
        if let Some(rule) = self.no_constant_condition.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
        }
        if let Some(rule) = self.no_constant_math_min_max_clamp.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
        }
        if let Some(rule) = self.no_constructor_return.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
        }
        if let Some(rule) = self.no_empty_character_class_in_regex.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
        }
        if let Some(rule) = self.no_empty_pattern.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
        }
        if let Some(rule) = self.no_global_dirname_filename.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
        }
        if let Some(rule) = self.no_global_object_calls.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
        }
        if let Some(rule) = self.no_inner_declarations.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
        }
        if let Some(rule) = self.no_invalid_builtin_instantiation.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
        }
        if let Some(rule) = self.no_invalid_constructor_super.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
        }
        if let Some(rule) = self.no_invalid_direction_in_linear_gradient.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
        }
        if let Some(rule) = self.no_invalid_grid_areas.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
        }
        if let Some(rule) = self.no_invalid_position_at_import_rule.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
        }
        if let Some(rule) = self.no_invalid_use_before_declaration.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
        }
        if let Some(rule) = self.no_missing_var_function.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
        }
        if let Some(rule) = self.no_nested_component_definitions.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
        }
        if let Some(rule) = self.no_nodejs_modules.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
        }
        if let Some(rule) = self.no_nonoctal_decimal_escape.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
        }
        if let Some(rule) = self.no_precision_loss.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
        }
        if let Some(rule) = self.no_private_imports.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
        }
        if let Some(rule) = self.no_process_global.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
        }
        if let Some(rule) = self.no_react_prop_assignments.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
        }
        if let Some(rule) = self.no_render_return_value.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
        }
        if let Some(rule) = self.no_restricted_elements.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
        }
        if let Some(rule) = self.no_self_assign.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
        }
        if let Some(rule) = self.no_setter_return.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
        }
        if let Some(rule) = self.no_solid_destructured_props.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
        }
        if let Some(rule) = self.no_string_case_mismatch.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
        }
        if let Some(rule) = self.no_switch_declarations.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
        }
        if let Some(rule) = self.no_undeclared_dependencies.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
        }
        if let Some(rule) = self.no_undeclared_variables.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
        }
        if let Some(rule) = self.no_unknown_function.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
        }
        if let Some(rule) = self.no_unknown_media_feature_name.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
        }
        if let Some(rule) = self.no_unknown_property.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
        }
        if let Some(rule) = self.no_unknown_pseudo_class.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
        }
        if let Some(rule) = self.no_unknown_pseudo_element.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
        }
        if let Some(rule) = self.no_unknown_type_selector.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
        }
        if let Some(rule) = self.no_unknown_unit.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
        }
        if let Some(rule) = self.no_unmatchable_anb_selector.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
        }
        if let Some(rule) = self.no_unreachable.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
        }
        if let Some(rule) = self.no_unreachable_super.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
        }
        if let Some(rule) = self.no_unsafe_finally.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]));
        }
        if let Some(rule) = self.no_unsafe_optional_chaining.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]));
        }
        if let Some(rule) = self.no_unused_function_parameters.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]));
        }
        if let Some(rule) = self.no_unused_imports.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]));
        }
        if let Some(rule) = self.no_unused_labels.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]));
        }
        if let Some(rule) = self.no_unused_private_class_members.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]));
        }
        if let Some(rule) = self.no_unused_variables.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]));
        }
        if let Some(rule) = self.no_void_elements_with_children.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]));
        }
        if let Some(rule) = self.no_void_type_return.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]));
        }
        if let Some(rule) = self.use_exhaustive_dependencies.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]));
        }
        if let Some(rule) = self.use_graphql_named_operations.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[53]));
        }
        if let Some(rule) = self.use_hook_at_top_level.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[54]));
        }
        if let Some(rule) = self.use_import_extensions.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[55]));
        }
        if let Some(rule) = self.use_is_nan.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[56]));
        }
        if let Some(rule) = self.use_json_import_attributes.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[57]));
        }
        if let Some(rule) = self.use_jsx_key_in_iterable.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[58]));
        }
        if let Some(rule) = self.use_parse_int_radix.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[59]));
        }
        if let Some(rule) = self.use_single_js_doc_asterisk.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[60]));
        }
        if let Some(rule) = self.use_unique_element_ids.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[61]));
        }
        if let Some(rule) = self.use_valid_for_direction.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[62]));
        }
        if let Some(rule) = self.use_valid_typeof.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[63]));
        }
        if let Some(rule) = self.use_yield.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[64]));
        }
        index_set
    }
    #[doc = r" Checks if, given a rule name, matches one of the rules contained in this category"]
    fn has_rule(rule_name: &str) -> Option<&'static str> {
        Some(Self::GROUP_RULES[Self::GROUP_RULES.binary_search(&rule_name).ok()?])
    }
    fn recommended_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
    fn all_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::ALL_RULES_AS_FILTERS
    }
    #[doc = r" Select preset rules"]
    fn collect_preset_rules(
        &self,
        parent_is_recommended: bool,
        enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
    ) {
        if self.is_recommended_true() || self.is_recommended_unset() && parent_is_recommended {
            enabled_rules.extend(Self::recommended_rules_as_filters());
        }
    }
    fn set_recommended(&mut self, value: Option<bool>) {
        self.recommended = value;
    }
    fn get_rule_configuration(
        &self,
        rule_name: &str,
    ) -> Option<(RulePlainConfiguration, Option<RuleOptions>)> {
        match rule_name {
            "noChildrenProp" => self
                .no_children_prop
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noConstAssign" => self
                .no_const_assign
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noConstantCondition" => self
                .no_constant_condition
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noConstantMathMinMaxClamp" => self
                .no_constant_math_min_max_clamp
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noConstructorReturn" => self
                .no_constructor_return
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noEmptyCharacterClassInRegex" => self
                .no_empty_character_class_in_regex
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noEmptyPattern" => self
                .no_empty_pattern
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noGlobalDirnameFilename" => self
                .no_global_dirname_filename
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noGlobalObjectCalls" => self
                .no_global_object_calls
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInnerDeclarations" => self
                .no_inner_declarations
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInvalidBuiltinInstantiation" => self
                .no_invalid_builtin_instantiation
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInvalidConstructorSuper" => self
                .no_invalid_constructor_super
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInvalidDirectionInLinearGradient" => self
                .no_invalid_direction_in_linear_gradient
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInvalidGridAreas" => self
                .no_invalid_grid_areas
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInvalidPositionAtImportRule" => self
                .no_invalid_position_at_import_rule
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInvalidUseBeforeDeclaration" => self
                .no_invalid_use_before_declaration
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noMissingVarFunction" => self
                .no_missing_var_function
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNestedComponentDefinitions" => self
                .no_nested_component_definitions
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNodejsModules" => self
                .no_nodejs_modules
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNonoctalDecimalEscape" => self
                .no_nonoctal_decimal_escape
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noPrecisionLoss" => self
                .no_precision_loss
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noPrivateImports" => self
                .no_private_imports
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noProcessGlobal" => self
                .no_process_global
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noReactPropAssignments" => self
                .no_react_prop_assignments
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noRenderReturnValue" => self
                .no_render_return_value
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noRestrictedElements" => self
                .no_restricted_elements
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSelfAssign" => self
                .no_self_assign
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSetterReturn" => self
                .no_setter_return
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSolidDestructuredProps" => self
                .no_solid_destructured_props
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noStringCaseMismatch" => self
                .no_string_case_mismatch
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSwitchDeclarations" => self
                .no_switch_declarations
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUndeclaredDependencies" => self
                .no_undeclared_dependencies
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUndeclaredVariables" => self
                .no_undeclared_variables
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnknownFunction" => self
                .no_unknown_function
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnknownMediaFeatureName" => self
                .no_unknown_media_feature_name
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnknownProperty" => self
                .no_unknown_property
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnknownPseudoClass" => self
                .no_unknown_pseudo_class
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnknownPseudoElement" => self
                .no_unknown_pseudo_element
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnknownTypeSelector" => self
                .no_unknown_type_selector
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnknownUnit" => self
                .no_unknown_unit
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnmatchableAnbSelector" => self
                .no_unmatchable_anb_selector
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnreachable" => self
                .no_unreachable
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnreachableSuper" => self
                .no_unreachable_super
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnsafeFinally" => self
                .no_unsafe_finally
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnsafeOptionalChaining" => self
                .no_unsafe_optional_chaining
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnusedFunctionParameters" => self
                .no_unused_function_parameters
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnusedImports" => self
                .no_unused_imports
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnusedLabels" => self
                .no_unused_labels
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnusedPrivateClassMembers" => self
                .no_unused_private_class_members
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnusedVariables" => self
                .no_unused_variables
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noVoidElementsWithChildren" => self
                .no_void_elements_with_children
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noVoidTypeReturn" => self
                .no_void_type_return
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useExhaustiveDependencies" => self
                .use_exhaustive_dependencies
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useGraphqlNamedOperations" => self
                .use_graphql_named_operations
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useHookAtTopLevel" => self
                .use_hook_at_top_level
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useImportExtensions" => self
                .use_import_extensions
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useIsNan" => self
                .use_is_nan
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useJsonImportAttributes" => self
                .use_json_import_attributes
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useJsxKeyInIterable" => self
                .use_jsx_key_in_iterable
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useParseIntRadix" => self
                .use_parse_int_radix
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useSingleJsDocAsterisk" => self
                .use_single_js_doc_asterisk
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useUniqueElementIds" => self
                .use_unique_element_ids
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useValidForDirection" => self
                .use_valid_for_direction
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useValidTypeof" => self
                .use_valid_typeof
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useYield" => self
                .use_yield
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            _ => None,
        }
    }
}
impl From<GroupPlainConfiguration> for Correctness {
    fn from(value: GroupPlainConfiguration) -> Self {
        Self {
            recommended: None,
            no_children_prop: Some(value.into()),
            no_const_assign: Some(value.into()),
            no_constant_condition: Some(value.into()),
            no_constant_math_min_max_clamp: Some(value.into()),
            no_constructor_return: Some(value.into()),
            no_empty_character_class_in_regex: Some(value.into()),
            no_empty_pattern: Some(value.into()),
            no_global_dirname_filename: Some(value.into()),
            no_global_object_calls: Some(value.into()),
            no_inner_declarations: Some(value.into()),
            no_invalid_builtin_instantiation: Some(value.into()),
            no_invalid_constructor_super: Some(value.into()),
            no_invalid_direction_in_linear_gradient: Some(value.into()),
            no_invalid_grid_areas: Some(value.into()),
            no_invalid_position_at_import_rule: Some(value.into()),
            no_invalid_use_before_declaration: Some(value.into()),
            no_missing_var_function: Some(value.into()),
            no_nested_component_definitions: Some(value.into()),
            no_nodejs_modules: Some(value.into()),
            no_nonoctal_decimal_escape: Some(value.into()),
            no_precision_loss: Some(value.into()),
            no_private_imports: Some(value.into()),
            no_process_global: Some(value.into()),
            no_react_prop_assignments: Some(value.into()),
            no_render_return_value: Some(value.into()),
            no_restricted_elements: Some(value.into()),
            no_self_assign: Some(value.into()),
            no_setter_return: Some(value.into()),
            no_solid_destructured_props: Some(value.into()),
            no_string_case_mismatch: Some(value.into()),
            no_switch_declarations: Some(value.into()),
            no_undeclared_dependencies: Some(value.into()),
            no_undeclared_variables: Some(value.into()),
            no_unknown_function: Some(value.into()),
            no_unknown_media_feature_name: Some(value.into()),
            no_unknown_property: Some(value.into()),
            no_unknown_pseudo_class: Some(value.into()),
            no_unknown_pseudo_element: Some(value.into()),
            no_unknown_type_selector: Some(value.into()),
            no_unknown_unit: Some(value.into()),
            no_unmatchable_anb_selector: Some(value.into()),
            no_unreachable: Some(value.into()),
            no_unreachable_super: Some(value.into()),
            no_unsafe_finally: Some(value.into()),
            no_unsafe_optional_chaining: Some(value.into()),
            no_unused_function_parameters: Some(value.into()),
            no_unused_imports: Some(value.into()),
            no_unused_labels: Some(value.into()),
            no_unused_private_class_members: Some(value.into()),
            no_unused_variables: Some(value.into()),
            no_void_elements_with_children: Some(value.into()),
            no_void_type_return: Some(value.into()),
            use_exhaustive_dependencies: Some(value.into()),
            use_graphql_named_operations: Some(value.into()),
            use_hook_at_top_level: Some(value.into()),
            use_import_extensions: Some(value.into()),
            use_is_nan: Some(value.into()),
            use_json_import_attributes: Some(value.into()),
            use_jsx_key_in_iterable: Some(value.into()),
            use_parse_int_radix: Some(value.into()),
            use_single_js_doc_asterisk: Some(value.into()),
            use_unique_element_ids: Some(value.into()),
            use_valid_for_direction: Some(value.into()),
            use_valid_typeof: Some(value.into()),
            use_yield: Some(value.into()),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Nursery { # [doc = r" It enables the recommended rules for this group"] # [serde (skip_serializing_if = "Option::is_none")] pub recommended : Option < bool > , # [doc = "Require Promise-like statements to be handled appropriately."] # [serde (skip_serializing_if = "Option::is_none")] pub no_floating_promises : Option < RuleFixConfiguration < biome_rule_options :: no_floating_promises :: NoFloatingPromisesOptions >> , # [doc = "Prevent import cycles."] # [serde (skip_serializing_if = "Option::is_none")] pub no_import_cycles : Option < RuleConfiguration < biome_rule_options :: no_import_cycles :: NoImportCyclesOptions >> , # [doc = "Succinct description of the rule."] # [serde (skip_serializing_if = "Option::is_none")] pub no_jsx_literals : Option < RuleConfiguration < biome_rule_options :: no_jsx_literals :: NoJsxLiteralsOptions >> , # [doc = "Disallow Promises to be used in places where they are almost certainly a mistake."] # [serde (skip_serializing_if = "Option::is_none")] pub no_misused_promises : Option < RuleFixConfiguration < biome_rule_options :: no_misused_promises :: NoMisusedPromisesOptions >> , # [doc = "Prevent client components from being async functions."] # [serde (skip_serializing_if = "Option::is_none")] pub no_next_async_client_component : Option < RuleConfiguration < biome_rule_options :: no_next_async_client_component :: NoNextAsyncClientComponentOptions >> , # [doc = "Disallow non-null assertions after optional chaining expressions."] # [serde (skip_serializing_if = "Option::is_none")] pub no_non_null_asserted_optional_chain : Option < RuleConfiguration < biome_rule_options :: no_non_null_asserted_optional_chain :: NoNonNullAssertedOptionalChainOptions >> , # [doc = "Disallow useVisibleTask$() functions in Qwik components."] # [serde (skip_serializing_if = "Option::is_none")] pub no_qwik_use_visible_task : Option < RuleConfiguration < biome_rule_options :: no_qwik_use_visible_task :: NoQwikUseVisibleTaskOptions >> , # [doc = "Disallow usage of sensitive data such as API keys and tokens."] # [serde (skip_serializing_if = "Option::is_none")] pub no_secrets : Option < RuleConfiguration < biome_rule_options :: no_secrets :: NoSecretsOptions >> , # [doc = "Disallow variable declarations from shadowing variables declared in the outer scope."] # [serde (skip_serializing_if = "Option::is_none")] pub no_shadow : Option < RuleConfiguration < biome_rule_options :: no_shadow :: NoShadowOptions >> , # [doc = "Disallow unnecessary type-based conditions that can be statically determined as redundant."] # [serde (skip_serializing_if = "Option::is_none")] pub no_unnecessary_conditions : Option < RuleConfiguration < biome_rule_options :: no_unnecessary_conditions :: NoUnnecessaryConditionsOptions >> , # [doc = "Warn when importing non-existing exports."] # [serde (skip_serializing_if = "Option::is_none")] pub no_unresolved_imports : Option < RuleConfiguration < biome_rule_options :: no_unresolved_imports :: NoUnresolvedImportsOptions >> , # [doc = "Disallow the use of useless undefined."] # [serde (skip_serializing_if = "Option::is_none")] pub no_useless_undefined : Option < RuleFixConfiguration < biome_rule_options :: no_useless_undefined :: NoUselessUndefinedOptions >> , # [doc = "Enforce that Vue component data options are declared as functions."] # [serde (skip_serializing_if = "Option::is_none")] pub no_vue_data_object_declaration : Option < RuleFixConfiguration < biome_rule_options :: no_vue_data_object_declaration :: NoVueDataObjectDeclarationOptions >> , # [doc = "Disallow reserved keys in Vue component data and computed properties."] # [serde (skip_serializing_if = "Option::is_none")] pub no_vue_reserved_keys : Option < RuleConfiguration < biome_rule_options :: no_vue_reserved_keys :: NoVueReservedKeysOptions >> , # [doc = "Disallow reserved names to be used as props."] # [serde (skip_serializing_if = "Option::is_none")] pub no_vue_reserved_props : Option < RuleConfiguration < biome_rule_options :: no_vue_reserved_props :: NoVueReservedPropsOptions >> , # [doc = "Enforces href attribute for \\<a> elements."] # [serde (skip_serializing_if = "Option::is_none")] pub use_anchor_href : Option < RuleConfiguration < biome_rule_options :: use_anchor_href :: UseAnchorHrefOptions >> , # [doc = "Enforce type definitions to consistently use either interface or type."] # [serde (skip_serializing_if = "Option::is_none")] pub use_consistent_type_definitions : Option < RuleFixConfiguration < biome_rule_options :: use_consistent_type_definitions :: UseConsistentTypeDefinitionsOptions >> , # [doc = "Require switch-case statements to be exhaustive."] # [serde (skip_serializing_if = "Option::is_none")] pub use_exhaustive_switch_cases : Option < RuleFixConfiguration < biome_rule_options :: use_exhaustive_switch_cases :: UseExhaustiveSwitchCasesOptions >> , # [doc = "Enforce types in functions, methods, variables, and parameters."] # [serde (skip_serializing_if = "Option::is_none")] pub use_explicit_type : Option < RuleConfiguration < biome_rule_options :: use_explicit_type :: UseExplicitTypeOptions >> , # [doc = "Enforces that \\<img> elements have both width and height attributes."] # [serde (skip_serializing_if = "Option::is_none")] pub use_image_size : Option < RuleConfiguration < biome_rule_options :: use_image_size :: UseImageSizeOptions >> , # [doc = "Enforce a maximum number of parameters in function definitions."] # [serde (skip_serializing_if = "Option::is_none")] pub use_max_params : Option < RuleConfiguration < biome_rule_options :: use_max_params :: UseMaxParamsOptions >> , # [doc = "Prefer using the class prop as a classlist over the classnames helper."] # [serde (skip_serializing_if = "Option::is_none")] pub use_qwik_classlist : Option < RuleConfiguration < biome_rule_options :: use_qwik_classlist :: UseQwikClasslistOptions >> , # [doc = "Enforce that components are defined as functions and never as classes."] # [serde (skip_serializing_if = "Option::is_none")] pub use_react_function_components : Option < RuleConfiguration < biome_rule_options :: use_react_function_components :: UseReactFunctionComponentsOptions >> , # [doc = "Enforce the sorting of CSS utility classes."] # [serde (skip_serializing_if = "Option::is_none")] pub use_sorted_classes : Option < RuleFixConfiguration < biome_rule_options :: use_sorted_classes :: UseSortedClassesOptions >> }
impl Nursery {
    const GROUP_NAME: &'static str = "nursery";
    pub(crate) const GROUP_RULES: &'static [&'static str] = &[
        "noFloatingPromises",
        "noImportCycles",
        "noJsxLiterals",
        "noMisusedPromises",
        "noNextAsyncClientComponent",
        "noNonNullAssertedOptionalChain",
        "noQwikUseVisibleTask",
        "noSecrets",
        "noShadow",
        "noUnnecessaryConditions",
        "noUnresolvedImports",
        "noUselessUndefined",
        "noVueDataObjectDeclaration",
        "noVueReservedKeys",
        "noVueReservedProps",
        "useAnchorHref",
        "useConsistentTypeDefinitions",
        "useExhaustiveSwitchCases",
        "useExplicitType",
        "useImageSize",
        "useMaxParams",
        "useQwikClasslist",
        "useReactFunctionComponents",
        "useSortedClasses",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] =
        &[RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5])];
    const ALL_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
    ];
}
impl RuleGroupExt for Nursery {
    fn is_recommended_true(&self) -> bool {
        matches!(self.recommended, Some(true))
    }
    fn is_recommended_unset(&self) -> bool {
        self.recommended.is_none()
    }
    fn get_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_floating_promises.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
        }
        if let Some(rule) = self.no_import_cycles.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
        }
        if let Some(rule) = self.no_jsx_literals.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
        }
        if let Some(rule) = self.no_misused_promises.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
        }
        if let Some(rule) = self.no_next_async_client_component.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
        }
        if let Some(rule) = self.no_non_null_asserted_optional_chain.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
        }
        if let Some(rule) = self.no_qwik_use_visible_task.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
        }
        if let Some(rule) = self.no_secrets.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
        }
        if let Some(rule) = self.no_shadow.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
        }
        if let Some(rule) = self.no_unnecessary_conditions.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
        }
        if let Some(rule) = self.no_unresolved_imports.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
        }
        if let Some(rule) = self.no_useless_undefined.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
        }
        if let Some(rule) = self.no_vue_data_object_declaration.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
        }
        if let Some(rule) = self.no_vue_reserved_keys.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
        }
        if let Some(rule) = self.no_vue_reserved_props.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
        }
        if let Some(rule) = self.use_anchor_href.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
        }
        if let Some(rule) = self.use_consistent_type_definitions.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
        }
        if let Some(rule) = self.use_exhaustive_switch_cases.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
        }
        if let Some(rule) = self.use_explicit_type.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
        }
        if let Some(rule) = self.use_image_size.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
        }
        if let Some(rule) = self.use_max_params.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
        }
        if let Some(rule) = self.use_qwik_classlist.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
        }
        if let Some(rule) = self.use_react_function_components.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
        }
        if let Some(rule) = self.use_sorted_classes.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
        }
        index_set
    }
    fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_floating_promises.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
        }
        if let Some(rule) = self.no_import_cycles.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
        }
        if let Some(rule) = self.no_jsx_literals.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
        }
        if let Some(rule) = self.no_misused_promises.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
        }
        if let Some(rule) = self.no_next_async_client_component.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
        }
        if let Some(rule) = self.no_non_null_asserted_optional_chain.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
        }
        if let Some(rule) = self.no_qwik_use_visible_task.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
        }
        if let Some(rule) = self.no_secrets.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
        }
        if let Some(rule) = self.no_shadow.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
        }
        if let Some(rule) = self.no_unnecessary_conditions.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
        }
        if let Some(rule) = self.no_unresolved_imports.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
        }
        if let Some(rule) = self.no_useless_undefined.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
        }
        if let Some(rule) = self.no_vue_data_object_declaration.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
        }
        if let Some(rule) = self.no_vue_reserved_keys.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
        }
        if let Some(rule) = self.no_vue_reserved_props.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
        }
        if let Some(rule) = self.use_anchor_href.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
        }
        if let Some(rule) = self.use_consistent_type_definitions.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
        }
        if let Some(rule) = self.use_exhaustive_switch_cases.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
        }
        if let Some(rule) = self.use_explicit_type.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
        }
        if let Some(rule) = self.use_image_size.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
        }
        if let Some(rule) = self.use_max_params.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
        }
        if let Some(rule) = self.use_qwik_classlist.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
        }
        if let Some(rule) = self.use_react_function_components.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
        }
        if let Some(rule) = self.use_sorted_classes.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
        }
        index_set
    }
    #[doc = r" Checks if, given a rule name, matches one of the rules contained in this category"]
    fn has_rule(rule_name: &str) -> Option<&'static str> {
        Some(Self::GROUP_RULES[Self::GROUP_RULES.binary_search(&rule_name).ok()?])
    }
    fn recommended_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
    fn all_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::ALL_RULES_AS_FILTERS
    }
    #[doc = r" Select preset rules"]
    fn collect_preset_rules(
        &self,
        parent_is_recommended: bool,
        enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
    ) {
        if self.is_recommended_true() || self.is_recommended_unset() && parent_is_recommended {
            enabled_rules.extend(Self::recommended_rules_as_filters());
        }
    }
    fn set_recommended(&mut self, value: Option<bool>) {
        self.recommended = value;
    }
    fn get_rule_configuration(
        &self,
        rule_name: &str,
    ) -> Option<(RulePlainConfiguration, Option<RuleOptions>)> {
        match rule_name {
            "noFloatingPromises" => self
                .no_floating_promises
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noImportCycles" => self
                .no_import_cycles
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noJsxLiterals" => self
                .no_jsx_literals
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noMisusedPromises" => self
                .no_misused_promises
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNextAsyncClientComponent" => self
                .no_next_async_client_component
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNonNullAssertedOptionalChain" => self
                .no_non_null_asserted_optional_chain
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noQwikUseVisibleTask" => self
                .no_qwik_use_visible_task
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSecrets" => self
                .no_secrets
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noShadow" => self
                .no_shadow
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnnecessaryConditions" => self
                .no_unnecessary_conditions
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnresolvedImports" => self
                .no_unresolved_imports
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessUndefined" => self
                .no_useless_undefined
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noVueDataObjectDeclaration" => self
                .no_vue_data_object_declaration
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noVueReservedKeys" => self
                .no_vue_reserved_keys
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noVueReservedProps" => self
                .no_vue_reserved_props
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useAnchorHref" => self
                .use_anchor_href
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useConsistentTypeDefinitions" => self
                .use_consistent_type_definitions
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useExhaustiveSwitchCases" => self
                .use_exhaustive_switch_cases
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useExplicitType" => self
                .use_explicit_type
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useImageSize" => self
                .use_image_size
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useMaxParams" => self
                .use_max_params
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useQwikClasslist" => self
                .use_qwik_classlist
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useReactFunctionComponents" => self
                .use_react_function_components
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useSortedClasses" => self
                .use_sorted_classes
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            _ => None,
        }
    }
}
impl From<GroupPlainConfiguration> for Nursery {
    fn from(value: GroupPlainConfiguration) -> Self {
        Self {
            recommended: None,
            no_floating_promises: Some(value.into()),
            no_import_cycles: Some(value.into()),
            no_jsx_literals: Some(value.into()),
            no_misused_promises: Some(value.into()),
            no_next_async_client_component: Some(value.into()),
            no_non_null_asserted_optional_chain: Some(value.into()),
            no_qwik_use_visible_task: Some(value.into()),
            no_secrets: Some(value.into()),
            no_shadow: Some(value.into()),
            no_unnecessary_conditions: Some(value.into()),
            no_unresolved_imports: Some(value.into()),
            no_useless_undefined: Some(value.into()),
            no_vue_data_object_declaration: Some(value.into()),
            no_vue_reserved_keys: Some(value.into()),
            no_vue_reserved_props: Some(value.into()),
            use_anchor_href: Some(value.into()),
            use_consistent_type_definitions: Some(value.into()),
            use_exhaustive_switch_cases: Some(value.into()),
            use_explicit_type: Some(value.into()),
            use_image_size: Some(value.into()),
            use_max_params: Some(value.into()),
            use_qwik_classlist: Some(value.into()),
            use_react_function_components: Some(value.into()),
            use_sorted_classes: Some(value.into()),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Performance { # [doc = r" It enables the recommended rules for this group"] # [serde (skip_serializing_if = "Option::is_none")] pub recommended : Option < bool > , # [doc = "Disallow the use of spread (...) syntax on accumulators."] # [serde (skip_serializing_if = "Option::is_none")] pub no_accumulating_spread : Option < RuleConfiguration < biome_rule_options :: no_accumulating_spread :: NoAccumulatingSpreadOptions >> , # [doc = "Disallow await inside loops."] # [serde (skip_serializing_if = "Option::is_none")] pub no_await_in_loops : Option < RuleConfiguration < biome_rule_options :: no_await_in_loops :: NoAwaitInLoopsOptions >> , # [doc = "Disallow the use of barrel file."] # [serde (skip_serializing_if = "Option::is_none")] pub no_barrel_file : Option < RuleConfiguration < biome_rule_options :: no_barrel_file :: NoBarrelFileOptions >> , # [doc = "Disallow the use of the delete operator."] # [serde (skip_serializing_if = "Option::is_none")] pub no_delete : Option < RuleFixConfiguration < biome_rule_options :: no_delete :: NoDeleteOptions >> , # [doc = "Disallow accessing namespace imports dynamically."] # [serde (skip_serializing_if = "Option::is_none")] pub no_dynamic_namespace_import_access : Option < RuleConfiguration < biome_rule_options :: no_dynamic_namespace_import_access :: NoDynamicNamespaceImportAccessOptions >> , # [doc = "Prevent usage of \\<img> element in a Next.js project."] # [serde (skip_serializing_if = "Option::is_none")] pub no_img_element : Option < RuleConfiguration < biome_rule_options :: no_img_element :: NoImgElementOptions >> , # [doc = "Disallow the use of namespace imports."] # [serde (skip_serializing_if = "Option::is_none")] pub no_namespace_import : Option < RuleConfiguration < biome_rule_options :: no_namespace_import :: NoNamespaceImportOptions >> , # [doc = "Avoid re-export all."] # [serde (skip_serializing_if = "Option::is_none")] pub no_re_export_all : Option < RuleConfiguration < biome_rule_options :: no_re_export_all :: NoReExportAllOptions >> , # [doc = "Prevent duplicate polyfills from Polyfill.io."] # [serde (skip_serializing_if = "Option::is_none")] pub no_unwanted_polyfillio : Option < RuleConfiguration < biome_rule_options :: no_unwanted_polyfillio :: NoUnwantedPolyfillioOptions >> , # [doc = "Ensure the preconnect attribute is used when using Google Fonts."] # [serde (skip_serializing_if = "Option::is_none")] pub use_google_font_preconnect : Option < RuleFixConfiguration < biome_rule_options :: use_google_font_preconnect :: UseGoogleFontPreconnectOptions >> , # [doc = "Enforce using Solid's \\<For /> component for mapping an array to JSX elements."] # [serde (skip_serializing_if = "Option::is_none")] pub use_solid_for_component : Option < RuleConfiguration < biome_rule_options :: use_solid_for_component :: UseSolidForComponentOptions >> , # [doc = "Require regex literals to be declared at the top level."] # [serde (skip_serializing_if = "Option::is_none")] pub use_top_level_regex : Option < RuleConfiguration < biome_rule_options :: use_top_level_regex :: UseTopLevelRegexOptions >> }
impl Performance {
    const GROUP_NAME: &'static str = "performance";
    pub(crate) const GROUP_RULES: &'static [&'static str] = &[
        "noAccumulatingSpread",
        "noAwaitInLoops",
        "noBarrelFile",
        "noDelete",
        "noDynamicNamespaceImportAccess",
        "noImgElement",
        "noNamespaceImport",
        "noReExportAll",
        "noUnwantedPolyfillio",
        "useGoogleFontPreconnect",
        "useSolidForComponent",
        "useTopLevelRegex",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
    ];
    const ALL_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]),
    ];
}
impl RuleGroupExt for Performance {
    fn is_recommended_true(&self) -> bool {
        matches!(self.recommended, Some(true))
    }
    fn is_recommended_unset(&self) -> bool {
        self.recommended.is_none()
    }
    fn get_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_accumulating_spread.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
        }
        if let Some(rule) = self.no_await_in_loops.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
        }
        if let Some(rule) = self.no_barrel_file.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
        }
        if let Some(rule) = self.no_delete.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
        }
        if let Some(rule) = self.no_dynamic_namespace_import_access.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
        }
        if let Some(rule) = self.no_img_element.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
        }
        if let Some(rule) = self.no_namespace_import.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
        }
        if let Some(rule) = self.no_re_export_all.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
        }
        if let Some(rule) = self.no_unwanted_polyfillio.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
        }
        if let Some(rule) = self.use_google_font_preconnect.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
        }
        if let Some(rule) = self.use_solid_for_component.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
        }
        if let Some(rule) = self.use_top_level_regex.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
        }
        index_set
    }
    fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_accumulating_spread.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
        }
        if let Some(rule) = self.no_await_in_loops.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
        }
        if let Some(rule) = self.no_barrel_file.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
        }
        if let Some(rule) = self.no_delete.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
        }
        if let Some(rule) = self.no_dynamic_namespace_import_access.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
        }
        if let Some(rule) = self.no_img_element.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
        }
        if let Some(rule) = self.no_namespace_import.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
        }
        if let Some(rule) = self.no_re_export_all.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
        }
        if let Some(rule) = self.no_unwanted_polyfillio.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
        }
        if let Some(rule) = self.use_google_font_preconnect.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
        }
        if let Some(rule) = self.use_solid_for_component.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
        }
        if let Some(rule) = self.use_top_level_regex.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
        }
        index_set
    }
    #[doc = r" Checks if, given a rule name, matches one of the rules contained in this category"]
    fn has_rule(rule_name: &str) -> Option<&'static str> {
        Some(Self::GROUP_RULES[Self::GROUP_RULES.binary_search(&rule_name).ok()?])
    }
    fn recommended_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
    fn all_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::ALL_RULES_AS_FILTERS
    }
    #[doc = r" Select preset rules"]
    fn collect_preset_rules(
        &self,
        parent_is_recommended: bool,
        enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
    ) {
        if self.is_recommended_true() || self.is_recommended_unset() && parent_is_recommended {
            enabled_rules.extend(Self::recommended_rules_as_filters());
        }
    }
    fn set_recommended(&mut self, value: Option<bool>) {
        self.recommended = value;
    }
    fn get_rule_configuration(
        &self,
        rule_name: &str,
    ) -> Option<(RulePlainConfiguration, Option<RuleOptions>)> {
        match rule_name {
            "noAccumulatingSpread" => self
                .no_accumulating_spread
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noAwaitInLoops" => self
                .no_await_in_loops
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noBarrelFile" => self
                .no_barrel_file
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDelete" => self
                .no_delete
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDynamicNamespaceImportAccess" => self
                .no_dynamic_namespace_import_access
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noImgElement" => self
                .no_img_element
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNamespaceImport" => self
                .no_namespace_import
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noReExportAll" => self
                .no_re_export_all
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnwantedPolyfillio" => self
                .no_unwanted_polyfillio
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useGoogleFontPreconnect" => self
                .use_google_font_preconnect
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useSolidForComponent" => self
                .use_solid_for_component
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useTopLevelRegex" => self
                .use_top_level_regex
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            _ => None,
        }
    }
}
impl From<GroupPlainConfiguration> for Performance {
    fn from(value: GroupPlainConfiguration) -> Self {
        Self {
            recommended: None,
            no_accumulating_spread: Some(value.into()),
            no_await_in_loops: Some(value.into()),
            no_barrel_file: Some(value.into()),
            no_delete: Some(value.into()),
            no_dynamic_namespace_import_access: Some(value.into()),
            no_img_element: Some(value.into()),
            no_namespace_import: Some(value.into()),
            no_re_export_all: Some(value.into()),
            no_unwanted_polyfillio: Some(value.into()),
            use_google_font_preconnect: Some(value.into()),
            use_solid_for_component: Some(value.into()),
            use_top_level_regex: Some(value.into()),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Security { # [doc = r" It enables the recommended rules for this group"] # [serde (skip_serializing_if = "Option::is_none")] pub recommended : Option < bool > , # [doc = "Disallow target=\"_blank\" attribute without rel=\"noopener\"."] # [serde (skip_serializing_if = "Option::is_none")] pub no_blank_target : Option < RuleFixConfiguration < biome_rule_options :: no_blank_target :: NoBlankTargetOptions >> , # [doc = "Prevent the usage of dangerous JSX props"] # [serde (skip_serializing_if = "Option::is_none")] pub no_dangerously_set_inner_html : Option < RuleConfiguration < biome_rule_options :: no_dangerously_set_inner_html :: NoDangerouslySetInnerHtmlOptions >> , # [doc = "Report when a DOM element or a component uses both children and dangerouslySetInnerHTML prop."] # [serde (skip_serializing_if = "Option::is_none")] pub no_dangerously_set_inner_html_with_children : Option < RuleConfiguration < biome_rule_options :: no_dangerously_set_inner_html_with_children :: NoDangerouslySetInnerHtmlWithChildrenOptions >> , # [doc = "Disallow the use of global eval()."] # [serde (skip_serializing_if = "Option::is_none")] pub no_global_eval : Option < RuleConfiguration < biome_rule_options :: no_global_eval :: NoGlobalEvalOptions >> }
impl Security {
    const GROUP_NAME: &'static str = "security";
    pub(crate) const GROUP_RULES: &'static [&'static str] = &[
        "noBlankTarget",
        "noDangerouslySetInnerHtml",
        "noDangerouslySetInnerHtmlWithChildren",
        "noGlobalEval",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
    ];
    const ALL_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
    ];
}
impl RuleGroupExt for Security {
    fn is_recommended_true(&self) -> bool {
        matches!(self.recommended, Some(true))
    }
    fn is_recommended_unset(&self) -> bool {
        self.recommended.is_none()
    }
    fn get_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_blank_target.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
        }
        if let Some(rule) = self.no_dangerously_set_inner_html.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
        }
        if let Some(rule) = self.no_dangerously_set_inner_html_with_children.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
        }
        if let Some(rule) = self.no_global_eval.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
        }
        index_set
    }
    fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_blank_target.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
        }
        if let Some(rule) = self.no_dangerously_set_inner_html.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
        }
        if let Some(rule) = self.no_dangerously_set_inner_html_with_children.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
        }
        if let Some(rule) = self.no_global_eval.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
        }
        index_set
    }
    #[doc = r" Checks if, given a rule name, matches one of the rules contained in this category"]
    fn has_rule(rule_name: &str) -> Option<&'static str> {
        Some(Self::GROUP_RULES[Self::GROUP_RULES.binary_search(&rule_name).ok()?])
    }
    fn recommended_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
    fn all_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::ALL_RULES_AS_FILTERS
    }
    #[doc = r" Select preset rules"]
    fn collect_preset_rules(
        &self,
        parent_is_recommended: bool,
        enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
    ) {
        if self.is_recommended_true() || self.is_recommended_unset() && parent_is_recommended {
            enabled_rules.extend(Self::recommended_rules_as_filters());
        }
    }
    fn set_recommended(&mut self, value: Option<bool>) {
        self.recommended = value;
    }
    fn get_rule_configuration(
        &self,
        rule_name: &str,
    ) -> Option<(RulePlainConfiguration, Option<RuleOptions>)> {
        match rule_name {
            "noBlankTarget" => self
                .no_blank_target
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDangerouslySetInnerHtml" => self
                .no_dangerously_set_inner_html
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDangerouslySetInnerHtmlWithChildren" => self
                .no_dangerously_set_inner_html_with_children
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noGlobalEval" => self
                .no_global_eval
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            _ => None,
        }
    }
}
impl From<GroupPlainConfiguration> for Security {
    fn from(value: GroupPlainConfiguration) -> Self {
        Self {
            recommended: None,
            no_blank_target: Some(value.into()),
            no_dangerously_set_inner_html: Some(value.into()),
            no_dangerously_set_inner_html_with_children: Some(value.into()),
            no_global_eval: Some(value.into()),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Style { # [doc = r" It enables the recommended rules for this group"] # [serde (skip_serializing_if = "Option::is_none")] pub recommended : Option < bool > , # [doc = "Disallow use of CommonJs module system in favor of ESM style imports."] # [serde (skip_serializing_if = "Option::is_none")] pub no_common_js : Option < RuleConfiguration < biome_rule_options :: no_common_js :: NoCommonJsOptions >> , # [doc = "Disallow default exports."] # [serde (skip_serializing_if = "Option::is_none")] pub no_default_export : Option < RuleConfiguration < biome_rule_options :: no_default_export :: NoDefaultExportOptions >> , # [doc = "Disallow a lower specificity selector from coming after a higher specificity selector."] # [serde (skip_serializing_if = "Option::is_none")] pub no_descending_specificity : Option < RuleConfiguration < biome_rule_options :: no_descending_specificity :: NoDescendingSpecificityOptions >> , # [doc = "Disallow using a callback in asynchronous tests and hooks."] # [serde (skip_serializing_if = "Option::is_none")] pub no_done_callback : Option < RuleConfiguration < biome_rule_options :: no_done_callback :: NoDoneCallbackOptions >> , # [doc = "Disallow TypeScript enum."] # [serde (skip_serializing_if = "Option::is_none")] pub no_enum : Option < RuleConfiguration < biome_rule_options :: no_enum :: NoEnumOptions >> , # [doc = "Disallow exporting an imported variable."] # [serde (skip_serializing_if = "Option::is_none")] pub no_exported_imports : Option < RuleConfiguration < biome_rule_options :: no_exported_imports :: NoExportedImportsOptions >> , # [doc = "Prevent usage of \\<head> element in a Next.js project."] # [serde (skip_serializing_if = "Option::is_none")] pub no_head_element : Option < RuleConfiguration < biome_rule_options :: no_head_element :: NoHeadElementOptions >> , # [doc = "Disallow implicit true values on JSX boolean attributes"] # [serde (skip_serializing_if = "Option::is_none")] pub no_implicit_boolean : Option < RuleFixConfiguration < biome_rule_options :: no_implicit_boolean :: NoImplicitBooleanOptions >> , # [doc = "Disallow type annotations for variables, parameters, and class properties initialized with a literal expression."] # [serde (skip_serializing_if = "Option::is_none")] pub no_inferrable_types : Option < RuleFixConfiguration < biome_rule_options :: no_inferrable_types :: NoInferrableTypesOptions >> , # [doc = "Reports usage of \"magic numbers\" — numbers used directly instead of being assigned to named constants."] # [serde (skip_serializing_if = "Option::is_none")] pub no_magic_numbers : Option < RuleConfiguration < biome_rule_options :: no_magic_numbers :: NoMagicNumbersOptions >> , # [doc = "Disallow the use of TypeScript's namespaces."] # [serde (skip_serializing_if = "Option::is_none")] pub no_namespace : Option < RuleConfiguration < biome_rule_options :: no_namespace :: NoNamespaceOptions >> , # [doc = "Disallow negation in the condition of an if statement if it has an else clause."] # [serde (skip_serializing_if = "Option::is_none")] pub no_negation_else : Option < RuleFixConfiguration < biome_rule_options :: no_negation_else :: NoNegationElseOptions >> , # [doc = "Disallow nested ternary expressions."] # [serde (skip_serializing_if = "Option::is_none")] pub no_nested_ternary : Option < RuleConfiguration < biome_rule_options :: no_nested_ternary :: NoNestedTernaryOptions >> , # [doc = "Disallow non-null assertions using the ! postfix operator."] # [serde (skip_serializing_if = "Option::is_none")] pub no_non_null_assertion : Option < RuleFixConfiguration < biome_rule_options :: no_non_null_assertion :: NoNonNullAssertionOptions >> , # [doc = "Disallow reassigning function parameters."] # [serde (skip_serializing_if = "Option::is_none")] pub no_parameter_assign : Option < RuleConfiguration < biome_rule_options :: no_parameter_assign :: NoParameterAssignOptions >> , # [doc = "Disallow the use of parameter properties in class constructors."] # [serde (skip_serializing_if = "Option::is_none")] pub no_parameter_properties : Option < RuleConfiguration < biome_rule_options :: no_parameter_properties :: NoParameterPropertiesOptions >> , # [doc = "Disallow the use of process.env."] # [serde (skip_serializing_if = "Option::is_none")] pub no_process_env : Option < RuleConfiguration < biome_rule_options :: no_process_env :: NoProcessEnvOptions >> , # [doc = "This rule allows you to specify global variable names that you don’t want to use in your application."] # [serde (skip_serializing_if = "Option::is_none")] pub no_restricted_globals : Option < RuleConfiguration < biome_rule_options :: no_restricted_globals :: NoRestrictedGlobalsOptions >> , # [doc = "Disallow specified modules when loaded by import or require."] # [serde (skip_serializing_if = "Option::is_none")] pub no_restricted_imports : Option < RuleConfiguration < biome_rule_options :: no_restricted_imports :: NoRestrictedImportsOptions >> , # [doc = "Disallow user defined types."] # [serde (skip_serializing_if = "Option::is_none")] pub no_restricted_types : Option < RuleFixConfiguration < biome_rule_options :: no_restricted_types :: NoRestrictedTypesOptions >> , # [doc = "Disallow the use of constants which its value is the upper-case version of its name."] # [serde (skip_serializing_if = "Option::is_none")] pub no_shouty_constants : Option < RuleFixConfiguration < biome_rule_options :: no_shouty_constants :: NoShoutyConstantsOptions >> , # [doc = "Enforce the use of String.slice() over String.substr() and String.substring()."] # [serde (skip_serializing_if = "Option::is_none")] pub no_substr : Option < RuleFixConfiguration < biome_rule_options :: no_substr :: NoSubstrOptions >> , # [doc = "Disallow template literals if interpolation and special-character handling are not needed"] # [serde (skip_serializing_if = "Option::is_none")] pub no_unused_template_literal : Option < RuleFixConfiguration < biome_rule_options :: no_unused_template_literal :: NoUnusedTemplateLiteralOptions >> , # [doc = "Disallow else block when the if block breaks early."] # [serde (skip_serializing_if = "Option::is_none")] pub no_useless_else : Option < RuleFixConfiguration < biome_rule_options :: no_useless_else :: NoUselessElseOptions >> , # [doc = "Disallow use of @value rule in css modules."] # [serde (skip_serializing_if = "Option::is_none")] pub no_value_at_rule : Option < RuleConfiguration < biome_rule_options :: no_value_at_rule :: NoValueAtRuleOptions >> , # [doc = "Disallow the use of yoda expressions."] # [serde (skip_serializing_if = "Option::is_none")] pub no_yoda_expression : Option < RuleFixConfiguration < biome_rule_options :: no_yoda_expression :: NoYodaExpressionOptions >> , # [doc = "Disallow Array constructors."] # [serde (skip_serializing_if = "Option::is_none")] pub use_array_literals : Option < RuleFixConfiguration < biome_rule_options :: use_array_literals :: UseArrayLiteralsOptions >> , # [doc = "Enforce the use of as const over literal type and type annotation."] # [serde (skip_serializing_if = "Option::is_none")] pub use_as_const_assertion : Option < RuleFixConfiguration < biome_rule_options :: use_as_const_assertion :: UseAsConstAssertionOptions >> , # [doc = "Use at() instead of integer index access."] # [serde (skip_serializing_if = "Option::is_none")] pub use_at_index : Option < RuleFixConfiguration < biome_rule_options :: use_at_index :: UseAtIndexOptions >> , # [doc = "Requires following curly brace conventions."] # [serde (skip_serializing_if = "Option::is_none")] pub use_block_statements : Option < RuleFixConfiguration < biome_rule_options :: use_block_statements :: UseBlockStatementsOptions >> , # [doc = "Enforce using else if instead of nested if in else clauses."] # [serde (skip_serializing_if = "Option::is_none")] pub use_collapsed_else_if : Option < RuleFixConfiguration < biome_rule_options :: use_collapsed_else_if :: UseCollapsedElseIfOptions >> , # [doc = "Enforce using single if instead of nested if clauses."] # [serde (skip_serializing_if = "Option::is_none")] pub use_collapsed_if : Option < RuleFixConfiguration < biome_rule_options :: use_collapsed_if :: UseCollapsedIfOptions >> , # [doc = "Enforce declaring components only within modules that export React Components exclusively."] # [serde (skip_serializing_if = "Option::is_none")] pub use_component_export_only_modules : Option < RuleConfiguration < biome_rule_options :: use_component_export_only_modules :: UseComponentExportOnlyModulesOptions >> , # [doc = "Require consistently using either T\\[] or Array\\<T>"] # [serde (skip_serializing_if = "Option::is_none")] pub use_consistent_array_type : Option < RuleFixConfiguration < biome_rule_options :: use_consistent_array_type :: UseConsistentArrayTypeOptions >> , # [doc = "Enforce the use of new for all builtins, except String, Number and Boolean."] # [serde (skip_serializing_if = "Option::is_none")] pub use_consistent_builtin_instantiation : Option < RuleFixConfiguration < biome_rule_options :: use_consistent_builtin_instantiation :: UseConsistentBuiltinInstantiationOptions >> , # [doc = "This rule enforces consistent use of curly braces inside JSX attributes and JSX children."] # [serde (skip_serializing_if = "Option::is_none")] pub use_consistent_curly_braces : Option < RuleFixConfiguration < biome_rule_options :: use_consistent_curly_braces :: UseConsistentCurlyBracesOptions >> , # [doc = "Require consistent accessibility modifiers on class properties and methods."] # [serde (skip_serializing_if = "Option::is_none")] pub use_consistent_member_accessibility : Option < RuleConfiguration < biome_rule_options :: use_consistent_member_accessibility :: UseConsistentMemberAccessibilityOptions >> , # [doc = "Require the consistent declaration of object literals. Defaults to explicit definitions."] # [serde (skip_serializing_if = "Option::is_none")] pub use_consistent_object_definitions : Option < RuleFixConfiguration < biome_rule_options :: use_consistent_object_definitions :: UseConsistentObjectDefinitionsOptions >> , # [doc = "Require const declarations for variables that are only assigned once."] # [serde (skip_serializing_if = "Option::is_none")] pub use_const : Option < RuleFixConfiguration < biome_rule_options :: use_const :: UseConstOptions >> , # [doc = "Enforce default function parameters and optional function parameters to be last."] # [serde (skip_serializing_if = "Option::is_none")] pub use_default_parameter_last : Option < RuleFixConfiguration < biome_rule_options :: use_default_parameter_last :: UseDefaultParameterLastOptions >> , # [doc = "Require the default clause in switch statements."] # [serde (skip_serializing_if = "Option::is_none")] pub use_default_switch_clause : Option < RuleConfiguration < biome_rule_options :: use_default_switch_clause :: UseDefaultSwitchClauseOptions >> , # [doc = "Require specifying the reason argument when using @deprecated directive"] # [serde (skip_serializing_if = "Option::is_none")] pub use_deprecated_reason : Option < RuleConfiguration < biome_rule_options :: use_deprecated_reason :: UseDeprecatedReasonOptions >> , # [doc = "Require that each enum member value be explicitly initialized."] # [serde (skip_serializing_if = "Option::is_none")] pub use_enum_initializers : Option < RuleFixConfiguration < biome_rule_options :: use_enum_initializers :: UseEnumInitializersOptions >> , # [doc = "Enforce explicitly comparing the length, size, byteLength or byteOffset property of a value."] # [serde (skip_serializing_if = "Option::is_none")] pub use_explicit_length_check : Option < RuleFixConfiguration < biome_rule_options :: use_explicit_length_check :: UseExplicitLengthCheckOptions >> , # [doc = "Disallow the use of Math.pow in favor of the ** operator."] # [serde (skip_serializing_if = "Option::is_none")] pub use_exponentiation_operator : Option < RuleFixConfiguration < biome_rule_options :: use_exponentiation_operator :: UseExponentiationOperatorOptions >> , # [doc = "Promotes the use of export type for types."] # [serde (skip_serializing_if = "Option::is_none")] pub use_export_type : Option < RuleFixConfiguration < biome_rule_options :: use_export_type :: UseExportTypeOptions >> , # [doc = "Require that all exports are declared after all non-export statements."] # [serde (skip_serializing_if = "Option::is_none")] pub use_exports_last : Option < RuleConfiguration < biome_rule_options :: use_exports_last :: UseExportsLastOptions >> , # [doc = "Enforce naming conventions for JavaScript and TypeScript filenames."] # [serde (skip_serializing_if = "Option::is_none")] pub use_filenaming_convention : Option < RuleConfiguration < biome_rule_options :: use_filenaming_convention :: UseFilenamingConventionOptions >> , # [doc = "Prefer using for...of loops over standard for loops where possible."] # [serde (skip_serializing_if = "Option::is_none")] pub use_for_of : Option < RuleConfiguration < biome_rule_options :: use_for_of :: UseForOfOptions >> , # [doc = "This rule enforces the use of \\<>...\\</> over \\<Fragment>...\\</Fragment>."] # [serde (skip_serializing_if = "Option::is_none")] pub use_fragment_syntax : Option < RuleFixConfiguration < biome_rule_options :: use_fragment_syntax :: UseFragmentSyntaxOptions >> , # [doc = "Validates that all enum values are capitalized."] # [serde (skip_serializing_if = "Option::is_none")] pub use_graphql_naming_convention : Option < RuleConfiguration < biome_rule_options :: use_graphql_naming_convention :: UseGraphqlNamingConventionOptions >> , # [doc = "Enforce that getters and setters for the same property are adjacent in class and object definitions."] # [serde (skip_serializing_if = "Option::is_none")] pub use_grouped_accessor_pairs : Option < RuleConfiguration < biome_rule_options :: use_grouped_accessor_pairs :: UseGroupedAccessorPairsOptions >> , # [doc = "Promotes the use of import type for types."] # [serde (skip_serializing_if = "Option::is_none")] pub use_import_type : Option < RuleFixConfiguration < biome_rule_options :: use_import_type :: UseImportTypeOptions >> , # [doc = "Require all enum members to be literal values."] # [serde (skip_serializing_if = "Option::is_none")] pub use_literal_enum_members : Option < RuleConfiguration < biome_rule_options :: use_literal_enum_members :: UseLiteralEnumMembersOptions >> , # [doc = "Enforce naming conventions for everything across a codebase."] # [serde (skip_serializing_if = "Option::is_none")] pub use_naming_convention : Option < RuleFixConfiguration < biome_rule_options :: use_naming_convention :: UseNamingConventionOptions >> , # [doc = "Promotes the usage of node:assert/strict over node:assert."] # [serde (skip_serializing_if = "Option::is_none")] pub use_node_assert_strict : Option < RuleFixConfiguration < biome_rule_options :: use_node_assert_strict :: UseNodeAssertStrictOptions >> , # [doc = "Enforces using the node: protocol for Node.js builtin modules."] # [serde (skip_serializing_if = "Option::is_none")] pub use_nodejs_import_protocol : Option < RuleFixConfiguration < biome_rule_options :: use_nodejs_import_protocol :: UseNodejsImportProtocolOptions >> , # [doc = "Use the Number properties instead of global ones."] # [serde (skip_serializing_if = "Option::is_none")] pub use_number_namespace : Option < RuleFixConfiguration < biome_rule_options :: use_number_namespace :: UseNumberNamespaceOptions >> , # [doc = "Enforce the use of numeric separators in numeric literals."] # [serde (skip_serializing_if = "Option::is_none")] pub use_numeric_separators : Option < RuleFixConfiguration < biome_rule_options :: use_numeric_separators :: UseNumericSeparatorsOptions >> , # [doc = "Prefer object spread over Object.assign() when constructing new objects."] # [serde (skip_serializing_if = "Option::is_none")] pub use_object_spread : Option < RuleFixConfiguration < biome_rule_options :: use_object_spread :: UseObjectSpreadOptions >> , # [doc = "Enforce marking members as readonly if they are never modified outside the constructor."] # [serde (skip_serializing_if = "Option::is_none")] pub use_readonly_class_properties : Option < RuleFixConfiguration < biome_rule_options :: use_readonly_class_properties :: UseReadonlyClassPropertiesOptions >> , # [doc = "Prevent extra closing tags for components without children."] # [serde (skip_serializing_if = "Option::is_none")] pub use_self_closing_elements : Option < RuleFixConfiguration < biome_rule_options :: use_self_closing_elements :: UseSelfClosingElementsOptions >> , # [doc = "Require assignment operator shorthand where possible."] # [serde (skip_serializing_if = "Option::is_none")] pub use_shorthand_assign : Option < RuleFixConfiguration < biome_rule_options :: use_shorthand_assign :: UseShorthandAssignOptions >> , # [doc = "Enforce using function types instead of object type with call signatures."] # [serde (skip_serializing_if = "Option::is_none")] pub use_shorthand_function_type : Option < RuleFixConfiguration < biome_rule_options :: use_shorthand_function_type :: UseShorthandFunctionTypeOptions >> , # [doc = "Disallow multiple variable declarations in the same variable statement"] # [serde (skip_serializing_if = "Option::is_none")] pub use_single_var_declarator : Option < RuleFixConfiguration < biome_rule_options :: use_single_var_declarator :: UseSingleVarDeclaratorOptions >> , # [doc = "Require a description parameter for the Symbol()."] # [serde (skip_serializing_if = "Option::is_none")] pub use_symbol_description : Option < RuleConfiguration < biome_rule_options :: use_symbol_description :: UseSymbolDescriptionOptions >> , # [doc = "Prefer template literals over string concatenation."] # [serde (skip_serializing_if = "Option::is_none")] pub use_template : Option < RuleFixConfiguration < biome_rule_options :: use_template :: UseTemplateOptions >> , # [doc = "Require new when throwing an error."] # [serde (skip_serializing_if = "Option::is_none")] pub use_throw_new_error : Option < RuleFixConfiguration < biome_rule_options :: use_throw_new_error :: UseThrowNewErrorOptions >> , # [doc = "Disallow throwing non-Error values."] # [serde (skip_serializing_if = "Option::is_none")] pub use_throw_only_error : Option < RuleConfiguration < biome_rule_options :: use_throw_only_error :: UseThrowOnlyErrorOptions >> , # [doc = "Enforce the use of String.trimStart() and String.trimEnd() over String.trimLeft() and String.trimRight()."] # [serde (skip_serializing_if = "Option::is_none")] pub use_trim_start_end : Option < RuleFixConfiguration < biome_rule_options :: use_trim_start_end :: UseTrimStartEndOptions >> , # [doc = "Disallow overload signatures that can be unified into a single signature."] # [serde (skip_serializing_if = "Option::is_none")] pub use_unified_type_signatures : Option < RuleFixConfiguration < biome_rule_options :: use_unified_type_signatures :: UseUnifiedTypeSignaturesOptions >> }
impl Style {
    const GROUP_NAME: &'static str = "style";
    pub(crate) const GROUP_RULES: &'static [&'static str] = &[
        "noCommonJs",
        "noDefaultExport",
        "noDescendingSpecificity",
        "noDoneCallback",
        "noEnum",
        "noExportedImports",
        "noHeadElement",
        "noImplicitBoolean",
        "noInferrableTypes",
        "noMagicNumbers",
        "noNamespace",
        "noNegationElse",
        "noNestedTernary",
        "noNonNullAssertion",
        "noParameterAssign",
        "noParameterProperties",
        "noProcessEnv",
        "noRestrictedGlobals",
        "noRestrictedImports",
        "noRestrictedTypes",
        "noShoutyConstants",
        "noSubstr",
        "noUnusedTemplateLiteral",
        "noUselessElse",
        "noValueAtRule",
        "noYodaExpression",
        "useArrayLiterals",
        "useAsConstAssertion",
        "useAtIndex",
        "useBlockStatements",
        "useCollapsedElseIf",
        "useCollapsedIf",
        "useComponentExportOnlyModules",
        "useConsistentArrayType",
        "useConsistentBuiltinInstantiation",
        "useConsistentCurlyBraces",
        "useConsistentMemberAccessibility",
        "useConsistentObjectDefinitions",
        "useConst",
        "useDefaultParameterLast",
        "useDefaultSwitchClause",
        "useDeprecatedReason",
        "useEnumInitializers",
        "useExplicitLengthCheck",
        "useExponentiationOperator",
        "useExportType",
        "useExportsLast",
        "useFilenamingConvention",
        "useForOf",
        "useFragmentSyntax",
        "useGraphqlNamingConvention",
        "useGroupedAccessorPairs",
        "useImportType",
        "useLiteralEnumMembers",
        "useNamingConvention",
        "useNodeAssertStrict",
        "useNodejsImportProtocol",
        "useNumberNamespace",
        "useNumericSeparators",
        "useObjectSpread",
        "useReadonlyClassProperties",
        "useSelfClosingElements",
        "useShorthandAssign",
        "useShorthandFunctionType",
        "useSingleVarDeclarator",
        "useSymbolDescription",
        "useTemplate",
        "useThrowNewError",
        "useThrowOnlyError",
        "useTrimStartEnd",
        "useUnifiedTypeSignatures",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[53]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[56]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[63]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[66]),
    ];
    const ALL_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[53]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[54]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[55]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[56]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[57]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[58]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[59]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[60]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[61]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[62]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[63]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[64]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[65]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[66]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[67]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[68]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[69]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[70]),
    ];
}
impl RuleGroupExt for Style {
    fn is_recommended_true(&self) -> bool {
        matches!(self.recommended, Some(true))
    }
    fn is_recommended_unset(&self) -> bool {
        self.recommended.is_none()
    }
    fn get_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_common_js.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
        }
        if let Some(rule) = self.no_default_export.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
        }
        if let Some(rule) = self.no_descending_specificity.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
        }
        if let Some(rule) = self.no_done_callback.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
        }
        if let Some(rule) = self.no_enum.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
        }
        if let Some(rule) = self.no_exported_imports.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
        }
        if let Some(rule) = self.no_head_element.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
        }
        if let Some(rule) = self.no_implicit_boolean.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
        }
        if let Some(rule) = self.no_inferrable_types.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
        }
        if let Some(rule) = self.no_magic_numbers.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
        }
        if let Some(rule) = self.no_namespace.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
        }
        if let Some(rule) = self.no_negation_else.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
        }
        if let Some(rule) = self.no_nested_ternary.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
        }
        if let Some(rule) = self.no_non_null_assertion.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
        }
        if let Some(rule) = self.no_parameter_assign.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
        }
        if let Some(rule) = self.no_parameter_properties.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
        }
        if let Some(rule) = self.no_process_env.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
        }
        if let Some(rule) = self.no_restricted_globals.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
        }
        if let Some(rule) = self.no_restricted_imports.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
        }
        if let Some(rule) = self.no_restricted_types.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
        }
        if let Some(rule) = self.no_shouty_constants.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
        }
        if let Some(rule) = self.no_substr.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
        }
        if let Some(rule) = self.no_unused_template_literal.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
        }
        if let Some(rule) = self.no_useless_else.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
        }
        if let Some(rule) = self.no_value_at_rule.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
        }
        if let Some(rule) = self.no_yoda_expression.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
        }
        if let Some(rule) = self.use_array_literals.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
        }
        if let Some(rule) = self.use_as_const_assertion.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
        }
        if let Some(rule) = self.use_at_index.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
        }
        if let Some(rule) = self.use_block_statements.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
        }
        if let Some(rule) = self.use_collapsed_else_if.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
        }
        if let Some(rule) = self.use_collapsed_if.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
        }
        if let Some(rule) = self.use_component_export_only_modules.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
        }
        if let Some(rule) = self.use_consistent_array_type.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
        }
        if let Some(rule) = self.use_consistent_builtin_instantiation.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
        }
        if let Some(rule) = self.use_consistent_curly_braces.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
        }
        if let Some(rule) = self.use_consistent_member_accessibility.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
        }
        if let Some(rule) = self.use_consistent_object_definitions.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
        }
        if let Some(rule) = self.use_const.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
        }
        if let Some(rule) = self.use_default_parameter_last.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
        }
        if let Some(rule) = self.use_default_switch_clause.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
        }
        if let Some(rule) = self.use_deprecated_reason.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
        }
        if let Some(rule) = self.use_enum_initializers.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
        }
        if let Some(rule) = self.use_explicit_length_check.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]));
        }
        if let Some(rule) = self.use_exponentiation_operator.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]));
        }
        if let Some(rule) = self.use_export_type.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]));
        }
        if let Some(rule) = self.use_exports_last.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]));
        }
        if let Some(rule) = self.use_filenaming_convention.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]));
        }
        if let Some(rule) = self.use_for_of.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]));
        }
        if let Some(rule) = self.use_fragment_syntax.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]));
        }
        if let Some(rule) = self.use_graphql_naming_convention.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]));
        }
        if let Some(rule) = self.use_grouped_accessor_pairs.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]));
        }
        if let Some(rule) = self.use_import_type.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]));
        }
        if let Some(rule) = self.use_literal_enum_members.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[53]));
        }
        if let Some(rule) = self.use_naming_convention.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[54]));
        }
        if let Some(rule) = self.use_node_assert_strict.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[55]));
        }
        if let Some(rule) = self.use_nodejs_import_protocol.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[56]));
        }
        if let Some(rule) = self.use_number_namespace.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[57]));
        }
        if let Some(rule) = self.use_numeric_separators.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[58]));
        }
        if let Some(rule) = self.use_object_spread.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[59]));
        }
        if let Some(rule) = self.use_readonly_class_properties.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[60]));
        }
        if let Some(rule) = self.use_self_closing_elements.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[61]));
        }
        if let Some(rule) = self.use_shorthand_assign.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[62]));
        }
        if let Some(rule) = self.use_shorthand_function_type.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[63]));
        }
        if let Some(rule) = self.use_single_var_declarator.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[64]));
        }
        if let Some(rule) = self.use_symbol_description.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[65]));
        }
        if let Some(rule) = self.use_template.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[66]));
        }
        if let Some(rule) = self.use_throw_new_error.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[67]));
        }
        if let Some(rule) = self.use_throw_only_error.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[68]));
        }
        if let Some(rule) = self.use_trim_start_end.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[69]));
        }
        if let Some(rule) = self.use_unified_type_signatures.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[70]));
        }
        index_set
    }
    fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_common_js.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
        }
        if let Some(rule) = self.no_default_export.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
        }
        if let Some(rule) = self.no_descending_specificity.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
        }
        if let Some(rule) = self.no_done_callback.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
        }
        if let Some(rule) = self.no_enum.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
        }
        if let Some(rule) = self.no_exported_imports.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
        }
        if let Some(rule) = self.no_head_element.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
        }
        if let Some(rule) = self.no_implicit_boolean.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
        }
        if let Some(rule) = self.no_inferrable_types.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
        }
        if let Some(rule) = self.no_magic_numbers.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
        }
        if let Some(rule) = self.no_namespace.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
        }
        if let Some(rule) = self.no_negation_else.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
        }
        if let Some(rule) = self.no_nested_ternary.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
        }
        if let Some(rule) = self.no_non_null_assertion.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
        }
        if let Some(rule) = self.no_parameter_assign.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
        }
        if let Some(rule) = self.no_parameter_properties.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
        }
        if let Some(rule) = self.no_process_env.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
        }
        if let Some(rule) = self.no_restricted_globals.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
        }
        if let Some(rule) = self.no_restricted_imports.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
        }
        if let Some(rule) = self.no_restricted_types.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
        }
        if let Some(rule) = self.no_shouty_constants.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
        }
        if let Some(rule) = self.no_substr.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
        }
        if let Some(rule) = self.no_unused_template_literal.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
        }
        if let Some(rule) = self.no_useless_else.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
        }
        if let Some(rule) = self.no_value_at_rule.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
        }
        if let Some(rule) = self.no_yoda_expression.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
        }
        if let Some(rule) = self.use_array_literals.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
        }
        if let Some(rule) = self.use_as_const_assertion.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
        }
        if let Some(rule) = self.use_at_index.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
        }
        if let Some(rule) = self.use_block_statements.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
        }
        if let Some(rule) = self.use_collapsed_else_if.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
        }
        if let Some(rule) = self.use_collapsed_if.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
        }
        if let Some(rule) = self.use_component_export_only_modules.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
        }
        if let Some(rule) = self.use_consistent_array_type.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
        }
        if let Some(rule) = self.use_consistent_builtin_instantiation.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
        }
        if let Some(rule) = self.use_consistent_curly_braces.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
        }
        if let Some(rule) = self.use_consistent_member_accessibility.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
        }
        if let Some(rule) = self.use_consistent_object_definitions.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
        }
        if let Some(rule) = self.use_const.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
        }
        if let Some(rule) = self.use_default_parameter_last.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
        }
        if let Some(rule) = self.use_default_switch_clause.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
        }
        if let Some(rule) = self.use_deprecated_reason.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
        }
        if let Some(rule) = self.use_enum_initializers.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
        }
        if let Some(rule) = self.use_explicit_length_check.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]));
        }
        if let Some(rule) = self.use_exponentiation_operator.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]));
        }
        if let Some(rule) = self.use_export_type.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]));
        }
        if let Some(rule) = self.use_exports_last.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]));
        }
        if let Some(rule) = self.use_filenaming_convention.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]));
        }
        if let Some(rule) = self.use_for_of.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]));
        }
        if let Some(rule) = self.use_fragment_syntax.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]));
        }
        if let Some(rule) = self.use_graphql_naming_convention.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]));
        }
        if let Some(rule) = self.use_grouped_accessor_pairs.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]));
        }
        if let Some(rule) = self.use_import_type.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]));
        }
        if let Some(rule) = self.use_literal_enum_members.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[53]));
        }
        if let Some(rule) = self.use_naming_convention.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[54]));
        }
        if let Some(rule) = self.use_node_assert_strict.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[55]));
        }
        if let Some(rule) = self.use_nodejs_import_protocol.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[56]));
        }
        if let Some(rule) = self.use_number_namespace.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[57]));
        }
        if let Some(rule) = self.use_numeric_separators.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[58]));
        }
        if let Some(rule) = self.use_object_spread.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[59]));
        }
        if let Some(rule) = self.use_readonly_class_properties.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[60]));
        }
        if let Some(rule) = self.use_self_closing_elements.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[61]));
        }
        if let Some(rule) = self.use_shorthand_assign.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[62]));
        }
        if let Some(rule) = self.use_shorthand_function_type.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[63]));
        }
        if let Some(rule) = self.use_single_var_declarator.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[64]));
        }
        if let Some(rule) = self.use_symbol_description.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[65]));
        }
        if let Some(rule) = self.use_template.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[66]));
        }
        if let Some(rule) = self.use_throw_new_error.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[67]));
        }
        if let Some(rule) = self.use_throw_only_error.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[68]));
        }
        if let Some(rule) = self.use_trim_start_end.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[69]));
        }
        if let Some(rule) = self.use_unified_type_signatures.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[70]));
        }
        index_set
    }
    #[doc = r" Checks if, given a rule name, matches one of the rules contained in this category"]
    fn has_rule(rule_name: &str) -> Option<&'static str> {
        Some(Self::GROUP_RULES[Self::GROUP_RULES.binary_search(&rule_name).ok()?])
    }
    fn recommended_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
    fn all_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::ALL_RULES_AS_FILTERS
    }
    #[doc = r" Select preset rules"]
    fn collect_preset_rules(
        &self,
        parent_is_recommended: bool,
        enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
    ) {
        if self.is_recommended_true() || self.is_recommended_unset() && parent_is_recommended {
            enabled_rules.extend(Self::recommended_rules_as_filters());
        }
    }
    fn set_recommended(&mut self, value: Option<bool>) {
        self.recommended = value;
    }
    fn get_rule_configuration(
        &self,
        rule_name: &str,
    ) -> Option<(RulePlainConfiguration, Option<RuleOptions>)> {
        match rule_name {
            "noCommonJs" => self
                .no_common_js
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDefaultExport" => self
                .no_default_export
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDescendingSpecificity" => self
                .no_descending_specificity
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDoneCallback" => self
                .no_done_callback
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noEnum" => self
                .no_enum
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noExportedImports" => self
                .no_exported_imports
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noHeadElement" => self
                .no_head_element
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noImplicitBoolean" => self
                .no_implicit_boolean
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noInferrableTypes" => self
                .no_inferrable_types
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noMagicNumbers" => self
                .no_magic_numbers
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNamespace" => self
                .no_namespace
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNegationElse" => self
                .no_negation_else
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNestedTernary" => self
                .no_nested_ternary
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noNonNullAssertion" => self
                .no_non_null_assertion
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noParameterAssign" => self
                .no_parameter_assign
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noParameterProperties" => self
                .no_parameter_properties
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noProcessEnv" => self
                .no_process_env
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noRestrictedGlobals" => self
                .no_restricted_globals
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noRestrictedImports" => self
                .no_restricted_imports
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noRestrictedTypes" => self
                .no_restricted_types
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noShoutyConstants" => self
                .no_shouty_constants
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSubstr" => self
                .no_substr
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnusedTemplateLiteral" => self
                .no_unused_template_literal
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessElse" => self
                .no_useless_else
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noValueAtRule" => self
                .no_value_at_rule
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noYodaExpression" => self
                .no_yoda_expression
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useArrayLiterals" => self
                .use_array_literals
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useAsConstAssertion" => self
                .use_as_const_assertion
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useAtIndex" => self
                .use_at_index
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useBlockStatements" => self
                .use_block_statements
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useCollapsedElseIf" => self
                .use_collapsed_else_if
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useCollapsedIf" => self
                .use_collapsed_if
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useComponentExportOnlyModules" => self
                .use_component_export_only_modules
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useConsistentArrayType" => self
                .use_consistent_array_type
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useConsistentBuiltinInstantiation" => self
                .use_consistent_builtin_instantiation
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useConsistentCurlyBraces" => self
                .use_consistent_curly_braces
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useConsistentMemberAccessibility" => self
                .use_consistent_member_accessibility
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useConsistentObjectDefinitions" => self
                .use_consistent_object_definitions
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useConst" => self
                .use_const
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useDefaultParameterLast" => self
                .use_default_parameter_last
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useDefaultSwitchClause" => self
                .use_default_switch_clause
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useDeprecatedReason" => self
                .use_deprecated_reason
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useEnumInitializers" => self
                .use_enum_initializers
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useExplicitLengthCheck" => self
                .use_explicit_length_check
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useExponentiationOperator" => self
                .use_exponentiation_operator
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useExportType" => self
                .use_export_type
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useExportsLast" => self
                .use_exports_last
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useFilenamingConvention" => self
                .use_filenaming_convention
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useForOf" => self
                .use_for_of
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useFragmentSyntax" => self
                .use_fragment_syntax
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useGraphqlNamingConvention" => self
                .use_graphql_naming_convention
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useGroupedAccessorPairs" => self
                .use_grouped_accessor_pairs
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useImportType" => self
                .use_import_type
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useLiteralEnumMembers" => self
                .use_literal_enum_members
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useNamingConvention" => self
                .use_naming_convention
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useNodeAssertStrict" => self
                .use_node_assert_strict
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useNodejsImportProtocol" => self
                .use_nodejs_import_protocol
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useNumberNamespace" => self
                .use_number_namespace
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useNumericSeparators" => self
                .use_numeric_separators
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useObjectSpread" => self
                .use_object_spread
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useReadonlyClassProperties" => self
                .use_readonly_class_properties
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useSelfClosingElements" => self
                .use_self_closing_elements
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useShorthandAssign" => self
                .use_shorthand_assign
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useShorthandFunctionType" => self
                .use_shorthand_function_type
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useSingleVarDeclarator" => self
                .use_single_var_declarator
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useSymbolDescription" => self
                .use_symbol_description
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useTemplate" => self
                .use_template
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useThrowNewError" => self
                .use_throw_new_error
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useThrowOnlyError" => self
                .use_throw_only_error
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useTrimStartEnd" => self
                .use_trim_start_end
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useUnifiedTypeSignatures" => self
                .use_unified_type_signatures
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            _ => None,
        }
    }
}
impl From<GroupPlainConfiguration> for Style {
    fn from(value: GroupPlainConfiguration) -> Self {
        Self {
            recommended: None,
            no_common_js: Some(value.into()),
            no_default_export: Some(value.into()),
            no_descending_specificity: Some(value.into()),
            no_done_callback: Some(value.into()),
            no_enum: Some(value.into()),
            no_exported_imports: Some(value.into()),
            no_head_element: Some(value.into()),
            no_implicit_boolean: Some(value.into()),
            no_inferrable_types: Some(value.into()),
            no_magic_numbers: Some(value.into()),
            no_namespace: Some(value.into()),
            no_negation_else: Some(value.into()),
            no_nested_ternary: Some(value.into()),
            no_non_null_assertion: Some(value.into()),
            no_parameter_assign: Some(value.into()),
            no_parameter_properties: Some(value.into()),
            no_process_env: Some(value.into()),
            no_restricted_globals: Some(value.into()),
            no_restricted_imports: Some(value.into()),
            no_restricted_types: Some(value.into()),
            no_shouty_constants: Some(value.into()),
            no_substr: Some(value.into()),
            no_unused_template_literal: Some(value.into()),
            no_useless_else: Some(value.into()),
            no_value_at_rule: Some(value.into()),
            no_yoda_expression: Some(value.into()),
            use_array_literals: Some(value.into()),
            use_as_const_assertion: Some(value.into()),
            use_at_index: Some(value.into()),
            use_block_statements: Some(value.into()),
            use_collapsed_else_if: Some(value.into()),
            use_collapsed_if: Some(value.into()),
            use_component_export_only_modules: Some(value.into()),
            use_consistent_array_type: Some(value.into()),
            use_consistent_builtin_instantiation: Some(value.into()),
            use_consistent_curly_braces: Some(value.into()),
            use_consistent_member_accessibility: Some(value.into()),
            use_consistent_object_definitions: Some(value.into()),
            use_const: Some(value.into()),
            use_default_parameter_last: Some(value.into()),
            use_default_switch_clause: Some(value.into()),
            use_deprecated_reason: Some(value.into()),
            use_enum_initializers: Some(value.into()),
            use_explicit_length_check: Some(value.into()),
            use_exponentiation_operator: Some(value.into()),
            use_export_type: Some(value.into()),
            use_exports_last: Some(value.into()),
            use_filenaming_convention: Some(value.into()),
            use_for_of: Some(value.into()),
            use_fragment_syntax: Some(value.into()),
            use_graphql_naming_convention: Some(value.into()),
            use_grouped_accessor_pairs: Some(value.into()),
            use_import_type: Some(value.into()),
            use_literal_enum_members: Some(value.into()),
            use_naming_convention: Some(value.into()),
            use_node_assert_strict: Some(value.into()),
            use_nodejs_import_protocol: Some(value.into()),
            use_number_namespace: Some(value.into()),
            use_numeric_separators: Some(value.into()),
            use_object_spread: Some(value.into()),
            use_readonly_class_properties: Some(value.into()),
            use_self_closing_elements: Some(value.into()),
            use_shorthand_assign: Some(value.into()),
            use_shorthand_function_type: Some(value.into()),
            use_single_var_declarator: Some(value.into()),
            use_symbol_description: Some(value.into()),
            use_template: Some(value.into()),
            use_throw_new_error: Some(value.into()),
            use_throw_only_error: Some(value.into()),
            use_trim_start_end: Some(value.into()),
            use_unified_type_signatures: Some(value.into()),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[doc = r" A list of rules that belong to this group"]
pub struct Suspicious { # [doc = r" It enables the recommended rules for this group"] # [serde (skip_serializing_if = "Option::is_none")] pub recommended : Option < bool > , # [doc = "Disallow the use of alert, confirm, and prompt."] # [serde (skip_serializing_if = "Option::is_none")] pub no_alert : Option < RuleConfiguration < biome_rule_options :: no_alert :: NoAlertOptions >> , # [doc = "Use standard constants instead of approximated literals."] # [serde (skip_serializing_if = "Option::is_none")] pub no_approximative_numeric_constant : Option < RuleFixConfiguration < biome_rule_options :: no_approximative_numeric_constant :: NoApproximativeNumericConstantOptions >> , # [doc = "Discourage the usage of Array index in keys."] # [serde (skip_serializing_if = "Option::is_none")] pub no_array_index_key : Option < RuleConfiguration < biome_rule_options :: no_array_index_key :: NoArrayIndexKeyOptions >> , # [doc = "Disallow assignments in expressions."] # [serde (skip_serializing_if = "Option::is_none")] pub no_assign_in_expressions : Option < RuleConfiguration < biome_rule_options :: no_assign_in_expressions :: NoAssignInExpressionsOptions >> , # [doc = "Disallows using an async function as a Promise executor."] # [serde (skip_serializing_if = "Option::is_none")] pub no_async_promise_executor : Option < RuleConfiguration < biome_rule_options :: no_async_promise_executor :: NoAsyncPromiseExecutorOptions >> , # [doc = "Prevents the use of the ! pattern in the first position of files.includes in the configuration file."] # [serde (skip_serializing_if = "Option::is_none")] pub no_biome_first_exception : Option < RuleFixConfiguration < biome_rule_options :: no_biome_first_exception :: NoBiomeFirstExceptionOptions >> , # [doc = "Disallow bitwise operators."] # [serde (skip_serializing_if = "Option::is_none")] pub no_bitwise_operators : Option < RuleConfiguration < biome_rule_options :: no_bitwise_operators :: NoBitwiseOperatorsOptions >> , # [doc = "Disallow reassigning exceptions in catch clauses."] # [serde (skip_serializing_if = "Option::is_none")] pub no_catch_assign : Option < RuleConfiguration < biome_rule_options :: no_catch_assign :: NoCatchAssignOptions >> , # [doc = "Disallow reassigning class members."] # [serde (skip_serializing_if = "Option::is_none")] pub no_class_assign : Option < RuleConfiguration < biome_rule_options :: no_class_assign :: NoClassAssignOptions >> , # [doc = "Prevent comments from being inserted as text nodes"] # [serde (skip_serializing_if = "Option::is_none")] pub no_comment_text : Option < RuleFixConfiguration < biome_rule_options :: no_comment_text :: NoCommentTextOptions >> , # [doc = "Disallow comparing against -0"] # [serde (skip_serializing_if = "Option::is_none")] pub no_compare_neg_zero : Option < RuleFixConfiguration < biome_rule_options :: no_compare_neg_zero :: NoCompareNegZeroOptions >> , # [doc = "Disallow labeled statements that are not loops."] # [serde (skip_serializing_if = "Option::is_none")] pub no_confusing_labels : Option < RuleConfiguration < biome_rule_options :: no_confusing_labels :: NoConfusingLabelsOptions >> , # [doc = "Disallow void type outside of generic or return types."] # [serde (skip_serializing_if = "Option::is_none")] pub no_confusing_void_type : Option < RuleFixConfiguration < biome_rule_options :: no_confusing_void_type :: NoConfusingVoidTypeOptions >> , # [doc = "Disallow the use of console."] # [serde (skip_serializing_if = "Option::is_none")] pub no_console : Option < RuleFixConfiguration < biome_rule_options :: no_console :: NoConsoleOptions >> , # [doc = "Disallow TypeScript const enum"] # [serde (skip_serializing_if = "Option::is_none")] pub no_const_enum : Option < RuleFixConfiguration < biome_rule_options :: no_const_enum :: NoConstEnumOptions >> , # [doc = "Disallow expressions where the operation doesn't affect the value"] # [serde (skip_serializing_if = "Option::is_none")] pub no_constant_binary_expressions : Option < RuleConfiguration < biome_rule_options :: no_constant_binary_expressions :: NoConstantBinaryExpressionsOptions >> , # [doc = "Prevents from having control characters and some escape sequences that match control characters in regular expression literals."] # [serde (skip_serializing_if = "Option::is_none")] pub no_control_characters_in_regex : Option < RuleConfiguration < biome_rule_options :: no_control_characters_in_regex :: NoControlCharactersInRegexOptions >> , # [doc = "Disallow the use of debugger"] # [serde (skip_serializing_if = "Option::is_none")] pub no_debugger : Option < RuleFixConfiguration < biome_rule_options :: no_debugger :: NoDebuggerOptions >> , # [doc = "Disallow direct assignments to document.cookie."] # [serde (skip_serializing_if = "Option::is_none")] pub no_document_cookie : Option < RuleConfiguration < biome_rule_options :: no_document_cookie :: NoDocumentCookieOptions >> , # [doc = "Prevents importing next/document outside of pages/_document.jsx in Next.js projects."] # [serde (skip_serializing_if = "Option::is_none")] pub no_document_import_in_page : Option < RuleConfiguration < biome_rule_options :: no_document_import_in_page :: NoDocumentImportInPageOptions >> , # [doc = "Require the use of === and !==."] # [serde (skip_serializing_if = "Option::is_none")] pub no_double_equals : Option < RuleFixConfiguration < biome_rule_options :: no_double_equals :: NoDoubleEqualsOptions >> , # [doc = "Disallow duplicate @import rules."] # [serde (skip_serializing_if = "Option::is_none")] pub no_duplicate_at_import_rules : Option < RuleConfiguration < biome_rule_options :: no_duplicate_at_import_rules :: NoDuplicateAtImportRulesOptions >> , # [doc = "Disallow duplicate case labels."] # [serde (skip_serializing_if = "Option::is_none")] pub no_duplicate_case : Option < RuleConfiguration < biome_rule_options :: no_duplicate_case :: NoDuplicateCaseOptions >> , # [doc = "Disallow duplicate class members."] # [serde (skip_serializing_if = "Option::is_none")] pub no_duplicate_class_members : Option < RuleConfiguration < biome_rule_options :: no_duplicate_class_members :: NoDuplicateClassMembersOptions >> , # [doc = "Disallow duplicate custom properties within declaration blocks."] # [serde (skip_serializing_if = "Option::is_none")] pub no_duplicate_custom_properties : Option < RuleConfiguration < biome_rule_options :: no_duplicate_custom_properties :: NoDuplicateCustomPropertiesOptions >> , # [doc = "Disallow duplicate conditions in if-else-if chains"] # [serde (skip_serializing_if = "Option::is_none")] pub no_duplicate_else_if : Option < RuleConfiguration < biome_rule_options :: no_duplicate_else_if :: NoDuplicateElseIfOptions >> , # [doc = "No duplicated fields in GraphQL operations."] # [serde (skip_serializing_if = "Option::is_none")] pub no_duplicate_fields : Option < RuleConfiguration < biome_rule_options :: no_duplicate_fields :: NoDuplicateFieldsOptions >> , # [doc = "Disallow duplicate names within font families."] # [serde (skip_serializing_if = "Option::is_none")] pub no_duplicate_font_names : Option < RuleConfiguration < biome_rule_options :: no_duplicate_font_names :: NoDuplicateFontNamesOptions >> , # [doc = "Prevents JSX properties to be assigned multiple times."] # [serde (skip_serializing_if = "Option::is_none")] pub no_duplicate_jsx_props : Option < RuleConfiguration < biome_rule_options :: no_duplicate_jsx_props :: NoDuplicateJsxPropsOptions >> , # [doc = "Disallow two keys with the same name inside objects."] # [serde (skip_serializing_if = "Option::is_none")] pub no_duplicate_object_keys : Option < RuleConfiguration < biome_rule_options :: no_duplicate_object_keys :: NoDuplicateObjectKeysOptions >> , # [doc = "Disallow duplicate function parameter name."] # [serde (skip_serializing_if = "Option::is_none")] pub no_duplicate_parameters : Option < RuleConfiguration < biome_rule_options :: no_duplicate_parameters :: NoDuplicateParametersOptions >> , # [doc = "Disallow duplicate properties within declaration blocks."] # [serde (skip_serializing_if = "Option::is_none")] pub no_duplicate_properties : Option < RuleConfiguration < biome_rule_options :: no_duplicate_properties :: NoDuplicatePropertiesOptions >> , # [doc = "Disallow duplicate selectors within keyframe blocks."] # [serde (skip_serializing_if = "Option::is_none")] pub no_duplicate_selectors_keyframe_block : Option < RuleConfiguration < biome_rule_options :: no_duplicate_selectors_keyframe_block :: NoDuplicateSelectorsKeyframeBlockOptions >> , # [doc = "A describe block should not contain duplicate hooks."] # [serde (skip_serializing_if = "Option::is_none")] pub no_duplicate_test_hooks : Option < RuleConfiguration < biome_rule_options :: no_duplicate_test_hooks :: NoDuplicateTestHooksOptions >> , # [doc = "Disallow CSS empty blocks."] # [serde (skip_serializing_if = "Option::is_none")] pub no_empty_block : Option < RuleConfiguration < biome_rule_options :: no_empty_block :: NoEmptyBlockOptions >> , # [doc = "Disallow empty block statements and static blocks."] # [serde (skip_serializing_if = "Option::is_none")] pub no_empty_block_statements : Option < RuleConfiguration < biome_rule_options :: no_empty_block_statements :: NoEmptyBlockStatementsOptions >> , # [doc = "Disallow the declaration of empty interfaces."] # [serde (skip_serializing_if = "Option::is_none")] pub no_empty_interface : Option < RuleFixConfiguration < biome_rule_options :: no_empty_interface :: NoEmptyInterfaceOptions >> , # [doc = "Disallow variables from evolving into any type through reassignments."] # [serde (skip_serializing_if = "Option::is_none")] pub no_evolving_types : Option < RuleConfiguration < biome_rule_options :: no_evolving_types :: NoEvolvingTypesOptions >> , # [doc = "Disallow the any type usage."] # [serde (skip_serializing_if = "Option::is_none")] pub no_explicit_any : Option < RuleConfiguration < biome_rule_options :: no_explicit_any :: NoExplicitAnyOptions >> , # [doc = "Disallow using export or module.exports in files containing tests"] # [serde (skip_serializing_if = "Option::is_none")] pub no_exports_in_test : Option < RuleConfiguration < biome_rule_options :: no_exports_in_test :: NoExportsInTestOptions >> , # [doc = "Prevents the wrong usage of the non-null assertion operator (!) in TypeScript files."] # [serde (skip_serializing_if = "Option::is_none")] pub no_extra_non_null_assertion : Option < RuleFixConfiguration < biome_rule_options :: no_extra_non_null_assertion :: NoExtraNonNullAssertionOptions >> , # [doc = "Disallow fallthrough of switch clauses."] # [serde (skip_serializing_if = "Option::is_none")] pub no_fallthrough_switch_clause : Option < RuleConfiguration < biome_rule_options :: no_fallthrough_switch_clause :: NoFallthroughSwitchClauseOptions >> , # [doc = "Disallow focused tests."] # [serde (skip_serializing_if = "Option::is_none")] pub no_focused_tests : Option < RuleFixConfiguration < biome_rule_options :: no_focused_tests :: NoFocusedTestsOptions >> , # [doc = "Disallow reassigning function declarations."] # [serde (skip_serializing_if = "Option::is_none")] pub no_function_assign : Option < RuleConfiguration < biome_rule_options :: no_function_assign :: NoFunctionAssignOptions >> , # [doc = "Disallow assignments to native objects and read-only global variables."] # [serde (skip_serializing_if = "Option::is_none")] pub no_global_assign : Option < RuleConfiguration < biome_rule_options :: no_global_assign :: NoGlobalAssignOptions >> , # [doc = "Use Number.isFinite instead of global isFinite."] # [serde (skip_serializing_if = "Option::is_none")] pub no_global_is_finite : Option < RuleFixConfiguration < biome_rule_options :: no_global_is_finite :: NoGlobalIsFiniteOptions >> , # [doc = "Use Number.isNaN instead of global isNaN."] # [serde (skip_serializing_if = "Option::is_none")] pub no_global_is_nan : Option < RuleFixConfiguration < biome_rule_options :: no_global_is_nan :: NoGlobalIsNanOptions >> , # [doc = "Prevent using the next/head module in pages/_document.js on Next.js projects."] # [serde (skip_serializing_if = "Option::is_none")] pub no_head_import_in_document : Option < RuleConfiguration < biome_rule_options :: no_head_import_in_document :: NoHeadImportInDocumentOptions >> , # [doc = "Disallow use of implicit any type on variable declarations."] # [serde (skip_serializing_if = "Option::is_none")] pub no_implicit_any_let : Option < RuleConfiguration < biome_rule_options :: no_implicit_any_let :: NoImplicitAnyLetOptions >> , # [doc = "Disallow assigning to imported bindings"] # [serde (skip_serializing_if = "Option::is_none")] pub no_import_assign : Option < RuleConfiguration < biome_rule_options :: no_import_assign :: NoImportAssignOptions >> , # [doc = "Disallow invalid !important within keyframe declarations"] # [serde (skip_serializing_if = "Option::is_none")] pub no_important_in_keyframe : Option < RuleConfiguration < biome_rule_options :: no_important_in_keyframe :: NoImportantInKeyframeOptions >> , # [doc = "Disallows the use of irregular whitespace characters."] # [serde (skip_serializing_if = "Option::is_none")] pub no_irregular_whitespace : Option < RuleConfiguration < biome_rule_options :: no_irregular_whitespace :: NoIrregularWhitespaceOptions >> , # [doc = "Disallow labels that share a name with a variable"] # [serde (skip_serializing_if = "Option::is_none")] pub no_label_var : Option < RuleConfiguration < biome_rule_options :: no_label_var :: NoLabelVarOptions >> , # [doc = "Disallow characters made with multiple code points in character class syntax."] # [serde (skip_serializing_if = "Option::is_none")] pub no_misleading_character_class : Option < RuleFixConfiguration < biome_rule_options :: no_misleading_character_class :: NoMisleadingCharacterClassOptions >> , # [doc = "Enforce proper usage of new and constructor."] # [serde (skip_serializing_if = "Option::is_none")] pub no_misleading_instantiator : Option < RuleConfiguration < biome_rule_options :: no_misleading_instantiator :: NoMisleadingInstantiatorOptions >> , # [doc = "Checks that the assertion function, for example expect, is placed inside an it() function call."] # [serde (skip_serializing_if = "Option::is_none")] pub no_misplaced_assertion : Option < RuleConfiguration < biome_rule_options :: no_misplaced_assertion :: NoMisplacedAssertionOptions >> , # [doc = "Disallow shorthand assign when variable appears on both sides."] # [serde (skip_serializing_if = "Option::is_none")] pub no_misrefactored_shorthand_assign : Option < RuleFixConfiguration < biome_rule_options :: no_misrefactored_shorthand_assign :: NoMisrefactoredShorthandAssignOptions >> , # [doc = "Disallow octal escape sequences in string literals"] # [serde (skip_serializing_if = "Option::is_none")] pub no_octal_escape : Option < RuleFixConfiguration < biome_rule_options :: no_octal_escape :: NoOctalEscapeOptions >> , # [doc = "Disallow direct use of Object.prototype builtins."] # [serde (skip_serializing_if = "Option::is_none")] pub no_prototype_builtins : Option < RuleFixConfiguration < biome_rule_options :: no_prototype_builtins :: NoPrototypeBuiltinsOptions >> , # [doc = "Disallow the use if quickfix.biome inside editor settings file."] # [serde (skip_serializing_if = "Option::is_none")] pub no_quickfix_biome : Option < RuleFixConfiguration < biome_rule_options :: no_quickfix_biome :: NoQuickfixBiomeOptions >> , # [doc = "Prevents React-specific JSX properties from being used."] # [serde (skip_serializing_if = "Option::is_none")] pub no_react_specific_props : Option < RuleFixConfiguration < biome_rule_options :: no_react_specific_props :: NoReactSpecificPropsOptions >> , # [doc = "Disallow variable, function, class, and type redeclarations in the same scope."] # [serde (skip_serializing_if = "Option::is_none")] pub no_redeclare : Option < RuleConfiguration < biome_rule_options :: no_redeclare :: NoRedeclareOptions >> , # [doc = "Prevents from having redundant \"use strict\"."] # [serde (skip_serializing_if = "Option::is_none")] pub no_redundant_use_strict : Option < RuleFixConfiguration < biome_rule_options :: no_redundant_use_strict :: NoRedundantUseStrictOptions >> , # [doc = "Disallow comparisons where both sides are exactly the same."] # [serde (skip_serializing_if = "Option::is_none")] pub no_self_compare : Option < RuleConfiguration < biome_rule_options :: no_self_compare :: NoSelfCompareOptions >> , # [doc = "Disallow identifiers from shadowing restricted names."] # [serde (skip_serializing_if = "Option::is_none")] pub no_shadow_restricted_names : Option < RuleConfiguration < biome_rule_options :: no_shadow_restricted_names :: NoShadowRestrictedNamesOptions >> , # [doc = "Disallow shorthand properties that override related longhand properties."] # [serde (skip_serializing_if = "Option::is_none")] pub no_shorthand_property_overrides : Option < RuleConfiguration < biome_rule_options :: no_shorthand_property_overrides :: NoShorthandPropertyOverridesOptions >> , # [doc = "Disallow disabled tests."] # [serde (skip_serializing_if = "Option::is_none")] pub no_skipped_tests : Option < RuleFixConfiguration < biome_rule_options :: no_skipped_tests :: NoSkippedTestsOptions >> , # [doc = "Prevents the use of sparse arrays (arrays with holes)."] # [serde (skip_serializing_if = "Option::is_none")] pub no_sparse_array : Option < RuleFixConfiguration < biome_rule_options :: no_sparse_array :: NoSparseArrayOptions >> , # [doc = "It detects possible \"wrong\" semicolons inside JSX elements."] # [serde (skip_serializing_if = "Option::is_none")] pub no_suspicious_semicolon_in_jsx : Option < RuleConfiguration < biome_rule_options :: no_suspicious_semicolon_in_jsx :: NoSuspiciousSemicolonInJsxOptions >> , # [doc = "Disallow template literal placeholder syntax in regular strings."] # [serde (skip_serializing_if = "Option::is_none")] pub no_template_curly_in_string : Option < RuleConfiguration < biome_rule_options :: no_template_curly_in_string :: NoTemplateCurlyInStringOptions >> , # [doc = "Disallow then property."] # [serde (skip_serializing_if = "Option::is_none")] pub no_then_property : Option < RuleConfiguration < biome_rule_options :: no_then_property :: NoThenPropertyOptions >> , # [doc = "Prevents the use of the TypeScript directive @ts-ignore."] # [serde (skip_serializing_if = "Option::is_none")] pub no_ts_ignore : Option < RuleFixConfiguration < biome_rule_options :: no_ts_ignore :: NoTsIgnoreOptions >> , # [doc = "Disallow let or var variables that are read but never assigned."] # [serde (skip_serializing_if = "Option::is_none")] pub no_unassigned_variables : Option < RuleConfiguration < biome_rule_options :: no_unassigned_variables :: NoUnassignedVariablesOptions >> , # [doc = "Disallow unknown at-rules."] # [serde (skip_serializing_if = "Option::is_none")] pub no_unknown_at_rules : Option < RuleConfiguration < biome_rule_options :: no_unknown_at_rules :: NoUnknownAtRulesOptions >> , # [doc = "Disallow unsafe declaration merging between interfaces and classes."] # [serde (skip_serializing_if = "Option::is_none")] pub no_unsafe_declaration_merging : Option < RuleConfiguration < biome_rule_options :: no_unsafe_declaration_merging :: NoUnsafeDeclarationMergingOptions >> , # [doc = "Disallow using unsafe negation."] # [serde (skip_serializing_if = "Option::is_none")] pub no_unsafe_negation : Option < RuleFixConfiguration < biome_rule_options :: no_unsafe_negation :: NoUnsafeNegationOptions >> , # [doc = "Disallow unnecessary escapes in string literals."] # [serde (skip_serializing_if = "Option::is_none")] pub no_useless_escape_in_string : Option < RuleFixConfiguration < biome_rule_options :: no_useless_escape_in_string :: NoUselessEscapeInStringOptions >> , # [doc = "Disallow useless backreferences in regular expression literals that always match an empty string."] # [serde (skip_serializing_if = "Option::is_none")] pub no_useless_regex_backrefs : Option < RuleConfiguration < biome_rule_options :: no_useless_regex_backrefs :: NoUselessRegexBackrefsOptions >> , # [doc = "Disallow the use of var"] # [serde (skip_serializing_if = "Option::is_none")] pub no_var : Option < RuleFixConfiguration < biome_rule_options :: no_var :: NoVarOptions >> , # [doc = "Disallow with statements in non-strict contexts."] # [serde (skip_serializing_if = "Option::is_none")] pub no_with : Option < RuleConfiguration < biome_rule_options :: no_with :: NoWithOptions >> , # [doc = "Disallow the use of overload signatures that are not next to each other."] # [serde (skip_serializing_if = "Option::is_none")] pub use_adjacent_overload_signatures : Option < RuleConfiguration < biome_rule_options :: use_adjacent_overload_signatures :: UseAdjacentOverloadSignaturesOptions >> , # [doc = "Ensure async functions utilize await."] # [serde (skip_serializing_if = "Option::is_none")] pub use_await : Option < RuleConfiguration < biome_rule_options :: use_await :: UseAwaitOptions >> , # [doc = "Promotes the correct usage for ignoring folders in the configuration file."] # [serde (skip_serializing_if = "Option::is_none")] pub use_biome_ignore_folder : Option < RuleFixConfiguration < biome_rule_options :: use_biome_ignore_folder :: UseBiomeIgnoreFolderOptions >> , # [doc = "Enforce default clauses in switch statements to be last"] # [serde (skip_serializing_if = "Option::is_none")] pub use_default_switch_clause_last : Option < RuleConfiguration < biome_rule_options :: use_default_switch_clause_last :: UseDefaultSwitchClauseLastOptions >> , # [doc = "Enforce passing a message value when creating a built-in error."] # [serde (skip_serializing_if = "Option::is_none")] pub use_error_message : Option < RuleConfiguration < biome_rule_options :: use_error_message :: UseErrorMessageOptions >> , # [doc = "Enforce get methods to always return a value."] # [serde (skip_serializing_if = "Option::is_none")] pub use_getter_return : Option < RuleConfiguration < biome_rule_options :: use_getter_return :: UseGetterReturnOptions >> , # [doc = "Enforces the use of a recommended display strategy with Google Fonts."] # [serde (skip_serializing_if = "Option::is_none")] pub use_google_font_display : Option < RuleConfiguration < biome_rule_options :: use_google_font_display :: UseGoogleFontDisplayOptions >> , # [doc = "Require for-in loops to include an if statement."] # [serde (skip_serializing_if = "Option::is_none")] pub use_guard_for_in : Option < RuleConfiguration < biome_rule_options :: use_guard_for_in :: UseGuardForInOptions >> , # [doc = "Use Array.isArray() instead of instanceof Array."] # [serde (skip_serializing_if = "Option::is_none")] pub use_is_array : Option < RuleFixConfiguration < biome_rule_options :: use_is_array :: UseIsArrayOptions >> , # [doc = "Enforce consistent return values in iterable callbacks."] # [serde (skip_serializing_if = "Option::is_none")] pub use_iterable_callback_return : Option < RuleConfiguration < biome_rule_options :: use_iterable_callback_return :: UseIterableCallbackReturnOptions >> , # [doc = "Require using the namespace keyword over the module keyword to declare TypeScript namespaces."] # [serde (skip_serializing_if = "Option::is_none")] pub use_namespace_keyword : Option < RuleFixConfiguration < biome_rule_options :: use_namespace_keyword :: UseNamespaceKeywordOptions >> , # [doc = "Enforce using the digits argument with Number#toFixed()."] # [serde (skip_serializing_if = "Option::is_none")] pub use_number_to_fixed_digits_argument : Option < RuleFixConfiguration < biome_rule_options :: use_number_to_fixed_digits_argument :: UseNumberToFixedDigitsArgumentOptions >> , # [doc = "Use static Response methods instead of new Response() constructor when possible."] # [serde (skip_serializing_if = "Option::is_none")] pub use_static_response_methods : Option < RuleFixConfiguration < biome_rule_options :: use_static_response_methods :: UseStaticResponseMethodsOptions >> , # [doc = "Enforce the use of the directive \"use strict\" in script files."] # [serde (skip_serializing_if = "Option::is_none")] pub use_strict_mode : Option < RuleFixConfiguration < biome_rule_options :: use_strict_mode :: UseStrictModeOptions >> }
impl Suspicious {
    const GROUP_NAME: &'static str = "suspicious";
    pub(crate) const GROUP_RULES: &'static [&'static str] = &[
        "noAlert",
        "noApproximativeNumericConstant",
        "noArrayIndexKey",
        "noAssignInExpressions",
        "noAsyncPromiseExecutor",
        "noBiomeFirstException",
        "noBitwiseOperators",
        "noCatchAssign",
        "noClassAssign",
        "noCommentText",
        "noCompareNegZero",
        "noConfusingLabels",
        "noConfusingVoidType",
        "noConsole",
        "noConstEnum",
        "noConstantBinaryExpressions",
        "noControlCharactersInRegex",
        "noDebugger",
        "noDocumentCookie",
        "noDocumentImportInPage",
        "noDoubleEquals",
        "noDuplicateAtImportRules",
        "noDuplicateCase",
        "noDuplicateClassMembers",
        "noDuplicateCustomProperties",
        "noDuplicateElseIf",
        "noDuplicateFields",
        "noDuplicateFontNames",
        "noDuplicateJsxProps",
        "noDuplicateObjectKeys",
        "noDuplicateParameters",
        "noDuplicateProperties",
        "noDuplicateSelectorsKeyframeBlock",
        "noDuplicateTestHooks",
        "noEmptyBlock",
        "noEmptyBlockStatements",
        "noEmptyInterface",
        "noEvolvingTypes",
        "noExplicitAny",
        "noExportsInTest",
        "noExtraNonNullAssertion",
        "noFallthroughSwitchClause",
        "noFocusedTests",
        "noFunctionAssign",
        "noGlobalAssign",
        "noGlobalIsFinite",
        "noGlobalIsNan",
        "noHeadImportInDocument",
        "noImplicitAnyLet",
        "noImportAssign",
        "noImportantInKeyframe",
        "noIrregularWhitespace",
        "noLabelVar",
        "noMisleadingCharacterClass",
        "noMisleadingInstantiator",
        "noMisplacedAssertion",
        "noMisrefactoredShorthandAssign",
        "noOctalEscape",
        "noPrototypeBuiltins",
        "noQuickfixBiome",
        "noReactSpecificProps",
        "noRedeclare",
        "noRedundantUseStrict",
        "noSelfCompare",
        "noShadowRestrictedNames",
        "noShorthandPropertyOverrides",
        "noSkippedTests",
        "noSparseArray",
        "noSuspiciousSemicolonInJsx",
        "noTemplateCurlyInString",
        "noThenProperty",
        "noTsIgnore",
        "noUnassignedVariables",
        "noUnknownAtRules",
        "noUnsafeDeclarationMerging",
        "noUnsafeNegation",
        "noUselessEscapeInString",
        "noUselessRegexBackrefs",
        "noVar",
        "noWith",
        "useAdjacentOverloadSignatures",
        "useAwait",
        "useBiomeIgnoreFolder",
        "useDefaultSwitchClauseLast",
        "useErrorMessage",
        "useGetterReturn",
        "useGoogleFontDisplay",
        "useGuardForIn",
        "useIsArray",
        "useIterableCallbackReturn",
        "useNamespaceKeyword",
        "useNumberToFixedDigitsArgument",
        "useStaticResponseMethods",
        "useStrictMode",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[53]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[54]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[56]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[57]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[58]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[59]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[61]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[62]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[63]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[64]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[65]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[67]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[68]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[69]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[70]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[71]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[73]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[74]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[75]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[76]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[77]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[79]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[80]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[82]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[83]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[85]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[86]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[88]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[89]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[90]),
    ];
    const ALL_RULES_AS_FILTERS: &'static [RuleFilter<'static>] = &[
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[53]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[54]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[55]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[56]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[57]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[58]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[59]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[60]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[61]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[62]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[63]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[64]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[65]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[66]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[67]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[68]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[69]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[70]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[71]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[72]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[73]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[74]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[75]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[76]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[77]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[78]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[79]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[80]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[81]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[82]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[83]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[84]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[85]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[86]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[87]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[88]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[89]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[90]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[91]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[92]),
        RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[93]),
    ];
}
impl RuleGroupExt for Suspicious {
    fn is_recommended_true(&self) -> bool {
        matches!(self.recommended, Some(true))
    }
    fn is_recommended_unset(&self) -> bool {
        self.recommended.is_none()
    }
    fn get_enabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_alert.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
        }
        if let Some(rule) = self.no_approximative_numeric_constant.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
        }
        if let Some(rule) = self.no_array_index_key.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
        }
        if let Some(rule) = self.no_assign_in_expressions.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
        }
        if let Some(rule) = self.no_async_promise_executor.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
        }
        if let Some(rule) = self.no_biome_first_exception.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
        }
        if let Some(rule) = self.no_bitwise_operators.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
        }
        if let Some(rule) = self.no_catch_assign.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
        }
        if let Some(rule) = self.no_class_assign.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
        }
        if let Some(rule) = self.no_comment_text.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
        }
        if let Some(rule) = self.no_compare_neg_zero.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
        }
        if let Some(rule) = self.no_confusing_labels.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
        }
        if let Some(rule) = self.no_confusing_void_type.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
        }
        if let Some(rule) = self.no_console.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
        }
        if let Some(rule) = self.no_const_enum.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
        }
        if let Some(rule) = self.no_constant_binary_expressions.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
        }
        if let Some(rule) = self.no_control_characters_in_regex.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
        }
        if let Some(rule) = self.no_debugger.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
        }
        if let Some(rule) = self.no_document_cookie.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
        }
        if let Some(rule) = self.no_document_import_in_page.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
        }
        if let Some(rule) = self.no_double_equals.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
        }
        if let Some(rule) = self.no_duplicate_at_import_rules.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
        }
        if let Some(rule) = self.no_duplicate_case.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
        }
        if let Some(rule) = self.no_duplicate_class_members.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
        }
        if let Some(rule) = self.no_duplicate_custom_properties.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
        }
        if let Some(rule) = self.no_duplicate_else_if.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
        }
        if let Some(rule) = self.no_duplicate_fields.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
        }
        if let Some(rule) = self.no_duplicate_font_names.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
        }
        if let Some(rule) = self.no_duplicate_jsx_props.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
        }
        if let Some(rule) = self.no_duplicate_object_keys.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
        }
        if let Some(rule) = self.no_duplicate_parameters.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
        }
        if let Some(rule) = self.no_duplicate_properties.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
        }
        if let Some(rule) = self.no_duplicate_selectors_keyframe_block.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
        }
        if let Some(rule) = self.no_duplicate_test_hooks.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
        }
        if let Some(rule) = self.no_empty_block.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
        }
        if let Some(rule) = self.no_empty_block_statements.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
        }
        if let Some(rule) = self.no_empty_interface.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
        }
        if let Some(rule) = self.no_evolving_types.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
        }
        if let Some(rule) = self.no_explicit_any.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
        }
        if let Some(rule) = self.no_exports_in_test.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
        }
        if let Some(rule) = self.no_extra_non_null_assertion.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
        }
        if let Some(rule) = self.no_fallthrough_switch_clause.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
        }
        if let Some(rule) = self.no_focused_tests.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
        }
        if let Some(rule) = self.no_function_assign.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]));
        }
        if let Some(rule) = self.no_global_assign.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]));
        }
        if let Some(rule) = self.no_global_is_finite.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]));
        }
        if let Some(rule) = self.no_global_is_nan.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]));
        }
        if let Some(rule) = self.no_head_import_in_document.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]));
        }
        if let Some(rule) = self.no_implicit_any_let.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]));
        }
        if let Some(rule) = self.no_import_assign.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]));
        }
        if let Some(rule) = self.no_important_in_keyframe.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]));
        }
        if let Some(rule) = self.no_irregular_whitespace.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]));
        }
        if let Some(rule) = self.no_label_var.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]));
        }
        if let Some(rule) = self.no_misleading_character_class.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[53]));
        }
        if let Some(rule) = self.no_misleading_instantiator.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[54]));
        }
        if let Some(rule) = self.no_misplaced_assertion.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[55]));
        }
        if let Some(rule) = self.no_misrefactored_shorthand_assign.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[56]));
        }
        if let Some(rule) = self.no_octal_escape.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[57]));
        }
        if let Some(rule) = self.no_prototype_builtins.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[58]));
        }
        if let Some(rule) = self.no_quickfix_biome.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[59]));
        }
        if let Some(rule) = self.no_react_specific_props.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[60]));
        }
        if let Some(rule) = self.no_redeclare.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[61]));
        }
        if let Some(rule) = self.no_redundant_use_strict.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[62]));
        }
        if let Some(rule) = self.no_self_compare.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[63]));
        }
        if let Some(rule) = self.no_shadow_restricted_names.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[64]));
        }
        if let Some(rule) = self.no_shorthand_property_overrides.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[65]));
        }
        if let Some(rule) = self.no_skipped_tests.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[66]));
        }
        if let Some(rule) = self.no_sparse_array.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[67]));
        }
        if let Some(rule) = self.no_suspicious_semicolon_in_jsx.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[68]));
        }
        if let Some(rule) = self.no_template_curly_in_string.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[69]));
        }
        if let Some(rule) = self.no_then_property.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[70]));
        }
        if let Some(rule) = self.no_ts_ignore.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[71]));
        }
        if let Some(rule) = self.no_unassigned_variables.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[72]));
        }
        if let Some(rule) = self.no_unknown_at_rules.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[73]));
        }
        if let Some(rule) = self.no_unsafe_declaration_merging.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[74]));
        }
        if let Some(rule) = self.no_unsafe_negation.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[75]));
        }
        if let Some(rule) = self.no_useless_escape_in_string.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[76]));
        }
        if let Some(rule) = self.no_useless_regex_backrefs.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[77]));
        }
        if let Some(rule) = self.no_var.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[78]));
        }
        if let Some(rule) = self.no_with.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[79]));
        }
        if let Some(rule) = self.use_adjacent_overload_signatures.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[80]));
        }
        if let Some(rule) = self.use_await.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[81]));
        }
        if let Some(rule) = self.use_biome_ignore_folder.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[82]));
        }
        if let Some(rule) = self.use_default_switch_clause_last.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[83]));
        }
        if let Some(rule) = self.use_error_message.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[84]));
        }
        if let Some(rule) = self.use_getter_return.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[85]));
        }
        if let Some(rule) = self.use_google_font_display.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[86]));
        }
        if let Some(rule) = self.use_guard_for_in.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[87]));
        }
        if let Some(rule) = self.use_is_array.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[88]));
        }
        if let Some(rule) = self.use_iterable_callback_return.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[89]));
        }
        if let Some(rule) = self.use_namespace_keyword.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[90]));
        }
        if let Some(rule) = self.use_number_to_fixed_digits_argument.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[91]));
        }
        if let Some(rule) = self.use_static_response_methods.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[92]));
        }
        if let Some(rule) = self.use_strict_mode.as_ref()
            && rule.is_enabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[93]));
        }
        index_set
    }
    fn get_disabled_rules(&self) -> FxHashSet<RuleFilter<'static>> {
        let mut index_set = FxHashSet::default();
        if let Some(rule) = self.no_alert.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[0]));
        }
        if let Some(rule) = self.no_approximative_numeric_constant.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[1]));
        }
        if let Some(rule) = self.no_array_index_key.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[2]));
        }
        if let Some(rule) = self.no_assign_in_expressions.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[3]));
        }
        if let Some(rule) = self.no_async_promise_executor.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[4]));
        }
        if let Some(rule) = self.no_biome_first_exception.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[5]));
        }
        if let Some(rule) = self.no_bitwise_operators.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[6]));
        }
        if let Some(rule) = self.no_catch_assign.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[7]));
        }
        if let Some(rule) = self.no_class_assign.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[8]));
        }
        if let Some(rule) = self.no_comment_text.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[9]));
        }
        if let Some(rule) = self.no_compare_neg_zero.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[10]));
        }
        if let Some(rule) = self.no_confusing_labels.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[11]));
        }
        if let Some(rule) = self.no_confusing_void_type.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[12]));
        }
        if let Some(rule) = self.no_console.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[13]));
        }
        if let Some(rule) = self.no_const_enum.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[14]));
        }
        if let Some(rule) = self.no_constant_binary_expressions.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[15]));
        }
        if let Some(rule) = self.no_control_characters_in_regex.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[16]));
        }
        if let Some(rule) = self.no_debugger.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[17]));
        }
        if let Some(rule) = self.no_document_cookie.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[18]));
        }
        if let Some(rule) = self.no_document_import_in_page.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[19]));
        }
        if let Some(rule) = self.no_double_equals.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[20]));
        }
        if let Some(rule) = self.no_duplicate_at_import_rules.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[21]));
        }
        if let Some(rule) = self.no_duplicate_case.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[22]));
        }
        if let Some(rule) = self.no_duplicate_class_members.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[23]));
        }
        if let Some(rule) = self.no_duplicate_custom_properties.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[24]));
        }
        if let Some(rule) = self.no_duplicate_else_if.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[25]));
        }
        if let Some(rule) = self.no_duplicate_fields.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[26]));
        }
        if let Some(rule) = self.no_duplicate_font_names.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[27]));
        }
        if let Some(rule) = self.no_duplicate_jsx_props.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[28]));
        }
        if let Some(rule) = self.no_duplicate_object_keys.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[29]));
        }
        if let Some(rule) = self.no_duplicate_parameters.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[30]));
        }
        if let Some(rule) = self.no_duplicate_properties.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[31]));
        }
        if let Some(rule) = self.no_duplicate_selectors_keyframe_block.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[32]));
        }
        if let Some(rule) = self.no_duplicate_test_hooks.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[33]));
        }
        if let Some(rule) = self.no_empty_block.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[34]));
        }
        if let Some(rule) = self.no_empty_block_statements.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[35]));
        }
        if let Some(rule) = self.no_empty_interface.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[36]));
        }
        if let Some(rule) = self.no_evolving_types.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[37]));
        }
        if let Some(rule) = self.no_explicit_any.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[38]));
        }
        if let Some(rule) = self.no_exports_in_test.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[39]));
        }
        if let Some(rule) = self.no_extra_non_null_assertion.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[40]));
        }
        if let Some(rule) = self.no_fallthrough_switch_clause.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[41]));
        }
        if let Some(rule) = self.no_focused_tests.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[42]));
        }
        if let Some(rule) = self.no_function_assign.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[43]));
        }
        if let Some(rule) = self.no_global_assign.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[44]));
        }
        if let Some(rule) = self.no_global_is_finite.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[45]));
        }
        if let Some(rule) = self.no_global_is_nan.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[46]));
        }
        if let Some(rule) = self.no_head_import_in_document.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[47]));
        }
        if let Some(rule) = self.no_implicit_any_let.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[48]));
        }
        if let Some(rule) = self.no_import_assign.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[49]));
        }
        if let Some(rule) = self.no_important_in_keyframe.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[50]));
        }
        if let Some(rule) = self.no_irregular_whitespace.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[51]));
        }
        if let Some(rule) = self.no_label_var.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[52]));
        }
        if let Some(rule) = self.no_misleading_character_class.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[53]));
        }
        if let Some(rule) = self.no_misleading_instantiator.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[54]));
        }
        if let Some(rule) = self.no_misplaced_assertion.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[55]));
        }
        if let Some(rule) = self.no_misrefactored_shorthand_assign.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[56]));
        }
        if let Some(rule) = self.no_octal_escape.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[57]));
        }
        if let Some(rule) = self.no_prototype_builtins.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[58]));
        }
        if let Some(rule) = self.no_quickfix_biome.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[59]));
        }
        if let Some(rule) = self.no_react_specific_props.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[60]));
        }
        if let Some(rule) = self.no_redeclare.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[61]));
        }
        if let Some(rule) = self.no_redundant_use_strict.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[62]));
        }
        if let Some(rule) = self.no_self_compare.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[63]));
        }
        if let Some(rule) = self.no_shadow_restricted_names.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[64]));
        }
        if let Some(rule) = self.no_shorthand_property_overrides.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[65]));
        }
        if let Some(rule) = self.no_skipped_tests.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[66]));
        }
        if let Some(rule) = self.no_sparse_array.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[67]));
        }
        if let Some(rule) = self.no_suspicious_semicolon_in_jsx.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[68]));
        }
        if let Some(rule) = self.no_template_curly_in_string.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[69]));
        }
        if let Some(rule) = self.no_then_property.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[70]));
        }
        if let Some(rule) = self.no_ts_ignore.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[71]));
        }
        if let Some(rule) = self.no_unassigned_variables.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[72]));
        }
        if let Some(rule) = self.no_unknown_at_rules.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[73]));
        }
        if let Some(rule) = self.no_unsafe_declaration_merging.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[74]));
        }
        if let Some(rule) = self.no_unsafe_negation.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[75]));
        }
        if let Some(rule) = self.no_useless_escape_in_string.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[76]));
        }
        if let Some(rule) = self.no_useless_regex_backrefs.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[77]));
        }
        if let Some(rule) = self.no_var.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[78]));
        }
        if let Some(rule) = self.no_with.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[79]));
        }
        if let Some(rule) = self.use_adjacent_overload_signatures.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[80]));
        }
        if let Some(rule) = self.use_await.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[81]));
        }
        if let Some(rule) = self.use_biome_ignore_folder.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[82]));
        }
        if let Some(rule) = self.use_default_switch_clause_last.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[83]));
        }
        if let Some(rule) = self.use_error_message.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[84]));
        }
        if let Some(rule) = self.use_getter_return.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[85]));
        }
        if let Some(rule) = self.use_google_font_display.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[86]));
        }
        if let Some(rule) = self.use_guard_for_in.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[87]));
        }
        if let Some(rule) = self.use_is_array.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[88]));
        }
        if let Some(rule) = self.use_iterable_callback_return.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[89]));
        }
        if let Some(rule) = self.use_namespace_keyword.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[90]));
        }
        if let Some(rule) = self.use_number_to_fixed_digits_argument.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[91]));
        }
        if let Some(rule) = self.use_static_response_methods.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[92]));
        }
        if let Some(rule) = self.use_strict_mode.as_ref()
            && rule.is_disabled()
        {
            index_set.insert(RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[93]));
        }
        index_set
    }
    #[doc = r" Checks if, given a rule name, matches one of the rules contained in this category"]
    fn has_rule(rule_name: &str) -> Option<&'static str> {
        Some(Self::GROUP_RULES[Self::GROUP_RULES.binary_search(&rule_name).ok()?])
    }
    fn recommended_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
    fn all_rules_as_filters() -> &'static [RuleFilter<'static>] {
        Self::ALL_RULES_AS_FILTERS
    }
    #[doc = r" Select preset rules"]
    fn collect_preset_rules(
        &self,
        parent_is_recommended: bool,
        enabled_rules: &mut FxHashSet<RuleFilter<'static>>,
    ) {
        if self.is_recommended_true() || self.is_recommended_unset() && parent_is_recommended {
            enabled_rules.extend(Self::recommended_rules_as_filters());
        }
    }
    fn set_recommended(&mut self, value: Option<bool>) {
        self.recommended = value;
    }
    fn get_rule_configuration(
        &self,
        rule_name: &str,
    ) -> Option<(RulePlainConfiguration, Option<RuleOptions>)> {
        match rule_name {
            "noAlert" => self
                .no_alert
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noApproximativeNumericConstant" => self
                .no_approximative_numeric_constant
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noArrayIndexKey" => self
                .no_array_index_key
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noAssignInExpressions" => self
                .no_assign_in_expressions
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noAsyncPromiseExecutor" => self
                .no_async_promise_executor
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noBiomeFirstException" => self
                .no_biome_first_exception
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noBitwiseOperators" => self
                .no_bitwise_operators
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noCatchAssign" => self
                .no_catch_assign
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noClassAssign" => self
                .no_class_assign
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noCommentText" => self
                .no_comment_text
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noCompareNegZero" => self
                .no_compare_neg_zero
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noConfusingLabels" => self
                .no_confusing_labels
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noConfusingVoidType" => self
                .no_confusing_void_type
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noConsole" => self
                .no_console
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noConstEnum" => self
                .no_const_enum
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noConstantBinaryExpressions" => self
                .no_constant_binary_expressions
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noControlCharactersInRegex" => self
                .no_control_characters_in_regex
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDebugger" => self
                .no_debugger
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDocumentCookie" => self
                .no_document_cookie
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDocumentImportInPage" => self
                .no_document_import_in_page
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDoubleEquals" => self
                .no_double_equals
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateAtImportRules" => self
                .no_duplicate_at_import_rules
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateCase" => self
                .no_duplicate_case
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateClassMembers" => self
                .no_duplicate_class_members
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateCustomProperties" => self
                .no_duplicate_custom_properties
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateElseIf" => self
                .no_duplicate_else_if
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateFields" => self
                .no_duplicate_fields
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateFontNames" => self
                .no_duplicate_font_names
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateJsxProps" => self
                .no_duplicate_jsx_props
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateObjectKeys" => self
                .no_duplicate_object_keys
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateParameters" => self
                .no_duplicate_parameters
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateProperties" => self
                .no_duplicate_properties
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateSelectorsKeyframeBlock" => self
                .no_duplicate_selectors_keyframe_block
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noDuplicateTestHooks" => self
                .no_duplicate_test_hooks
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noEmptyBlock" => self
                .no_empty_block
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noEmptyBlockStatements" => self
                .no_empty_block_statements
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noEmptyInterface" => self
                .no_empty_interface
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noEvolvingTypes" => self
                .no_evolving_types
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noExplicitAny" => self
                .no_explicit_any
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noExportsInTest" => self
                .no_exports_in_test
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noExtraNonNullAssertion" => self
                .no_extra_non_null_assertion
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noFallthroughSwitchClause" => self
                .no_fallthrough_switch_clause
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noFocusedTests" => self
                .no_focused_tests
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noFunctionAssign" => self
                .no_function_assign
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noGlobalAssign" => self
                .no_global_assign
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noGlobalIsFinite" => self
                .no_global_is_finite
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noGlobalIsNan" => self
                .no_global_is_nan
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noHeadImportInDocument" => self
                .no_head_import_in_document
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noImplicitAnyLet" => self
                .no_implicit_any_let
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noImportAssign" => self
                .no_import_assign
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noImportantInKeyframe" => self
                .no_important_in_keyframe
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noIrregularWhitespace" => self
                .no_irregular_whitespace
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noLabelVar" => self
                .no_label_var
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noMisleadingCharacterClass" => self
                .no_misleading_character_class
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noMisleadingInstantiator" => self
                .no_misleading_instantiator
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noMisplacedAssertion" => self
                .no_misplaced_assertion
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noMisrefactoredShorthandAssign" => self
                .no_misrefactored_shorthand_assign
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noOctalEscape" => self
                .no_octal_escape
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noPrototypeBuiltins" => self
                .no_prototype_builtins
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noQuickfixBiome" => self
                .no_quickfix_biome
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noReactSpecificProps" => self
                .no_react_specific_props
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noRedeclare" => self
                .no_redeclare
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noRedundantUseStrict" => self
                .no_redundant_use_strict
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSelfCompare" => self
                .no_self_compare
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noShadowRestrictedNames" => self
                .no_shadow_restricted_names
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noShorthandPropertyOverrides" => self
                .no_shorthand_property_overrides
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSkippedTests" => self
                .no_skipped_tests
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSparseArray" => self
                .no_sparse_array
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noSuspiciousSemicolonInJsx" => self
                .no_suspicious_semicolon_in_jsx
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noTemplateCurlyInString" => self
                .no_template_curly_in_string
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noThenProperty" => self
                .no_then_property
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noTsIgnore" => self
                .no_ts_ignore
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnassignedVariables" => self
                .no_unassigned_variables
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnknownAtRules" => self
                .no_unknown_at_rules
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnsafeDeclarationMerging" => self
                .no_unsafe_declaration_merging
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUnsafeNegation" => self
                .no_unsafe_negation
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessEscapeInString" => self
                .no_useless_escape_in_string
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noUselessRegexBackrefs" => self
                .no_useless_regex_backrefs
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noVar" => self
                .no_var
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "noWith" => self
                .no_with
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useAdjacentOverloadSignatures" => self
                .use_adjacent_overload_signatures
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useAwait" => self
                .use_await
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useBiomeIgnoreFolder" => self
                .use_biome_ignore_folder
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useDefaultSwitchClauseLast" => self
                .use_default_switch_clause_last
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useErrorMessage" => self
                .use_error_message
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useGetterReturn" => self
                .use_getter_return
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useGoogleFontDisplay" => self
                .use_google_font_display
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useGuardForIn" => self
                .use_guard_for_in
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useIsArray" => self
                .use_is_array
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useIterableCallbackReturn" => self
                .use_iterable_callback_return
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useNamespaceKeyword" => self
                .use_namespace_keyword
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useNumberToFixedDigitsArgument" => self
                .use_number_to_fixed_digits_argument
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useStaticResponseMethods" => self
                .use_static_response_methods
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            "useStrictMode" => self
                .use_strict_mode
                .as_ref()
                .map(|conf| (conf.level(), conf.get_options())),
            _ => None,
        }
    }
}
impl From<GroupPlainConfiguration> for Suspicious {
    fn from(value: GroupPlainConfiguration) -> Self {
        Self {
            recommended: None,
            no_alert: Some(value.into()),
            no_approximative_numeric_constant: Some(value.into()),
            no_array_index_key: Some(value.into()),
            no_assign_in_expressions: Some(value.into()),
            no_async_promise_executor: Some(value.into()),
            no_biome_first_exception: Some(value.into()),
            no_bitwise_operators: Some(value.into()),
            no_catch_assign: Some(value.into()),
            no_class_assign: Some(value.into()),
            no_comment_text: Some(value.into()),
            no_compare_neg_zero: Some(value.into()),
            no_confusing_labels: Some(value.into()),
            no_confusing_void_type: Some(value.into()),
            no_console: Some(value.into()),
            no_const_enum: Some(value.into()),
            no_constant_binary_expressions: Some(value.into()),
            no_control_characters_in_regex: Some(value.into()),
            no_debugger: Some(value.into()),
            no_document_cookie: Some(value.into()),
            no_document_import_in_page: Some(value.into()),
            no_double_equals: Some(value.into()),
            no_duplicate_at_import_rules: Some(value.into()),
            no_duplicate_case: Some(value.into()),
            no_duplicate_class_members: Some(value.into()),
            no_duplicate_custom_properties: Some(value.into()),
            no_duplicate_else_if: Some(value.into()),
            no_duplicate_fields: Some(value.into()),
            no_duplicate_font_names: Some(value.into()),
            no_duplicate_jsx_props: Some(value.into()),
            no_duplicate_object_keys: Some(value.into()),
            no_duplicate_parameters: Some(value.into()),
            no_duplicate_properties: Some(value.into()),
            no_duplicate_selectors_keyframe_block: Some(value.into()),
            no_duplicate_test_hooks: Some(value.into()),
            no_empty_block: Some(value.into()),
            no_empty_block_statements: Some(value.into()),
            no_empty_interface: Some(value.into()),
            no_evolving_types: Some(value.into()),
            no_explicit_any: Some(value.into()),
            no_exports_in_test: Some(value.into()),
            no_extra_non_null_assertion: Some(value.into()),
            no_fallthrough_switch_clause: Some(value.into()),
            no_focused_tests: Some(value.into()),
            no_function_assign: Some(value.into()),
            no_global_assign: Some(value.into()),
            no_global_is_finite: Some(value.into()),
            no_global_is_nan: Some(value.into()),
            no_head_import_in_document: Some(value.into()),
            no_implicit_any_let: Some(value.into()),
            no_import_assign: Some(value.into()),
            no_important_in_keyframe: Some(value.into()),
            no_irregular_whitespace: Some(value.into()),
            no_label_var: Some(value.into()),
            no_misleading_character_class: Some(value.into()),
            no_misleading_instantiator: Some(value.into()),
            no_misplaced_assertion: Some(value.into()),
            no_misrefactored_shorthand_assign: Some(value.into()),
            no_octal_escape: Some(value.into()),
            no_prototype_builtins: Some(value.into()),
            no_quickfix_biome: Some(value.into()),
            no_react_specific_props: Some(value.into()),
            no_redeclare: Some(value.into()),
            no_redundant_use_strict: Some(value.into()),
            no_self_compare: Some(value.into()),
            no_shadow_restricted_names: Some(value.into()),
            no_shorthand_property_overrides: Some(value.into()),
            no_skipped_tests: Some(value.into()),
            no_sparse_array: Some(value.into()),
            no_suspicious_semicolon_in_jsx: Some(value.into()),
            no_template_curly_in_string: Some(value.into()),
            no_then_property: Some(value.into()),
            no_ts_ignore: Some(value.into()),
            no_unassigned_variables: Some(value.into()),
            no_unknown_at_rules: Some(value.into()),
            no_unsafe_declaration_merging: Some(value.into()),
            no_unsafe_negation: Some(value.into()),
            no_useless_escape_in_string: Some(value.into()),
            no_useless_regex_backrefs: Some(value.into()),
            no_var: Some(value.into()),
            no_with: Some(value.into()),
            use_adjacent_overload_signatures: Some(value.into()),
            use_await: Some(value.into()),
            use_biome_ignore_folder: Some(value.into()),
            use_default_switch_clause_last: Some(value.into()),
            use_error_message: Some(value.into()),
            use_getter_return: Some(value.into()),
            use_google_font_display: Some(value.into()),
            use_guard_for_in: Some(value.into()),
            use_is_array: Some(value.into()),
            use_iterable_callback_return: Some(value.into()),
            use_namespace_keyword: Some(value.into()),
            use_number_to_fixed_digits_argument: Some(value.into()),
            use_static_response_methods: Some(value.into()),
            use_strict_mode: Some(value.into()),
        }
    }
}
#[test]
fn test_order() {
    for items in A11y::GROUP_RULES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in Complexity::GROUP_RULES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in Correctness::GROUP_RULES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in Nursery::GROUP_RULES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in Performance::GROUP_RULES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in Security::GROUP_RULES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in Style::GROUP_RULES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in Suspicious::GROUP_RULES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
