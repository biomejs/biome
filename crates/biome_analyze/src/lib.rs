#![deny(rustdoc::broken_intra_doc_links)]

use biome_console::markup;
use biome_parser::AnyParse;
use std::collections::{BTreeMap, BinaryHeap};
use std::fmt::{Debug, Display, Formatter};
use std::ops;
use std::sync::Arc;

mod analyzer_plugin;
mod categories;
pub mod context;
mod diagnostics;
mod matcher;
pub mod options;
mod query;
mod registry;
mod rule;
mod services;
mod signals;
mod suppression_action;
mod suppressions;
mod syntax;
mod visitor;

// Re-exported for use in the `declare_group` macro
pub use biome_diagnostics::category_concat;

pub use crate::analyzer_plugin::{AnalyzerPlugin, AnalyzerPluginSlice, AnalyzerPluginVec};
pub use crate::categories::{
    ActionCategory, OtherActionCategory, RefactorKind, RuleCategories, RuleCategoriesBuilder,
    RuleCategory, SourceActionKind, SUPPRESSION_INLINE_ACTION_CATEGORY,
    SUPPRESSION_TOP_LEVEL_ACTION_CATEGORY,
};
pub use crate::diagnostics::{AnalyzerDiagnostic, AnalyzerSuppressionDiagnostic, RuleError};
pub use crate::matcher::{InspectMatcher, MatchQueryParams, QueryMatcher, RuleKey, SignalEntry};
pub use crate::options::{AnalyzerConfiguration, AnalyzerOptions, AnalyzerRules};
pub use crate::query::{AddVisitor, QueryKey, QueryMatch, Queryable};
pub use crate::registry::{
    LanguageRoot, MetadataRegistry, Phase, Phases, RegistryRuleMetadata, RegistryVisitor,
    RuleRegistry, RuleRegistryBuilder, RuleSuppressions,
};
pub use crate::rule::{
    CategoryLanguage, FixKind, GroupCategory, GroupLanguage, Rule, RuleAction, RuleDiagnostic,
    RuleDomain, RuleGroup, RuleMeta, RuleMetadata, RuleSource, RuleSourceKind, SuppressAction,
};
pub use crate::services::{FromServices, MissingServicesDiagnostic, ServiceBag};
pub use crate::signals::{
    AnalyzerAction, AnalyzerSignal, AnalyzerTransformation, DiagnosticSignal,
};
use crate::suppressions::Suppressions;
pub use crate::syntax::{Ast, SyntaxVisitor};
pub use crate::visitor::{NodeVisitor, Visitor, VisitorContext, VisitorFinishContext};
use biome_diagnostics::{category, Diagnostic, DiagnosticExt};
use biome_rowan::{
    AstNode, BatchMutation, Direction, Language, SyntaxElement, SyntaxToken, TextRange, TextSize,
    TokenAtOffset, TriviaPieceKind, WalkEvent,
};
use biome_suppression::{Suppression, SuppressionKind};
pub use suppression_action::{ApplySuppression, SuppressionAction};

/// The analyzer is the main entry point into the `biome_analyze` infrastructure.
/// Its role is to run a collection of [Visitor]s over a syntax tree, with each
/// visitor implementing various analysis over this syntax tree to generate
/// auxiliary data structures as well as emit "query match" events to be
/// processed by lint rules and in turn emit "analyzer signals" in the form of
/// diagnostics, code actions or both.
/// The analyzer also has support for plugins, although do not (as of yet)
/// support the same visitor pattern. This makes them slower to execute, but
/// otherwise they act the same for consumers of the analyzer. They respect the
/// same suppression comments, and report signals in the same format.
pub struct Analyzer<'analyzer, L: Language, Matcher, Break, Diag> {
    /// List of visitors being run by this instance of the analyzer for each phase
    phases: BTreeMap<Phases, Vec<Box<dyn Visitor<Language = L> + 'analyzer>>>,
    /// Plugins to be run after the phases for built-in rules.
    plugins: AnalyzerPluginVec,
    /// Holds the metadata for all the rules statically known to the analyzer
    metadata: &'analyzer MetadataRegistry,
    /// Executor for the query matches emitted by the visitors
    query_matcher: Matcher,
    /// Language-specific suppression comment parsing function
    parse_suppression_comment: SuppressionParser<Diag>,
    /// Language-specific suppression comment emitter
    suppression_action: Box<dyn SuppressionAction<Language = L>>,
    /// Handles analyzer signals emitted by individual rules
    emit_signal: SignalHandler<'analyzer, L, Break>,
}

