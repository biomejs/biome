//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::analyzer::DomainSelector;
use biome_analyze::{Rule, RuleFilter};
use std::sync::LazyLock;
static NEXT_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("correctness", "useExhaustiveDependencies"),
        RuleFilter::Rule("correctness", "useHookAtTopLevel"),
        RuleFilter::Rule("nursery", "noNextAsyncClientComponent"),
        RuleFilter::Rule("nursery", "noSyncScripts"),
        RuleFilter::Rule("performance", "noImgElement"),
        RuleFilter::Rule("performance", "noUnwantedPolyfillio"),
        RuleFilter::Rule("performance", "useGoogleFontPreconnect"),
        RuleFilter::Rule("style", "noHeadElement"),
        RuleFilter::Rule("suspicious", "noDocumentImportInPage"),
        RuleFilter::Rule("suspicious", "noHeadImportInDocument"),
    ]
});
static PROJECT_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("correctness", "noPrivateImports"),
        RuleFilter::Rule("correctness", "noUndeclaredDependencies"),
        RuleFilter::Rule("correctness", "useImportExtensions"),
        RuleFilter::Rule("correctness", "useJsonImportAttributes"),
        RuleFilter::Rule("nursery", "noDeprecatedImports"),
        RuleFilter::Rule("nursery", "noFloatingPromises"),
        RuleFilter::Rule("nursery", "noImportCycles"),
        RuleFilter::Rule("nursery", "noMisusedPromises"),
        RuleFilter::Rule("nursery", "noUnnecessaryConditions"),
        RuleFilter::Rule("nursery", "noUnresolvedImports"),
        RuleFilter::Rule("nursery", "useArraySortCompare"),
        RuleFilter::Rule("nursery", "useExhaustiveSwitchCases"),
        RuleFilter::Rule("nursery", "useFind"),
        RuleFilter::Rule("nursery", "useRegexpExec"),
    ]
});
static QWIK_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("correctness", "noQwikUseVisibleTask"),
        RuleFilter::Rule("correctness", "useImageSize"),
        RuleFilter::Rule("correctness", "useJsxKeyInIterable"),
        RuleFilter::Rule("correctness", "useQwikClasslist"),
        RuleFilter::Rule("nursery", "useQwikMethodUsage"),
        RuleFilter::Rule("nursery", "useQwikValidLexicalScope"),
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
        RuleFilter::Rule("nursery", "noDuplicatedSpreadProps"),
        RuleFilter::Rule("nursery", "noLeakedRender"),
        RuleFilter::Rule("nursery", "noReactForwardRef"),
        RuleFilter::Rule("nursery", "noSyncScripts"),
        RuleFilter::Rule("nursery", "noUnknownAttribute"),
        RuleFilter::Rule("security", "noDangerouslySetInnerHtml"),
        RuleFilter::Rule("security", "noDangerouslySetInnerHtmlWithChildren"),
        RuleFilter::Rule("style", "useComponentExportOnlyModules"),
        RuleFilter::Rule("style", "useReactFunctionComponents"),
        RuleFilter::Rule("suspicious", "noArrayIndexKey"),
    ]
});
static SOLID_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("correctness", "noSolidDestructuredProps"),
        RuleFilter::Rule("nursery", "noDuplicatedSpreadProps"),
        RuleFilter::Rule("performance", "useSolidForComponent"),
        RuleFilter::Rule("suspicious", "noReactSpecificProps"),
    ]
});
static TEST_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("complexity", "noExcessiveNestedTestSuites"),
        RuleFilter::Rule("suspicious", "noDuplicateTestHooks"),
        RuleFilter::Rule("suspicious", "noExportsInTest"),
        RuleFilter::Rule("suspicious", "noFocusedTests"),
    ]
});
static VUE_FILTERS: LazyLock<Vec<RuleFilter<'static>>> = LazyLock::new(|| {
    vec![
        RuleFilter::Rule("nursery", "noVueDataObjectDeclaration"),
        RuleFilter::Rule("nursery", "noVueDuplicateKeys"),
        RuleFilter::Rule("nursery", "noVueReservedKeys"),
        RuleFilter::Rule("nursery", "noVueReservedProps"),
        RuleFilter::Rule("nursery", "useVueDefineMacrosOrder"),
        RuleFilter::Rule("nursery", "useVueMultiWordComponentNames"),
    ]
});
impl DomainSelector {
    pub fn as_rule_filters(&self) -> Vec<RuleFilter<'static>> {
        match self.0 {
            "next" => NEXT_FILTERS.clone(),
            "project" => PROJECT_FILTERS.clone(),
            "qwik" => QWIK_FILTERS.clone(),
            "react" => REACT_FILTERS.clone(),
            "solid" => SOLID_FILTERS.clone(),
            "test" => TEST_FILTERS.clone(),
            "vue" => VUE_FILTERS.clone(),
            _ => unreachable!("DomainFilter::as_rule_filters: domain {} not found", self.0),
        }
    }
    pub fn match_rule<R>(&self) -> bool
    where
        R: Rule,
    {
        match self.0 {
            "next" => NEXT_FILTERS.iter().any(|filter| filter.match_rule::<R>()),
            "project" => PROJECT_FILTERS
                .iter()
                .any(|filter| filter.match_rule::<R>()),
            "qwik" => QWIK_FILTERS.iter().any(|filter| filter.match_rule::<R>()),
            "react" => REACT_FILTERS.iter().any(|filter| filter.match_rule::<R>()),
            "solid" => SOLID_FILTERS.iter().any(|filter| filter.match_rule::<R>()),
            "test" => TEST_FILTERS.iter().any(|filter| filter.match_rule::<R>()),
            "vue" => VUE_FILTERS.iter().any(|filter| filter.match_rule::<R>()),
            _ => false,
        }
    }
}
