use biome_rowan::{AstNode, Language, SyntaxNode, WalkEvent};
use std::marker::PhantomData;

use crate::{
    AddVisitor, Phases, QueryKey, QueryMatch, Queryable, ServiceBag, Visitor, VisitorContext,
    registry::NodeLanguage,
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
pub struct SyntaxVisitor<L: Language>(PhantomData<L>);

impl<L: Language> Default for SyntaxVisitor<L> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<L: Language + 'static> Visitor for SyntaxVisitor<L> {
    type Language = L;

    fn visit(&mut self, event: &WalkEvent<SyntaxNode<Self::Language>>, mut ctx: VisitorContext<L>) {
        let WalkEvent::Enter(node) = event else {
            return;
        };

        ctx.match_query(node.clone());
    }
}

#[cfg(test)]
mod tests {
    use biome_rowan::{
        AstNode, BatchMutation, SyntaxNode, SyntaxToken, TextRange, TextSize,
        raw_language::{RawLanguage, RawLanguageKind, RawLanguageRoot, RawSyntaxTreeBuilder},
    };
    use std::convert::Infallible;

    use crate::{
        Analyzer, AnalyzerContext, AnalyzerOptions, AnalyzerSignal, AnalyzerSuppression,
        ApplySuppression, ControlFlow, MetadataRegistry, Never, QueryMatcher, ServiceBag,
        Suppression, SuppressionAction, SyntaxVisitor, matcher::MatchQueryParams, registry::Phases,
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

    /// Checks the syntax visitor emits a [QueryMatch] for each node in the syntax tree,
    /// including query nodes outside the requested signal range.
    #[test]
    fn syntax_visitor_emits_queries_outside_the_signal_range() {
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
                _: &biome_rowan::TextRange,
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

            fn suppression_top_level_comment(&self, _suppression_text: &str) -> String {
                unreachable!("")
            }
        }

        struct TestSuppression;

        impl Suppression for TestSuppression {
            type Diagnostic = Infallible;

            fn parse_comment<'a>(
                &self,
                _: &'a str,
                _: biome_rowan::TextRange,
            ) -> Vec<Result<AnalyzerSuppression<'a>, Infallible>> {
                unreachable!()
            }
        }

        let mut analyzer = Analyzer::new(
            &metadata,
            &mut matcher,
            Box::new(TestSuppression),
            Box::new(TestAction),
            &mut emit_signal,
        );

        analyzer.add_visitor(Phases::Syntax, Box::<SyntaxVisitor<RawLanguage>>::default());

        let ctx: AnalyzerContext<RawLanguage> = AnalyzerContext {
            root,
            range: Some(TextRange::new(TextSize::from(1), TextSize::from(2))),
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
