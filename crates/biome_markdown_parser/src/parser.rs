use biome_markdown_syntax::MarkdownSyntaxKind;
use biome_parser::ParserContext;
use biome_parser::event::Event;
use biome_parser::prelude::*;
use biome_parser::token_source::{BumpWithContext, Trivia};
use biome_parser::{ParserContextCheckpoint, diagnostic::merge_diagnostics};
use biome_rowan::{TextRange, TextSize, TriviaPieceKind};
use std::cell::Cell;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Range;
use std::rc::Rc;

use crate::lexer::{MarkdownLexContext, MarkdownReLexContext};
use crate::syntax::TAB_STOP_SPACES;
use crate::syntax::inline::EmphasisContext;
use crate::syntax::parse_error::DEFAULT_MAX_NESTING_DEPTH;
use crate::syntax::reference::normalize_reference_label;
use crate::token_source::{MarkdownTokenSource, MarkdownTokenSourceCheckpoint};

/// Options for configuring the markdown parser.
#[derive(Debug, Clone)]
pub struct MarkdownParserOptions {
    /// Maximum nesting depth for block quotes and lists.
    ///
    /// This limits recursion on pathological input to avoid stack overflow.
    pub max_nesting_depth: usize,
    pub(crate) frontmatter: bool,
}

impl MarkdownParserOptions {
    /// Controls whether a complete `---` pair at the start of the document is parsed as frontmatter.
    pub fn with_frontmatter(mut self, frontmatter: bool) -> Self {
        self.frontmatter = frontmatter;
        self
    }
}

impl Default for MarkdownParserOptions {
    fn default() -> Self {
        Self {
            max_nesting_depth: DEFAULT_MAX_NESTING_DEPTH,
            frontmatter: false,
        }
    }
}

/// Internal parser state for tracking nesting and context.
///
/// # Depth Tracking
///
/// These fields track nesting depth to prevent stack overflow from pathological
/// input (e.g., `>>>>...` with hundreds of levels). CommonMark doesn't specify
/// limits, but practical implementations need them.
///
/// # Future Use
///
/// - **Lazy continuation**: CommonMark §5.1 allows block quote content to continue
///   without `>` prefix on subsequent lines. Proper implementation requires tracking
///   the current quote depth to know when lazy continuation applies.
///
/// - **List tight/loose determination**: CommonMark §5.3 distinguishes tight lists
///   (no blank lines between items) from loose lists. This affects HTML output and
///   requires tracking list context during parsing.
#[derive(Default, Debug)]
pub(crate) struct MarkdownParserState {
    /// Block quote nesting depth for lazy continuation and depth limits.
    /// See CommonMark §5.1 for block quote continuation rules.
    pub(crate) block_quote_depth: usize,
    /// List nesting depth for tight/loose determination and depth limits.
    /// See CommonMark §5.3 for list tightness rules.
    pub(crate) list_nesting_depth: usize,
    /// Required indentation for list item content continuation.
    /// Per CommonMark §5.2, continuation lines must be indented to at least
    /// this column (marker width + space width). Zero means no indent required.
    pub(crate) list_item_required_indent: usize,
    /// Indentation column where the current list marker starts.
    /// Used to detect sibling list items after blank lines.
    pub(crate) list_item_marker_indent: usize,
    /// The bullet marker kind of the parent list (e.g. `-`, `*`, `+`).
    /// Used to detect marker changes at blank-line boundaries.
    pub(crate) list_item_marker_kind: Option<MarkdownSyntaxKind>,
    /// The ordered list delimiter of the parent list (`.` or `)`).
    /// Used to detect delimiter changes at blank-line boundaries.
    pub(crate) list_item_ordered_delim: Option<char>,
    /// Emphasis parsing context for the current inline item list.
    pub(crate) emphasis_context: Option<Rc<EmphasisContext>>,
    /// Link reference definitions accepted while parsing block structure.
    pub(crate) link_reference_definitions: LinkReferenceDefinitions,
    /// Inline event subtrees reparsed after all definitions are known.
    pub(crate) deferred_inlines: Vec<DeferredInline>,
    /// Whether a following non-blank line should be treated as paragraph
    /// continuation after a link reference definition.
    pub(crate) link_reference_definition_continuation: bool,
    /// Recorded tight/loose list results keyed by list node range.
    pub(crate) list_tightness: Vec<ListTightness>,
    /// Recorded list item indents keyed by bullet node range.
    pub(crate) list_item_indents: Vec<ListItemIndent>,
    /// Recorded quote marker indents keyed by quote node range.
    pub(crate) quote_indents: Vec<QuoteIndent>,
    /// Whether the most recently parsed list ended with a blank line.
    pub(crate) last_list_ends_with_blank: bool,
    /// Virtual line start override for container prefixes (e.g., block quotes).
    pub(crate) virtual_line_start: Option<TextSize>,
    /// Flag to unwind quote parsing when nesting exceeds the maximum depth.
    pub(crate) quote_depth_exceeded: bool,
}

