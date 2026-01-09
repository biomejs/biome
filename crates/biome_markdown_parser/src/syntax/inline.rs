//! Inline element parsing for Markdown.
//!
//! Handles inline code spans, emphasis (bold/italic), links, images, line breaks, and raw HTML.
//!
//! # CommonMark Specification References
//!
//! This module implements the following CommonMark 0.31.2 sections:
//!
//! - **§6.1 Code spans**: Backtick-delimited inline code (`code`)
//! - **§6.2 Emphasis and strong emphasis**: `*italic*`, `**bold**`, `_italic_`, `__bold__`
//! - **§6.3 Links**: `[text](url)` inline links
//! - **§6.4 Autolinks (URI)**: `<https://example.com>`
//! - **§6.5 Autolinks (email)**: `<user@example.com>`
//! - **§6.6 Hard line breaks**: Trailing spaces or backslash before newline
//! - **§6.7 Soft line breaks**: Single newline within paragraph
//! - **§6.8 Raw HTML**: `<tag>`, `</tag>`, `<!-- comment -->`, `<?...?>`, `<!...>`, `<![CDATA[...]]>`
//!
//! # Emphasis Algorithm (§6.4)
//!
//! This module implements the CommonMark delimiter stack algorithm for emphasis:
//!
//! 1. **First pass**: Collect delimiter runs from the inline content
//! 2. **Second pass**: Match openers and closers using the delimiter stack algorithm
//! 3. **Rule of 3**: If (opener_count + closer_count) % 3 == 0 and both can open/close,
//!    skip the match unless both counts are divisible by 3
//!
//! # Emphasis Flanking Rules (§6.2)
//!
//! A delimiter run is **left-flanking** if:
//! 1. Not followed by Unicode whitespace, AND
//! 2. Not followed by punctuation, OR preceded by whitespace/punctuation
//!
//! A delimiter run is **right-flanking** if:
//! 1. Not preceded by Unicode whitespace, AND
//! 2. Not preceded by punctuation, OR followed by whitespace/punctuation
//!
//! Underscore (`_`) has additional intraword restrictions (§6.2 rules 2, 5, 7, 8).

use biome_markdown_syntax::MarkdownSyntaxKind;
use biome_markdown_syntax::T;
use biome_markdown_syntax::kind::MarkdownSyntaxKind::*;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax::{self, *};

use biome_rowan::{TextRange, TextSize};

use crate::MarkdownParser;
use crate::link_reference::normalize_reference_label;

// ============================================================================
// Delimiter Stack Types for Emphasis Parsing
// ============================================================================

/// Kind of emphasis delimiter (* or _)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DelimKind {
    Star,
    Underscore,
}

/// A delimiter run collected during the first pass
#[derive(Debug, Clone)]
struct DelimRun {
    /// The delimiter character kind
    kind: DelimKind,
    /// Number of delimiter characters in this run
    count: usize,
    /// Whether this can open emphasis (left-flanking)
    can_open: bool,
    /// Whether this can close emphasis (right-flanking)
    can_close: bool,
    /// Byte offset in the source where this run starts
    start_offset: usize,
}

/// A matched emphasis span (opener + closer)
#[derive(Debug, Clone)]
struct EmphasisMatch {
    /// Byte offset where the opener delimiter starts
    opener_start: usize,
    /// Byte offset where the closer delimiter starts
    closer_start: usize,
    /// Whether this is strong (2 chars) or regular (1 char) emphasis
    is_strong: bool,
}

/// Check if a character is Unicode whitespace for flanking rules.
fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

/// Check if a character is Unicode punctuation for flanking rules.
/// Per CommonMark spec, this includes ASCII punctuation and Unicode punctuation categories.
fn is_punctuation(c: char) -> bool {
    // ASCII punctuation + Unicode punctuation categories
    matches!(
        c,
        '!' | '"'
            | '#'
            | '$'
            | '%'
            | '&'
            | '\''
            | '('
            | ')'
            | '*'
            | '+'
            | ','
            | '-'
            | '.'
            | '/'
            | ':'
            | ';'
            | '<'
            | '='
            | '>'
            | '?'
            | '@'
            | '['
            | '\\'
            | ']'
            | '^'
            | '_'
            | '`'
            | '{'
            | '|'
            | '}'
            | '~'
    ) || c.is_ascii_punctuation()
        || matches!(c, '\u{2000}'..='\u{206F}' | '\u{2E00}'..='\u{2E7F}')
}

/// Check if an opening delimiter is left-flanking per CommonMark rules.
/// A left-flanking delimiter run is one that is:
/// - Not followed by Unicode whitespace, AND
/// - Either (a) not followed by punctuation, OR (b) preceded by whitespace/punctuation
fn is_left_flanking_delimiter(char_after: Option<char>, char_before: Option<char>) -> bool {
    match char_after {
        None => false,                        // At end of input, can't be left-flanking
        Some(c) if is_whitespace(c) => false, // Followed by whitespace
        Some(c) if is_punctuation(c) => {
            // Followed by punctuation - only left-flanking if preceded by whitespace or punctuation
            match char_before {
                None => true, // Start of input counts as whitespace
                Some(b) => is_whitespace(b) || is_punctuation(b),
            }
        }
        Some(_) => true, // Not followed by whitespace or punctuation = left-flanking
    }
}

/// Check if a closing delimiter is right-flanking per CommonMark rules.
/// A right-flanking delimiter run is one that is:
/// - Not preceded by Unicode whitespace, AND
/// - Either (a) not preceded by punctuation, OR (b) followed by whitespace/punctuation
fn is_right_flanking_delimiter(char_before: Option<char>, char_after: Option<char>) -> bool {
    match char_before {
        None => false,                        // At start of input, can't be right-flanking
        Some(c) if is_whitespace(c) => false, // Preceded by whitespace
        Some(c) if is_punctuation(c) => {
            // Preceded by punctuation - only right-flanking if followed by whitespace or punctuation
            match char_after {
                None => true, // End of input counts as whitespace
                Some(a) => is_whitespace(a) || is_punctuation(a),
            }
        }
        Some(_) => true, // Not preceded by whitespace or punctuation = right-flanking
    }
}