pub struct AnalyzerContext<'a, L: Language> {
    pub root: LanguageRoot<L>,
    pub services: ServiceBag,
    pub range: Option<TextRange>,
    pub options: &'a AnalyzerOptions,
}

impl<'analyzer, L, Matcher, Break, Diag> Analyzer<'analyzer, L, Matcher, Break, Diag>
where
    L: Language + 'static,
    Matcher: QueryMatcher<L>,
    Diag: Diagnostic + Clone + Send + Sync + 'static,
{
    /// Construct a new instance of the analyzer with the given rule registry
    /// and suppression comment parser
    pub fn new(
        metadata: &'analyzer MetadataRegistry,
        query_matcher: Matcher,
        parse_suppression_comment: SuppressionParser<Diag>,
        suppression_action: Box<dyn SuppressionAction<Language = L>>,
        emit_signal: SignalHandler<'analyzer, L, Break>,
    ) -> Self {
        Self {
            phases: BTreeMap::new(),
            plugins: Vec::new(),
            metadata,
            query_matcher,
            parse_suppression_comment,
            suppression_action,
            emit_signal,
        }
    }

    /// Registers a [Visitor] to be executed as part of a given `phase` of the analyzer run
    pub fn add_visitor(
        &mut self,
        phase: Phases,
        visitor: Box<dyn Visitor<Language = L> + 'analyzer>,
    ) {
        self.phases.entry(phase).or_default().push(visitor);
    }

    /// Registers an [AnalyzerPlugin] to be executed after the regular phases.
    pub fn add_plugin(&mut self, plugin: Arc<Box<dyn AnalyzerPlugin>>) {
        self.plugins.push(plugin);
    }

    pub fn run(self, mut ctx: AnalyzerContext<L>) -> Option<Break> {
        let Self {
            phases,
            plugins,
            mut query_matcher,
            parse_suppression_comment,
            mut emit_signal,
            suppression_action,
            metadata: _,
        } = self;

        let mut line_index = 0;
        let mut suppressions = Suppressions::new(self.metadata);

        for (index, (phase, mut visitors)) in phases.into_iter().enumerate() {
            let runner = PhaseRunner {
                phase,
                visitors: &mut visitors,
                query_matcher: &mut query_matcher,
                signal_queue: BinaryHeap::new(),
                parse_suppression_comment,
                line_index: &mut line_index,
                emit_signal: &mut emit_signal,
                root: &ctx.root,
                services: &ctx.services,
                range: ctx.range,
                suppression_action: suppression_action.as_ref(),
                options: ctx.options,
                suppressions: &mut suppressions,
            };

            // The first phase being run will inspect the tokens and parse the
            // suppression comments, then subsequent phases only needs to read
            // this data again since it's already cached in `line_suppressions`
            let result = if index == 0 {
                runner.run_first_phase()
            } else {
                runner.run_remaining_phases()
            };

            if let ControlFlow::Break(br) = result {
                return Some(br);
            }

            // Finish all the active visitors, this is executed outside of the
            // phase runner as it needs mutable access to the service bag (the
            // runner borrows the services for the entire phase)
            for visitor in visitors {
                visitor.finish(VisitorFinishContext {
                    root: &ctx.root,
                    services: &mut ctx.services,
                });
            }
        }

        for plugin in plugins {
            let root: AnyParse = ctx.root.syntax().as_send().expect("not a root node").into();
            for diagnostic in plugin.evaluate(root, ctx.options.file_path.clone()) {
                let signal = DiagnosticSignal::new(|| diagnostic.clone());

                if let ControlFlow::Break(br) = (emit_signal)(&signal) {
                    return Some(br);
                }
            }
        }

        for range_suppression in suppressions.range_suppressions.suppressions {
            if range_suppression.did_suppress_signal {
                continue;
            }
            if let Some(range) = range_suppression.already_suppressed {
                let signal = DiagnosticSignal::new(|| {
                    AnalyzerSuppressionDiagnostic::new(
                        category!("suppressions/unused"),
                        range_suppression.start_comment_range,
                        "Suppression comment has no effect because another suppression comment suppresses the same rule.",
                    ).note(
                        markup!{"This is the suppression comment that was used."}.to_owned(),
                        range
                    )
                });
                if let ControlFlow::Break(br) = (emit_signal)(&signal) {
                    return Some(br);
                }
            }
        }

        for suppression in suppressions.line_suppressions {
            if suppression.did_suppress_signal {
                continue;
            }

            let signal = DiagnosticSignal::new(|| {
                if let Some(range) = suppression.already_suppressed {
                    AnalyzerSuppressionDiagnostic::new(
                        category!("suppressions/unused"),
                        suppression.comment_span,
                        "Suppression comment has no effect because another suppression comment suppresses the same rule.",
                    ).note(
                        markup!{"This is the suppression comment that was used."}.to_owned(),
                        range
                    )
                } else {
                    AnalyzerSuppressionDiagnostic::new(
                    category!("suppressions/unused"),
                    suppression.comment_span,
                    "Suppression comment has no effect. Remove the suppression or make sure you are suppressing the correct rule.",
                    )
                }
            });

            if let ControlFlow::Break(br) = (emit_signal)(&signal) {
                return Some(br);
            }
        }

        None
    }
}

