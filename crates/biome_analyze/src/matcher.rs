use crate::{
    AnalyzerOptions, AnalyzerSignal, Phases, QueryMatch, Rule, RuleFilter, RuleGroup, ServiceBag,
    SuppressionAction,
};
use biome_rowan::{Language, TextRange};
use std::{
    any::{Any, TypeId},
    cmp::Ordering,
    collections::BinaryHeap,
};

/// The [QueryMatcher] trait is responsible of running lint rules on
/// [QueryMatch](crate::QueryMatch) instances emitted by the various
/// [Visitor](crate::Visitor) and push signals wrapped in [SignalEntry]
/// to the signal queue
pub trait QueryMatcher<L: Language> {
    /// Execute a single query match
    fn match_query(&mut self, params: MatchQueryParams<L>);
}

/// Parameters provided to [QueryMatcher::match_query] and require to run lint rules
pub struct MatchQueryParams<'phase, 'query, L: Language> {
    pub phase: Phases,
    pub root: &'phase L::Root,
    pub query: Query,
    pub services: &'phase ServiceBag,
    pub signal_queue: &'query mut BinaryHeap<SignalEntry<'phase, L>>,
    pub suppression_action: &'phase dyn SuppressionAction<Language = L>,
    pub options: &'phase AnalyzerOptions,
}

/// Wrapper type for a [QueryMatch]
///
/// This type is functionally equivalent to `Box<dyn QueryMatch + Any>`, it
/// emulates dynamic dispatch for both traits and allows downcasting to a
/// reference or owned type.
pub struct Query {
    data: Box<dyn Any>,
    read_text_range: fn(&dyn Any) -> TextRange,
}

impl Query {
    /// Construct a new [Query] instance from a [QueryMatch]
    pub fn new<T: QueryMatch>(data: T) -> Self {
        Self {
            data: Box::new(data),
            read_text_range: |query| query.downcast_ref::<T>().unwrap().text_range(),
        }
    }

    /// Attempt to downcast the query to an owned type.
    pub fn downcast<T: QueryMatch>(self) -> Option<T> {
        Some(*self.data.downcast::<T>().ok()?)
    }

    /// Attempt to downcast the query to a reference type.
    pub fn downcast_ref<T: QueryMatch>(&self) -> Option<&T> {
        self.data.downcast_ref::<T>()
    }

    /// Returns the [TypeId] of this query, equivalent to calling [Any::type_id].
    pub(crate) fn type_id(&self) -> TypeId {
        self.data.as_ref().type_id()
    }

    /// Returns the [TextRange] of this query, equivalent to calling [QueryMatch::text_range].
    pub(crate) fn text_range(&self) -> TextRange {
        (self.read_text_range)(self.data.as_ref())
    }
}

/// Opaque identifier for a group of rule
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GroupKey {
    group: &'static str,
}

impl GroupKey {
    pub(crate) fn new(group: &'static str) -> Self {
        Self { group }
    }

    pub fn group<G: RuleGroup>() -> Self {
        Self::new(G::NAME)
    }
}

impl From<GroupKey> for RuleFilter<'static> {
    fn from(key: GroupKey) -> Self {
        RuleFilter::Group(key.group)
    }
}

/// Opaque identifier for a single rule
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RuleKey {
    group: &'static str,
    rule: &'static str,
}

impl RuleKey {
    pub fn new(group: &'static str, rule: &'static str) -> Self {
        Self { group, rule }
    }

    pub fn rule<R: Rule>() -> Self {
        Self::new(<R::Group as RuleGroup>::NAME, R::METADATA.name)
    }

    pub fn group(&self) -> &'static str {
        self.group
    }

    pub fn rule_name(&self) -> &'static str {
        self.rule
    }
}

impl From<RuleKey> for RuleFilter<'static> {
    fn from(key: RuleKey) -> Self {
        RuleFilter::Rule(key.group, key.rule)
    }
}

impl PartialEq<RuleKey> for RuleFilter<'static> {
    fn eq(&self, other: &RuleKey) -> bool {
        match *self {
            RuleFilter::Group(group) => group == other.group,
            RuleFilter::Rule(group, rule) => group == other.group && rule == other.rule,
        }
    }
}