/// Check if underscore can open emphasis (stricter rules than asterisk).
/// Per CommonMark 6.2, underscore can open emphasis iff it is left-flanking AND either:
/// - Not part of a right-flanking delimiter run, OR
/// - Preceded by a punctuation character
fn can_underscore_open(char_before: Option<char>, char_after: Option<char>) -> bool {
    // Must be left-flanking
    if !is_left_flanking_delimiter(char_after, char_before) {
        return false;
    }
    // If also right-flanking, must be preceded by punctuation
    if is_right_flanking_delimiter(char_before, char_after) {
        return matches!(char_before, Some(c) if is_punctuation(c));
    }
    true
}

/// Check if underscore can close emphasis (stricter rules than asterisk).
/// Per CommonMark 6.2, underscore can close emphasis iff it is right-flanking AND either:
/// - Not part of a left-flanking delimiter run, OR
/// - Followed by a punctuation character
fn can_underscore_close(char_before: Option<char>, char_after: Option<char>) -> bool {
    // Must be right-flanking
    if !is_right_flanking_delimiter(char_before, char_after) {
        return false;
    }
    // If also left-flanking, must be followed by punctuation
    if is_left_flanking_delimiter(char_after, char_before) {
        return matches!(char_after, Some(c) if is_punctuation(c));
    }
    true
}

// ============================================================================
// Delimiter Stack Algorithm Implementation
// ============================================================================

/// Collect all delimiter runs from source text.
///
/// This is the first pass of the CommonMark emphasis algorithm. It scans
/// the source text and identifies all potential delimiter runs (sequences
/// of `*` or `_`), computing their flanking status.
fn collect_delimiter_runs(source: &str) -> Vec<DelimRun> {
    let mut runs = Vec::new();
    let bytes = source.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];

        // Check for delimiter characters
        if b == b'*' || b == b'_' {
            let kind = if b == b'*' {
                DelimKind::Star
            } else {
                DelimKind::Underscore
            };
            let start_offset = i;

            // Count consecutive delimiter characters
            let mut count = 1;
            while i + count < bytes.len() && bytes[i + count] == b {
                count += 1;
            }
            let end_offset = i + count;

            // Get character before delimiter run
            let char_before = if start_offset > 0 {
                // Get the char ending at start_offset
                let before_slice = &source[..start_offset];
                before_slice.chars().next_back()
            } else {
                None
            };

            // Get character after delimiter run
            let char_after = source[end_offset..].chars().next();

            // Compute flanking status
            let (can_open, can_close) = if kind == DelimKind::Underscore {
                (
                    can_underscore_open(char_before, char_after),
                    can_underscore_close(char_before, char_after),
                )
            } else {
                // Asterisk: can open if left-flanking, can close if right-flanking
                (
                    is_left_flanking_delimiter(char_after, char_before),
                    is_right_flanking_delimiter(char_before, char_after),
                )
            };

            runs.push(DelimRun {
                kind,
                count,
                can_open,
                can_close,
                start_offset,
            });

            i = end_offset;
        } else if b == b'`' {
            // Skip code spans - they block emphasis
            let backtick_count = {
                let mut c = 1;
                while i + c < bytes.len() && bytes[i + c] == b'`' {
                    c += 1;
                }
                c
            };
            i += backtick_count;

            // Find closing backticks
            while i < bytes.len() {
                if bytes[i] == b'`' {
                    let close_count = {
                        let mut c = 1;
                        while i + c < bytes.len() && bytes[i + c] == b'`' {
                            c += 1;
                        }
                        c
                    };
                    i += close_count;
                    if close_count == backtick_count {
                        break;
                    }
                } else {
                    i += 1;
                }
            }
        } else if b == b'<' {
            // Skip potential HTML tags and autolinks
            i += 1;
            while i < bytes.len() && bytes[i] != b'>' && bytes[i] != b'\n' {
                i += 1;
            }
            if i < bytes.len() && bytes[i] == b'>' {
                i += 1;
            }
        } else if b == b'\\' && i + 1 < bytes.len() {
            // Skip escaped characters
            i += 2;
        } else {
            i += 1;
        }
    }

    runs
}

/// Match delimiter runs using the CommonMark algorithm.
///
/// This is the second pass. It processes closers from left to right,
/// searching backward for matching openers. Returns a list of matched
/// emphasis spans sorted by opener position.
fn match_delimiters(runs: &mut [DelimRun]) -> Vec<EmphasisMatch> {
    let mut matches = Vec::new();
    let mut opener_stack: Vec<usize> = Vec::new();

    for idx in 0..runs.len() {
        if runs[idx].can_close && runs[idx].count > 0 {
            loop {
                let mut opener_stack_pos = None;
                let prefer_strong = runs[idx].count >= 2;

                for pass in 0..2 {
                    for (pos, &opener_idx) in opener_stack.iter().enumerate().rev() {
                        let opener = &runs[opener_idx];
                        let closer = &runs[idx];

                        if opener.kind != closer.kind || !opener.can_open || opener.count == 0 {
                            continue;
                        }

                        if prefer_strong && pass == 0 && opener.count < 2 {
                            continue;
                        }

                        // Rule of 3: if (opener_count + closer_count) % 3 == 0 and
                        // the closer can open or the opener can close, skip unless
                        // both counts are divisible by 3
                        let opener_count = opener.count;
                        let closer_count = closer.count;
                        if ((opener.can_open && opener.can_close)
                            || (closer.can_open && closer.can_close))
                            && (opener_count + closer_count).is_multiple_of(3)
                            && (!opener_count.is_multiple_of(3) || !closer_count.is_multiple_of(3))
                        {
                            continue;
                        }

                        opener_stack_pos = Some(pos);
                        break;
                    }

                    if opener_stack_pos.is_some() {
                        break;
                    }
                }

                let Some(pos) = opener_stack_pos else { break };
                let opener_idx = opener_stack[pos];
                let use_count = if runs[opener_idx].count >= 2 && runs[idx].count >= 2 {
                    2
                } else {
                    1
                };

                let opener_start = runs[opener_idx].start_offset;
                let closer_start = runs[idx].start_offset;

                matches.push(EmphasisMatch {
                    opener_start,
                    closer_start,
                    is_strong: use_count == 2,
                });

                runs[opener_idx].count -= use_count;
                runs[opener_idx].start_offset += use_count;
                runs[idx].count -= use_count;
                runs[idx].start_offset += use_count;

                // Remove openers between the matched opener and this closer.
                opener_stack.truncate(pos + 1);
                if runs[opener_idx].count == 0 {
                    opener_stack.pop();
                }

                if use_count == 2 && runs[opener_idx].count > 0 && runs[idx].count > 0 {
                    // Avoid crossing matches from odd-length runs (e.g. ***foo***).
                    break;
                }

                if runs[idx].count == 0 {
                    break;
                }
            }
        }

        if runs[idx].can_open && runs[idx].count > 0 {
            opener_stack.push(idx);
        }
    }

    // Sort matches by opener position for nested processing
    matches.sort_by_key(|m| m.opener_start);

    matches
}

