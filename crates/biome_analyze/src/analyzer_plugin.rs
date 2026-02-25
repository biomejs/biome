use biome_diagnostics::Applicability;
use biome_rowan::TextRange;
use camino::{Utf8Path, Utf8PathBuf};
use rustc_hash::{FxHashMap, FxHashSet};
use std::hash::Hash;
use std::{fmt::Debug, sync::Arc};

use biome_rowan::{
    AnySyntaxNode, AstNode, Language, RawSyntaxKind, SyntaxKind, SyntaxNode, WalkEvent,
};

use crate::matcher::SignalRuleKey;
use crate::options::PluginDomainFilter;
use crate::registry::Phases;
use crate::rule::RuleDomain;
use crate::services::ServiceBag;
use crate::{
    PluginSignal, RuleCategory, RuleDiagnostic, SignalEntry, Visitor, VisitorContext, profiling,
};

/// A single text replacement edit from a plugin code action.
#[derive(Debug, Clone)]
pub struct PluginTextEdit {
    pub range: TextRange,
    pub replacement: String,
}

/// A code action (fix) produced by a plugin rule.
#[derive(Debug, Clone)]
pub struct PluginCodeAction {
    pub message: String,
    /// How safe it is to automatically apply this action.
    pub applicability: Applicability,
    pub edits: Vec<PluginTextEdit>,
}

/// A diagnostic paired with code actions from a plugin.
#[derive(Debug)]
pub struct PluginDiagnosticEntry {
    pub diagnostic: RuleDiagnostic,
    pub actions: Vec<PluginCodeAction>,
}

/// Result returned by [`AnalyzerPlugin::evaluate`].
#[derive(Debug, Default)]
pub struct PluginEvaluationResult {
    pub diagnostics: Vec<PluginDiagnosticEntry>,
}

impl PluginEvaluationResult {
    /// Create a result with diagnostics only (no code actions).
    pub fn from_diagnostics(diagnostics: Vec<RuleDiagnostic>) -> Self {
        Self {
            diagnostics: diagnostics
                .into_iter()
                .map(|d| PluginDiagnosticEntry {
                    diagnostic: d,
                    actions: vec![],
                })
                .collect(),
        }
    }
}

/// Slice of analyzer plugins that can be cheaply cloned.
pub type AnalyzerPluginSlice<'a> = &'a [Arc<dyn AnalyzerPlugin>];

/// Vector of analyzer plugins that can be cheaply cloned.
pub type AnalyzerPluginVec = Vec<Arc<dyn AnalyzerPlugin>>;

/// Definition of an analyzer plugin.
///
/// Implemented by WASM, GritQL, and JavaScript plugin loaders. Each plugin
/// exposes one or more lint rules that participate in the analyzer pipeline
/// alongside native rules.
///
/// The analyzer calls [`query`](Self::query) to learn which syntax node kinds
/// the plugin cares about, then invokes [`evaluate`](Self::evaluate) for each
/// matching node during traversal. Diagnostics returned by `evaluate` are
/// displayed with the category `plugin/<rule_name>`.
pub trait AnalyzerPlugin: Debug + Send + Sync {
    /// The target language this plugin analyzes (JavaScript, CSS, or JSON).
    fn language(&self) -> PluginTargetLanguage;

    /// The analysis phase this plugin runs in.
    /// Override to [`Phases::Semantic`] to access the semantic model via services.
    fn phase(&self) -> Phases {
        Phases::Syntax
    }

    /// Syntax node kinds this plugin wants to inspect, as raw `u32` values.
    /// The analyzer only calls [`evaluate`](Self::evaluate) for nodes whose
    /// kind is in this list.
    fn query(&self) -> Vec<RawSyntaxKind>;

    /// Evaluate a matched syntax node and return diagnostics with optional
    /// code actions. Called once per matched node during tree traversal.
    fn evaluate(
        &self,
        node: AnySyntaxNode,
        path: Arc<Utf8PathBuf>,
        services: &ServiceBag,
    ) -> PluginEvaluationResult;

    /// Returns true if this plugin should run on the given file path.
    ///
    /// Stub that always returns `true` — file-scoping will be implemented
    /// in a companion PR (#9171) via the `includes` plugin option.
    fn applies_to_file(&self, _path: &Utf8Path) -> bool {
        true
    }

