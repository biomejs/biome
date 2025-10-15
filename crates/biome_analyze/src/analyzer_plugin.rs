use camino::Utf8PathBuf;
use rustc_hash::FxHashSet;
use std::hash::Hash;
use std::{fmt::Debug, sync::Arc};

use biome_rowan::{AnySyntaxNode, Language, RawSyntaxKind, SyntaxKind, SyntaxNode, WalkEvent};

use crate::matcher::SignalRuleKey;
use crate::{DiagnosticSignal, RuleCategory, RuleDiagnostic, SignalEntry, Visitor, VisitorContext};

/// Slice of analyzer plugins that can be cheaply cloned.
pub type AnalyzerPluginSlice<'a> = &'a [Arc<Box<dyn AnalyzerPlugin>>];

/// Vector of analyzer plugins that can be cheaply cloned.
pub type AnalyzerPluginVec = Vec<Arc<Box<dyn AnalyzerPlugin>>>;

/// Definition of an analyzer plugin.
pub trait AnalyzerPlugin: Debug + Send + Sync {
    fn language(&self) -> PluginTargetLanguage;

    fn query(&self) -> Vec<RawSyntaxKind>;

    fn evaluate(&self, node: AnySyntaxNode, path: Arc<Utf8PathBuf>) -> Vec<RuleDiagnostic>;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PluginTargetLanguage {
    JavaScript,
    Css,
}

/// A syntax visitor that queries nodes and evaluates in a plugin.
/// Based on [`crate::SyntaxVisitor`].
pub struct PluginVisitor<L: Language> {
    query: FxHashSet<L::Kind>,
    plugin: Arc<Box<dyn AnalyzerPlugin>>,
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

        let signals = self
            .plugin
            .evaluate(node.clone().into(), ctx.options.file_path.clone())
            .into_iter()
            .map(|diagnostic| {
                let name = diagnostic
                    .subcategory
                    .clone()
                    .unwrap_or_else(|| "anonymous".into());

                SignalEntry {
                    text_range: diagnostic.span().unwrap_or_default(),
                    signal: Box::new(DiagnosticSignal::new(move || diagnostic.clone())),
                    rule: SignalRuleKey::Plugin(name.into()),
                    category: RuleCategory::Lint,
                    instances: Default::default(),
                }
            });

        ctx.signal_queue.extend(signals);
    }
}