/// Holds all the state required to run a single analysis phase to completion
struct PhaseRunner<'analyzer, 'phase, L: Language, Matcher, Break, Diag> {
    /// Identifier of the phase this runner is executing
    phase: Phases,
    /// List of visitors being run by this instance of the analyzer for each phase
    visitors: &'phase mut [Box<dyn Visitor<Language = L> + 'analyzer>],
    /// Executor for the query matches emitted by the visitors
    query_matcher: &'phase mut Matcher,
    /// Queue for pending analyzer signals
    signal_queue: BinaryHeap<SignalEntry<'phase, L>>,
    /// Language-specific suppression comment parsing function
    parse_suppression_comment: SuppressionParser<Diag>,
    /// Language-specific suppression comment emitter
    suppression_action: &'phase dyn SuppressionAction<Language = L>,
    /// Line index at the current position of the traversal
    line_index: &'phase mut usize,
    /// Handles analyzer signals emitted by individual rules
    emit_signal: &'phase mut SignalHandler<'analyzer, L, Break>,
    /// Root node of the file being analyzed
    root: &'phase L::Root,
    /// Service bag handle for this phase
    services: &'phase ServiceBag,
    /// Optional text range to restrict the analysis to
    range: Option<TextRange>,
    /// Analyzer options
    options: &'phase AnalyzerOptions,
    /// Tracks all suppressions during the analyzer phase
    suppressions: &'phase mut Suppressions<'analyzer>,
}

