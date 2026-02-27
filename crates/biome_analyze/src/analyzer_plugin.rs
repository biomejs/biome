use camino::{Utf8Path, Utf8PathBuf};
use rustc_hash::{FxHashMap, FxHashSet};
use std::hash::Hash;
use std::{fmt::Debug, sync::Arc};

use biome_rowan::{AnySyntaxNode, Language, RawSyntaxKind, SyntaxKind, SyntaxNode, WalkEvent};

use crate::matcher::SignalRuleKey;
use crate::{
    PluginSignal, RuleCategory, RuleDiagnostic, SignalEntry, Visitor, VisitorContext, profiling,
};

/// Slice of analyzer plugins that can be cheaply cloned.
pub type AnalyzerPluginSlice<'a> = &'a [Arc<Box<dyn AnalyzerPlugin>>];

/// Vector of analyzer plugins that can be cheaply cloned.
pub type AnalyzerPluginVec = Vec<Arc<Box<dyn AnalyzerPlugin>>>;

/// Definition of an analyzer plugin.
pub trait AnalyzerPlugin: Debug + Send + Sync {
    fn language(&self) -> PluginTargetLanguage;

    fn query(&self) -> Vec<RawSyntaxKind>;

    fn evaluate(&self, node: AnySyntaxNode, path: Arc<Utf8PathBuf>) -> Vec<RuleDiagnostic>;

    /// Returns true if this plugin should run on the given file path.
    ///
    /// Stub that always returns `true` — file-scoping will be implemented
    /// in a companion PR (#9171) via the `includes` plugin option.
    fn applies_to_file(&self, _path: &Utf8Path) -> bool {
        true
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
    plugin: Arc<Box<dyn AnalyzerPlugin>>,

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
    pub unsafe fn new_unchecked(plugin: Arc<Box<dyn AnalyzerPlugin>>) -> Self {
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

        let rule_timer = profiling::start_plugin_rule("plugin");
        let diagnostics = self
            .plugin
            .evaluate(node.clone().into(), ctx.options.file_path.clone());
        rule_timer.stop();

        let signals = diagnostics.into_iter().map(|diagnostic| {
            let name = diagnostic
                .subcategory
                .clone()
                .unwrap_or_else(|| "anonymous".into());

            SignalEntry {
                text_range: diagnostic.span().unwrap_or_default(),
                signal: Box::new(PluginSignal::<L>::new(diagnostic)),
                rule: SignalRuleKey::Plugin(name.into()),
                category: RuleCategory::Lint,
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
    plugins: Vec<Arc<Box<dyn AnalyzerPlugin>>>,

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
            let rule_timer = profiling::start_plugin_rule("plugin");
            let diagnostics = plugin.evaluate(node.clone().into(), ctx.options.file_path.clone());
            rule_timer.stop();

            let signals = diagnostics.into_iter().map(|diagnostic| {
                let name = diagnostic
                    .subcategory
                    .clone()
                    .unwrap_or_else(|| "anonymous".into());

                SignalEntry {
                    text_range: diagnostic.span().unwrap_or_default(),
                    signal: Box::new(PluginSignal::<L>::new(diagnostic)),
                    rule: SignalRuleKey::Plugin(name.into()),
                    category: RuleCategory::Lint,
                    instances: Default::default(),
                }
            });

            ctx.signal_queue.extend(signals);
        }
    }
}
