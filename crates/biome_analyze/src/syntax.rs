use biome_rowan::{AstNode, Language, SyntaxNode, WalkEvent};

use crate::{
    registry::NodeLanguage, AddVisitor, Phases, QueryKey, QueryMatch, Queryable, ServiceBag,
    Visitor, VisitorContext,
};

/// Query type usable by lint rules to match on specific [AstNode] types
#[derive(Clone)]
pub struct Ast<N>(pub N);

impl<N> Queryable for Ast<N>
where
    N: AstNode + 'static,
{
    type Input = SyntaxNode<NodeLanguage<N>>;
    type Output = N;

    type Language = NodeLanguage<N>;
    type Services = ();

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _: &<Self::Language as Language>::Root,
    ) {
        analyzer.add_visitor(Phases::Syntax, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}

impl<L: Language + 'static> QueryMatch for SyntaxNode<L> {
    fn text_range(&self) -> biome_rowan::TextRange {
        self.text_trimmed_range()
    }
}

/// The [SyntaxVisitor] is the simplest form of visitor implemented for the
/// analyzer, it simply broadcast each [WalkEvent::Enter] as a query match
/// event for the [SyntaxNode] being entered
pub struct SyntaxVisitor<L: Language> {
    /// If a subtree is currently being skipped by the visitor, for instance
    /// because it has a suppression comment, this stores the root [SyntaxNode]
    /// of that subtree. The visitor will then ignore all events until it
    /// receives a [WalkEvent::Leave] for the `skip_subtree` node
    skip_subtree: Option<SyntaxNode<L>>,
}

impl<L: Language> Default for SyntaxVisitor<L> {
    fn default() -> Self {
        Self { skip_subtree: None }
    }
}

impl<L: Language + 'static> Visitor for SyntaxVisitor<L> {
    type Language = L;

    fn visit(&mut self, event: &WalkEvent<SyntaxNode<Self::Language>>, mut ctx: VisitorContext<L>) {
        let node = match event {
            WalkEvent::Enter(node) => node,
            WalkEvent::Leave(node) => {
                if let Some(skip_subtree) = &self.skip_subtree {
                    if skip_subtree == node {
                        self.skip_subtree = None;
                    }
                }

                return;
            }
        };

        if self.skip_subtree.is_some() {
            return;
        }

        if let Some(range) = ctx.range {
            if node.text_range_with_trivia().ordering(range).is_ne() {
                self.skip_subtree = Some(node.clone());
                return;
            }
        }

        ctx.match_query(node.clone());
    }
}

#[cfg(test)]
mod tests {
    use biome_rowan::{
        raw_language::{RawLanguage, RawLanguageKind, RawLanguageRoot, RawSyntaxTreeBuilder},
        AstNode, BatchMutation, SyntaxNode, SyntaxToken,
    };
    use std::convert::Infallible;

    use crate::{
        matcher::MatchQueryParams, registry::Phases, Analyzer, AnalyzerContext, AnalyzerOptions,
        AnalyzerSignal, ApplySuppression, ControlFlow, MetadataRegistry, Never, QueryMatcher,
        ServiceBag, SuppressionAction, SyntaxVisitor,
    };

    #[derive(Default)]
    struct BufferMatcher {
        nodes: Vec<RawLanguageKind>,
    }

    impl QueryMatcher<RawLanguage> for &mut BufferMatcher {
        fn match_query(&mut self, params: MatchQueryParams<RawLanguage>) {
            self.nodes.push(
                params
                    .query
                    .downcast::<SyntaxNode<RawLanguage>>()
                    .unwrap()
                    .kind(),
            );
        }
    }

    /// Checks the syntax visitor emits a [QueryMatch] for each node in the syntax tree
    #[test]
    fn syntax_visitor() {
        let root = {
            let mut builder = RawSyntaxTreeBuilder::new();

            builder.start_node(RawLanguageKind::ROOT);
            builder.start_node(RawLanguageKind::EXPRESSION_LIST);

            builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
            builder.token(RawLanguageKind::NUMBER_TOKEN, "1");
            builder.finish_node();

            builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
            builder.token(RawLanguageKind::NUMBER_TOKEN, "2");
            builder.finish_node();

            builder.finish_node();
            builder.finish_node();

            RawLanguageRoot::unwrap_cast(builder.finish())
        };

        let mut matcher = BufferMatcher::default();
        let mut emit_signal =
            |_: &dyn AnalyzerSignal<RawLanguage>| -> ControlFlow<Never> { unreachable!() };

        let metadata = MetadataRegistry::default();

        struct TestAction;
        impl SuppressionAction for TestAction {
            type Language = RawLanguage;

            fn find_token_for_inline_suppression(
                &self,
                _: SyntaxToken<Self::Language>,
            ) -> Option<ApplySuppression<Self::Language>> {
                None
            }

            fn apply_inline_suppression(
                &self,
                _: &mut BatchMutation<Self::Language>,
                _: ApplySuppression<Self::Language>,
                _: &str,
                _: &str,
            ) {
                unreachable!("")
            }

            fn apply_top_level_suppression(
                &self,
                _: &mut BatchMutation<Self::Language>,
                _: SyntaxToken<Self::Language>,
                _: &str,
            ) {
                unreachable!("")
            }
        }

        let mut analyzer = Analyzer::new(
            &metadata,
            &mut matcher,
            |_, _| -> Vec<Result<_, Infallible>> { unreachable!() },
            Box::new(TestAction),
            &mut emit_signal,
        );

        analyzer.add_visitor(Phases::Syntax, Box::<SyntaxVisitor<RawLanguage>>::default());

        let ctx: AnalyzerContext<RawLanguage> = AnalyzerContext {
            root,
            range: None,
            services: ServiceBag::default(),
            options: &AnalyzerOptions::default(),
        };

        let result: Option<Never> = analyzer.run(ctx);
        assert!(result.is_none());

        assert_eq!(
            matcher.nodes.as_slice(),
            &[
                RawLanguageKind::ROOT,
                RawLanguageKind::EXPRESSION_LIST,
                RawLanguageKind::LITERAL_EXPRESSION,
                RawLanguageKind::LITERAL_EXPRESSION
            ]
        );
    }
}