impl<L, Matcher, Break, Diag> PhaseRunner<'_, '_, L, Matcher, Break, Diag>
where
    L: Language,
    Matcher: QueryMatcher<L>,
    Diag: Diagnostic + Clone + Send + Sync + 'static,
{
    /// Runs phase 0 over nodes and tokens to process line breaks and
    /// suppression comments
    fn run_first_phase(mut self) -> ControlFlow<Break> {
        let iter = self.root.syntax().preorder_with_tokens(Direction::Next);
        for event in iter {
            let node_event = match event {
                WalkEvent::Enter(SyntaxElement::Node(node)) => WalkEvent::Enter(node),
                WalkEvent::Leave(SyntaxElement::Node(node)) => WalkEvent::Leave(node),

                // If this is a token enter event, process its text content
                WalkEvent::Enter(SyntaxElement::Token(token)) => {
                    self.handle_token(token)?;

                    continue;
                }
                WalkEvent::Leave(SyntaxElement::Token(_)) => {
                    continue;
                }
            };
            // If this is a node event pass it to the visitors for this phase
            for visitor in self.visitors.iter_mut() {
                let ctx = VisitorContext {
                    phase: self.phase,
                    root: self.root,
                    services: self.services,
                    range: self.range,
                    query_matcher: self.query_matcher,
                    signal_queue: &mut self.signal_queue,
                    suppression_action: self.suppression_action,
                    options: self.options,
                };

                visitor.visit(&node_event, ctx);
            }
        }

        // Flush all remaining pending events
        self.flush_matches(None)
    }

    /// Runs phases 1..N over nodes, since suppression comments were already
    /// processed and cached in `run_initial_phase`
    fn run_remaining_phases(mut self) -> ControlFlow<Break> {
        for event in self.root.syntax().preorder() {
            // Run all the active visitors for the phase on the event
            for visitor in self.visitors.iter_mut() {
                let ctx = VisitorContext {
                    phase: self.phase,
                    root: self.root,
                    services: self.services,
                    range: self.range,
                    query_matcher: self.query_matcher,
                    signal_queue: &mut self.signal_queue,
                    suppression_action: self.suppression_action,
                    options: self.options,
                };

                visitor.visit(&event, ctx);
            }

            // Flush all pending query signals
            self.flush_matches(None)?;
        }

        ControlFlow::Continue(())
    }

    /// Process the text for a single token, parsing suppression comments and
    /// handling line breaks, then flush all pending query signals in the queue
    /// whose position is less than the end of the token within the file
    fn handle_token(&mut self, token: SyntaxToken<L>) -> ControlFlow<Break> {
        // Process the content of the token for comments and newline
        for (index, piece) in token.leading_trivia().pieces().enumerate() {
            if matches!(
                piece.kind(),
                TriviaPieceKind::Newline
                    | TriviaPieceKind::MultiLineComment
                    | TriviaPieceKind::Skipped
            ) {
                self.bump_line_index(piece.text(), piece.text_range());
            }

            if let Some(comment) = piece.as_comments() {
                self.handle_comment(&token, true, index, comment.text(), piece.text_range())?;
            }
        }

        self.bump_line_index(token.text_trimmed(), token.text_trimmed_range());

        for (index, piece) in token.trailing_trivia().pieces().enumerate() {
            if matches!(
                piece.kind(),
                TriviaPieceKind::Newline
                    | TriviaPieceKind::MultiLineComment
                    | TriviaPieceKind::Skipped
            ) {
                self.bump_line_index(piece.text(), piece.text_range());
            }

            if let Some(comment) = piece.as_comments() {
                self.handle_comment(&token, false, index, comment.text(), piece.text_range())?;
            }
        }

        // Flush signals from the queue until the end of the current token is reached
        let cutoff = token.text_range().end();
        self.flush_matches(Some(cutoff))
    }

    /// Flush all pending query signals in the queue.  If `cutoff` is specified,
    /// signals that start after this position in the file will be skipped
    fn flush_matches(&mut self, cutoff: Option<TextSize>) -> ControlFlow<Break> {
        while let Some(entry) = self.signal_queue.peek() {
            let start = entry.text_range.start();
            if let Some(cutoff) = cutoff {
                if start >= cutoff {
                    break;
                }
            }

            if self
                .suppressions
                .top_level_suppression
                .suppressed_rule(&entry.rule)
                || self.suppressions.top_level_suppression.suppress_all
            {
                self.signal_queue.pop();
                break;
            }

            if self
                .suppressions
                .range_suppressions
                .suppressed_rule(&entry.rule, &entry.text_range)
            {
                self.signal_queue.pop();
                break;
            }

            // Search for an active line suppression comment covering the range of
            // this signal: first try to load the last line suppression and see
            // if it matches the current line index, otherwise perform a binary
            // search over all the previously seen suppressions to find one
            // with a matching range
            let suppression =
                self.suppressions
                    .line_suppressions
                    .last_mut()
                    .filter(|suppression| {
                        suppression.line_index == *self.line_index
                            && suppression.text_range.start() <= start
                    });

            let suppression = match suppression {
                Some(suppression) => Some(suppression),
                None => {
                    let index =
                        self.suppressions
                            .line_suppressions
                            .partition_point(|suppression| {
                                suppression.text_range.end() < entry.text_range.start()
                            });

                    if index >= self.suppressions.line_suppressions.len() {
                        None
                    } else {
                        Some(&mut self.suppressions.line_suppressions[index])
                    }
                }
            };

            let suppression = suppression.filter(|suppression| {
                if suppression.suppress_all {
                    return true;
                }
                if suppression.suppressed_instances.is_empty() {
                    suppression
                        .suppressed_rules
                        .iter()
                        .any(|filter| *filter == entry.rule)
                } else {
                    entry.instances.iter().all(|value| {
                        suppression
                            .suppressed_instances
                            .iter()
                            .any(|(v, filter)| *filter == entry.rule && v == value.as_ref())
                    })
                }
            });

            // If the signal is being suppressed, mark the line suppression as
            // hit, otherwise emit the signal
            if let Some(suppression) = suppression {
                suppression.did_suppress_signal = true;
            } else if range_match(self.range, entry.text_range) {
                (self.emit_signal)(&*entry.signal)?;
            }

            // Remove signal from the queue.
            self.signal_queue.pop();
        }

        ControlFlow::Continue(())
    }

    /// Parse the text content of a comment trivia piece for suppression
    /// comments, and create line suppression entries accordingly
    fn handle_comment(
        &mut self,
        token: &SyntaxToken<L>,
        _is_leading: bool,
        _index: usize,
        text: &str,
        range: TextRange,
    ) -> ControlFlow<Break> {
        let mut has_suppressions = false;

        for result in (self.parse_suppression_comment)(text, range) {
            let suppression = match result {
                Ok(kind) => kind,
                Err(diag) => {
                    // Emit the suppression parser diagnostic
                    let signal = DiagnosticSignal::new(move || {
                        let location = diag.location();
                        let span = location.span.map_or(range, |span| span + range.start());
                        diag.clone().with_file_span(span)
                    });

                    (self.emit_signal)(&signal)?;
                    continue;
                }
            };

            if let Err(diagnostic) =
                self.suppressions
                    .push_suppression(&suppression, range, token.text_range())
            {
                let signal = DiagnosticSignal::new(|| diagnostic.clone());
                (self.emit_signal)(&signal)?;
                continue;
            }

            has_suppressions = true;
        }

        // Suppression comments apply to the next line
        if has_suppressions {
            let line_index = *self.line_index + 1;

            self.suppressions
                .overlap_last_suppression(line_index, range);
        }

        ControlFlow::Continue(())
    }

    /// Check a piece of source text (token or trivia) for line breaks and
    /// increment the line index accordingly, extending the range of the
    /// current suppression as required
    fn bump_line_index(&mut self, text: &str, range: TextRange) {
        let mut did_match = false;
        for (index, _) in text.match_indices(['\n']) {
            let index = TextSize::try_from(index)
                .expect("integer overflow while converting a suppression line to `TextSize`");
            let range = TextRange::at(range.start(), index);
            did_match = self.suppressions.expand_range(range, *self.line_index);

            *self.line_index += 1;
            self.suppressions.bump_line_index(*self.line_index);
        }

        if !did_match {
            self.suppressions.expand_range(range, *self.line_index);
        }
    }
}

