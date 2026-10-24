//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::analyzer::DomainSelector;
use biome_analyze::{Rule, RuleFilter, RuleGroup};
use std::sync::LazyLock;
static ASTRO_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("nursery", "noAstroConflictingSetDirectives"),
        RuleFilter::Rule("nursery", "noAstroSetHtmlDirective"),
        RuleFilter::Rule("nursery", "useAstroClientOnlyDirectiveValue"),
    ]
});
static DRIZZLE_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("suspicious", "noDrizzleDeleteWithoutWhere"),
        RuleFilter::Rule("suspicious", "noDrizzleUpdateWithoutWhere"),
    ]
});
static NEXT_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("correctness", "noBeforeInteractiveScriptOutsideDocument"),
        RuleFilter::Rule("correctness", "noNextAsyncClientComponent"),
        RuleFilter::Rule("correctness", "useExhaustiveDependencies"),
        RuleFilter::Rule("correctness", "useHookAtTopLevel"),
        RuleFilter::Rule("correctness", "useInlineScriptId"),
        RuleFilter::Rule("performance", "noImgElement"),
        RuleFilter::Rule("performance", "noSyncScripts"),
        RuleFilter::Rule("performance", "noUnwantedPolyfillio"),
        RuleFilter::Rule("performance", "useGoogleFontPreconnect"),
        RuleFilter::Rule("style", "noHeadElement"),
        RuleFilter::Rule("suspicious", "noDocumentImportInPage"),
        RuleFilter::Rule("suspicious", "noHeadImportInDocument"),
    ]
});
static PLAYWRIGHT_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("complexity", "noPlaywrightUselessAwait"),
        RuleFilter::Rule("correctness", "noPlaywrightMissingAwait"),
        RuleFilter::Rule("correctness", "usePlaywrightValidDescribeCallback"),
        RuleFilter::Rule("nursery", "noPlaywrightNthMethods"),
        RuleFilter::Rule("style", "noPlaywrightElementHandle"),
        RuleFilter::Rule("style", "noPlaywrightEval"),
        RuleFilter::Rule("style", "noPlaywrightWaitForSelector"),
        RuleFilter::Rule("suspicious", "noPlaywrightForceOption"),
        RuleFilter::Rule("suspicious", "noPlaywrightNetworkidle"),
        RuleFilter::Rule("suspicious", "noPlaywrightPagePause"),
        RuleFilter::Rule("suspicious", "noPlaywrightWaitForNavigation"),
        RuleFilter::Rule("suspicious", "noPlaywrightWaitForTimeout"),
    ]
});
static PROJECT_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("correctness", "noPrivateImports"),
        RuleFilter::Rule("correctness", "noUndeclaredDependencies"),
        RuleFilter::Rule("correctness", "noUnresolvedImports"),
        RuleFilter::Rule("correctness", "useImportExtensions"),
        RuleFilter::Rule("correctness", "useJsonImportAttributes"),
        RuleFilter::Rule("nursery", "noSelfImport"),
        RuleFilter::Rule("nursery", "noTailwindRawColors"),
        RuleFilter::Rule("nursery", "noUndeclaredClasses"),
        RuleFilter::Rule("nursery", "noUndeclaredCustomProperties"),
        RuleFilter::Rule("nursery", "useTailwindSortedClasses"),
        RuleFilter::Rule("suspicious", "noDeprecatedImports"),
        RuleFilter::Rule("suspicious", "noImportCycles"),
    ]
});
static QWIK_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("correctness", "noQwikUseVisibleTask"),
        RuleFilter::Rule("correctness", "useImageSize"),
        RuleFilter::Rule("correctness", "useJsxKeyInIterable"),
        RuleFilter::Rule("correctness", "useQwikClasslist"),
        RuleFilter::Rule("correctness", "useQwikLoaderLocation"),
        RuleFilter::Rule("correctness", "useQwikMethodUsage"),
        RuleFilter::Rule("correctness", "useQwikValidLexicalScope"),
        RuleFilter::Rule("suspicious", "noReactSpecificProps"),
    ]
});
static REACT_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("correctness", "noChildrenProp"),
        RuleFilter::Rule("correctness", "noComponentHookFactories"),
        RuleFilter::Rule("correctness", "noJsxNamespace"),
        RuleFilter::Rule("correctness", "noNestedComponentDefinitions"),
        RuleFilter::Rule("correctness", "noReactPropAssignments"),
        RuleFilter::Rule("correctness", "noRenderReturnValue"),
        RuleFilter::Rule("correctness", "useExhaustiveDependencies"),
        RuleFilter::Rule("correctness", "useHookAtTopLevel"),
        RuleFilter::Rule("correctness", "useJsxKeyInIterable"),
        RuleFilter::Rule("correctness", "useReactAsyncServerFunction"),
        RuleFilter::Rule("correctness", "useUniqueElementIds"),
        RuleFilter::Rule("nursery", "noReactObjectTypeAsDefaultProp"),
        RuleFilter::Rule("nursery", "useReactCompiler"),
        RuleFilter::Rule("nursery", "useReactFunctionComponentDefinition"),
        RuleFilter::Rule("nursery", "useReactNamingConvention"),
        RuleFilter::Rule("nursery", "useTanstackQueryStableQueryClient"),
        RuleFilter::Rule("performance", "noJsxPropsBind"),
        RuleFilter::Rule("performance", "noSyncScripts"),
        RuleFilter::Rule("security", "noDangerouslySetInnerHtml"),
        RuleFilter::Rule("security", "noDangerouslySetInnerHtmlWithChildren"),
        RuleFilter::Rule("style", "useComponentExportOnlyModules"),
        RuleFilter::Rule("style", "useReactFunctionComponents"),
        RuleFilter::Rule("suspicious", "noArrayIndexKey"),
        RuleFilter::Rule("suspicious", "noDuplicatedSpreadProps"),
        RuleFilter::Rule("suspicious", "noJsxLeakedDollar"),
        RuleFilter::Rule("suspicious", "noLeakedRender"),
        RuleFilter::Rule("suspicious", "noReactForwardRef"),
        RuleFilter::Rule("suspicious", "noReactStringRefs"),
        RuleFilter::Rule("suspicious", "noUnknownAttribute"),
    ]
});
static REACTNATIVE_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("correctness", "noReactNativeRawText"),
        RuleFilter::Rule("correctness", "useReactNativePlatformComponents"),
        RuleFilter::Rule("nursery", "useReactNativeValidPlatformColors"),
        RuleFilter::Rule("style", "noReactNativeLiteralColors"),
        RuleFilter::Rule("suspicious", "noReactNativeDeepImports"),
    ]
});
static SOLID_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("correctness", "noSolidDestructuredProps"),
        RuleFilter::Rule("performance", "useSolidForComponent"),
        RuleFilter::Rule("suspicious", "noDuplicatedSpreadProps"),
        RuleFilter::Rule("suspicious", "noReactSpecificProps"),
    ]
});
static SVELTE_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("nursery", "noSvelteAddEventListener"),
        RuleFilter::Rule("nursery", "noSvelteAtDebugTags"),
        RuleFilter::Rule("nursery", "noSvelteAtHtmlTags"),
        RuleFilter::Rule("nursery", "noSvelteClassDirective"),
        RuleFilter::Rule("nursery", "noSvelteExportLet"),
        RuleFilter::Rule("nursery", "noSvelteInspect"),
        RuleFilter::Rule("nursery", "noSvelteInternal"),
        RuleFilter::Rule("nursery", "noSvelteLegacyConst"),
        RuleFilter::Rule("nursery", "noSvelteObjectInTextMustaches"),
        RuleFilter::Rule("nursery", "noSvelteUnnecessaryStateWrap"),
        RuleFilter::Rule("nursery", "noSvelteUselessDerivedBy"),
        RuleFilter::Rule("nursery", "useConsistentBlockLang"),
        RuleFilter::Rule("nursery", "useSvelteEventDispatcherTypes"),
        RuleFilter::Rule("nursery", "useSvelteKitResolve"),
        RuleFilter::Rule("nursery", "useSvelteKitRuneImports"),
        RuleFilter::Rule("nursery", "useSvelteReactiveClasses"),
        RuleFilter::Rule("nursery", "useSvelteRequireEachKey"),
        RuleFilter::Rule("nursery", "useSvelteShorthandDirective"),
    ]
});
static TAILWIND_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("nursery", "noTailwindArbitraryValue"),
        RuleFilter::Rule("nursery", "noTailwindLegacyUtilities"),
        RuleFilter::Rule("nursery", "noTailwindRawColors"),
        RuleFilter::Rule("nursery", "noTailwindRestyledComponents"),
        RuleFilter::Rule("nursery", "useTailwindShorthandClasses"),
        RuleFilter::Rule("nursery", "useTailwindSortedClasses"),
        RuleFilter::Rule("nursery", "useTailwindStaticClassStrings"),
        RuleFilter::Rule("nursery", "useTailwindVariablesShorthand"),
    ]
});
static TANSTACKQUERY_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("nursery", "noTanstackQueryRestDestructuring"),
        RuleFilter::Rule("nursery", "noTanstackQueryVoidQueryFn"),
        RuleFilter::Rule("nursery", "useTanstackQueryInfiniteQueryPropertyOrder"),
        RuleFilter::Rule("nursery", "useTanstackQueryStableQueryClient"),
    ]
});
static TEST_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("complexity", "noExcessiveNestedTestSuites"),
        RuleFilter::Rule("nursery", "useExpectToContain"),
        RuleFilter::Rule("nursery", "useLowercaseTestTitle"),
        RuleFilter::Rule("nursery", "useValidTestTitle"),
        RuleFilter::Rule("style", "useConsistentTestIt"),
        RuleFilter::Rule("style", "useTestHooksInOrder"),
        RuleFilter::Rule("style", "useTestHooksOnTop"),
        RuleFilter::Rule("suspicious", "noConditionalExpect"),
        RuleFilter::Rule("suspicious", "noDuplicateTestHooks"),
        RuleFilter::Rule("suspicious", "noExportsInTest"),
        RuleFilter::Rule("suspicious", "noFocusedTests"),
        RuleFilter::Rule("suspicious", "noIdenticalTestTitle"),
        RuleFilter::Rule("suspicious", "noSkippedTests"),
        RuleFilter::Rule("suspicious", "useExpect"),
    ]
});
static TURBOREPO_FILTERS: LazyLock<Vec<RuleFilter<'static>>> =
    LazyLock::new(|| vec![RuleFilter::Rule("suspicious", "noUndeclaredEnvVars")]);