/// Entry for a pending signal in the `signal_queue`
pub struct SignalEntry<'phase, L: Language> {
    /// Boxed analyzer signal to be emitted
    pub signal: Box<dyn AnalyzerSignal<L> + 'phase>,
    /// Unique identifier for the rule that emitted this signal
    pub rule: RuleKey,
    /// Optional rule instances being suppressed
    pub instances: Box<[Box<str>]>,
    /// Text range in the document this signal covers
    pub text_range: TextRange,
}

// SignalEntry is ordered based on the starting point of its `text_range`
impl<L: Language> Ord for SignalEntry<'_, L> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.text_range.start().cmp(&self.text_range.start())
    }
}

impl<L: Language> PartialOrd for SignalEntry<'_, L> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<L: Language> Eq for SignalEntry<'_, L> {}

impl<L: Language> PartialEq for SignalEntry<'_, L> {
    fn eq(&self, other: &Self) -> bool {
        self.text_range.start() == other.text_range.start()
    }
}

/// Adapter type wrapping a [QueryMatcher] type with a function that can be
/// used to inspect the query matches emitted by the analyzer
pub struct InspectMatcher<F, I> {
    func: F,
    inner: I,
}

impl<F, I> InspectMatcher<F, I> {
    ///  Create a new instance of [InspectMatcher] from an existing [QueryMatcher]
    /// object and an inspection function
    pub fn new<L>(inner: I, func: F) -> Self
    where
        L: Language,
        F: FnMut(&MatchQueryParams<L>),
        I: QueryMatcher<L>,
    {
        Self { func, inner }
    }
}

impl<L, F, I> QueryMatcher<L> for InspectMatcher<F, I>
where
    L: Language,
    F: FnMut(&MatchQueryParams<L>),
    I: QueryMatcher<L>,
{
    fn match_query(&mut self, params: MatchQueryParams<L>) {
        (self.func)(&params);
        self.inner.match_query(params);
    }
}

#[cfg(test)]
mod tests {
    use super::MatchQueryParams;
    use crate::{
        signals::DiagnosticSignal, Analyzer, AnalyzerContext, AnalyzerSignal, ApplySuppression,
        ControlFlow, MetadataRegistry, Never, Phases, QueryMatcher, RuleKey, ServiceBag,
        SignalEntry, SuppressionAction, SyntaxVisitor,
    };
    use crate::{AnalyzerOptions, AnalyzerSuppression};
    use biome_diagnostics::{category, DiagnosticExt};
    use biome_diagnostics::{Diagnostic, Severity};
    use biome_rowan::{
        raw_language::{RawLanguage, RawLanguageKind, RawLanguageRoot, RawSyntaxTreeBuilder},
        AstNode, BatchMutation, SyntaxNode, SyntaxToken, TextRange, TextSize, TriviaPiece,
    };
    use std::convert::Infallible;

    struct SuppressionMatcher;

    #[derive(Debug, Diagnostic)]
    #[diagnostic(category = "args/fileNotFound", message = "test_suppression")]
    struct TestDiagnostic {
        #[location(span)]
        span: TextRange,
    }

    impl QueryMatcher<RawLanguage> for SuppressionMatcher {
        /// Emits a warning diagnostic for all literal expressions
        fn match_query(&mut self, params: MatchQueryParams<RawLanguage>) {
            let node = params.query.downcast::<SyntaxNode<RawLanguage>>().unwrap();

            if node.kind() != RawLanguageKind::LITERAL_EXPRESSION {
                return;
            }

            let span = node.text_trimmed_range();
            params.signal_queue.push(SignalEntry {
                signal: Box::new(DiagnosticSignal::new(move || TestDiagnostic { span })),
                rule: RuleKey::new("group", "rule"),
                instances: Default::default(),
                text_range: span,
            });
        }
    }