fn range_match(filter: Option<TextRange>, range: TextRange) -> bool {
    filter.map_or(true, |filter| filter.intersect(range).is_some())
}

/// Signature for a suppression comment parser function
///
/// This function receives two parameters:
/// 1. The text content of a comment.
/// 2. The range of the token the comment belongs too. The range is calculated from [SyntaxToken::text_range], so the range
///    includes all trivia.
///
/// It returns the lint suppressions as an optional lint rule (if the lint rule is `None` the
/// comment is interpreted as suppressing all lints)
///
/// # Examples
///
/// - `// biome-ignore format` -> `vec![]`
/// - `// biome-ignore lint` -> `vec![Everything]`
/// - `// biome-ignore lint/complexity/useWhile` -> `vec![Rule("complexity/useWhile")]`
/// - `// biome-ignore lint/complexity/useWhile(foo)` -> `vec![RuleWithValue("complexity/useWhile", "foo")]`
/// - `// biome-ignore lint/complexity/useWhile lint/nursery/noUnreachable` -> `vec![Rule("complexity/useWhile"), Rule("nursery/noUnreachable")]`
/// - `/** biome-ignore lint/complexity/useWhile */` if the comment is top-level -> `vec![TopLevel("complexity/useWhile")]`
type SuppressionParser<D> =
    for<'a> fn(&'a str, TextRange) -> Vec<Result<AnalyzerSuppression<'a>, D>>;