struct MarkdownParserStateCheckpoint {
    block_quote_depth: usize,
    list_nesting_depth: usize,
    list_item_required_indent: usize,
    list_item_marker_indent: usize,
    list_item_marker_kind: Option<MarkdownSyntaxKind>,
    list_item_ordered_delim: Option<char>,
    emphasis_context: Option<Rc<EmphasisContext>>,
    link_reference_definitions_len: usize,
    deferred_inlines_len: usize,
    link_reference_definition_continuation: bool,
    list_tightness_len: usize,
    list_item_indents_len: usize,
    quote_indents_len: usize,
    last_list_ends_with_blank: bool,
    virtual_line_start: Option<TextSize>,
    quote_depth_exceeded: bool,
}

impl MarkdownParserState {
    fn checkpoint(&self) -> MarkdownParserStateCheckpoint {
        MarkdownParserStateCheckpoint {
            block_quote_depth: self.block_quote_depth,
            list_nesting_depth: self.list_nesting_depth,
            list_item_required_indent: self.list_item_required_indent,
            list_item_marker_indent: self.list_item_marker_indent,
            list_item_marker_kind: self.list_item_marker_kind,
            list_item_ordered_delim: self.list_item_ordered_delim,
            emphasis_context: self.emphasis_context.clone(),
            link_reference_definitions_len: self.link_reference_definitions.len(),
            deferred_inlines_len: self.deferred_inlines.len(),
            link_reference_definition_continuation: self.link_reference_definition_continuation,
            list_tightness_len: self.list_tightness.len(),
            list_item_indents_len: self.list_item_indents.len(),
            quote_indents_len: self.quote_indents.len(),
            last_list_ends_with_blank: self.last_list_ends_with_blank,
            virtual_line_start: self.virtual_line_start,
            quote_depth_exceeded: self.quote_depth_exceeded,
        }
    }