/// Context for emphasis-aware inline parsing
#[derive(Debug)]
pub(crate) struct EmphasisContext {
    /// Matched emphasis spans, sorted by opener_start
    matches: Vec<EmphasisMatch>,
    /// Base offset of the inline content in the source
    base_offset: usize,
}

impl EmphasisContext {
    /// Create a new emphasis context by analyzing the source text
    pub(crate) fn new(source: &str, base_offset: usize) -> Self {
        let mut runs = collect_delimiter_runs(source);
        let matches = match_delimiters(&mut runs);
        Self {
            matches,
            base_offset,
        }
    }

    /// Check if there's an emphasis opener at the given offset
    fn opener_at(&self, offset: usize) -> Option<&EmphasisMatch> {
        let abs_offset = offset;
        self.matches
            .iter()
            .find(|m| m.opener_start + self.base_offset == abs_offset)
    }
}

/// Parse a hard line break.
///
/// Grammar: MdHardLine = value: 'md_hard_line_literal'
///
/// A hard line break is created by either:
/// - Two or more trailing spaces followed by a newline
/// - A backslash followed by a newline
pub(crate) fn parse_hard_line(p: &mut MarkdownParser) -> ParsedSyntax {
    if !p.at(MD_HARD_LINE_LITERAL) {
        return Absent;
    }

    let m = p.start();
    p.bump(MD_HARD_LINE_LITERAL);
    Present(m.complete(p, MD_HARD_LINE))
}

/// Parse inline code span (`` `code` `` or ``` `` `code` `` ```).
///
/// Grammar: MdInlineCode = l_tick: '`' content: MdInlineItemList r_tick: '`'
///
/// Per CommonMark, code spans can use multiple backticks to allow literal
/// backticks inside: ``` `` `code` `` ``` wraps around code containing backticks.
/// The opening and closing backtick strings must be the same length.
pub(crate) fn parse_inline_code(p: &mut MarkdownParser) -> ParsedSyntax {
    if !p.at(BACKTICK) {
        return Absent;
    }

    let m = p.start();

    // Count opening backticks from token text
    let opening_count = p.cur_text().len();
    let opening_range = p.cur_range();

    // Opening backtick(s)
    p.bump(BACKTICK);

    // Content - parse until we find a BACKTICK with matching count, or EOF
    let content = p.start();
    let mut found_closing = false;
    loop {
        if p.at_inline_end() {
            break;
        }

        // Check for matching closing backticks
        if p.at(BACKTICK) {
            let closing_count = p.cur_text().len();
            if closing_count == opening_count {
                // Found matching closing backticks
                found_closing = true;
                break;
            }
            // Not matching - consume as content
            let text_m = p.start();
            p.bump_remap(MD_TEXTUAL_LITERAL);
            text_m.complete(p, MD_TEXTUAL);
            continue;
        }

        // Regular content
        let text_m = p.start();
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);
    }
    content.complete(p, MD_INLINE_ITEM_LIST);

    // Closing backtick(s) - emit custom diagnostic if missing
    if found_closing {
        p.bump(BACKTICK);
    } else {
        p.error(super::parse_error::unclosed_code_span(
            p,
            opening_range,
            opening_count,
        ));
    }

    Present(m.complete(p, MD_INLINE_CODE))
}

/// Parse emphasis using the delimiter stack matches.
fn parse_emphasis_from_context(p: &mut MarkdownParser, expect_strong: bool) -> ParsedSyntax {
    let context = match p.emphasis_context() {
        Some(context) => context,
        None => return Absent,
    };

    let offset = u32::from(p.cur_range().start()) as usize;
    let matched = match context.opener_at(offset) {
        Some(matched) => matched,
        None => return Absent,
    };

    if matched.is_strong != expect_strong {
        return Absent;
    }

    let (opener_kind, closer_kind, opener_text) = if expect_strong {
        if p.at(DOUBLE_STAR) {
            (DOUBLE_STAR, DOUBLE_STAR, "**")
        } else if p.at(DOUBLE_UNDERSCORE) {
            (DOUBLE_UNDERSCORE, DOUBLE_UNDERSCORE, "__")
        } else {
            return Absent;
        }
    } else if p.at(T![*]) {
        (T![*], T![*], "*")
    } else if p.at(UNDERSCORE) {
        (UNDERSCORE, UNDERSCORE, "_")
    } else {
        return Absent;
    };

    let closer_offset = matched.closer_start + context.base_offset;
    let m = p.start();
    let opening_range = p.cur_range();

    p.bump(opener_kind);

    let content = p.start();
    loop {
        if p.at_inline_end() {
            break;
        }

        let current_offset = u32::from(p.cur_range().start()) as usize;
        if current_offset == closer_offset {
            break;
        }

        if parse_any_inline(p).is_absent() {
            break;
        }
    }
    content.complete(p, MD_INLINE_ITEM_LIST);

    if p.at(closer_kind) && u32::from(p.cur_range().start()) as usize == closer_offset {
        p.bump(closer_kind);
    } else {
        p.error(super::parse_error::unclosed_emphasis(
            p,
            opening_range,
            opener_text,
        ));
    }

    if expect_strong {
        Present(m.complete(p, MD_INLINE_EMPHASIS))
    } else {
        Present(m.complete(p, MD_INLINE_ITALIC))
    }
}