#[derive(Debug, Clone)]
/// This enum is used to categorize what is disabled by a suppression comment and with what syntax
pub struct AnalyzerSuppression<'a> {
    /// The kind of suppression
    pub(crate) kind: AnalyzerSuppressionKind<'a>,

    /// The range where the `biome-ignore` comment is placed inside the whole text
    pub(crate) ignore_range: Option<TextRange>,

    /// The kind of `biome-ignore` comment used for this suppression
    pub(crate) variant: AnalyzerSuppressionVariant,
}

#[derive(Debug, Clone)]
pub enum AnalyzerSuppressionVariant {
    /// biome-ignore
    Line,
    /// biome-ignore-all
    TopLevel,
    /// biome-ignore-start
    RangeStart,
    /// biome-ignore-end
    RangeEnd,
}

impl From<&SuppressionKind> for AnalyzerSuppressionVariant {
    fn from(value: &SuppressionKind) -> Self {
        match value {
            SuppressionKind::Classic => AnalyzerSuppressionVariant::Line,
            SuppressionKind::All => AnalyzerSuppressionVariant::TopLevel,
            SuppressionKind::RangeStart => AnalyzerSuppressionVariant::RangeStart,
            SuppressionKind::RangeEnd => AnalyzerSuppressionVariant::RangeEnd,
        }
    }
}

impl<'a> AnalyzerSuppression<'a> {
    pub fn everything() -> Self {
        Self {
            kind: AnalyzerSuppressionKind::Everything,
            ignore_range: None,
            variant: AnalyzerSuppressionVariant::Line,
        }
    }

    pub fn rule_instance(rule: &'a str, instance: &'a str) -> Self {
        Self {
            kind: AnalyzerSuppressionKind::RuleInstance(rule, instance),
            ignore_range: None,
            variant: AnalyzerSuppressionVariant::Line,
        }
    }
    pub fn rule(rule: &'a str) -> Self {
        Self {
            kind: AnalyzerSuppressionKind::Rule(rule),
            ignore_range: None,
            variant: AnalyzerSuppressionVariant::Line,
        }
    }

    #[must_use]
    pub fn with_ignore_range(mut self, ignore_range: TextRange) -> Self {
        self.ignore_range = Some(ignore_range);
        self
    }