static TYPES_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("complexity", "useArrayFind"),
        RuleFilter::Rule("nursery", "noBaseToString"),
        RuleFilter::Rule("nursery", "noFloatingPromises"),
        RuleFilter::Rule("nursery", "noMeaninglessVoidOperator"),
        RuleFilter::Rule("nursery", "noMisleadingReturnType"),
        RuleFilter::Rule("nursery", "noMisusedPromises"),
        RuleFilter::Rule("nursery", "noTanstackQueryVoidQueryFn"),
        RuleFilter::Rule("nursery", "noUnsafePlusOperands"),
        RuleFilter::Rule("nursery", "noUnsafeReturn"),
        RuleFilter::Rule("nursery", "noUnsafeUnaryMinus"),
        RuleFilter::Rule("nursery", "noUselessTypeAssertion"),
        RuleFilter::Rule("nursery", "noUselessTypeConversion"),
        RuleFilter::Rule("nursery", "useAwaitThenable"),
        RuleFilter::Rule("nursery", "useDisposables"),
        RuleFilter::Rule("nursery", "useExhaustiveSwitchCases"),
        RuleFilter::Rule("nursery", "useIncludes"),
        RuleFilter::Rule("nursery", "useNullishCoalescing"),
        RuleFilter::Rule("nursery", "useRegexpExec"),
        RuleFilter::Rule("nursery", "useStrictBooleanExpressions"),
        RuleFilter::Rule("nursery", "useStringStartsEndsWith"),
        RuleFilter::Rule("style", "useConsistentEnumValueType"),
        RuleFilter::Rule("suspicious", "noUnnecessaryConditions"),
        RuleFilter::Rule("suspicious", "useArraySortCompare"),
    ]
});
static VUE_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("correctness", "noVueDataObjectDeclaration"),
        RuleFilter::Rule("correctness", "noVueDuplicateKeys"),
        RuleFilter::Rule("correctness", "noVueImportCompilerMacros"),
        RuleFilter::Rule("correctness", "noVueRefAsOperand"),
        RuleFilter::Rule("correctness", "noVueReservedKeys"),
        RuleFilter::Rule("correctness", "noVueReservedProps"),
        RuleFilter::Rule("correctness", "noVueSetupPropsReactivityLoss"),
        RuleFilter::Rule("correctness", "noVueVIfWithVFor"),
        RuleFilter::Rule("correctness", "noVueVOnNumberValues"),
        RuleFilter::Rule("correctness", "useVueVForKey"),
        RuleFilter::Rule("correctness", "useVueValidTemplateRoot"),
        RuleFilter::Rule("correctness", "useVueValidVBind"),
        RuleFilter::Rule("correctness", "useVueValidVCloak"),
        RuleFilter::Rule("correctness", "useVueValidVElse"),
        RuleFilter::Rule("correctness", "useVueValidVElseIf"),
        RuleFilter::Rule("correctness", "useVueValidVFor"),
        RuleFilter::Rule("correctness", "useVueValidVHtml"),
        RuleFilter::Rule("correctness", "useVueValidVIf"),
        RuleFilter::Rule("correctness", "useVueValidVOn"),
        RuleFilter::Rule("correctness", "useVueValidVOnce"),
        RuleFilter::Rule("correctness", "useVueValidVPre"),
        RuleFilter::Rule("correctness", "useVueValidVText"),
        RuleFilter::Rule("nursery", "noVueBooleanDefault"),
        RuleFilter::Rule("nursery", "noVueComputedPropertiesInData"),
        RuleFilter::Rule("nursery", "noVueDeprecatedConfigKeyCodes"),
        RuleFilter::Rule("nursery", "noVueDeprecatedDeleteSet"),
        RuleFilter::Rule("nursery", "noVueDeprecatedDestroyedLifecycle"),
        RuleFilter::Rule("nursery", "noVueDeprecatedEventsApi"),
        RuleFilter::Rule("nursery", "noVueDeprecatedFunctionalTemplate"),
        RuleFilter::Rule("nursery", "noVueDeprecatedHtmlElementIs"),
        RuleFilter::Rule("nursery", "noVueDeprecatedInlineTemplate"),
        RuleFilter::Rule("nursery", "noVueDeprecatedListeners"),
        RuleFilter::Rule("nursery", "noVueDeprecatedModelDefinition"),
        RuleFilter::Rule("nursery", "noVueDeprecatedPropsDefaultThis"),
        RuleFilter::Rule("nursery", "noVueDeprecatedRouterLinkTagProp"),
        RuleFilter::Rule("nursery", "noVueDeprecatedScopeAttribute"),
        RuleFilter::Rule("nursery", "noVueDeprecatedScopedSlots"),
        RuleFilter::Rule("nursery", "noVueDeprecatedSlotAttribute"),
        RuleFilter::Rule("nursery", "noVueDeprecatedSlotScope"),
        RuleFilter::Rule("nursery", "noVueDeprecatedVBindSync"),
        RuleFilter::Rule("nursery", "noVueDeprecatedVOnNativeModifier"),
        RuleFilter::Rule("nursery", "noVueExcessiveProps"),
        RuleFilter::Rule("nursery", "noVueLifecycleAfterAwait"),
        RuleFilter::Rule("nursery", "noVueLoneTemplate"),
        RuleFilter::Rule("nursery", "noVueMixin"),
        RuleFilter::Rule("nursery", "noVueMultipleObjectsInClass"),
        RuleFilter::Rule("nursery", "noVueMultipleSlotArgs"),
        RuleFilter::Rule("nursery", "noVueMutatingProps"),
        RuleFilter::Rule("nursery", "noVueRefObjectReactivityLoss"),
        RuleFilter::Rule("nursery", "noVueRequiredPropWithDefault"),
        RuleFilter::Rule("nursery", "noVueRestrictedCallAfterAwait"),
        RuleFilter::Rule("nursery", "noVueRestrictedVOn"),
        RuleFilter::Rule("nursery", "noVueRootVIf"),
        RuleFilter::Rule("nursery", "noVueShadowNativeEvents"),
        RuleFilter::Rule("nursery", "noVueSideEffectsInComputed"),
        RuleFilter::Rule("nursery", "noVueTemplateKey"),
        RuleFilter::Rule("nursery", "noVueThisInBeforeRouteEnter"),
        RuleFilter::Rule("nursery", "noVueUndeclaredDirectives"),
        RuleFilter::Rule("nursery", "noVueUndeclaredEmits"),
        RuleFilter::Rule("nursery", "noVueUndeclaredSlots"),
        RuleFilter::Rule("nursery", "noVueUnusedEmitDeclarations"),
        RuleFilter::Rule("nursery", "noVueUnusedRefs"),
        RuleFilter::Rule("nursery", "noVueUselessVBind"),
        RuleFilter::Rule("nursery", "noVueVForTemplateKeyOnChild"),
        RuleFilter::Rule("nursery", "noVueVHtml"),
        RuleFilter::Rule("nursery", "noVueVText"),
        RuleFilter::Rule("nursery", "noVueVTextVHtmlOnComponent"),
        RuleFilter::Rule("nursery", "noVueWatchAfterAwait"),
        RuleFilter::Rule("nursery", "useConsistentBlockLang"),
        RuleFilter::Rule("nursery", "useVueBaseImport"),
        RuleFilter::Rule("nursery", "useVueBlockOrder"),
        RuleFilter::Rule("nursery", "useVueComponentIs"),
        RuleFilter::Rule("nursery", "useVueComputedPropertyReturn"),
        RuleFilter::Rule("nursery", "useVueConsistentDefineEmitsDeclaration"),
        RuleFilter::Rule("nursery", "useVueConsistentEventHyphenation"),
        RuleFilter::Rule("nursery", "useVueDefaultProp"),
        RuleFilter::Rule("nursery", "useVueDefineOptions"),
        RuleFilter::Rule("nursery", "useVueDefinePropsDestructuring"),
        RuleFilter::Rule("nursery", "useVueMatchingComponentFileName"),
        RuleFilter::Rule("nursery", "useVuePropTypeConstructor"),
        RuleFilter::Rule("nursery", "useVueRenderReturn"),
        RuleFilter::Rule("nursery", "useVueSlotNameCasing"),
        RuleFilter::Rule("nursery", "useVueTemplateRef"),
        RuleFilter::Rule("nursery", "useVueToggleInsideTransition"),
        RuleFilter::Rule("nursery", "useVueTypedObjectProps"),
        RuleFilter::Rule("nursery", "useVueTypedProps"),
        RuleFilter::Rule("nursery", "useVueTypedRef"),
        RuleFilter::Rule("nursery", "useVueVModel"),
        RuleFilter::Rule("nursery", "useVueVOnExact"),
        RuleFilter::Rule("nursery", "useVueValidAttributeName"),
        RuleFilter::Rule("nursery", "useVueValidDefineEmits"),
        RuleFilter::Rule("nursery", "useVueValidDefineOptions"),
        RuleFilter::Rule("nursery", "useVueValidDefineProps"),
        RuleFilter::Rule("nursery", "useVueValidNextTick"),
        RuleFilter::Rule("nursery", "useVueValidVIs"),
        RuleFilter::Rule("nursery", "useVueValidVMemo"),
        RuleFilter::Rule("nursery", "useVueValidVModel"),
        RuleFilter::Rule("nursery", "useVueValidVShow"),
        RuleFilter::Rule("nursery", "useVueValidVSlot"),
        RuleFilter::Rule("performance", "useVueVapor"),
        RuleFilter::Rule("style", "noVueOptionsApi"),
        RuleFilter::Rule("style", "useVueConsistentDefinePropsDeclaration"),
        RuleFilter::Rule("style", "useVueConsistentVBindStyle"),
        RuleFilter::Rule("style", "useVueConsistentVOnStyle"),
        RuleFilter::Rule("style", "useVueDefineMacrosOrder"),
        RuleFilter::Rule("style", "useVueHyphenatedAttributes"),
        RuleFilter::Rule("style", "useVueMultiWordComponentNames"),
        RuleFilter::Rule("style", "useVueNextTickPromise"),
        RuleFilter::Rule("suspicious", "noVueArrowFuncInWatch"),
        RuleFilter::Rule("suspicious", "useScopedStyles"),
    ]
});
impl DomainSelector {
    pub fn as_rule_filters(&self) -> Vec<RuleFilter<'static>> {
        match self.0 {
            "astro" => ASTRO_FILTERS.clone(),
            "drizzle" => DRIZZLE_FILTERS.clone(),
            "next" => NEXT_FILTERS.clone(),
            "playwright" => PLAYWRIGHT_FILTERS.clone(),
            "project" => PROJECT_FILTERS.clone(),
            "qwik" => QWIK_FILTERS.clone(),
            "react" => REACT_FILTERS.clone(),
            "reactNative" => REACTNATIVE_FILTERS.clone(),
            "solid" => SOLID_FILTERS.clone(),
            "svelte" => SVELTE_FILTERS.clone(),
            "tailwind" => TAILWIND_FILTERS.clone(),
            "tanstackQuery" => TANSTACKQUERY_FILTERS.clone(),
            "test" => TEST_FILTERS.clone(),
            "turborepo" => TURBOREPO_FILTERS.clone(),
            "types" => TYPES_FILTERS.clone(),
            "vue" => VUE_FILTERS.clone(),
            _ => unreachable!("DomainFilter::as_rule_filters: domain {} not found", self.0),
        }
    }
    pub fn match_rule<R>(&self) -> bool
    where
        R: Rule,
    {
        self.match_rule_name(<R::Group as RuleGroup>::NAME, R::METADATA.name)
    }
    pub(crate) fn match_rule_name(&self, group_name: &str, rule_name: &str) -> bool {
        match self.0 {
            "astro" => ASTRO_FILTERS
                .iter()
                .any(|filter| filter.match_rule_name(group_name, rule_name)),
            "drizzle" => DRIZZLE_FILTERS
                .iter()
                .any(|filter| filter.match_rule_name(group_name, rule_name)),
            "next" => NEXT_FILTERS
                .iter()
                .any(|filter| filter.match_rule_name(group_name, rule_name)),
            "playwright" => PLAYWRIGHT_FILTERS
                .iter()
                .any(|filter| filter.match_rule_name(group_name, rule_name)),
            "project" => PROJECT_FILTERS
                .iter()
                .any(|filter| filter.match_rule_name(group_name, rule_name)),
            "qwik" => QWIK_FILTERS
                .iter()
                .any(|filter| filter.match_rule_name(group_name, rule_name)),
            "react" => REACT_FILTERS
                .iter()
                .any(|filter| filter.match_rule_name(group_name, rule_name)),
            "reactNative" => REACTNATIVE_FILTERS
                .iter()
                .any(|filter| filter.match_rule_name(group_name, rule_name)),
            "solid" => SOLID_FILTERS
                .iter()
                .any(|filter| filter.match_rule_name(group_name, rule_name)),
            "svelte" => SVELTE_FILTERS
                .iter()
                .any(|filter| filter.match_rule_name(group_name, rule_name)),
            "tailwind" => TAILWIND_FILTERS
                .iter()
                .any(|filter| filter.match_rule_name(group_name, rule_name)),
            "tanstackQuery" => TANSTACKQUERY_FILTERS
                .iter()
                .any(|filter| filter.match_rule_name(group_name, rule_name)),
            "test" => TEST_FILTERS
                .iter()
                .any(|filter| filter.match_rule_name(group_name, rule_name)),
            "turborepo" => TURBOREPO_FILTERS
                .iter()
                .any(|filter| filter.match_rule_name(group_name, rule_name)),
            "types" => TYPES_FILTERS
                .iter()
                .any(|filter| filter.match_rule_name(group_name, rule_name)),
            "vue" => VUE_FILTERS
                .iter()
                .any(|filter| filter.match_rule_name(group_name, rule_name)),
            _ => false,
        }
    }
}
