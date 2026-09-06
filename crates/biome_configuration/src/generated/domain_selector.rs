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
        RuleFilter::Rule("nursery", "noDrizzleDeleteWithoutWhere"),
        RuleFilter::Rule("nursery", "noDrizzleUpdateWithoutWhere"),
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
        RuleFilter::Rule("nursery", "noPlaywrightElementHandle"),
        RuleFilter::Rule("nursery", "noPlaywrightEval"),
        RuleFilter::Rule("nursery", "noPlaywrightForceOption"),
        RuleFilter::Rule("nursery", "noPlaywrightMissingAwait"),
        RuleFilter::Rule("nursery", "noPlaywrightNetworkidle"),
        RuleFilter::Rule("nursery", "noPlaywrightPagePause"),
        RuleFilter::Rule("nursery", "noPlaywrightUselessAwait"),
        RuleFilter::Rule("nursery", "noPlaywrightWaitForNavigation"),
        RuleFilter::Rule("nursery", "noPlaywrightWaitForSelector"),
        RuleFilter::Rule("nursery", "noPlaywrightWaitForTimeout"),
        RuleFilter::Rule("nursery", "usePlaywrightValidDescribeCallback"),
    ]
});
static PROJECT_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("correctness", "noPrivateImports"),
        RuleFilter::Rule("correctness", "noUndeclaredDependencies"),
        RuleFilter::Rule("correctness", "noUnresolvedImports"),
        RuleFilter::Rule("correctness", "useImportExtensions"),
        RuleFilter::Rule("correctness", "useJsonImportAttributes"),
        RuleFilter::Rule("nursery", "noUndeclaredClasses"),
        RuleFilter::Rule("nursery", "noUndeclaredCustomProperties"),
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
        RuleFilter::Rule("correctness", "useQwikMethodUsage"),
        RuleFilter::Rule("correctness", "useQwikValidLexicalScope"),
        RuleFilter::Rule("nursery", "useQwikLoaderLocation"),
        RuleFilter::Rule("suspicious", "noReactSpecificProps"),
    ]
});
static REACT_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("correctness", "noChildrenProp"),
        RuleFilter::Rule("correctness", "noNestedComponentDefinitions"),
        RuleFilter::Rule("correctness", "noReactPropAssignments"),
        RuleFilter::Rule("correctness", "noRenderReturnValue"),
        RuleFilter::Rule("correctness", "useExhaustiveDependencies"),
        RuleFilter::Rule("correctness", "useHookAtTopLevel"),
        RuleFilter::Rule("correctness", "useJsxKeyInIterable"),
        RuleFilter::Rule("correctness", "useUniqueElementIds"),
        RuleFilter::Rule("nursery", "noComponentHookFactories"),
        RuleFilter::Rule("nursery", "noJsxLeakedDollar"),
        RuleFilter::Rule("nursery", "noJsxNamespace"),
        RuleFilter::Rule("nursery", "noReactStringRefs"),
        RuleFilter::Rule("nursery", "useReactAsyncServerFunction"),
        RuleFilter::Rule("nursery", "useReactCompiler"),
        RuleFilter::Rule("nursery", "useReactFunctionComponentDefinition"),
        RuleFilter::Rule("nursery", "useReactNamingConvention"),
        RuleFilter::Rule("performance", "noJsxPropsBind"),
        RuleFilter::Rule("performance", "noSyncScripts"),
        RuleFilter::Rule("security", "noDangerouslySetInnerHtml"),
        RuleFilter::Rule("security", "noDangerouslySetInnerHtmlWithChildren"),
        RuleFilter::Rule("style", "useComponentExportOnlyModules"),
        RuleFilter::Rule("style", "useReactFunctionComponents"),
        RuleFilter::Rule("suspicious", "noArrayIndexKey"),
        RuleFilter::Rule("suspicious", "noDuplicatedSpreadProps"),
        RuleFilter::Rule("suspicious", "noLeakedRender"),
        RuleFilter::Rule("suspicious", "noReactForwardRef"),
        RuleFilter::Rule("suspicious", "noUnknownAttribute"),
    ]
});
static REACTNATIVE_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("nursery", "noReactNativeDeepImports"),
        RuleFilter::Rule("nursery", "noReactNativeLiteralColors"),
        RuleFilter::Rule("nursery", "noReactNativeRawText"),
        RuleFilter::Rule("nursery", "useReactNativePlatformComponents"),
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
        RuleFilter::Rule("nursery", "noSvelteAtHtmlTags"),
        RuleFilter::Rule("nursery", "noSvelteLegacyConst"),
        RuleFilter::Rule("nursery", "noSvelteUnnecessaryStateWrap"),
        RuleFilter::Rule("nursery", "useSvelteRequireEachKey"),
    ]
});
static TAILWIND_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("nursery", "noTailwindArbitraryValue"),
        RuleFilter::Rule("nursery", "useTailwindShorthandClasses"),
    ]
});
static TEST_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("complexity", "noExcessiveNestedTestSuites"),
        RuleFilter::Rule("nursery", "noConditionalExpect"),
        RuleFilter::Rule("nursery", "noIdenticalTestTitle"),
        RuleFilter::Rule("nursery", "useConsistentTestIt"),
        RuleFilter::Rule("nursery", "useExpect"),
        RuleFilter::Rule("nursery", "useTestHooksInOrder"),
        RuleFilter::Rule("nursery", "useTestHooksOnTop"),
        RuleFilter::Rule("suspicious", "noDuplicateTestHooks"),
        RuleFilter::Rule("suspicious", "noExportsInTest"),
        RuleFilter::Rule("suspicious", "noFocusedTests"),
        RuleFilter::Rule("suspicious", "noSkippedTests"),
    ]
});
static TURBOREPO_FILTERS: LazyLock<Vec<RuleFilter<'static>>> =
    LazyLock::new(|| vec![RuleFilter::Rule("suspicious", "noUndeclaredEnvVars")]);