    #[must_use]
    pub fn with_variant(mut self, variant: impl Into<AnalyzerSuppressionVariant>) -> Self {
        self.variant = variant.into();
        self
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AnalyzerSuppressionKind<'a> {
    /// A suppression disabling all lints eg. `// biome-ignore lint`
    Everything,
    /// A suppression disabling a specific rule eg. `// biome-ignore lint/complexity/useWhile`
    Rule(&'a str),
    /// A suppression to be evaluated by a specific rule eg. `// biome-ignore lint/correctness/useExhaustiveDependencies(foo)`
    RuleInstance(&'a str, &'a str),
}

/// Takes a [Suppression] and returns a [AnalyzerSuppression]
pub fn to_analyzer_suppressions(
    suppression: Suppression,
    piece_range: TextRange,
) -> Vec<AnalyzerSuppression> {
    let mut result = Vec::with_capacity(suppression.categories.len());
    let ignore_range = TextRange::new(
        piece_range.add_start(suppression.range().start()).start(),
        piece_range.add_start(suppression.range().end()).start(),
    );
    for (key, value) in suppression.categories {
        if key == category!("lint") {
            result.push(AnalyzerSuppression::everything().with_variant(&suppression.kind));
        } else {
            let category = key.name();
            if let Some(rule) = category.strip_prefix("lint/") {
                let suppression = if let Some(instance) = value {
                    AnalyzerSuppression::rule_instance(rule, instance)
                        .with_ignore_range(ignore_range)
                } else {
                    AnalyzerSuppression::rule(rule).with_ignore_range(ignore_range)
                }
                .with_variant(&suppression.kind);
                result.push(suppression);
            }
        }
    }

    result
}

impl AnalyzerSuppression<'_> {
    pub const fn is_top_level(&self) -> bool {
        matches!(self.variant, AnalyzerSuppressionVariant::TopLevel)
    }
    pub const fn is_range_start(&self) -> bool {
        matches!(self.variant, AnalyzerSuppressionVariant::RangeStart)
    }
    pub const fn is_range_end(&self) -> bool {
        matches!(self.variant, AnalyzerSuppressionVariant::RangeEnd)
    }
    pub const fn is_line(&self) -> bool {
        matches!(self.variant, AnalyzerSuppressionVariant::Line)
    }
}

/// Payload received by the function responsible to mark a suppression comment
pub struct SuppressionCommentEmitterPayload<'a, L: Language> {
    /// The possible offset found in the [TextRange] of the emitted diagnostic
    pub token_offset: TokenAtOffset<SyntaxToken<L>>,
    /// A [BatchMutation] where the consumer can apply the suppression comment
    pub mutation: &'a mut BatchMutation<L>,
    /// A string equals to "biome-ignore: lint(<RULE_GROUP>/<RULE_NAME>)"
    pub suppression_text: &'a str,
    /// The original range of the diagnostic where the rule was triggered
    pub diagnostic_text_range: &'a TextRange,
    /// Explanation for the suppression to be used with `--suppress` and `--reason`
    pub suppression_reason: &'a str,
}

type SignalHandler<'a, L, Break> = &'a mut dyn FnMut(&dyn AnalyzerSignal<L>) -> ControlFlow<Break>;

/// Allow filtering a single rule or group of rules by their names
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RuleFilter<'a> {
    Group(&'a str),
    Rule(&'a str, &'a str),
}

impl<'a> RuleFilter<'a> {
    // Returns the group name of this filter.
    pub fn group(self) -> &'a str {
        match self {
            RuleFilter::Group(group) => group,
            RuleFilter::Rule(group, _) => group,
        }
    }
    /// Return `true` if the group `G` matches this filter
    pub fn match_group<G: RuleGroup>(self) -> bool {
        match self {
            RuleFilter::Group(group) => group == G::NAME,
            RuleFilter::Rule(group, _) => group == G::NAME,
        }
    }

    /// Return `true` if the rule `R` matches this filter
    pub fn match_rule<R>(self) -> bool
    where
        R: Rule,
    {
        match self {
            RuleFilter::Group(group) => group == <R::Group as RuleGroup>::NAME,
            RuleFilter::Rule(group, rule) => {
                group == <R::Group as RuleGroup>::NAME && rule == R::METADATA.name
            }
        }
    }
}

