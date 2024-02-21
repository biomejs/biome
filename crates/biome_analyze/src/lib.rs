#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../CONTRIBUTING.md")]

use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};
use std::fmt::{Debug, Display, Formatter};
use std::ops;
use tracing::trace;

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
mod syntax;
mod visitor;

// Re-exported for use in the `declare_group` macro
pub use biome_diagnostics::category_concat;

pub use crate::categories::{
    ActionCategory, RefactorKind, RuleCategories, RuleCategory, SourceActionKind,
};
pub use crate::diagnostics::AnalyzerDiagnostic;
pub use crate::diagnostics::SuppressionDiagnostic;
pub use crate::matcher::{InspectMatcher, MatchQueryParams, QueryMatcher, RuleKey, SignalEntry};
pub use crate::options::{AnalyzerConfiguration, AnalyzerOptions, AnalyzerRules};
pub use crate::query::{AddVisitor, QueryKey, QueryMatch, Queryable};
pub use crate::registry::{
    LanguageRoot, MetadataRegistry, Phase, Phases, RegistryRuleMetadata, RegistryVisitor,
    RuleRegistry, RuleRegistryBuilder, RuleSuppressions,
};
pub use crate::rule::{
    CategoryLanguage, FixKind, GroupCategory, GroupLanguage, Rule, RuleAction, RuleDiagnostic,
    RuleGroup, RuleMeta, RuleMetadata, RuleSource, RuleSourceKind, SuppressAction,
};
pub use crate::services::{FromServices, MissingServicesDiagnostic, ServiceBag};
pub use crate::signals::{
    AnalyzerAction, AnalyzerSignal, AnalyzerTransformation, DiagnosticSignal,
};
pub use crate::syntax::{Ast, SyntaxVisitor};
pub use crate::visitor::{NodeVisitor, Visitor, VisitorContext, VisitorFinishContext};

use biome_console::markup;
use biome_diagnostics::{
    category, Applicability, Diagnostic, DiagnosticExt, DiagnosticTags, Severity,
};
use biome_rowan::{
    AstNode, BatchMutation, Direction, Language, SyntaxElement, SyntaxToken, TextLen, TextRange,
    TextSize, TokenAtOffset, TriviaPiece, TriviaPieceKind, WalkEvent,
};