static TYPES_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("complexity", "useArrayFind"),
        RuleFilter::Rule("nursery", "noBaseToString"),
        RuleFilter::Rule("nursery", "noFloatingPromises"),
        RuleFilter::Rule("nursery", "noMisleadingReturnType"),
        RuleFilter::Rule("nursery", "noMisusedPromises"),
        RuleFilter::Rule("nursery", "noUnsafePlusOperands"),
        RuleFilter::Rule("nursery", "noUselessTypeConversion"),
        RuleFilter::Rule("nursery", "useAwaitThenable"),
        RuleFilter::Rule("nursery", "useDisposables"),
        RuleFilter::Rule("nursery", "useExhaustiveSwitchCases"),
        RuleFilter::Rule("nursery", "useIncludes"),
        RuleFilter::Rule("nursery", "useNullishCoalescing"),
        RuleFilter::Rule("nursery", "useRegexpExec"),
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
        RuleFilter::Rule("correctness", "noVueReservedKeys"),
        RuleFilter::Rule("correctness", "noVueReservedProps"),
        RuleFilter::Rule("correctness", "noVueSetupPropsReactivityLoss"),
        RuleFilter::Rule("correctness", "noVueVIfWithVFor"),
        RuleFilter::Rule("correctness", "useVueVForKey"),
        RuleFilter::Rule("correctness", "useVueValidTemplateRoot"),
        RuleFilter::Rule("correctness", "useVueValidVBind"),
        RuleFilter::Rule("correctness", "useVueValidVCloak"),
        RuleFilter::Rule("correctness", "useVueValidVElse"),
        RuleFilter::Rule("correctness", "useVueValidVElseIf"),
        RuleFilter::Rule("correctness", "useVueValidVHtml"),
        RuleFilter::Rule("correctness", "useVueValidVIf"),
        RuleFilter::Rule("correctness", "useVueValidVOn"),
        RuleFilter::Rule("correctness", "useVueValidVOnce"),
        RuleFilter::Rule("correctness", "useVueValidVPre"),
        RuleFilter::Rule("correctness", "useVueValidVText"),
        RuleFilter::Rule("nursery", "noVueDeprecatedScopedSlots"),
        RuleFilter::Rule("nursery", "noVueImportCompilerMacros"),
        RuleFilter::Rule("nursery", "noVueRefAsOperand"),
        RuleFilter::Rule("nursery", "noVueVOnNumberValues"),
        RuleFilter::Rule("nursery", "useScopedStyles"),
        RuleFilter::Rule("nursery", "useVueBaseImport"),
        RuleFilter::Rule("nursery", "useVueConsistentDefinePropsDeclaration"),
        RuleFilter::Rule("nursery", "useVueNextTickPromise"),
        RuleFilter::Rule("nursery", "useVueValidVFor"),
        RuleFilter::Rule("performance", "useVueVapor"),
        RuleFilter::Rule("style", "noVueOptionsApi"),
        RuleFilter::Rule("style", "useVueConsistentVBindStyle"),
        RuleFilter::Rule("style", "useVueConsistentVOnStyle"),
        RuleFilter::Rule("style", "useVueDefineMacrosOrder"),
        RuleFilter::Rule("style", "useVueHyphenatedAttributes"),
        RuleFilter::Rule("style", "useVueMultiWordComponentNames"),
        RuleFilter::Rule("suspicious", "noVueArrowFuncInWatch"),
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