impl Debug for RuleFilter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for RuleFilter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RuleFilter::Group(group) => {
                write!(f, "{group}")
            }
            RuleFilter::Rule(group, rule) => {
                write!(f, "{group}/{rule}")
            }
        }
    }
}

impl biome_console::fmt::Display for RuleFilter<'_> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        match self {
            RuleFilter::Group(group) => {
                write!(fmt, "{group}")
            }
            RuleFilter::Rule(group, rule) => {
                write!(fmt, "{group}/{rule}")
            }
        }
    }
}

/// Allows filtering the list of rules that will be executed in a run of the analyzer,
/// and at what source code range signals (diagnostics or actions) may be raised
#[derive(Debug, Default, Clone, Copy)]
pub struct AnalysisFilter<'a> {
    /// Only allow rules with these categories to emit signals
    pub categories: RuleCategories,
    /// Only allow rules matching these names to emit signals
    /// If `enabled_rules` is set to `None`, then all rules are enabled.
    pub enabled_rules: Option<&'a [RuleFilter<'a>]>,
    /// Do not allow rules matching these names to emit signals
    pub disabled_rules: &'a [RuleFilter<'a>],
    /// Only emit signals matching this text range
    pub range: Option<TextRange>,
}

impl<'analysis> AnalysisFilter<'analysis> {
    /// It creates a new filter with the set of [enabled rules](RuleFilter) passed as argument
    pub fn from_enabled_rules(enabled_rules: &'analysis [RuleFilter<'analysis>]) -> Self {
        Self {
            enabled_rules: Some(enabled_rules),
            ..AnalysisFilter::default()
        }
    }

    /// Return `true` if the category `C` matches this filter
    pub fn match_category<C: GroupCategory>(&self) -> bool {
        self.categories.contains(C::CATEGORY)
    }

    /// Return `true` if the group `G` matches this filter
    pub fn match_group<G: RuleGroup>(&self) -> bool {
        self.match_category::<G::Category>()
            && self.enabled_rules.map_or(true, |enabled_rules| {
                enabled_rules.iter().any(|filter| filter.match_group::<G>())
            })
            && !self
                .disabled_rules
                .iter()
                .any(|filter| matches!(filter, RuleFilter::Group(_)) && filter.match_group::<G>())
    }

    /// Return `true` if the rule `R` matches this filter
    pub fn match_rule<R: Rule>(&self) -> bool {
        self.match_category::<<R::Group as RuleGroup>::Category>()
            && self.enabled_rules.map_or(true, |enabled_rules| {
                enabled_rules.iter().any(|filter| filter.match_rule::<R>())
            })
            && !self
                .disabled_rules
                .iter()
                .any(|filter| filter.match_rule::<R>())
    }
}

/// Utility type to be used as a default value for the `B` generic type on
/// `analyze` when the provided callback never breaks
///
/// This should eventually get replaced with the `!` type when it gets stabilized
pub enum Never {}

/// Type alias of [ops::ControlFlow] with the `B` generic type defaulting to [Never]
///
/// By default, the analysis loop never breaks, so it behaves mostly like
/// `let b = loop {};` and has a "break type" of `!` (the `!` type isn't stable
/// yet, so I'm using an empty enum instead, but they're identical for this
/// purpose)
///
/// In practice, it's not really a `loop` but a `for` because it's iterating on
/// all nodes in the syntax tree, so when it reaches the end of the iterator
/// the loop will exit but without producing a value of type `B`: for this
/// reason the `analyze` function returns an `Option<B>` that's set to
/// `Some(B)` if the callback did break, and `None` if the analysis reached the
/// end of the file.
///
/// Most consumers of the analyzer will want to analyze the entire file at once
/// and never break, so using [Never] as the type of `B` in this case lets the
/// compiler know the `ControlFlow::Break` branch will never be taken and can
/// be optimized out, as well as completely remove the `return Some` case
/// (`Option<Never>` has a size of 0 and can be elided, while `Option<()>` has
/// a size of 1 as it still need to store a discriminant)
pub type ControlFlow<B = Never> = ops::ControlFlow<B>;