/// The analyzer is the main entry point into the `biome_analyze` infrastructure.
/// Its role is to run a collection of [Visitor]s over a syntax tree, with each
/// visitor implementing various analysis over this syntax tree to generate
/// auxiliary data structures as well as emit "query match" events to be
/// processed by lint rules and in turn emit "analyzer signals" in the form of
/// diagnostics, code actions or both
pub struct Analyzer<'analyzer, L: Language, Matcher, Break, Diag> {
    /// List of visitors being run by this instance of the analyzer for each phase
    phases: BTreeMap<Phases, Vec<Box<dyn Visitor<Language = L> + 'analyzer>>>,
    /// Holds the metadata for all the rules statically known to the analyzer
    metadata: &'analyzer MetadataRegistry,
    /// Executor for the query matches emitted by the visitors
    query_matcher: Matcher,
    /// Language-specific suppression comment parsing function
    parse_suppression_comment: SuppressionParser<Diag>,
    /// Language-specific suppression comment emitter
    apply_suppression_comment: SuppressionCommentEmitter<L>,
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
    L: Language,
    Matcher: QueryMatcher<L>,
    Diag: Diagnostic + Clone + Send + Sync + 'static,
{
    /// Construct a new instance of the analyzer with the given rule registry
    /// and suppression comment parser
    pub fn new(
        metadata: &'analyzer MetadataRegistry,
        query_matcher: Matcher,
        parse_suppression_comment: SuppressionParser<Diag>,
        apply_suppression_comment: SuppressionCommentEmitter<L>,
        emit_signal: SignalHandler<'analyzer, L, Break>,
    ) -> Self {
        Self {
            phases: BTreeMap::new(),
            metadata,
            query_matcher,
            parse_suppression_comment,
            apply_suppression_comment,
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

    pub fn run(self, mut ctx: AnalyzerContext<L>) -> Option<Break> {
        let Self {
            phases,
            metadata,
            mut query_matcher,
            parse_suppression_comment,
            mut emit_signal,
            apply_suppression_comment,
        } = self;

        let mut line_index = 0;
        let mut line_suppressions = Vec::new();

        for (index, (phase, mut visitors)) in phases.into_iter().enumerate() {
            let runner = PhaseRunner {
                phase,
                visitors: &mut visitors,
                metadata,
                query_matcher: &mut query_matcher,
                signal_queue: BinaryHeap::new(),
                parse_suppression_comment,
                line_index: &mut line_index,
                line_suppressions: &mut line_suppressions,
                emit_signal: &mut emit_signal,
                root: &ctx.root,
                services: &ctx.services,
                range: ctx.range,
                apply_suppression_comment,
                options: ctx.options,
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

        for suppression in line_suppressions {
            if suppression.did_suppress_signal {
                continue;
            }

            let signal = DiagnosticSignal::new(|| {
                SuppressionDiagnostic::new(
                    category!("suppressions/unused"),
                    suppression.comment_span,
                    "Suppression comment is not being used",
                )
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
    /// Holds the metadata for all the rules statically known to the analyzer
    metadata: &'analyzer MetadataRegistry,
    /// Executor for the query matches emitted by the visitors
    query_matcher: &'phase mut Matcher,
    /// Queue for pending analyzer signals
    signal_queue: BinaryHeap<SignalEntry<'phase, L>>,
    /// Language-specific suppression comment parsing function
    parse_suppression_comment: SuppressionParser<Diag>,
    /// Language-specific suppression comment emitter
    apply_suppression_comment: SuppressionCommentEmitter<L>,
    /// Line index at the current position of the traversal
    line_index: &'phase mut usize,
    /// Track active suppression comments per-line, ordered by line index
    line_suppressions: &'phase mut Vec<LineSuppression>,
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
}

/// Single entry for a suppression comment in the `line_suppressions` buffer
#[derive(Debug)]
struct LineSuppression {
    /// Line index this comment is suppressing lint rules for
    line_index: usize,
    /// Range of source text covered by the suppression comment
    comment_span: TextRange,
    /// Range of source text this comment is suppressing lint rules for
    text_range: TextRange,
    /// Set to true if this comment has set the `suppress_all` flag to true
    /// (must be restored to false on expiration)
    suppress_all: bool,
    /// List of all the rules this comment has started suppressing (must be
    /// removed from the suppressed set on expiration)
    suppressed_rules: Vec<RuleFilter<'static>>,
    /// Set to `true` when a signal matching this suppression was emitted and
    /// suppressed
    did_suppress_signal: bool,
}

impl<'a, 'phase, L, Matcher, Break, Diag> PhaseRunner<'a, 'phase, L, Matcher, Break, Diag>
where
    L: Language,
    Matcher: QueryMatcher<L>,
    Diag: Diagnostic + Clone + Send + Sync + 'static,
{
    /// Runs phase 0 over nodes and tokens to process line breaks and
    /// suppression comments
    fn run_first_phase(mut self) -> ControlFlow<Break> {
        trace!("Running first analyzer phase");
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
            trace!("Event {:?}", &node_event);

            // If this is a node event pass it to the visitors for this phase
            for visitor in self.visitors.iter_mut() {
                let ctx = VisitorContext {
                    phase: self.phase,
                    root: self.root,
                    services: self.services,
                    range: self.range,
                    query_matcher: self.query_matcher,
                    signal_queue: &mut self.signal_queue,
                    apply_suppression_comment: self.apply_suppression_comment,
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
                    apply_suppression_comment: self.apply_suppression_comment,
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
    /// whose position is less then the end of the token within the file
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

            // Search for an active suppression comment covering the range of
            // this signal: first try to load the last line suppression and see
            // if it matches the current line index, otherwise perform a binary
            // search over all the previously seen suppressions to find one
            // with a matching range
            let suppression = self.line_suppressions.last_mut().filter(|suppression| {
                suppression.line_index == *self.line_index
                    && suppression.text_range.start() <= start
            });

            let suppression = match suppression {
                Some(suppression) => Some(suppression),
                None => {
                    let index = self.line_suppressions.binary_search_by(|suppression| {
                        if suppression.text_range.end() < entry.text_range.start() {
                            Ordering::Less
                        } else if entry.text_range.end() < suppression.text_range.start() {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    });

                    index.ok().map(|index| &mut self.line_suppressions[index])
                }
            };

            let suppression = suppression.filter(|suppression| {
                if suppression.suppress_all {
                    return true;
                }

                suppression
                    .suppressed_rules
                    .iter()
                    .any(|filter| *filter == entry.rule)
            });

            // If the signal is being suppressed mark the line suppression as
            // hit, otherwise emit the signal
            if let Some(suppression) = suppression {
                suppression.did_suppress_signal = true;
            } else if range_match(self.range, entry.text_range) {
                (self.emit_signal)(&*entry.signal)?;
            }

            // SAFETY: This removes `query` from the queue, it is known to
            // exist since the `while let Some` block was entered
            self.signal_queue.pop().unwrap();
        }

        ControlFlow::Continue(())
    }

    /// Parse the text content of a comment trivia piece for suppression
    /// comments, and create line suppression entries accordingly
    fn handle_comment(
        &mut self,
        token: &SyntaxToken<L>,
        is_leading: bool,
        index: usize,
        text: &str,
        range: TextRange,
    ) -> ControlFlow<Break> {
        let mut suppress_all = false;
        let mut suppressions = Vec::new();
        let mut has_legacy = false;

        for result in (self.parse_suppression_comment)(text) {
            let kind = match result {
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

            if matches!(kind, SuppressionKind::Deprecated) {
                let signal = DiagnosticSignal::new(move || {
                    SuppressionDiagnostic::new(
                        category!("suppressions/deprecatedSuppressionComment"),
                        range,
                        "// rome-ignore is deprecated, use // biome-ignore instead",
                    )
                    .with_tags(DiagnosticTags::DEPRECATED_CODE)
                    .with_severity(Severity::Information)
                })
                .with_action(move || create_suppression_comment_action(token));

                (self.emit_signal)(&signal)?;
            }

            let rule = match kind {
                SuppressionKind::Everything => None,
                SuppressionKind::Rule(rule) => Some(rule),
                SuppressionKind::MaybeLegacy(rule) => Some(rule),
                SuppressionKind::Deprecated => None,
            };

            if let Some(rule) = rule {
                let group_rule = rule.find('/').map(|index| {
                    let (start, end) = rule.split_at(index);
                    (start, &end[1..])
                });

                let key = match group_rule {
                    None => self.metadata.find_group(rule).map(RuleFilter::from),
                    Some((group, rule)) => {
                        self.metadata.find_rule(group, rule).map(RuleFilter::from)
                    }
                };

                if let Some(key) = key {
                    suppressions.push(key);
                    has_legacy |= matches!(kind, SuppressionKind::MaybeLegacy(_));
                } else if range_match(self.range, range) {
                    // Emit a warning for the unknown rule
                    let signal = DiagnosticSignal::new(move || match group_rule {
                        Some((group, rule)) => SuppressionDiagnostic::new(
                            category!("suppressions/unknownRule"),
                            range,
                            format_args!("Unknown lint rule {group}/{rule} in suppression comment"),
                        ),

                        None => SuppressionDiagnostic::new(
                            category!("suppressions/unknownGroup"),
                            range,
                            format_args!("Unknown lint rule group {rule} in suppression comment"),
                        ),
                    });

                    (self.emit_signal)(&signal)?;
                }
            } else {
                suppressions.clear();
                suppress_all = true;
                // If this if a "suppress all lints" comment, no need to
                // parse anything else
                break;
            }
        }

        // Emit a warning for legacy suppression syntax
        if has_legacy && range_match(self.range, range) {
            let signal = DiagnosticSignal::new(move || {
                SuppressionDiagnostic::new(
                    category!("suppressions/deprecatedSuppressionComment"),
                    range,
                    "Suppression is using a deprecated syntax",
                )
                .with_tags(DiagnosticTags::DEPRECATED_CODE)
            });

            let signal = signal
                .with_action(|| update_suppression(self.root, token, is_leading, index, text));

            (self.emit_signal)(&signal)?;
        }

        if !suppress_all && suppressions.is_empty() {
            return ControlFlow::Continue(());
        }

        // Suppression comments apply to the next line
        let line_index = *self.line_index + 1;

        // If the last suppression was on the same or previous line, extend its
        // range and set of supressed rules with the content for the new suppression
        if let Some(last_suppression) = self.line_suppressions.last_mut() {
            if last_suppression.line_index == line_index
                || last_suppression.line_index + 1 == line_index
            {
                last_suppression.line_index = line_index;
                last_suppression.text_range = last_suppression.text_range.cover(range);
                last_suppression.suppress_all |= suppress_all;
                if !last_suppression.suppress_all {
                    last_suppression.suppressed_rules.extend(suppressions);
                } else {
                    last_suppression.suppressed_rules.clear();
                }
                return ControlFlow::Continue(());
            }
        }

        let entry = LineSuppression {
            line_index,
            comment_span: range,
            text_range: range,
            suppress_all,
            suppressed_rules: suppressions,
            did_suppress_signal: false,
        };

        self.line_suppressions.push(entry);

        ControlFlow::Continue(())
    }

    /// Check a piece of source text (token or trivia) for line breaks and
    /// increment the line index accordingly, extending the range of the
    /// current suppression as required
    fn bump_line_index(&mut self, text: &str, range: TextRange) {
        let mut did_match = false;
        for (index, _) in text.match_indices('\n') {
            if let Some(last_suppression) = self.line_suppressions.last_mut() {
                if last_suppression.line_index == *self.line_index {
                    let index = TextSize::try_from(index).expect(
                        "integer overflow while converting a suppression line to `TextSize`",
                    );
                    let range = TextRange::at(range.start(), index);
                    last_suppression.text_range = last_suppression.text_range.cover(range);
                    did_match = true;
                }
            }

            *self.line_index += 1;
        }

        if !did_match {
            if let Some(last_suppression) = self.line_suppressions.last_mut() {
                if last_suppression.line_index == *self.line_index {
                    last_suppression.text_range = last_suppression.text_range.cover(range);
                }
            }
        }
    }
}

fn create_suppression_comment_action<L: Language>(
    token: &SyntaxToken<L>,
) -> Option<AnalyzerAction<L>> {
    let first_node = token.parent()?;
    let mut new_leading_trivia = vec![];
    let mut token_text = String::new();
    let mut new_trailing_trivia = vec![];
    let mut mutation = BatchMutation::new(first_node);

    for piece in token.leading_trivia().pieces() {
        if !piece.is_comments() {
            new_leading_trivia.push(TriviaPiece::new(piece.kind(), piece.text_len()));
            token_text.push_str(piece.text());
        }

        if piece.text().contains("rome-ignore") {
            let new_text = piece.text().replace("rome-ignore", "biome-ignore");
            new_leading_trivia.push(TriviaPiece::new(piece.kind(), new_text.text_len()));
            token_text.push_str(&new_text);
        }
    }

    token_text.push_str(token.text_trimmed());

    for piece in token.trailing_trivia().pieces() {
        new_trailing_trivia.push(TriviaPiece::new(piece.kind(), piece.text_len()));
        token_text.push_str(piece.text());
    }

    let new_token = SyntaxToken::new_detached(
        token.kind(),
        &token_text,
        new_leading_trivia,
        new_trailing_trivia,
    );

    mutation.replace_token_discard_trivia(token.clone(), new_token);
    Some(AnalyzerAction {
        mutation,
        applicability: Applicability::MaybeIncorrect,
        category: ActionCategory::QuickFix,
        message: markup! {
            "Use // biome-ignore instead"
        }
        .to_owned(),
        rule_name: None,
    })
}

fn range_match(filter: Option<TextRange>, range: TextRange) -> bool {
    filter.map_or(true, |filter| filter.intersect(range).is_some())
}

/// Signature for a suppression comment parser function
///
/// This function receives the text content of a comment and returns a list of
/// lint suppressions as an optional lint rule (if the lint rule is `None` the
/// comment is interpreted as suppressing all lints)
///
/// # Examples
///
/// - `// rome-ignore format` -> `vec![]`
/// - `// rome-ignore lint` -> `vec![Everything]`
/// - `// rome-ignore lint/style/useWhile` -> `vec![Rule("style/useWhile")]`
/// - `// rome-ignore lint/style/useWhile lint/nursery/noUnreachable` -> `vec![Rule("style/useWhile"), Rule("nursery/noUnreachable")]`
/// - `// rome-ignore lint(style/useWhile)` -> `vec![MaybeLegacy("style/useWhile")]`
/// - `// rome-ignore lint(style/useWhile) lint(nursery/noUnreachable)` -> `vec![MaybeLegacy("style/useWhile"), MaybeLegacy("nursery/noUnreachable")]`
type SuppressionParser<D> = fn(&str) -> Vec<Result<SuppressionKind, D>>;

/// This enum is used to categorize what is disabled by a suppression comment and with what syntax
pub enum SuppressionKind<'a> {
    /// A suppression disabling all lints eg. `// rome-ignore lint`
    Everything,
    /// A suppression disabling a specific rule eg. `// rome-ignore lint/style/useWhile`
    Rule(&'a str),
    /// A suppression using the legacy syntax to disable a specific rule eg. `// rome-ignore lint(style/useWhile)`
    MaybeLegacy(&'a str),
    /// `rome-ignore` is legacy
    Deprecated,
}

fn update_suppression<L: Language>(
    root: &L::Root,
    token: &SyntaxToken<L>,
    is_leading: bool,
    index: usize,
    text: &str,
) -> Option<AnalyzerAction<L>> {
    let old_token = token.clone();
    let new_token = token.clone().detach();

    let old_trivia = if is_leading {
        old_token.leading_trivia()
    } else {
        old_token.trailing_trivia()
    };

    let old_trivia: Vec<_> = old_trivia.pieces().collect();

    let mut text = text.to_string();

    while let Some(range_start) = text.find("lint(") {
        let range_end = range_start + text[range_start..].find(')')?;
        text.replace_range(range_end..range_end + 1, "");
        text.replace_range(range_start + 4..range_start + 5, "/");
    }

    let new_trivia = old_trivia.iter().enumerate().map(|(piece_index, piece)| {
        if piece_index == index {
            (piece.kind(), text.as_str())
        } else {
            (piece.kind(), piece.text())
        }
    });

    let new_token = if is_leading {
        new_token.with_leading_trivia(new_trivia)
    } else {
        new_token.with_trailing_trivia(new_trivia)
    };

    let mut mutation = BatchMutation::new(root.syntax().clone());
    mutation.replace_token_discard_trivia(old_token, new_token);

    Some(AnalyzerAction {
        rule_name: None,
        category: ActionCategory::QuickFix,
        applicability: Applicability::Always,
        message: markup! {
            "Rewrite suppression to use the newer syntax"
        }
        .to_owned(),
        mutation,
    })
}

/// Payload received by the function responsible to mark a suppression comment
pub struct SuppressionCommentEmitterPayload<'a, L: Language> {
    /// The possible offset found in the [TextRange] of the emitted diagnostic
    pub token_offset: TokenAtOffset<SyntaxToken<L>>,
    /// A [BatchMutation] where the consumer can apply the suppression comment
    pub mutation: &'a mut BatchMutation<L>,
    /// A string equals to "rome-ignore: lint(<RULE_GROUP>/<RULE_NAME>)"
    pub suppression_text: &'a str,
    /// The original range of the diagnostic where the rule was triggered
    pub diagnostic_text_range: &'a TextRange,
}

/// Convenient type that to mark a function that is responsible to create a mutation to add a suppression comment.
type SuppressionCommentEmitter<L> = fn(SuppressionCommentEmitterPayload<L>);

type SignalHandler<'a, L, Break> = &'a mut dyn FnMut(&dyn AnalyzerSignal<L>) -> ControlFlow<Break>;

/// Allow filtering a single rule or group of rules by their names
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum RuleFilter<'a> {
    Group(&'a str),
    Rule(&'a str, &'a str),
}

impl RuleFilter<'_> {
    /// Return `true` if the group `G` matches this filter
    fn match_group<G: RuleGroup>(self) -> bool {
        match self {
            RuleFilter::Group(group) => group == G::NAME,
            RuleFilter::Rule(group, _) => group == G::NAME,
        }
    }

    /// Return `true` if the rule `R` matches this filter
    fn match_rule<R>(self) -> bool
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

impl<'a> Debug for RuleFilter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<'a> Display for RuleFilter<'a> {
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

/// Allows filtering the list of rules that will be executed in a run of the analyzer,
/// and at what source code range signals (diagnostics or actions) may be raised
#[derive(Debug, Default, Clone, Copy)]
pub struct AnalysisFilter<'a> {
    /// Only allow rules with these categories to emit signals
    pub categories: RuleCategories,
    /// Only allow rules matching these names to emit signals
    pub enabled_rules: Option<&'a [RuleFilter<'a>]>,
    /// Do not allow rules matching these names to emit signals
    pub disabled_rules: Option<&'a [RuleFilter<'a>]>,
    /// Only emit signals matching this text range
    pub range: Option<TextRange>,
}

impl<'analysis> AnalysisFilter<'analysis> {
    /// Return `true` if the category `C` matches this filter
    pub fn match_category<C: GroupCategory>(&self) -> bool {
        self.categories.contains(C::CATEGORY.into())
    }

    /// Return `true` if the group `G` matches this filter
    pub fn match_group<G: RuleGroup>(&self) -> bool {
        self.match_category::<G::Category>()
            && self.enabled_rules.map_or(true, |enabled_rules| {
                enabled_rules.iter().any(|filter| filter.match_group::<G>())
            })
            && self.disabled_rules.map_or(true, |disabled_rules| {
                !disabled_rules
                    .iter()
                    .any(|filter| filter.match_group::<G>())
            })
    }

    /// Return `true` if the rule `R` matches this filter
    pub fn match_rule<R>(&self) -> bool
    where
        R: Rule,
    {
        self.match_group::<R::Group>()
            && self.enabled_rules.map_or(true, |enabled_rules| {
                enabled_rules.iter().any(|filter| filter.match_rule::<R>())
            })
            && self.disabled_rules.map_or(true, |disabled_rules| {
                !disabled_rules.iter().any(|filter| filter.match_rule::<R>())
            })
    }

    /// It creates a new filter with the set of [enabled rules](RuleFilter) passed as argument
    pub fn from_enabled_rules(enabled_rules: Option<&'analysis [RuleFilter<'analysis>]>) -> Self {
        Self {
            enabled_rules,
            ..AnalysisFilter::default()
        }
    }
}

/// Utility type to be used as a default value for the `B` generic type on
/// `analyze` when the provided callback never breaks
///
/// This should eventually get replaced with the `!` type when it gets stabilized
pub enum Never {}

/// Type alias of [ops::ControlFlow] with the `B` generic type defaulting to [Never]
///
/// By default the analysis loop never breaks, so it behaves mostly like
/// `let b = loop {};` and has a "break type" of `!` (the `!` type isn't stable
/// yet so I'm using an empty enum instead but they're identical for this
/// purpose)
///
/// In practice it's not really a `loop` but a `for` because it's iterating on
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