/// Parse inline emphasis (bold: `**text**` or `__text__`).
pub(crate) fn parse_inline_emphasis(p: &mut MarkdownParser) -> ParsedSyntax {
    parse_emphasis_from_context(p, true)
}

/// Parse inline italic (`*text*` or `_text_`).
pub(crate) fn parse_inline_italic(p: &mut MarkdownParser) -> ParsedSyntax {
    parse_emphasis_from_context(p, false)
}

fn parse_inline_item_list_until_no_links(p: &mut MarkdownParser, stop: MarkdownSyntaxKind) {
    let m = p.start();
    let prev_context = set_inline_emphasis_context_until(p, stop);

    loop {
        if p.at(stop) || p.at_inline_end() {
            break;
        }

        if parse_any_inline_no_links(p).is_absent() {
            break;
        }
    }

    m.complete(p, MD_INLINE_ITEM_LIST);
    p.set_emphasis_context(prev_context);
}

fn parse_any_inline_no_links(p: &mut MarkdownParser) -> ParsedSyntax {
    if (p.at(BANG) && p.nth_at(1, L_BRACK)) || p.at(L_BRACK) {
        return super::parse_textual(p);
    }

    parse_any_inline(p)
}

fn set_inline_emphasis_context_until(
    p: &mut MarkdownParser,
    stop: MarkdownSyntaxKind,
) -> Option<EmphasisContext> {
    let source_len = inline_list_source_len_until(p, stop);
    let source = p.source_after_current();
    let inline_source = if source_len <= source.len() {
        &source[..source_len]
    } else {
        source
    };
    let base_offset = u32::from(p.cur_range().start()) as usize;
    let context = EmphasisContext::new(inline_source, base_offset);
    p.set_emphasis_context(Some(context))
}

fn inline_list_source_len_until(p: &mut MarkdownParser, stop: MarkdownSyntaxKind) -> usize {
    p.lookahead(|p| {
        let mut len = 0usize;

        loop {
            if p.at(T![EOF]) || p.at(stop) || p.at_inline_end() {
                break;
            }

            len += p.cur_text().len();
            p.bump(p.cur());
        }

        len
    })
}

/// Parse link starting with `[` - dispatches to inline link or reference link.
///
/// After parsing `[text]`:
/// - If followed by `(` → inline link `[text](url)`
/// - If followed by `[` → reference link `[text][label]` or `[text][]`
/// - Otherwise → shortcut reference `[text]`
pub(crate) fn parse_link_or_reference(p: &mut MarkdownParser) -> ParsedSyntax {
    parse_link_or_image(p, LinkParseKind::Link)
}

/// Parse reference link label `[label]` or `[]`.
///
/// Grammar: `MdReferenceLinkLabel = '[' label: MdInlineItemList ']'`
///
/// Returns Present if `[` and `]` are found (even if empty for collapsed reference).
/// On failure (missing `]`), rewinds to the checkpoint so no tokens are consumed.
fn parse_reference_label(p: &mut MarkdownParser) -> ParsedSyntax {
    if !p.at(L_BRACK) {
        return Absent;
    }

    // Checkpoint so we can rewind if ] is missing
    let checkpoint = p.checkpoint();
    let m = p.start();

    // [
    p.bump(L_BRACK);

    // Label content (may be empty for collapsed reference)
    let label = p.start();
    while !p.at(R_BRACK) && !p.at_inline_end() {
        let text_m = p.start();
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);
    }
    label.complete(p, MD_INLINE_ITEM_LIST);

    // ]
    if !p.eat(R_BRACK) {
        // Missing closing bracket - abandon and rewind to not consume tokens
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    Present(m.complete(p, MD_REFERENCE_LINK_LABEL))
}

/// Parse inline link (`[text](url)`).
///
/// Grammar: `MdInlineLink = '[' text: MdInlineItemList ']' '(' source: MdInlineItemList ')'`
///
/// Note: This is kept for backwards compatibility but `parse_link_or_reference`
/// is the preferred entry point for link parsing.
pub(crate) fn parse_inline_link(p: &mut MarkdownParser) -> ParsedSyntax {
    parse_link_or_reference(p)
}

/// Parse image starting with `![` - dispatches to inline image or reference image.
///
/// After parsing `![alt]`:
/// - If followed by `(` → inline image `![alt](url)`
/// - If followed by `[` → reference image `![alt][label]` or `![alt][]`
/// - Otherwise → shortcut reference image `![alt]`
pub(crate) fn parse_image_or_reference(p: &mut MarkdownParser) -> ParsedSyntax {
    parse_link_or_image(p, LinkParseKind::Image)
}

#[derive(Copy, Clone)]
enum LinkParseKind {
    Link,
    Image,
}

impl LinkParseKind {
    fn starts_here(self, p: &mut MarkdownParser) -> bool {
        match self {
            Self::Link => p.at(L_BRACK),
            Self::Image => p.at(BANG) && p.nth_at(1, L_BRACK),
        }
    }

    fn bump_opening(self, p: &mut MarkdownParser) {
        if matches!(self, Self::Image) {
            p.bump(BANG);
        }
        p.bump(L_BRACK);
    }

    fn lookahead_reference(self, p: &mut MarkdownParser) -> Option<ReferenceLinkLookahead> {
        match self {
            Self::Link => lookahead_reference_link(p),
            Self::Image => lookahead_reference_image(p),
        }
    }

    fn inline_kind(self) -> MarkdownSyntaxKind {
        match self {
            Self::Link => MD_INLINE_LINK,
            Self::Image => MD_INLINE_IMAGE,
        }
    }

