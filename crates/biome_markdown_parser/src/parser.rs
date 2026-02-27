use biome_markdown_syntax::MarkdownSyntaxKind;
use biome_parser::ParserContext;
use biome_parser::event::Event;
use biome_parser::prelude::*;
use biome_parser::token_source::Trivia;
use biome_parser::{ParserContextCheckpoint, diagnostic::merge_diagnostics};
use biome_rowan::{TextRange, TextSize};
use std::collections::HashSet;

use crate::lexer::{MarkdownLexContext, MarkdownReLexContext};
use crate::syntax::TAB_STOP_SPACES;
use crate::syntax::inline::EmphasisContext;
use crate::syntax::parse_error::DEFAULT_MAX_NESTING_DEPTH;
use crate::token_source::{MarkdownTokenSource, MarkdownTokenSourceCheckpoint};

/// Options for configuring the markdown parser.
#[derive(Debug, Clone)]
pub struct MarkdownParseOptions {
    /// Maximum nesting depth for block quotes and lists.
    ///
    /// This limits recursion on pathological input to avoid stack overflow.
    pub max_nesting_depth: usize,
    // Reserved for future GFM options
}

impl Default for MarkdownParseOptions {
    fn default() -> Self {
        Self {
            max_nesting_depth: DEFAULT_MAX_NESTING_DEPTH,
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
    /// Emphasis parsing context for the current inline item list.
    pub(crate) emphasis_context: Option<EmphasisContext>,
    /// Normalized link reference definitions collected in a prepass.
    pub(crate) link_reference_definitions: HashSet<String>,
    /// Recorded tight/loose list results keyed by list node range.
    pub(crate) list_tightness: Vec<ListTightness>,
    /// Recorded list item indents keyed by bullet node range.
    pub(crate) list_item_indents: Vec<ListItemIndent>,
    /// Recorded quote marker indents keyed by quote node range.
    pub(crate) quote_indents: Vec<QuoteIndent>,
    /// Virtual line start override for container prefixes (e.g., block quotes).
    pub(crate) virtual_line_start: Option<TextSize>,
    /// Flag to unwind quote parsing when nesting exceeds the maximum depth.
    pub(crate) quote_depth_exceeded: bool,
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

type FinishResult = (
    Vec<Event<MarkdownSyntaxKind>>,
    Vec<ParseDiagnostic>,
    Vec<Trivia>,
    Vec<ListTightness>,
    Vec<ListItemIndent>,
    Vec<QuoteIndent>,
);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuoteIndent {
    pub range: TextRange,
    pub indent: usize,
}

pub(crate) struct MarkdownParser<'source> {
    context: ParserContext<MarkdownSyntaxKind>,
    source: MarkdownTokenSource<'source>,
    options: MarkdownParseOptions,
    state: MarkdownParserState,
}

impl<'source> MarkdownParser<'source> {
    pub fn new(source: &'source str, options: MarkdownParseOptions) -> Self {
        Self {
            context: ParserContext::default(),
            source: MarkdownTokenSource::from_str(source),
            options,
            state: MarkdownParserState::default(),
        }
    }

    /// Returns parser options. Reserved for GFM extensions.
    pub(crate) fn options(&self) -> &MarkdownParseOptions {
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
        self.state.emphasis_context.as_ref()
    }

    /// Replace the emphasis context, returning the previous value.
    pub(crate) fn set_emphasis_context(
        &mut self,
        context: Option<EmphasisContext>,
    ) -> Option<EmphasisContext> {
        std::mem::replace(&mut self.state.emphasis_context, context)
    }

    /// Replace the set of normalized link reference definitions.
    pub(crate) fn set_link_reference_definitions(&mut self, definitions: HashSet<String>) {
        self.state.link_reference_definitions = definitions;
    }

    /// Returns true if a normalized label has a link reference definition.
    pub(crate) fn has_link_reference_definition(&self, label: &str) -> bool {
        self.state.link_reference_definitions.contains(label)
    }

    /// Record tight/loose information for a parsed list node.
    pub(crate) fn record_list_tightness(&mut self, range: TextRange, is_tight: bool) {
        let range = self.trim_range(range);
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
        let range = self.trim_range(range);
        self.state.list_item_indents.push(ListItemIndent {
            range,
            indent,
            marker_indent,
            marker_width,
            spaces_after_marker,
        });
    }

    pub(crate) fn record_quote_indent(&mut self, range: TextRange, indent: usize) {
        let range = self.trim_range(range);
        self.state.quote_indents.push(QuoteIndent { range, indent });
    }

    /// Re-lex the current token using LinkDefinition context.
    /// This makes whitespace produce separate tokens for destination/title parsing.
    pub(crate) fn re_lex_link_definition(&mut self) {
        self.source.re_lex(MarkdownReLexContext::LinkDefinition);
    }

    /// Force re-lex the current token in Regular context.
    ///
    /// Use this when switching from LinkDefinition context back to Regular context,
    /// e.g., when entering title content where whitespace should not split tokens.
    pub(crate) fn force_relex_regular(&mut self) {
        self.source
            .force_relex_in_context(MarkdownLexContext::Regular);
    }

    /// Force re-lex the current token in CodeSpan context.
    /// In this context, backslash is literal (not an escape character).
    /// Used for autolinks where `\>` should be `\` + `>` as separate tokens.
    pub(crate) fn relex_code_span(&mut self) {
        self.source
            .force_relex_in_context(MarkdownLexContext::CodeSpan);
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

    pub(crate) fn set_force_ordered_list_marker(&mut self, value: bool) {
        self.source.set_force_ordered_list_marker(value);
    }

    /// Bump the current token using LinkDefinition context.
    /// The next token will be lexed with whitespace as separate tokens.
    pub(crate) fn bump_link_definition(&mut self) {
        self.source.bump_link_definition();
    }

    pub fn checkpoint(&self) -> MarkdownParserCheckpoint {
        MarkdownParserCheckpoint {
            context: self.context.checkpoint(),
            source: self.source.checkpoint(),
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
            return count_leading_indent(&source[start..]);
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

    pub(crate) fn trim_range(&self, range: TextRange) -> TextRange {
        let start: usize = range.start().into();
        let end: usize = range.end().into();
        if start >= end {
            return range;
        }

        let source = self.source.source_text();
        let slice = &source[start..end];
        if slice
            .trim_matches(|c: char| matches!(c, ' ' | '\t' | '\r'))
            .is_empty()
        {
            return TextRange::new(range.start(), range.start());
        }
        let leading = slice
            .len()
            .saturating_sub(slice.trim_start_matches([' ', '\t', '\r']).len());
        let trailing = slice
            .len()
            .saturating_sub(slice.trim_end_matches([' ', '\t', '\r']).len());
        let new_start = start + leading;
        let new_end = end.saturating_sub(trailing);

        TextRange::new((new_start as u32).into(), (new_end as u32).into())
    }

    /// Skip an optional indentation token at line start if it is whitespace-only
    /// and does not exceed `max_indent` columns.
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
            self.parse_as_skipped_trivia_tokens(|p| p.bump(MarkdownSyntaxKind::MD_TEXTUAL_LITERAL));
        }

        did_skip
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

    pub fn rewind(&mut self, checkpoint: MarkdownParserCheckpoint) {
        let MarkdownParserCheckpoint { context, source } = checkpoint;

        self.context.rewind(context);
        self.source.rewind(source);
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

    pub fn finish(self) -> FinishResult {
        let (trivia, lexer_diagnostics) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        let diagnostics = merge_diagnostics(lexer_diagnostics, parse_diagnostics);

        (
            events,
            diagnostics,
            trivia,
            self.state.list_tightness,
            self.state.list_item_indents,
            self.state.quote_indents,
        )
    }
}

fn count_leading_indent(text: &str) -> usize {
    let mut count = 0usize;
    for c in text.chars() {
        match c {
            ' ' => count += 1,
            '\t' => count += TAB_STOP_SPACES,
            _ => break,
        }
    }
    count
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
}