    #[test]
    fn suppressions() {
        let root = {
            let mut builder = RawSyntaxTreeBuilder::new();

            builder.start_node(RawLanguageKind::ROOT);
            builder.start_node(RawLanguageKind::SEPARATED_EXPRESSION_LIST);

            builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
            builder.token_with_trivia(
                RawLanguageKind::STRING_TOKEN,
                "//group\n\"warn_here\"",
                &[TriviaPiece::single_line_comment(7), TriviaPiece::newline(1)],
                &[],
            );
            builder.finish_node();

            builder.token_with_trivia(
                RawLanguageKind::SEMICOLON_TOKEN,
                ";\n",
                &[],
                &[TriviaPiece::newline(1)],
            );

            builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
            builder.token_with_trivia(
                RawLanguageKind::STRING_TOKEN,
                "//group/rule\n\"warn_here\"",
                &[
                    TriviaPiece::single_line_comment(12),
                    TriviaPiece::newline(1),
                ],
                &[],
            );
            builder.finish_node();

            builder.token_with_trivia(
                RawLanguageKind::SEMICOLON_TOKEN,
                ";\n",
                &[],
                &[TriviaPiece::newline(1)],
            );

            builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
            builder.token_with_trivia(
                RawLanguageKind::STRING_TOKEN,
                "//unknown_group\n\"warn_here\"",
                &[
                    TriviaPiece::single_line_comment(15),
                    TriviaPiece::newline(1),
                ],
                &[],
            );
            builder.finish_node();

            builder.token_with_trivia(
                RawLanguageKind::SEMICOLON_TOKEN,
                ";\n",
                &[],
                &[TriviaPiece::newline(1)],
            );

            builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
            builder.token_with_trivia(
                RawLanguageKind::STRING_TOKEN,
                "//group/unknown_rule\n\"warn_here\"",
                &[
                    TriviaPiece::single_line_comment(20),
                    TriviaPiece::newline(1),
                ],
                &[],
            );
            builder.finish_node();

            builder.token_with_trivia(
                RawLanguageKind::SEMICOLON_TOKEN,
                ";\n",
                &[],
                &[TriviaPiece::newline(1)],
            );

            builder.token_with_trivia(
                RawLanguageKind::SEMICOLON_TOKEN,
                "//group/rule\n;\n",
                &[
                    TriviaPiece::single_line_comment(12),
                    TriviaPiece::newline(1),
                ],
                &[TriviaPiece::newline(1)],
            );

            builder.finish_node();
            builder.finish_node();

            RawLanguageRoot::unwrap_cast(builder.finish())
        };

        let mut diagnostics = Vec::new();
        let mut emit_signal = |signal: &dyn AnalyzerSignal<RawLanguage>| -> ControlFlow<Never> {
            let diag = signal.diagnostic().expect("diagnostic");
            let range = diag.get_span().expect("range");
            let error = diag.with_severity(Severity::Warning);
            let code = error.category().expect("code");

            diagnostics.push((code, range));
            ControlFlow::Continue(())
        };

        fn parse_suppression_comment(
            comment: &str,
            _piece_range: TextRange,
        ) -> Vec<Result<AnalyzerSuppression, Infallible>> {
            comment
                .trim_start_matches("//")
                .split(' ')
                .map(AnalyzerSuppression::rule)
                .map(Ok)
                .collect()
        }

        let mut metadata = MetadataRegistry::default();
        metadata.insert_rule("group", "rule");

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
            SuppressionMatcher,
            parse_suppression_comment,
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
            diagnostics.as_slice(),
            &[
                (
                    category!("suppressions/unknownGroup"),
                    TextRange::new(TextSize::from(47), TextSize::from(62))
                ),
                (
                    category!("args/fileNotFound"),
                    TextRange::new(TextSize::from(63), TextSize::from(74))
                ),
                (
                    category!("suppressions/unknownRule"),
                    TextRange::new(TextSize::from(76), TextSize::from(96))
                ),
                (
                    category!("args/fileNotFound"),
                    TextRange::new(TextSize::from(97), TextSize::from(108))
                ),
                (
                    category!("suppressions/unused"),
                    TextRange::new(TextSize::from(110), TextSize::from(122))
                ),
            ]
        );
    }
}