    /// The rule name used for diagnostic headers, suppression comments, and
    /// per-rule configuration overrides. For example, `"booleanNaming"` results
    /// in the header `plugin/booleanNaming` and suppression comment
    /// `biome-ignore lint/plugin/booleanNaming`.
    fn rule_name(&self) -> &str;

    /// The rule category (lint, action, syntax, transformation). Defaults to Lint.
    fn category(&self) -> RuleCategory {
        RuleCategory::Lint
    }

    /// Domains this rule belongs to (e.g. React, Test). When a rule has
    /// domains, it is only enabled when the user opts into those domains.
    /// Defaults to empty (always enabled if recommended).
    fn domains(&self) -> &[RuleDomain] {
        &[]
    }

    /// Whether this rule is recommended (enabled by default). Defaults to `true`.
    fn is_recommended(&self) -> bool {
        true
    }

    /// If the rule is deprecated, returns the deprecation reason string.
    fn deprecated(&self) -> Option<&str> {
        None
    }

    /// GitHub issue number for rules still under development. When set,
    /// a footnote with a link to the issue is added to diagnostics.
    fn issue_number(&self) -> Option<&str> {
        None
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PluginTargetLanguage {
    JavaScript,
    Css,
    Json,
}

/// A syntax visitor that queries nodes and evaluates in a plugin.
/// Based on [`crate::SyntaxVisitor`].
pub struct PluginVisitor<L: Language> {
    query: FxHashSet<L::Kind>,
    plugin: Arc<dyn AnalyzerPlugin>,

    /// When set, all nodes in this subtree are skipped until we leave it.
    /// Used to skip subtrees that fall entirely outside the analysis range
    /// (see the `ctx.range` check in `visit`).
    skip_subtree: Option<SyntaxNode<L>>,
}

impl<L> PluginVisitor<L>
where
    L: Language + 'static,
    L::Kind: Eq + Hash,
{
    /// Creates a syntax visitor from the plugin.
    ///
    /// # Safety
    /// Do not forget to check the plugin is targeted for the language `L`.
    pub unsafe fn new_unchecked(plugin: Arc<dyn AnalyzerPlugin>) -> Self {
        let query = plugin.query().into_iter().map(L::Kind::from_raw).collect();

        Self {
            query,
            plugin,
            skip_subtree: None,
        }
    }
}

impl<L> Visitor for PluginVisitor<L>
where
    L: Language + 'static,
    L::Kind: Eq + Hash,
{
    type Language = L;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        ctx: VisitorContext<Self::Language>,
    ) {
        let node = match event {
            WalkEvent::Enter(node) => node,
            WalkEvent::Leave(node) => {
                if let Some(skip_subtree) = &self.skip_subtree
                    && skip_subtree == node
                {
                    self.skip_subtree = None;
                }

                return;
            }
        };

        if self.skip_subtree.is_some() {
            return;
        }

        if let Some(range) = ctx.range
            && node.text_range_with_trivia().ordering(range).is_ne()
        {
            self.skip_subtree = Some(node.clone());
            return;
        }

        // TODO: Integrate to [`VisitorContext::match_query`]?
        let kind = node.kind();
        if !self.query.contains(&kind) {
            return;
        }

        if !self.plugin.applies_to_file(&ctx.options.file_path) {
            return;
        }

        // Per-rule disable check via configuration overrides.
        let rule_name = self.plugin.rule_name();
        if let Some(ovr) = ctx.options.plugin_rule_override(rule_name)
            && ovr.disabled
        {
            return;
        }

        // Domain filtering: skip plugin if its domain is disabled or
        // restricted to recommended-only and the plugin is not recommended.
        let plugin_domains = self.plugin.domains();
        if !plugin_domains.is_empty() {
            let domain_cfg = ctx.options.linter_domains();
            if !domain_cfg.is_empty() {
                for domain in plugin_domains {
                    match domain_cfg.get(domain) {
                        Some(&PluginDomainFilter::Disabled) => return,
                        Some(&PluginDomainFilter::Recommended) if !self.plugin.is_recommended() => {
                            return;
                        }
                        _ => {}
                    }
                }
            }
        }

        let rule_timer = profiling::start_plugin_rule("plugin");
        let result = self.plugin.evaluate(
            node.clone().into(),
            ctx.options.file_path.clone(),
            ctx.services,
        );
        rule_timer.stop();

        if result.diagnostics.is_empty() {
            return;
        }

        // Obtain the full source text from the file root — needed for
        // constructing TextEdit diffs for plugin code actions. Only
        // materialized when there are diagnostics, and shared via Arc
        // to avoid cloning per-signal.
        let has_any_action = result.diagnostics.iter().any(|e| !e.actions.is_empty());
        let source_text: Arc<str> = if has_any_action {
            ctx.root.syntax().text_with_trivia().to_string().into()
        } else {
            Arc::from("")
        };

        let root = ctx.root;
        let suppression_action = ctx.suppression_action;
        let options = ctx.options;
        let plugin_category = self.plugin.category();
        let plugin_deprecated: Option<Arc<str>> = self.plugin.deprecated().map(Arc::from);
        let plugin_issue_number: Option<Arc<str>> = self.plugin.issue_number().map(Arc::from);

        let signals = result.diagnostics.into_iter().map(move |entry| {
            let name = entry
                .diagnostic
                .subcategory
                .clone()
                .unwrap_or_else(|| "anonymous".into());

            SignalEntry {
                text_range: entry.diagnostic.span().unwrap_or_default(),
                signal: Box::new(
                    PluginSignal::<L>::new(
                        entry.diagnostic,
                        entry.actions,
                        Arc::clone(&source_text),
                        root,
                        suppression_action,
                        options,
                    )
                    .with_category(plugin_category)
                    .with_deprecated(plugin_deprecated.clone())
                    .with_issue_number(plugin_issue_number.clone()),
                ),
                rule: SignalRuleKey::Plugin(name.into()),
                category: plugin_category,
                instances: Default::default(),
            }
        });

        ctx.signal_queue.extend(signals);
    }
}

/// A batched syntax visitor that evaluates multiple plugins in a single visitor.
///
/// Instead of registering N separate `PluginVisitor` instances (one per plugin),
/// this holds all plugins together and dispatches using a kind-to-plugin lookup
/// map. This reduces visitor-dispatch overhead and enables O(1) kind matching
/// per node instead of iterating all plugins.
pub struct BatchPluginVisitor<L: Language> {
    plugins: Vec<Arc<dyn AnalyzerPlugin>>,