    fn rewind(&mut self, checkpoint: MarkdownParserStateCheckpoint) {
        self.block_quote_depth = checkpoint.block_quote_depth;
        self.list_nesting_depth = checkpoint.list_nesting_depth;
        self.list_item_required_indent = checkpoint.list_item_required_indent;
        self.list_item_marker_indent = checkpoint.list_item_marker_indent;
        self.list_item_marker_kind = checkpoint.list_item_marker_kind;
        self.list_item_ordered_delim = checkpoint.list_item_ordered_delim;
        self.emphasis_context = checkpoint.emphasis_context;
        self.link_reference_definitions
            .truncate(checkpoint.link_reference_definitions_len);
        self.deferred_inlines
            .truncate(checkpoint.deferred_inlines_len);
        self.link_reference_definition_continuation =
            checkpoint.link_reference_definition_continuation;
        self.list_tightness.truncate(checkpoint.list_tightness_len);
        self.list_item_indents
            .truncate(checkpoint.list_item_indents_len);
        self.quote_indents.truncate(checkpoint.quote_indents_len);
        self.last_list_ends_with_blank = checkpoint.last_list_ends_with_blank;
        self.virtual_line_start = checkpoint.virtual_line_start;
        self.quote_depth_exceeded = checkpoint.quote_depth_exceeded;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListTightness {
    pub range: TextRange,
    pub is_tight: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItemIndent {
    pub range: TextRange,
    pub indent: usize,
    pub marker_indent: usize,
    pub marker_width: usize,
    pub spaces_after_marker: usize,
}

/// Products of a Markdown parse before construction of the green tree.
///
/// `deferred_inlines` identifies provisional inline event subtrees that may
/// depend on definitions discovered later in the document. The inline phase
/// resolves those records before `events`, `trivia`, and `diagnostics` are sent
/// to the lossless tree sink.
pub(crate) struct MarkdownParserOutput {
    /// Syntax events in source order.
    pub(crate) events: Vec<Event<MarkdownSyntaxKind>>,
    /// Lexer and parser diagnostics in source order.
    pub(crate) diagnostics: Vec<ParseDiagnostic>,
    /// Trivia associated with the event stream.
    pub(crate) trivia: Vec<Trivia>,
    /// Provisional inline event subtrees awaiting document-global resolution.
    pub(crate) deferred_inlines: Vec<DeferredInline>,
    /// Link reference definitions accepted while parsing block structure.
    pub(crate) link_reference_definitions: LinkReferenceDefinitions,
    /// Tightness metadata keyed by final list ranges.
    pub(crate) list_tightness: Vec<ListTightness>,
    /// Indentation metadata keyed by final list-item ranges.
    pub(crate) list_item_indents: Vec<ListItemIndent>,
    /// Indentation metadata keyed by final quote ranges.
    pub(crate) quote_indents: Vec<QuoteIndent>,
}

/// CST wrapper required when rebuilding a deferred inline event subtree.
#[derive(Debug, Clone, Copy)]
pub(crate) enum DeferredInlineFlavor {
    /// An `MdInlineItemList` that is already enclosed by its block node.
    Paragraph,
    /// An ATX heading's `MdParagraph`, including its inline item list.
    AtxParagraph,
}

/// Identifies a provisional inline subtree and the parser state needed to
/// rebuild it after all link reference definitions are known.
///
/// `event_range` and `source_range` describe the same inline subtree in the
/// parser event stream and original source respectively. Event ranges are
/// ordered and non-overlapping.
#[derive(Debug, Clone)]
pub(crate) struct DeferredInline {
    /// Half-open range of provisional events replaced by the inline phase.
    event_range: Range<usize>,
    /// Original source covered by the provisional inline subtree.
    source_range: TextRange,
    /// Wrapper shape expected at the replacement site.
    flavor: DeferredInlineFlavor,
    /// Block-container state at the start of the inline subtree.
    context: InlineContainerContext,
    /// Number of definitions known when the provisional subtree was parsed.
    ///
    /// A smaller value than the final definition count means later definitions
    /// may change reference and emphasis parsing in this subtree.
    definitions_len: usize,
}

pub(crate) struct DeferredInlineStart {
    event_start: usize,
    source_start: TextSize,
    flavor: DeferredInlineFlavor,
    context: InlineContainerContext,
    definitions_len: usize,
}

impl DeferredInline {
    pub(crate) fn event_range(&self) -> &Range<usize> {
        &self.event_range
    }

    pub(crate) fn source_range(&self) -> TextRange {
        self.source_range
    }

    pub(crate) fn flavor(&self) -> DeferredInlineFlavor {
        self.flavor
    }

    pub(crate) fn context(&self) -> InlineContainerContext {
        self.context
    }

    pub(crate) fn definitions_len(&self) -> usize {
        self.definitions_len
    }

    #[cfg(test)]
    pub(crate) fn for_test(
        event_range: Range<usize>,
        source_range: TextRange,
        flavor: DeferredInlineFlavor,
        context: InlineContainerContext,
        definitions_len: usize,
    ) -> Self {
        Self {
            event_range,
            source_range,
            flavor,
            context,
            definitions_len,
        }
    }
}

/// Block-container state that affects parsing of a detached inline source range.
///
/// Inline reparsing starts at the original absolute source offset but outside
/// the recursive block parser. This snapshot restores the quote, list, and
/// virtual-line context needed to classify continuation prefixes and indentation
/// exactly as they appear in the final CST.
#[derive(Debug, Clone, Copy)]
pub(crate) struct InlineContainerContext {
    /// Active block quote nesting at the start of the inline range.
    block_quote_depth: usize,
    /// Active list nesting at the start of the inline range.
    list_nesting_depth: usize,
    /// Columns required for a line to continue the current list item.
    list_item_required_indent: usize,
    /// Column where the current list marker starts.
    list_item_marker_indent: usize,
    /// Bullet marker of the containing unordered list, when present.
    list_item_marker_kind: Option<MarkdownSyntaxKind>,
    /// Delimiter of the containing ordered list, when present.
    list_item_ordered_delim: Option<char>,
    /// Logical line start after a container prefix has been consumed.
    virtual_line_start: Option<TextSize>,
}

/// Index of link reference definitions accepted by the block parser.
///
/// Definition labels remain in the source and are represented by ranges. The
/// hash index uses CommonMark's normalized-label equivalence only to narrow
/// lookup candidates; it never replaces the original label text.
#[derive(Debug, Default)]
pub(crate) struct LinkReferenceDefinitions {
    /// Source ranges of definition labels in document order.
    ranges: Vec<TextRange>,
    /// Normalized-label hashes corresponding to `ranges`.
    hashes: Vec<u64>,
    /// Normalized-label hash to indices in `ranges`.
    ///
    /// Each hash can identify multiple ranges because duplicate definitions and
    /// hash collisions are both possible. Lookup compares normalized source text
    /// from every candidate range before reporting a match.
    ranges_by_hash: HashMap<u64, Vec<usize>>,
}

impl LinkReferenceDefinitions {
    fn insert(&mut self, range: TextRange, hash: u64) {
        let index = self.ranges.len();
        self.ranges.push(range);
        self.hashes.push(hash);
        self.ranges_by_hash.entry(hash).or_default().push(index);
    }

    pub(crate) fn len(&self) -> usize {
        self.ranges.len()
    }

    fn contains(&self, source: &str, normalized_label: &str) -> bool {
        let hash = hash_normalized_label(normalized_label);
        self.ranges_by_hash
            .get(&hash)
            .into_iter()
            .flatten()
            .filter_map(|index| self.ranges.get(*index))
            .filter_map(|range| source.get(usize::from(range.start())..usize::from(range.end())))
            .any(|label| {
                crate::syntax::reference::normalize_reference_label(label) == normalized_label
            })
    }

    fn truncate(&mut self, len: usize) {
        while self.ranges.len() > len {
            let index = self.ranges.len() - 1;
            self.ranges.pop();
            let Some(hash) = self.hashes.pop() else {
                self.ranges.truncate(len);
                self.ranges_by_hash.clear();
                return;
            };
            let Some(indices) = self.ranges_by_hash.get_mut(&hash) else {
                continue;
            };
            if indices.last() == Some(&index) {
                indices.pop();
            } else {
                indices.retain(|candidate| *candidate != index);
            }
            let remove_bucket = indices.is_empty();
            if remove_bucket {
                self.ranges_by_hash.remove(&hash);
            }
        }
    }
}

fn normalized_label_hash(label: &str) -> u64 {
    hash_normalized_label(&normalize_reference_label(label))
}

fn hash_normalized_label(label: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    label.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuoteIndent {
    pub range: TextRange,
    pub indent: usize,
}

pub(crate) struct MarkdownParser<'source> {
    context: ParserContext<MarkdownSyntaxKind>,
    source: MarkdownTokenSource<'source>,
    options: MarkdownParserOptions,
    known_link_reference_definitions: Option<&'source LinkReferenceDefinitions>,
    state: MarkdownParserState,
    /// Single-entry memo for `absolute_column_at` queries. Most callers ask
    /// for the column of the current token offset many times in a row while
    /// the parser does not advance, so caching the last computed pair avoids
    /// repeated O(line-length) scans for the preceding newline.
    abs_col_cache: Cell<Option<(u32, usize)>>,
}

impl<'source> MarkdownParser<'source> {
    pub fn new(source: &'source str, options: MarkdownParserOptions) -> Self {
        Self {
            context: ParserContext::default(),
            source: MarkdownTokenSource::from_str(source),
            options,
            known_link_reference_definitions: None,
            state: MarkdownParserState::default(),
            abs_col_cache: Cell::new(None),
        }
    }

    pub(crate) fn new_range(
        source: &'source str,
        range: TextRange,
        options: MarkdownParserOptions,
        context: InlineContainerContext,
        definitions: &'source LinkReferenceDefinitions,
    ) -> Option<Self> {
        let state = MarkdownParserState {
            block_quote_depth: context.block_quote_depth,
            list_nesting_depth: context.list_nesting_depth,
            list_item_required_indent: context.list_item_required_indent,
            list_item_marker_indent: context.list_item_marker_indent,
            list_item_marker_kind: context.list_item_marker_kind,
            list_item_ordered_delim: context.list_item_ordered_delim,
            virtual_line_start: context.virtual_line_start,
            ..MarkdownParserState::default()
        };

        Some(Self {
            context: ParserContext::default(),
            source: MarkdownTokenSource::from_range(source, range)?,
            options,
            known_link_reference_definitions: Some(definitions),
            state,
            abs_col_cache: Cell::new(None),
        })
    }

    /// Cached wrapper around [`absolute_column_at`]. Returns the column of
    /// `offset` in the parser source, reusing the previous result when the
    /// offset matches. Callers like `line_start_leading_indent` and
    /// `line_indent_from_current` hit the same offset repeatedly while no
    /// tokens are consumed.
    pub(crate) fn cached_absolute_column_at(&self, offset: usize) -> usize {
        let key = offset as u32;
        if let Some((cached_key, cached_col)) = self.abs_col_cache.get()
            && cached_key == key
        {
            return cached_col;
        }
        let col = absolute_column_at(self.source.source_text(), offset);
        self.abs_col_cache.set(Some((key, col)));
        col
    }

    /// Returns parser options. Reserved for GFM extensions.
    pub(crate) fn options(&self) -> &MarkdownParserOptions {
        &self.options
    }

    /// Returns immutable state reference for nesting depth checks.
    pub(crate) fn state(&self) -> &MarkdownParserState {
        &self.state
    }

    /// Returns mutable state reference for nesting depth updates.
    pub(crate) fn state_mut(&mut self) -> &mut MarkdownParserState {
        &mut self.state
    }

    /// Returns the emphasis context for the current inline list, if any.
    pub(crate) fn emphasis_context(&self) -> Option<&EmphasisContext> {
        self.state.emphasis_context.as_deref()
    }

    /// Replace the emphasis context, returning the previous value.
    pub(crate) fn set_emphasis_context(
        &mut self,
        context: Option<Rc<EmphasisContext>>,
    ) -> Option<Rc<EmphasisContext>> {
        std::mem::replace(&mut self.state.emphasis_context, context)
    }

    pub(crate) fn set_new_emphasis_context(
        &mut self,
        context: EmphasisContext,
    ) -> Option<Rc<EmphasisContext>> {
        self.set_emphasis_context(Some(Rc::new(context)))
    }

    pub(crate) fn record_link_reference_definition(&mut self, label: TextRange) {
        let Some(label_text) = self
            .source
            .source_text()
            .get(usize::from(label.start())..usize::from(label.end()))
        else {
            return;
        };
        let hash = normalized_label_hash(label_text);
        self.state.link_reference_definitions.insert(label, hash);
    }

    /// Returns true if a normalized label has a link reference definition.
    pub(crate) fn has_link_reference_definition(&self, label: &str) -> bool {
        self.known_link_reference_definitions
            .is_some_and(|definitions| definitions.contains(self.source.source_text(), label))
            || self
                .state
                .link_reference_definitions
                .contains(self.source.source_text(), label)
    }

    pub(crate) fn inline_container_context(&self) -> InlineContainerContext {
        InlineContainerContext {
            block_quote_depth: self.state.block_quote_depth,
            list_nesting_depth: self.state.list_nesting_depth,
            list_item_required_indent: self.state.list_item_required_indent,
            list_item_marker_indent: self.state.list_item_marker_indent,
            list_item_marker_kind: self.state.list_item_marker_kind,
            list_item_ordered_delim: self.state.list_item_ordered_delim,
            virtual_line_start: self.state.virtual_line_start,
        }
    }

    pub(crate) fn start_deferred_inline(
        &self,
        flavor: DeferredInlineFlavor,
    ) -> DeferredInlineStart {
        DeferredInlineStart {
            event_start: self.context.events().len(),
            source_start: self.cur_range().start(),
            flavor,
            context: self.inline_container_context(),
            definitions_len: self.link_reference_definitions_len(),
        }
    }

    pub(crate) fn finish_deferred_inline(&mut self, start: DeferredInlineStart) {
        let event_end = self.context.events().len();
        let Some(events) = self.context.events().get(start.event_start..event_end) else {
            return;
        };
        if !events
            .iter()
            .any(|event| matches!(event, Event::Token { .. }))
        {
            return;
        }
        self.state.deferred_inlines.push(DeferredInline {
            event_range: start.event_start..event_end,
            source_range: TextRange::new(start.source_start, self.cur_range().start()),
            flavor: start.flavor,
            context: start.context,
            definitions_len: start.definitions_len,
        });
    }

    pub(crate) fn link_reference_definitions_len(&self) -> usize {
        self.state.link_reference_definitions.len()
    }

    /// Record tight/loose information for a parsed list node.
    pub(crate) fn record_list_tightness(&mut self, range: TextRange, is_tight: bool) {
        self.state
            .list_tightness
            .push(ListTightness { range, is_tight });
    }

    pub(crate) fn record_list_item_indent(
        &mut self,
        range: TextRange,
        indent: usize,
        marker_indent: usize,
        marker_width: usize,
        spaces_after_marker: usize,
    ) {
        self.state.list_item_indents.push(ListItemIndent {
            range,
            indent,
            marker_indent,
            marker_width,
            spaces_after_marker,
        });
    }

    pub(crate) fn record_quote_indent(&mut self, range: TextRange, indent: usize) {
        self.state.quote_indents.push(QuoteIndent { range, indent });
    }

    pub(crate) fn set_last_list_ends_with_blank(&mut self, value: bool) {
        self.state.last_list_ends_with_blank = value;
    }

    pub(crate) fn take_last_list_ends_with_blank(&mut self) -> bool {
        std::mem::take(&mut self.state.last_list_ends_with_blank)
    }

    /// Re-lex the current token using LinkDefinition context.
    /// This makes whitespace produce separate tokens for destination/title parsing.
    pub(crate) fn re_lex_link_definition(&mut self) {
        self.source.re_lex(MarkdownReLexContext::LinkDefinition);
    }

    /// Re-lex the current token as plain text, merging it with the plain
    /// characters that follow. Used when a construct token (failed emphasis
    /// marker, unmatched bracket, ...) falls back to being ordinary text,
    /// so it becomes one text node instead of a one-character node.
    pub(crate) fn re_lex_textual_fallback(&mut self) {
        self.source.re_lex(MarkdownReLexContext::TextualFallback);
    }

    /// Re-lex from the current token start up to `end` as a single token of
    /// the given kind. Used for regions whose extent only the parser can
    /// measure, like HTML block content (whitespace and newlines included)
    /// or a run of indentation characters. Must NOT be called inside
    /// lookahead.
    pub(crate) fn re_lex_span(&mut self, end: TextSize, kind: MarkdownSyntaxKind) {
        self.source.re_lex_span(end, kind);
    }

    /// Consume the span from the current token start to `end` as one token
    /// of `kind`, wrapped in a single `wrapper` node. Used to fold runs of
    /// per-character tokens (like indentation) into one node.
    pub(crate) fn emit_span_as(
        &mut self,
        end: TextSize,
        kind: MarkdownSyntaxKind,
        wrapper: MarkdownSyntaxKind,
    ) {
        self.re_lex_span(end, kind);
        let m = self.start();
        self.bump(kind);
        m.complete(self, wrapper);
    }

    /// Force re-lex the current token in Regular context.
    ///
    /// Use this when switching from LinkDefinition context back to Regular context,
    /// e.g., when entering title content where whitespace should not split tokens.
    pub(crate) fn force_relex_regular(&mut self) {
        self.source
            .force_relex_in_context(MarkdownLexContext::Regular);
    }

    /// Re-lex the current token in Regular context, treating the position as
    /// a line start. After consuming a blockquote prefix, the lexer's
    /// `after_newline` flag is false, which prevents it from producing
    /// line-start-gated tokens like `MD_THEMATIC_BREAK_LITERAL`. This method
    /// overrides that flag so the lexer behaves as if at line start.
    pub(crate) fn force_relex_at_line_start(&mut self) {
        self.source.force_relex_at_line_start();
    }

    /// Re-lexes the current token in the specified context. Returns the kind
    /// of the re-lexed token (can be the same as before if the context doesn't make a difference for the current token)
    pub fn re_lex(&mut self, context: MarkdownReLexContext) -> MarkdownSyntaxKind {
        self.source_mut().re_lex(context)
    }

    /// Re-lex the current token as single-char emphasis delimiter.
    ///
    /// Use this when the emphasis matching algorithm needs to partially consume
    /// a DOUBLE_STAR or DOUBLE_UNDERSCORE token. After re-lexing, the token will
    /// be either STAR or UNDERSCORE (single char).
    ///
    /// Note
    /// Only call on the current token, NOT inside lookahead closures.
    /// This invalidates any buffered lookahead, so ensure no lookahead is active.
    pub(crate) fn force_relex_emphasis_inline(&mut self) -> MarkdownSyntaxKind {
        self.source.re_lex(MarkdownReLexContext::EmphasisInline)
    }

    /// Re-lex the current token in HeadingContent context.
    ///
    /// Splits MD_HARD_LINE_LITERAL (trailing spaces + newline) into
    /// MD_TEXTUAL_LITERAL (spaces only) + separate NEWLINE. Use this in
    /// heading parsing where trailing spaces are not hard breaks (§4.2).
    pub(crate) fn force_relex_heading_content(&mut self) -> MarkdownSyntaxKind {
        self.source.force_relex_heading_content()
    }

    pub(crate) fn set_force_ordered_list_marker(&mut self, value: bool) {
        self.source.set_force_ordered_list_marker(value);
    }

    /// Bump the current token using LinkDefinition context.
    /// The next token will be lexed with whitespace as separate tokens.
    pub(crate) fn bump_link_definition(&mut self) {
        self.source.bump_link_definition();
    }

    /// Force re-lex the current token in ThematicBreakParts context.
    /// Decomposes MD_THEMATIC_BREAK_LITERAL into individual marker/space tokens.
    /// Must NOT be called inside lookahead.
    pub(crate) fn force_relex_thematic_break_parts(&mut self) {
        self.source
            .force_relex_in_context(MarkdownLexContext::ThematicBreakParts);
    }

    /// Bump the current token and lex the next in ThematicBreakParts context,
    /// ensuring sustained parts-mode tokenization across the loop.
    ///
    /// Unlike `source.bump_thematic_break_parts()` (which only advances the lexer),
    /// this method also registers the token with the tree builder via `push_token`,
    /// so the token appears in the CST.
    pub(crate) fn bump_thematic_break_parts(&mut self) {
        let kind = self.cur();
        let end = self.cur_range().end();
        self.context_mut().push_token(kind, end);
        self.source.bump_thematic_break_parts();
    }

    pub fn checkpoint(&self) -> MarkdownParserCheckpoint {
        MarkdownParserCheckpoint {
            context: self.context.checkpoint(),
            source: self.source.checkpoint(),
            state: self.state.checkpoint(),
        }
    }

    /// Clear any buffered lookahead without changing the current position.
    pub(crate) fn reset_lookahead(&mut self) {
        let checkpoint = self.source.checkpoint();
        self.source.rewind(checkpoint);
    }

    /// Returns leading indentation on the current line, including whitespace
    /// inside the current token.
    pub fn line_start_leading_indent(&self) -> usize {
        if self.state.virtual_line_start == Some(self.cur_range().start()) {
            let source = self.source.source_text();
            let start: usize = self.cur_range().start().into();
            let rest = &source[start..];
            // Fast path: when the leading whitespace contains no tabs, the
            // column-aware count is identical to a plain space count and we
            // can avoid the O(line-length) scan to find the line start.
            if !rest
                .as_bytes()
                .iter()
                .take_while(|b| matches!(b, b' ' | b'\t'))
                .any(|b| *b == b'\t')
            {
                return rest.bytes().take_while(|b| *b == b' ').count();
            }
            let start_col = self.cached_absolute_column_at(start);
            return count_leading_indent_from_column(rest, start_col);
        }

        self.source.line_start_leading_indent()
    }

    /// Returns true if the parser is at the start of input (position 0).
    /// This is used for detecting block-level constructs at the start of a document.
    ///
    /// Uses position-based check rather than trivia_len, so it works correctly
    /// when NEWLINE becomes an explicit token (not trivia).
    pub fn at_start_of_input(&self) -> bool {
        self.source.at_start_of_input()
    }

    /// Returns true if the parser is at the start of a line.
    ///
    /// This is true when:
    /// - At start of input (position 0)
    /// - The current token has a preceding line break (lexer's after_newline flag)
    ///
    /// Used for detecting block-level constructs that must start at line beginning
    /// (e.g., headers, list items, thematic breaks).
    pub fn at_line_start(&self) -> bool {
        self.at_start_of_input()
            || self.has_preceding_line_break()
            || self.source.at_line_start_with_whitespace()
            || self.state.virtual_line_start == Some(self.cur_range().start())
    }

    pub(crate) fn set_virtual_line_start(&mut self) {
        self.state.virtual_line_start = Some(self.cur_range().start());
    }

    /// Emit an MdIndentTokenList for optional block prefix indentation at line start.
    ///
    /// Like `skip_line_indent()` but emits real CST nodes (`MdIndentToken` /
    /// `MdIndentTokenList`) instead of skipped trivia. Use this for non-lookahead,
    /// non-error-recovery paths where the indent tokens should be visible in the tree.
    pub fn emit_line_indent(&mut self, max_indent: usize) -> bool {
        if !self.at_line_start() {
            let list_m = self.start();
            list_m.complete(self, MarkdownSyntaxKind::MD_INDENT_TOKEN_LIST);
            return false;
        }

        let list_m = self.start();
        let did_emit = self.emit_indent_tokens_core(max_indent);
        list_m.complete(self, MarkdownSyntaxKind::MD_INDENT_TOKEN_LIST);
        did_emit
    }

    /// Emit individual `MdIndentToken` nodes (no list wrapper) for indentation.
    ///
    /// Use this inside inline item lists or content lists where `MdIndentToken`
    /// is already a valid child (via `AnyMdInline`). Unlike `emit_line_indent()`,
    /// this does NOT wrap tokens in an `MdIndentTokenList`.
    pub fn emit_indent_tokens(&mut self, max_indent: usize) -> bool {
        if !self.at_line_start() {
            return false;
        }

        self.emit_indent_tokens_core(max_indent)
    }

    /// Shared core: emit one `MdIndentToken` holding the whole run of
    /// whitespace that fits within `max_indent` columns.
    ///
    /// Indentation arrives from the lexer as one whitespace token per
    /// character at line start, so the run is measured directly on the
    /// source and consumed as a single `MD_INDENT_CHAR` token.
    fn emit_indent_tokens_core(&mut self, max_indent: usize) -> bool {
        if !self.at(MarkdownSyntaxKind::MD_TEXTUAL_LITERAL) {
            return false;
        }

        let start: usize = self.cur_range().start().into();
        let source = self.source.source_text();
        let mut consumed = 0usize;
        let mut end = start;

        for byte in source[start..].bytes() {
            let width = match byte {
                b' ' => 1,
                b'\t' => TAB_STOP_SPACES,
                _ => break,
            };
            if consumed + width > max_indent {
                break;
            }
            consumed += width;
            end += 1;
        }

        if end == start {
            return false;
        }

        self.re_lex_span(
            TextSize::from(end as u32),
            MarkdownSyntaxKind::MD_INDENT_CHAR,
        );
        let token_m = self.start();
        self.bump(MarkdownSyntaxKind::MD_INDENT_CHAR);
        token_m.complete(self, MarkdownSyntaxKind::MD_INDENT_TOKEN);
        true
    }

    /// Consume optional indentation whitespace at line start, up to `max_indent`
    /// columns. Each whitespace token is consumed as `Whitespace` trivia
    /// (attached to the next real token).
    ///
    /// This avoids producing `Skipped` trivia, which should be reserved for
    /// error-recovery paths.
    pub fn skip_line_indent(&mut self, max_indent: usize) -> bool {
        if !self.at_line_start() {
            return false;
        }

        let mut consumed = 0usize;
        let mut did_skip = false;

        while self.at(MarkdownSyntaxKind::MD_TEXTUAL_LITERAL) {
            let text = self.cur_text();
            if text.is_empty() || !text.chars().all(|c| c == ' ' || c == '\t') {
                break;
            }

            let indent = text
                .chars()
                .map(|c| if c == '\t' { TAB_STOP_SPACES } else { 1 })
                .sum::<usize>();

            if consumed + indent > max_indent {
                break;
            }

            consumed += indent;
            did_skip = true;
            self.consume_as_whitespace_trivia();
        }

        did_skip
    }

    /// Consume the current token as `Whitespace` trivia (not `Skipped`).
    ///
    /// Use this for spec-driven structural whitespace that should be consumed
    /// but not appear as explicit CST nodes. The token is removed from the
    /// event stream and attached as `Whitespace` trivia on the next real token.
    pub fn consume_as_whitespace_trivia(&mut self) {
        self.source_mut().skip_as_trivia_of_kind_with_context(
            TriviaPieceKind::Whitespace,
            MarkdownLexContext::Regular,
        );
    }

    /// Returns true if inline content should stop parsing.
    ///
    /// Inline content ends at:
    /// - EOF
    /// - NEWLINE token (NEWLINE is an explicit token in Markdown)
    /// - A preceding line break (lexer flag, for compatibility during transition)
    ///
    /// This provides a unified check for inline parsing loops.
    pub fn at_inline_end(&self) -> bool {
        self.at(MarkdownSyntaxKind::EOF)
            || self.at(MarkdownSyntaxKind::NEWLINE)
            || self.has_preceding_line_break()
    }

    /// Returns true if the parser is at a blank line boundary.
    ///
    /// A blank line is a NEWLINE followed by optional whitespace, then another
    /// NEWLINE or EOF. This is a token-based lookahead check.
    ///
    /// Used for:
    /// - Paragraph boundaries
    /// - Tight/loose list determination
    /// - Block quote continuation
    ///
    /// # NEWLINE Consumption Policy
    ///
    /// When this returns true, the parser should NOT consume the NEWLINE.
    /// Instead, the block-level parser should handle the paragraph boundary.
    /// The NEWLINE at a blank line marks the end of the current block.
    pub fn at_blank_line(&self) -> bool {
        if !self.at(MarkdownSyntaxKind::NEWLINE) {
            return false;
        }

        // Look at source after the current NEWLINE token
        let source = self.source_after_current();
        let newline_len = self.cur_text().len();

        // Get text after this NEWLINE
        if source.len() <= newline_len {
            // NEWLINE at end of input = blank line (paragraph ends)
            return true;
        }

        let after_newline = &source[newline_len..];

        // Skip optional whitespace/tabs (these are still trivia)
        let trimmed = after_newline.trim_start_matches([' ', '\t']);

        // Blank line if what remains is empty (EOF) or starts with another newline
        // Handle all line ending variants: LF (\n), CRLF (\r\n), and CR (\r)
        trimmed.is_empty() || trimmed.starts_with('\n') || trimmed.starts_with('\r')
    }

    /// Returns the source text starting from the current token position.
    /// This is useful for lookahead when detecting HTML blocks.
    pub fn source_after_current(&self) -> &str {
        self.source.source_after_current()
    }

    pub(crate) fn has_frontmatter_closing_fence(&self) -> bool {
        self.source.has_frontmatter_closing_fence()
    }

    pub fn rewind(&mut self, checkpoint: MarkdownParserCheckpoint) {
        let MarkdownParserCheckpoint {
            context,
            source,
            state,
        } = checkpoint;

        self.context.rewind(context);
        self.source.rewind(source);
        self.state.rewind(state);
    }

    /// Execute a lookahead operation without consuming tokens.
    ///
    /// This saves a checkpoint, executes the provided closure, then rewinds
    /// to the checkpoint. The closure's return value is passed through.
    ///
    /// Use this for speculative parsing where you need to examine tokens
    /// ahead without committing to parsing them.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let is_valid = p.lookahead(|p| {
    ///     p.expect(L_BRACK);
    ///     // ... check pattern
    ///     true
    /// });
    /// ```
    pub fn lookahead<F, R>(&mut self, op: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        let checkpoint = self.checkpoint();
        let result = op(self);
        self.rewind(checkpoint);
        result
    }

    pub fn finish(self) -> MarkdownParserOutput {
        let (trivia, lexer_diagnostics) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        let diagnostics = merge_diagnostics(lexer_diagnostics, parse_diagnostics);

        MarkdownParserOutput {
            events,
            diagnostics,
            trivia,
            deferred_inlines: self.state.deferred_inlines,
            link_reference_definitions: self.state.link_reference_definitions,
            list_tightness: self.state.list_tightness,
            list_item_indents: self.state.list_item_indents,
            quote_indents: self.state.quote_indents,
        }
    }
}

/// Compute the absolute column position (0-indexed) of `offset` in `source`,
/// honoring CommonMark tab expansion to the next multiple of `TAB_STOP_SPACES`.
pub(crate) fn absolute_column_at(source: &str, offset: usize) -> usize {
    let bytes = source.as_bytes();
    let line_start = bytes[..offset]
        .iter()
        .rposition(|&b| b == b'\n' || b == b'\r')
        .map_or(0, |idx| idx + 1);
    let mut col = 0usize;
    for c in source[line_start..offset].chars() {
        match c {
            '\t' => col += TAB_STOP_SPACES - (col % TAB_STOP_SPACES),
            _ => col += 1,
        }
    }
    col
}

/// Count the number of indent columns consumed by leading spaces/tabs in
/// `text`, starting from `start_col`. Tabs expand to the next multiple of
/// `TAB_STOP_SPACES` relative to the absolute column position.
pub(crate) fn count_leading_indent_from_column(text: &str, start_col: usize) -> usize {
    let mut col = start_col;
    for c in text.chars() {
        match c {
            ' ' => col += 1,
            '\t' => col += TAB_STOP_SPACES - (col % TAB_STOP_SPACES),
            _ => break,
        }
    }
    col - start_col
}

impl<'source> Parser for MarkdownParser<'source> {
    type Kind = MarkdownSyntaxKind;
    type Source = MarkdownTokenSource<'source>;

    fn context(&self) -> &ParserContext<Self::Kind> {
        &self.context
    }

    fn context_mut(&mut self) -> &mut ParserContext<Self::Kind> {
        &mut self.context
    }

    fn source(&self) -> &Self::Source {
        &self.source
    }

    fn source_mut(&mut self) -> &mut Self::Source {
        &mut self.source
    }
}

pub struct MarkdownParserCheckpoint {
    pub(super) context: ParserContextCheckpoint,
    pub(super) source: MarkdownTokenSourceCheckpoint,
    state: MarkdownParserStateCheckpoint,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewind_removes_only_speculative_link_definitions() {
        let source = "foo Foo bar";
        let mut parser = MarkdownParser::new(source, MarkdownParserOptions::default());
        parser.record_link_reference_definition(TextRange::new(0.into(), 3.into()));
        let checkpoint = parser.checkpoint();

        parser.record_link_reference_definition(TextRange::new(4.into(), 7.into()));
        parser.record_link_reference_definition(TextRange::new(8.into(), 11.into()));
        parser.rewind(checkpoint);

        assert!(parser.has_link_reference_definition("foo"));
        assert!(!parser.has_link_reference_definition("bar"));
        assert_eq!(parser.state.link_reference_definitions.ranges.len(), 1);
        assert_eq!(parser.state.link_reference_definitions.hashes.len(), 1);
        assert_eq!(
            parser.state.link_reference_definitions.ranges_by_hash.len(),
            1
        );
    }
}