    fn reference_kind(self) -> MarkdownSyntaxKind {
        match self {
            Self::Link => MD_REFERENCE_LINK,
            Self::Image => MD_REFERENCE_IMAGE,
        }
    }

    fn report_unclosed_text(self, p: &mut MarkdownParser, opening_range: TextRange) {
        match self {
            Self::Link => p.error(super::parse_error::unclosed_link(
                p,
                opening_range,
                "expected `]` to close link text",
            )),
            Self::Image => p.error(super::parse_error::unclosed_image(
                p,
                opening_range,
                "expected `]` to close alt text",
            )),
        }
    }

    fn report_unclosed_destination(self, p: &mut MarkdownParser, opening_range: TextRange) {
        match self {
            Self::Link => p.error(super::parse_error::unclosed_link(
                p,
                opening_range,
                "expected `)` to close URL",
            )),
            Self::Image => p.error(super::parse_error::unclosed_image(
                p,
                opening_range,
                "expected `)` to close image URL",
            )),
        }
    }
}

fn parse_link_or_image(p: &mut MarkdownParser, kind: LinkParseKind) -> ParsedSyntax {
    if !kind.starts_here(p) {
        return Absent;
    }

    let checkpoint = p.checkpoint();
    let m = p.start();
    let opening_range = p.cur_range();
    let reference = kind.lookahead_reference(p);
    // Clear any cached lookahead tokens before switching lexing context.
    p.reset_lookahead();

    kind.bump_opening(p);

    // Link text / alt text
    parse_inline_item_list_until_no_links(p, R_BRACK);

    // ] - if missing at inline end, emit diagnostic; otherwise rewind
    if !p.eat(R_BRACK) {
        if p.at_inline_end() {
            // Unclosed link/image at end of inline content - emit diagnostic
            kind.report_unclosed_text(p, opening_range);
            // Return as reference link/image (shortcut) with missing closing bracket
            return Present(m.complete(p, kind.reference_kind()));
        }
        // Not at inline end but missing ] - rewind and treat as text
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    // Now decide based on what follows ]
    if p.at(L_PAREN) {
        // Inline link/image: [text](url) or ![alt](url)
        // Bump past ( and lex the following tokens in LinkDefinition context
        // so whitespace separates destination and title.
        p.expect_with_context(L_PAREN, crate::lexer::MarkdownLexContext::LinkDefinition);

        let destination = p.start();
        parse_inline_link_destination_tokens(p);
        let has_title = inline_title_starts_after_whitespace_tokens(p);
        while is_whitespace_token(p) {
            bump_textual_link_def(p);
        }
        destination.complete(p, MD_INLINE_ITEM_LIST);

        if has_title {
            let title_m = p.start();
            let list_m = p.start();
            parse_title_content(p, get_title_close_char(p));
            list_m.complete(p, MD_INLINE_ITEM_LIST);
            title_m.complete(p, MD_LINK_TITLE);
        }

        if !p.eat(R_PAREN) {
            kind.report_unclosed_destination(p, opening_range);
        }

        Present(m.complete(p, kind.inline_kind()))
    } else if p.at(L_BRACK) {
        // Reference link/image: [text][label] or [text][]
        let label = parse_reference_label(p);
        let reference = reference.filter(|reference| {
            if label.is_absent() {
                reference.is_shortcut
            } else {
                true
            }
        });

        if let Some(reference) = reference
            && !reference.is_defined(p)
        {
            m.abandon(p);
            p.rewind(checkpoint);
            return consume_textual_until_offset(p, reference.end_offset);
        }

        Present(m.complete(p, kind.reference_kind()))
    } else {
        // Shortcut reference: [text] or ![alt]
        // No label part - the text/alt IS the label for resolution
        if let Some(reference) = reference
            && reference.is_shortcut
            && !reference.is_defined(p)
        {
            m.abandon(p);
            p.rewind(checkpoint);
            return consume_textual_until_offset(p, reference.end_offset);
        }
        Present(m.complete(p, kind.reference_kind()))
    }
}

struct ReferenceLinkLookahead {
    end_offset: TextSize,
    label_raw: String,
    is_shortcut: bool,
}

impl ReferenceLinkLookahead {
    fn is_defined(&self, p: &MarkdownParser) -> bool {
        let normalized = normalize_reference_label(&self.label_raw);
        p.has_link_reference_definition(&normalized)
    }
}

fn lookahead_reference_link(p: &mut MarkdownParser) -> Option<ReferenceLinkLookahead> {
    lookahead_reference_common(p, false)
}

fn lookahead_reference_image(p: &mut MarkdownParser) -> Option<ReferenceLinkLookahead> {
    lookahead_reference_common(p, true)
}

fn lookahead_reference_common(
    p: &mut MarkdownParser,
    is_image: bool,
) -> Option<ReferenceLinkLookahead> {
    p.lookahead(|p| {
        if is_image {
            if !p.at(BANG) || !p.nth_at(1, L_BRACK) {
                return None;
            }
            p.bump(BANG);
        }

        if !p.at(L_BRACK) {
            return None;
        }

        p.bump(L_BRACK);

        let link_text = collect_bracket_text(p)?;
        let end_offset = p.cur_range().end();
        p.bump(R_BRACK);

        if p.at(L_PAREN) {
            return None;
        }

        if p.at(L_BRACK) {
            p.bump(L_BRACK);
            let label_text = collect_bracket_text(p);
            if let Some(label_text) = label_text {
                let label = if label_text.is_empty() {
                    link_text.clone()
                } else {
                    label_text
                };
                let end_offset = p.cur_range().end();
                p.bump(R_BRACK);
                return Some(ReferenceLinkLookahead {
                    end_offset,
                    label_raw: label,
                    is_shortcut: false,
                });
            }
        }

        Some(ReferenceLinkLookahead {
            end_offset,
            label_raw: link_text,
            is_shortcut: true,
        })
    })
}

fn collect_bracket_text(p: &mut MarkdownParser) -> Option<String> {
    let mut text = String::new();
    loop {
        if p.at(T![EOF]) || p.at_inline_end() {
            return None;
        }

        if p.at(R_BRACK) {
            return Some(text);
        }

        text.push_str(p.cur_text());
        p.bump(p.cur());
    }
}

fn consume_textual_until_offset(p: &mut MarkdownParser, end_offset: TextSize) -> ParsedSyntax {
    let mut last = Absent;

    while !p.at(T![EOF]) {
        let end = p.cur_range().end();
        last = super::parse_textual(p);
        if end >= end_offset {
            break;
        }
    }

    last
}

fn bump_textual_link_def(p: &mut MarkdownParser) {
    use crate::lexer::MarkdownLexContext;

    let item = p.start();
    p.bump_remap_with_context(MD_TEXTUAL_LITERAL, MarkdownLexContext::LinkDefinition);
    item.complete(p, MD_TEXTUAL);
}
fn is_whitespace_token(p: &MarkdownParser) -> bool {
    let text = p.cur_text();
    !text.is_empty() && text.chars().all(|c| c == ' ' || c == '\t')
}

fn inline_title_starts_after_whitespace_tokens(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        while is_whitespace_token(p) {
            bump_textual_link_def(p);
        }
        get_title_close_char(p).is_some()
    })
}