    /// Maps each syntax kind to the indices of plugins that query for it.
    kind_to_plugins: FxHashMap<L::Kind, Vec<usize>>,

    /// When set, all nodes in this subtree are skipped until we leave it.
    /// Used to skip subtrees that fall entirely outside the analysis range
    /// (see the `ctx.range` check in `visit`).
    skip_subtree: Option<SyntaxNode<L>>,

    /// Cached per-plugin results of `applies_to_file`. Populated lazily on
    /// first `WalkEvent::Enter` — the file path is constant for the entire walk.
    applicable: Option<Vec<bool>>,
}

impl<L> BatchPluginVisitor<L>
where
    L: Language + 'static,
    L::Kind: Eq + Hash,
{
    /// Creates a batched plugin visitor from a slice of plugins.
    ///
    /// # Safety
    /// Caller must ensure all plugins target language `L`. The `RawSyntaxKind`
    /// values returned by each plugin's `query()` are converted to `L::Kind`
    /// via `from_raw` without validation.
    pub unsafe fn new_unchecked(plugins: AnalyzerPluginSlice) -> Self {
        let mut all_plugins = Vec::with_capacity(plugins.len());
        let mut kind_to_plugins: FxHashMap<L::Kind, Vec<usize>> = FxHashMap::default();

        for (idx, plugin) in plugins.iter().enumerate() {
            all_plugins.push(Arc::clone(plugin));
            let mut seen_kinds = FxHashSet::default();
            for raw_kind in plugin.query() {
                let kind = L::Kind::from_raw(raw_kind);
                if seen_kinds.insert(kind) {
                    kind_to_plugins.entry(kind).or_default().push(idx);
                }
            }
        }

        Self {
            plugins: all_plugins,
            kind_to_plugins,
            skip_subtree: None,
            applicable: None,
        }
    }
}

impl<L> Visitor for BatchPluginVisitor<L>
where
    L: Language + 'static,
    L::Kind: Eq + Hash,
{
    type Language = L;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        ctx: VisitorContext<Self::Language>,
    ) {
        let node = match event {
            WalkEvent::Enter(node) => node,
            WalkEvent::Leave(node) => {
                if let Some(skip_subtree) = &self.skip_subtree
                    && skip_subtree == node
                {
                    self.skip_subtree = None;
                }

                return;
            }
        };

        if self.skip_subtree.is_some() {
            return;
        }

        if let Some(range) = ctx.range
            && node.text_range_with_trivia().ordering(range).is_ne()
        {
            self.skip_subtree = Some(node.clone());
            return;
        }

        let kind = node.kind();

        let Some(plugin_indices) = self.kind_to_plugins.get(&kind) else {
            return;
        };

        let applicable = self.applicable.get_or_insert_with(|| {
            self.plugins
                .iter()
                .map(|p| p.applies_to_file(&ctx.options.file_path))
                .collect()
        });

        for &idx in plugin_indices {
            if !applicable[idx] {
                continue;
            }

            let plugin = &self.plugins[idx];

            // Per-rule disable check via configuration overrides.
            let rule_name = plugin.rule_name();
            if let Some(ovr) = ctx.options.plugin_rule_override(rule_name)
                && ovr.disabled
            {
                continue;
            }

            // Domain filtering: skip plugin if its domain is disabled or
            // restricted to recommended-only and the plugin is not recommended.
            let plugin_domains = plugin.domains();
            if !plugin_domains.is_empty() {
                let domain_cfg = ctx.options.linter_domains();
                if !domain_cfg.is_empty() {
                    let mut skip = false;
                    for domain in plugin_domains {
                        match domain_cfg.get(domain) {
                            Some(&PluginDomainFilter::Disabled) => {
                                skip = true;
                                break;
                            }
                            Some(&PluginDomainFilter::Recommended)
                                if !plugin.is_recommended() =>
                            {
                                skip = true;
                                break;
                            }
                            _ => {}
                        }
                    }
                    if skip {
                        continue;
                    }
                }
            }

            let rule_timer = profiling::start_plugin_rule("plugin");
            let result = plugin.evaluate(
                node.clone().into(),
                ctx.options.file_path.clone(),
                ctx.services,
            );
            rule_timer.stop();

            if result.diagnostics.is_empty() {
                continue;
            }

            // Obtain the full source text from the file root — needed for
            // constructing TextEdit diffs for plugin code actions.
            let has_any_action = result.diagnostics.iter().any(|e| !e.actions.is_empty());
            let source_text: Arc<str> = if has_any_action {
                ctx.root.syntax().text_with_trivia().to_string().into()
            } else {
                Arc::from("")
            };

            let root = ctx.root;
            let suppression_action = ctx.suppression_action;
            let options = ctx.options;
            let plugin_category = plugin.category();
            let plugin_deprecated: Option<Arc<str>> = plugin.deprecated().map(Arc::from);
            let plugin_issue_number: Option<Arc<str>> = plugin.issue_number().map(Arc::from);

            let signals = result.diagnostics.into_iter().map(move |entry| {
                let name = entry
                    .diagnostic
                    .subcategory
                    .clone()
                    .unwrap_or_else(|| "anonymous".into());

                SignalEntry {
                    text_range: entry.diagnostic.span().unwrap_or_default(),
                    signal: Box::new(
                        PluginSignal::<L>::new(
                            entry.diagnostic,
                            entry.actions,
                            Arc::clone(&source_text),
                            root,
                            suppression_action,
                            options,
                        )
                        .with_category(plugin_category)
                        .with_deprecated(plugin_deprecated.clone())
                        .with_issue_number(plugin_issue_number.clone()),
                    ),
                    rule: SignalRuleKey::Plugin(name.into()),
                    category: plugin_category,
                    instances: Default::default(),
                }
            });

            ctx.signal_queue.extend(signals);
        }
    }
}
