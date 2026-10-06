//! Exports the metadata of every lint rule and assist action as JSON, along
//! with what Biome knows about the upstream rules they come from: which ones
//! are implemented and which ones are deliberately unsupported.
//!
//! The output is meant for tooling that tracks rule coverage, so its shape is
//! versioned with [SCHEMA_VERSION]. Upstream rules that Biome doesn't
//! reference anywhere aren't listed; such tooling lists them itself, and the
//! `ruleUrl` of each source lets it link to them.

use biome_analyze::{
    GroupCategory, Queryable, RegistryVisitor, Rule, RuleCategory, RuleGroup, RuleSource,
    RuleSourceKind, UNSUPPORTED_RULES, UnsupportedRuleReason,
};
use biome_rowan::syntax::Language;
use biome_string_case::Case;
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::PathBuf;
use xtask_glue::Result;

/// Bump when the shape of the output changes.
const SCHEMA_VERSION: u32 = 1;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RulesMetadata {
    schema_version: u32,
    /// Every upstream tool in `RuleSource`, in declaration order.
    sources: Vec<SourceInfo>,
    /// Every Biome lint rule and assist action.
    biome_rules: Vec<BiomeRule>,
    /// Every upstream rule a Biome rule's `sources` or [UNSUPPORTED_RULES]
    /// references.
    upstream_rules: Vec<UpstreamRule>,
    /// The reasons an [UNSUPPORTED_RULES] entry can give.
    unsupported_reasons: Vec<UnsupportedReason>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SourceInfo {
    /// The `RuleSource` variant, in camelCase (the same name serde uses).
    id: String,
    /// Human-readable name of the tool or plugin.
    name: String,
    /// The prefix of the rule names in the upstream configuration, e.g. `jest`.
    namespace: &'static str,
    /// The URL of a rule's documentation, with `{rule}` standing for the rule
    /// name and, for markdownlint, `{id}` for its ID. Tools that document all
    /// their rules on one page have a URL without either.
    rule_url: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct BiomeRule {
    name: &'static str,
    group: &'static str,
    /// `lint` or `assist`.
    category: &'static str,
    language: &'static str,
    version: &'static str,
    recommended: bool,
    deprecated: Option<&'static str>,
    fix_kind: serde_json::Value,
    domains: serde_json::Value,
    url: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpstreamRule {
    source: String,
    rule: String,
    /// The name as written in the upstream tool's configuration.
    namespaced_name: String,
    url: String,
    implemented_by: Vec<Implementation>,
    unsupported: Option<Unsupported>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Implementation {
    /// The Biome rule's name.
    rule: &'static str,
    /// `sameLogic` or `inspired`.
    kind: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Unsupported {
    /// The `id` of one of the `unsupportedReasons`.
    reason: &'static str,
    /// The formatter option or Biome rule, for the reasons that name one.
    detail: Option<&'static str>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UnsupportedReason {
    /// The camelCase name of the variant.
    id: &'static str,
    label: &'static str,
    /// What the variant's argument names: `formatterOption`, `biomeRule`, or
    /// null for variants without one.
    detail: Option<&'static str>,
}

/// Every [UnsupportedRuleReason] variant, with placeholder arguments.
const ALL_UNSUPPORTED_REASONS: [UnsupportedRuleReason; 11] = [
    UnsupportedRuleReason::Stylistic,
    UnsupportedRuleReason::FormatterCovers,
    UnsupportedRuleReason::FormatterOption(""),
    UnsupportedRuleReason::CoveredByRule(""),
    UnsupportedRuleReason::NotApplicable,
    UnsupportedRuleReason::Deprecated,
    UnsupportedRuleReason::Legacy,
    UnsupportedRuleReason::RequiresExternalTool,
    UnsupportedRuleReason::ParserCovers,
    UnsupportedRuleReason::KnownSourceNotImplemented,
    UnsupportedRuleReason::UnknownSource,
];

/// Describes a reason, or returns `None` for the reasons that describe
/// migration results rather than entries of [UNSUPPORTED_RULES].
///
/// When adding a variant here, also add it to [ALL_UNSUPPORTED_REASONS].
fn unsupported_reason(reason: &UnsupportedRuleReason) -> Option<UnsupportedReason> {
    let (id, label, detail) = match reason {
        UnsupportedRuleReason::Stylistic => (
            "stylistic",
            "Stylistic, incompatible with the formatter",
            None,
        ),
        UnsupportedRuleReason::FormatterCovers => (
            "formatterCovers",
            "Redundant, the formatter covers it",
            None,
        ),
        UnsupportedRuleReason::FormatterOption(_) => (
            "formatterOption",
            "Covered by a formatter option",
            Some("formatterOption"),
        ),
        UnsupportedRuleReason::CoveredByRule(_) => (
            "coveredByRule",
            "Covered by another Biome rule",
            Some("biomeRule"),
        ),
        UnsupportedRuleReason::NotApplicable => ("notApplicable", "Not applicable to Biome", None),
        UnsupportedRuleReason::Deprecated => ("deprecated", "Deprecated upstream", None),
        UnsupportedRuleReason::Legacy => ("legacy", "Legacy, at odds with modern code", None),
        UnsupportedRuleReason::RequiresExternalTool => {
            ("requiresExternalTool", "Requires an external tool", None)
        }
        UnsupportedRuleReason::ParserCovers => {
            ("parserCovers", "Redundant, the parser covers it", None)
        }
        UnsupportedRuleReason::KnownSourceNotImplemented | UnsupportedRuleReason::UnknownSource => {
            return None;
        }
    };
    Some(UnsupportedReason { id, label, detail })
}

/// Every `RuleSource` variant in declaration order, with placeholders for the
/// rule name and ID. The export checks the order against
/// `RuleSource::variant_index`, and fails if a rule references a variant
/// missing here.
const ALL_SOURCES: [RuleSource<'static>; 60] = {
    use RuleSource::*;
    const RULE: &str = "{rule}";
    [
        Clippy(RULE),
        DenoLint(RULE),
        Eslint(RULE),
        EslintBarrelFiles(RULE),
        EslintBetterTailwindcss(RULE),
        EslintE18e(RULE),
        EslintGraphql(RULE),
        EslintImport(RULE),
        EslintImportAccess(RULE),
        EslintJest(RULE),
        EslintJsDoc(RULE),
        EslintJsxA11y(RULE),
        EslintMysticatea(RULE),
        EslintN(RULE),
        EslintNext(RULE),
        EslintNoSecrets(RULE),
        EslintPackageJson(RULE),
        EslintPackageJsonDependencies(RULE),
        EslintPerfectionist(RULE),
        EslintPromise(RULE),
        EslintQwik(RULE),
        EslintReact(RULE),
        EslintReactHooks(RULE),
        EslintReactNative(RULE),
        EslintReactNativeIntellicode(RULE),
        EslintReactPreferFunctionComponent(RULE),
        EslintReactRefresh(RULE),
        EslintReactXyz(RULE),
        EslintReactX(RULE),
        EslintReactJsx(RULE),
        EslintReactDom(RULE),
        EslintReactRsc(RULE),
        EslintReactNamingConvention(RULE),
        EslintRegexp(RULE),
        EslintShadcn(RULE),
        EslintSolid(RULE),
        EslintSvelte(RULE),
        EslintSonarJs(RULE),
        EslintStylistic(RULE),
        EslintTailwindcss(RULE),
        EslintTypeScript(RULE),
        EslintUnicorn(RULE),
        EslintUnusedImports(RULE),
        EslintVitest(RULE),
        EslintVueJs(RULE),
        GraphqlSchemaLinter(RULE),
        Stylelint(RULE),
        EslintTurbo(RULE),
        HtmlEslint(RULE),
        EslintPlaywright(RULE),
        EslintJson(RULE),
        EslintMarkdown(RULE),
        EslintYml(RULE),
        EslintCss(RULE),
        EslintAstro(RULE),
        EslintDrizzle(RULE),
        SortPackageJson,
        Sherif(RULE),
        EslintTypescriptSortKeys(RULE),
        MarkdownLint("{id}", RULE),
    ]
};

/// Returns the camelCase name of a `RuleSource` variant.
fn source_id(source: &RuleSource) -> String {
    match serde_json::to_value(source).expect("RuleSource to serialize") {
        serde_json::Value::String(id) => id,
        serde_json::Value::Object(map) => map
            .into_iter()
            .next()
            .map(|(id, _)| id)
            .expect("RuleSource to serialize as a single-key object"),
        other => panic!("unexpected RuleSource serialization: {other}"),
    }
}

/// Identifies an upstream rule. `MarkdownLint` sources carry both an ID and a
/// name, so the key uses the name like everything else does.
fn upstream_key(source: &RuleSource) -> (String, String) {
    (source_id(source), source.as_rule_name().to_string())
}

#[derive(Default)]
struct Collector {
    /// The category whose groups are being recorded.
    category: Option<RuleCategory>,
    /// Keyed by id; every variant is added up front.
    sources: BTreeMap<String, SourceInfo>,
    biome_rules: Vec<BiomeRule>,
    upstream_rules: BTreeMap<(String, String), UpstreamRule>,
}

impl Collector {
    fn upstream(&mut self, source: &RuleSource) -> &mut UpstreamRule {
        let key = upstream_key(source);
        assert!(
            self.sources.contains_key(&key.0),
            "{source:?} isn't in ALL_SOURCES; add its RuleSource variant there"
        );
        self.upstream_rules
            .entry(key.clone())
            .or_insert_with(|| UpstreamRule {
                source: key.0,
                rule: key.1,
                namespaced_name: source.to_namespaced_rule_name(),
                url: source.to_rule_url(),
                implemented_by: Vec::new(),
                unsupported: None,
            })
    }
}

impl<L: Language> RegistryVisitor<L> for Collector {
    fn record_category<C: GroupCategory<Language = L>>(&mut self) {
        if matches!(C::CATEGORY, RuleCategory::Lint | RuleCategory::Action) {
            self.category = Some(C::CATEGORY);
            C::record_groups(self);
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Query: Queryable<Language = L, Output: Clone>> + 'static,
    {
        let metadata = &R::METADATA;
        let category = match self.category {
            Some(RuleCategory::Action) => "assist",
            _ => "lint",
        };
        let url = match category {
            "assist" => format!(
                "https://biomejs.dev/assist/actions/{}/",
                Case::Kebab.convert(metadata.name)
            ),
            _ => format!(
                "https://biomejs.dev/linter/rules/{}/",
                Case::Kebab.convert(metadata.name)
            ),
        };
        self.biome_rules.push(BiomeRule {
            name: metadata.name,
            group: <R::Group as RuleGroup>::NAME,
            category,
            language: metadata.language,
            version: metadata.version,
            recommended: metadata.recommended,
            deprecated: metadata.deprecated,
            fix_kind: serde_json::to_value(metadata.fix_kind).expect("FixKind to serialize"),
            domains: serde_json::to_value(metadata.domains).expect("domains to serialize"),
            url,
        });
        for source in metadata.sources {
            let kind = match source.kind {
                RuleSourceKind::SameLogic => "sameLogic",
                RuleSourceKind::Inspired => "inspired",
            };
            let upstream = self.upstream(&source.source);
            if !upstream
                .implemented_by
                .iter()
                .any(|implementation| implementation.rule == metadata.name)
            {
                upstream.implemented_by.push(Implementation {
                    rule: metadata.name,
                    kind,
                });
            }
        }
    }
}

pub(crate) fn generate_rules_metadata(out: Option<PathBuf>) -> Result<()> {
    let mut collector = Collector::default();
    for (index, source) in ALL_SOURCES.iter().enumerate() {
        assert_eq!(
            usize::from(source.variant_index()),
            index,
            "ALL_SOURCES must list RuleSource variants in declaration order, but {source:?} is at {index}"
        );
        let id = source_id(source);
        collector.sources.insert(
            id.clone(),
            SourceInfo {
                id,
                name: source.to_string(),
                namespace: source.namespace(),
                rule_url: source.to_rule_url(),
            },
        );
    }
    biome_js_analyze::visit_registry(&mut collector);
    biome_json_analyze::visit_registry(&mut collector);
    biome_css_analyze::visit_registry(&mut collector);
    biome_graphql_analyze::visit_registry(&mut collector);
    biome_html_analyze::visit_registry(&mut collector);
    biome_markdown_analyze::visit_registry(&mut collector);

    for rule in UNSUPPORTED_RULES {
        let Some(reason) = unsupported_reason(&rule.1) else {
            continue;
        };
        let detail = match &rule.1 {
            UnsupportedRuleReason::FormatterOption(detail)
            | UnsupportedRuleReason::CoveredByRule(detail) => Some(*detail),
            _ => None,
        };
        collector.upstream(&rule.0).unsupported = Some(Unsupported {
            reason: reason.id,
            detail,
        });
    }

    collector
        .biome_rules
        .sort_by(|a, b| (a.category, a.name).cmp(&(b.category, b.name)));

    let metadata = RulesMetadata {
        schema_version: SCHEMA_VERSION,
        sources: ALL_SOURCES
            .iter()
            .filter_map(|source| collector.sources.remove(&source_id(source)))
            .collect(),
        biome_rules: collector.biome_rules,
        upstream_rules: collector.upstream_rules.into_values().collect(),
        unsupported_reasons: ALL_UNSUPPORTED_REASONS
            .iter()
            .filter_map(unsupported_reason)
            .collect(),
    };
    let json = serde_json::to_string_pretty(&metadata)?;
    match out {
        Some(path) => std::fs::write(path, json)?,
        None => println!("{json}"),
    }
    Ok(())
}