fn parse_inline_link_destination_tokens(p: &mut MarkdownParser) {
    p.re_lex_link_definition();

    if p.at(L_ANGLE) {
        bump_textual_link_def(p);
        while !p.at(EOF) && !p.at(NEWLINE) {
            if p.at(R_ANGLE) {
                bump_textual_link_def(p);
                break;
            }
            if is_whitespace_token(p) {
                break;
            }
            bump_textual_link_def(p);
        }
        return;
    }

    let mut paren_depth: i32 = 0;
    while !p.at(EOF) && !p.at(NEWLINE) {
        if is_whitespace_token(p) {
            break;
        }

        if p.at(L_PAREN) {
            paren_depth += 1;
        } else if p.at(R_PAREN) {
            if paren_depth == 0 {
                break;
            }
            paren_depth -= 1;
        }

        bump_textual_link_def(p);
    }
}

fn get_title_close_char(p: &MarkdownParser) -> Option<char> {
    let text = p.cur_text();
    if text.starts_with('"') {
        Some('"')
    } else if text.starts_with('\'') {
        Some('\'')
    } else if p.at(L_PAREN) {
        Some(')')
    } else {
        None
    }
}

fn parse_title_content(p: &mut MarkdownParser, close_char: Option<char>) {
    let Some(close_char) = close_char else {
        return;
    };

    let text = p.cur_text();
    let is_complete = text.len() >= 2
        && ((close_char == ')' && text.ends_with(')'))
            || (close_char != ')' && text.ends_with(close_char)));

    bump_textual_link_def(p);
    if is_complete {
        return;
    }

    loop {
        if p.at(EOF) || p.at(NEWLINE) {
            return;
        }

        let text = p.cur_text();
        if text.ends_with(close_char) {
            bump_textual_link_def(p);
            return;
        }

        bump_textual_link_def(p);
    }
}

/// Parse inline image (`![alt](url)`).
///
/// Grammar: `MdInlineImage = '!' '[' alt: MdInlineItemList ']' '(' source: MdInlineItemList ')'`
///
/// Note: This is kept for backwards compatibility but `parse_image_or_reference`
/// is the preferred entry point for image parsing.
pub(crate) fn parse_inline_image(p: &mut MarkdownParser) -> ParsedSyntax {
    parse_image_or_reference(p)
}

/// Check if text starting with `<` is valid inline HTML per CommonMark §6.8.
/// Returns the length of the HTML element if valid, None otherwise.
///
/// Valid patterns:
/// - Open tags: `<tagname>`, `<tagname attr="value">`, `<tagname />`
/// - Close tags: `</tagname>`
/// - Comments: `<!-- ... -->`
/// - Processing instructions: `<? ... ?>`
/// - Declarations: `<! ... >`
/// - CDATA: `<![CDATA[ ... ]]>`
fn is_inline_html(text: &str) -> Option<usize> {
    let bytes = text.as_bytes();
    if bytes.len() < 2 || bytes[0] != b'<' {
        return None;
    }

    // HTML comment: <!-- ... -->
    if bytes.starts_with(b"<!--") {
        // Find closing -->
        if let Some(pos) = text[4..].find("-->") {
            let body = &text[4..4 + pos];
            // CommonMark: comment cannot start with '>' or '->', and must not contain "--"
            if body.starts_with('>') || body.starts_with("->") || body.contains("--") {
                return None;
            }
            return Some(4 + pos + 3);
        }
        return None;
    }

    // Processing instruction: <? ... ?>
    if bytes.len() >= 2 && bytes[1] == b'?' {
        // Find closing ?>
        if let Some(pos) = text[2..].find("?>") {
            return Some(2 + pos + 2);
        }
        return None;
    }

    // CDATA section: <![CDATA[ ... ]]>
    if bytes.starts_with(b"<![CDATA[") {
        // Find closing ]]>
        if let Some(pos) = text[9..].find("]]>") {
            return Some(9 + pos + 3);
        }
        return None;
    }

    // Declaration: <! followed by uppercase letter, then content until >
    // e.g., <!DOCTYPE html>
    if bytes.len() >= 3 && bytes[1] == b'!' && bytes[2].is_ascii_alphabetic() {
        // Find closing >
        if let Some(pos) = text[2..].find('>') {
            return Some(2 + pos + 1);
        }
        return None;
    }

    // Close tag: </tagname>
    if bytes.len() >= 4 && bytes[1] == b'/' {
        if !bytes[2].is_ascii_alphabetic() {
            return None;
        }
        // Tag name: [A-Za-z][A-Za-z0-9-]*
        let mut i = 3;
        while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'-') {
            i += 1;
        }
        // Skip optional whitespace
        while i < bytes.len()
            && (bytes[i] == b' '
                || bytes[i] == b'\t'
                || bytes[i] == b'\n'
                || bytes[i] == b'\r'
                || bytes[i] == b'\x0c')
        {
            i += 1;
        }
        // Must end with >
        if i < bytes.len() && bytes[i] == b'>' {
            return Some(i + 1);
        }
        return None;
    }

    // Open tag: <tagname ...> or <tagname ... />
    // Defensive bounds check - should be guaranteed by earlier len check but be explicit
    if bytes.len() < 2 || !bytes[1].is_ascii_alphabetic() {
        return None;
    }

    // Tag name: [A-Za-z][A-Za-z0-9-]*
    // Note: tag names cannot contain `.` (so <example.com> is NOT a valid tag)
    let mut i = 2;
    while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'-') {
        i += 1;
    }

    // After tag name, must have valid boundary: whitespace, >, or /
    // This prevents <example.com> from being treated as HTML
    if i >= bytes.len() {
        return None;
    }
    let boundary = bytes[i];
    if boundary != b' '
        && boundary != b'\t'
        && boundary != b'\n'
        && boundary != b'\r'
        && boundary != b'\x0c'
        && boundary != b'>'
        && boundary != b'/'
    {
        return None;
    }

    // Handle immediate close or self-close
    if boundary == b'>' {
        return Some(i + 1);
    }
    if boundary == b'/' {
        if i + 1 < bytes.len() && bytes[i + 1] == b'>' {
            return Some(i + 2);
        }
        return None;
    }

    // Has attributes - validate per CommonMark §6.8

    let skip_spaces = |i: &mut usize| -> Option<bool> {
        let mut skipped = false;
        while *i < bytes.len() {
            match bytes[*i] {
                b' ' | b'\t' | b'\n' | b'\r' | b'\x0c' => {
                    skipped = true;
                    *i += 1;
                }
                _ => break,
            }
        }
        Some(skipped)
    };

    let is_attr_name_start = |b: u8| b.is_ascii_alphabetic() || b == b'_' || b == b':';
    let is_attr_name_continue =
        |b: u8| b.is_ascii_alphanumeric() || b == b'_' || b == b':' || b == b'.' || b == b'-';

    loop {
        let had_space = skip_spaces(&mut i)?;
        if i >= bytes.len() {
            return None;
        }

        // End or self-close
        if bytes[i] == b'>' {
            return Some(i + 1);
        }
        if bytes[i] == b'/' {
            if i + 1 < bytes.len() && bytes[i + 1] == b'>' {
                return Some(i + 2);
            }
            return None;
        }

        // Attributes must be separated by whitespace
        if !had_space {
            return None;
        }

        // Parse attribute name
        if !is_attr_name_start(bytes[i]) {
            return None;
        }
        i += 1;
        while i < bytes.len() && is_attr_name_continue(bytes[i]) {
            i += 1;
        }

        // Optional whitespace and value
        skip_spaces(&mut i)?;
        if i < bytes.len() && bytes[i] == b'=' {
            i += 1;
            skip_spaces(&mut i)?;
            if i >= bytes.len() {
                return None;
            }

            match bytes[i] {
                b'"' => {
                    i += 1;
                    while i < bytes.len() && bytes[i] != b'"' {
                        i += 1;
                    }
                    if i >= bytes.len() {
                        return None;
                    }
                    i += 1;
                }
                b'\'' => {
                    i += 1;
                    while i < bytes.len() && bytes[i] != b'\'' {
                        i += 1;
                    }
                    if i >= bytes.len() {
                        return None;
                    }
                    i += 1;
                }
                _ => {
                    let start = i;
                    while i < bytes.len() {
                        let b = bytes[i];
                        if b <= b' '
                            || b == b'"'
                            || b == b'\''
                            || b == b'='
                            || b == b'<'
                            || b == b'>'
                            || b == b'`'
                        {
                            break;
                        }
                        i += 1;
                    }
                    if i == start {
                        return None;
                    }
                }
            }
        }
    }
}

/// Parse entity or numeric character reference per CommonMark §6.2.
///
/// Grammar: MdEntityReference = value: 'md_entity_literal'
///
/// Valid patterns:
/// - Named entity: `&name;` where name is 2-31 alphanumeric chars starting with letter
/// - Decimal numeric: `&#digits;` where digits is 1-7 decimal digits
/// - Hexadecimal: `&#xhex;` or `&#Xhex;` where hex is 1-6 hex digits
///
/// The lexer has already validated and tokenized valid entity references as
/// MD_ENTITY_LITERAL tokens. Invalid patterns remain as textual.
pub(crate) fn parse_entity_reference(p: &mut MarkdownParser) -> ParsedSyntax {
    if !p.at(MD_ENTITY_LITERAL) {
        return Absent;
    }

    let m = p.start();
    p.bump(MD_ENTITY_LITERAL);
    Present(m.complete(p, MD_ENTITY_REFERENCE))
}

/// Parse raw inline HTML per CommonMark §6.8.
///
/// Grammar: MdInlineHtml = value: MdInlineItemList
///
/// Includes: open tags, close tags, comments, processing instructions,
/// declarations, and CDATA sections.
pub(crate) fn parse_inline_html(p: &mut MarkdownParser) -> ParsedSyntax {
    if !p.at(L_ANGLE) {
        return Absent;
    }

    // Get the source text starting from current position
    let source = p.source_after_current();

    // Check if this is valid inline HTML
    let html_len = match is_inline_html(source) {
        Some(len) => len,
        None => return Absent,
    };

    // Valid inline HTML - create the node
    // Use checkpoint so we can rewind if token boundaries don't align
    let checkpoint = p.checkpoint();
    let m = p.start();

    // Create content as inline item list containing textual nodes
    let content = p.start();

    // Track remaining bytes to consume
    let mut remaining = html_len;

    while remaining > 0 && !p.at(T![EOF]) {
        let token_len = p.cur_text().len();

        // If the current token is larger than remaining bytes, token boundaries
        // don't align with our validated HTML - rewind and treat as text
        if token_len > remaining {
            m.abandon(p);
            p.rewind(checkpoint);
            return Absent;
        }

        let text_m = p.start();
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);
        remaining -= token_len;
    }

    content.complete(p, MD_INLINE_ITEM_LIST);

    Present(m.complete(p, MD_INLINE_HTML))
}

/// Check if the text after `<` looks like a URI autolink.
/// Per CommonMark §6.4: scheme must be 2-32 chars, start with letter,
/// followed by letters/digits/+/-/., then `:`.
fn is_uri_autolink(text: &str) -> bool {
    let bytes = text.as_bytes();
    if bytes.is_empty() {
        return false;
    }

    // Must start with a letter
    if !bytes[0].is_ascii_alphabetic() {
        return false;
    }

    // Find the colon
    let mut colon_pos = None;
    for (i, &b) in bytes.iter().enumerate().skip(1) {
        if b == b':' {
            colon_pos = Some(i);
            break;
        }
        // Scheme chars: letters, digits, +, -, .
        if !b.is_ascii_alphanumeric() && b != b'+' && b != b'-' && b != b'.' {
            return false;
        }
    }

    // Scheme must be 2-32 chars and followed by colon
    match colon_pos {
        Some(pos) if (2..=32).contains(&pos) => {
            // Must have content after the colon and no whitespace/< in URI
            let rest = &text[pos + 1..];
            !rest.is_empty()
                && !rest.contains('<')
                && !rest.contains('>')
                && !rest.chars().any(|c| c.is_whitespace())
        }
        _ => false,
    }
}

/// Check if the text after `<` looks like an email autolink.
/// Per CommonMark §6.5: local@domain pattern with specific char restrictions.
fn is_email_autolink(text: &str) -> bool {
    // Must contain exactly one @ not at start or end
    let at_pos = match text.find('@') {
        Some(pos) if pos > 0 && pos < text.len() - 1 => pos,
        _ => return false,
    };

    // Check no second @
    if text[at_pos + 1..].contains('@') {
        return false;
    }

    // Local part: alphanumerics and .!#$%&'*+/=?^_`{|}~-
    let local = &text[..at_pos];
    for c in local.chars() {
        if !c.is_ascii_alphanumeric()
            && !matches!(
                c,
                '.' | '!'
                    | '#'
                    | '$'
                    | '%'
                    | '&'
                    | '\''
                    | '*'
                    | '+'
                    | '/'
                    | '='
                    | '?'
                    | '^'
                    | '_'
                    | '`'
                    | '{'
                    | '|'
                    | '}'
                    | '~'
                    | '-'
            )
        {
            return false;
        }
    }

    // Domain part: alphanumerics and hyphens, dots for subdomains
    let domain = &text[at_pos + 1..];
    if domain.is_empty() || domain.starts_with('.') || domain.ends_with('.') {
        return false;
    }

    for c in domain.chars() {
        if !c.is_ascii_alphanumeric() && c != '-' && c != '.' {
            return false;
        }
    }

    true
}

/// Parse an autolink (`<https://example.com>` or `<user@example.com>`).
///
/// Grammar: MdAutolink = '<' value: MdInlineItemList '>'
///
/// Per CommonMark §6.4 and §6.5, autolinks are URIs or email addresses
/// wrapped in angle brackets.
pub(crate) fn parse_autolink(p: &mut MarkdownParser) -> ParsedSyntax {
    if !p.at(L_ANGLE) {
        return Absent;
    }

    // Look ahead to find the closing > and check if content is valid
    let source = p.source_after_current();

    // Skip the < and find >
    let after_open = &source[1..];
    let close_pos = match after_open.find('>') {
        Some(pos) => pos,
        None => return Absent, // No closing >
    };

    // Check for newline before > (not allowed in autolinks)
    let content = &after_open[..close_pos];
    if content.contains('\n') || content.contains('\r') {
        return Absent;
    }

    // Must be either URI or email autolink
    if !is_uri_autolink(content) && !is_email_autolink(content) {
        return Absent;
    }

    // Valid autolink - parse it
    let m = p.start();

    // <
    p.bump(L_ANGLE);

    // Content as inline item list containing textual nodes
    let content = p.start();
    while !p.at(R_ANGLE) && !p.at_inline_end() {
        let text_m = p.start();
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);
    }
    content.complete(p, MD_INLINE_ITEM_LIST);

    // >
    p.expect(R_ANGLE);

    Present(m.complete(p, MD_AUTOLINK))
}

/// Dispatch to the appropriate inline parser based on current token.
pub(crate) fn parse_any_inline(p: &mut MarkdownParser) -> ParsedSyntax {
    if p.at(MD_HARD_LINE_LITERAL) {
        parse_hard_line(p)
    } else if p.at(BACKTICK) {
        parse_inline_code(p)
    } else if p.at(DOUBLE_STAR) || p.at(DOUBLE_UNDERSCORE) {
        // Try emphasis, fall back to literal text if flanking rules fail
        let result = parse_inline_emphasis(p);
        if result.is_absent() {
            super::parse_textual(p)
        } else {
            result
        }
    } else if p.at(T![*]) || p.at(UNDERSCORE) {
        // Try italic, fall back to literal text if flanking rules fail
        let result = parse_inline_italic(p);
        if result.is_absent() {
            super::parse_textual(p)
        } else {
            result
        }
    } else if p.at(BANG) && p.nth_at(1, L_BRACK) {
        // Try image, fall back to literal text if parsing fails
        let result = parse_inline_image(p);
        if result.is_absent() {
            super::parse_textual(p)
        } else {
            result
        }
    } else if p.at(L_BRACK) {
        // Try link, fall back to literal text if parsing fails
        let result = parse_inline_link(p);
        if result.is_absent() {
            super::parse_textual(p)
        } else {
            result
        }
    } else if p.at(L_ANGLE) {
        // Try autolink first (takes priority per CommonMark)
        let result = parse_autolink(p);
        if result.is_present() {
            return result;
        }
        // Then try inline HTML
        let result = parse_inline_html(p);
        if result.is_present() {
            return result;
        }
        // Fall back to textual
        super::parse_textual(p)
    } else if p.at(MD_ENTITY_LITERAL) {
        // Entity or numeric character reference (already validated by lexer)
        parse_entity_reference(p)
    } else {
        super::parse_textual(p)
    }
}
