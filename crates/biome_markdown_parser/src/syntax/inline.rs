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
use biome_unicode_table::is_unicode_punctuation;

use biome_rowan::TextRange;

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
    /// Bracket nesting depth for scoping emphasis within link text.
    /// Delimiters inside brackets (links) should only match with each other,
    /// not with delimiters outside the brackets. 0 = outside brackets.
    label_id: usize,
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

fn is_emphasis_marker(c: char) -> bool {
    matches!(c, '*' | '_')
}

/// Check if a character is Unicode punctuation for flanking rules.
/// Per CommonMark spec, this includes ASCII punctuation and Unicode punctuation categories.
fn is_punctuation(c: char) -> bool {
    is_unicode_punctuation(c)
}

/// Check if an opening delimiter is left-flanking per CommonMark rules.
/// A left-flanking delimiter run is one that is:
/// - Not followed by Unicode whitespace, AND
/// - Either (a) not followed by punctuation, OR (b) preceded by whitespace/punctuation
fn is_left_flanking_delimiter(char_after: Option<char>, char_before: Option<char>) -> bool {
    match char_after {
        None => false,                        // At end of input, can't be left-flanking
        Some(c) if is_whitespace(c) => false, // Followed by whitespace
        Some(c) if is_emphasis_marker(c) => true,
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
        Some(c) if is_emphasis_marker(c) => true,
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
/// Result of checking if a bracket forms a valid link.
/// Contains the closing bracket position if found.
struct BracketCheckResult {
    /// Position of the closing `]` (or 0 if not found)
    close_pos: usize,
    /// Whether this is a valid inline link `[...](` or full reference `[...][`
    is_inline_or_full_ref: bool,
}

/// Check if a bracket at position `start` forms a valid link pattern.
/// Returns the closing bracket position and whether it's an inline link or full reference.
fn check_bracket_pattern(bytes: &[u8], start: usize) -> Option<BracketCheckResult> {
    if start >= bytes.len() || bytes[start] != b'[' {
        return None;
    }

    // Find matching ] with proper nesting
    let mut depth = 1;
    let mut i = start + 1;
    while i < bytes.len() && depth > 0 {
        match bytes[i] {
            b'[' => depth += 1,
            b']' => depth -= 1,
            b'\\' if i + 1 < bytes.len() => i += 1, // Skip escaped char
            b'`' => {
                // Skip code spans
                let backtick_count = {
                    let mut c = 1;
                    while i + c < bytes.len() && bytes[i + c] == b'`' {
                        c += 1;
                    }
                    c
                };
                i += backtick_count;
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
                continue;
            }
            b'<' => {
                // Skip potential HTML/autolinks
                i += 1;
                while i < bytes.len() && bytes[i] != b'>' && bytes[i] != b'\n' {
                    i += 1;
                }
                if i < bytes.len() && bytes[i] == b'>' {
                    i += 1;
                }
                continue;
            }
            _ => {}
        }
        i += 1;
    }

    if depth != 0 {
        return None;
    }

    // i now points to position after `]`
    let close_pos = i - 1;
    let is_inline_or_full_ref = i < bytes.len() && (bytes[i] == b'(' || bytes[i] == b'[');

    Some(BracketCheckResult {
        close_pos,
        is_inline_or_full_ref,
    })
}

/// Extract label text from a bracket pattern for reference lookup.
fn extract_label_text(source: &str, start: usize, close_pos: usize) -> &str {
    if start < close_pos && close_pos <= source.len() {
        &source[start + 1..close_pos]
    } else {
        ""
    }
}

fn collect_delimiter_runs(source: &str, reference_checker: impl Fn(&str) -> bool) -> Vec<DelimRun> {
    let mut runs = Vec::new();
    let bytes = source.as_bytes();
    let mut i = 0;

    // Pre-compute valid link bracket positions.
    // A bracket is considered a valid link if:
    // 1. It's followed by `(` (inline link) or `[` (full reference), OR
    // 2. It's a shortcut reference with a defined reference (checked via reference_checker)
    let mut link_bracket_starts = Vec::new();
    for pos in 0..bytes.len() {
        if bytes[pos] == b'['
            && let Some(result) = check_bracket_pattern(bytes, pos)
        {
            if result.is_inline_or_full_ref {
                // Inline link or full reference link
                link_bracket_starts.push(pos);
            } else {
                // Could be a shortcut reference - check if definition exists
                let label = extract_label_text(source, pos, result.close_pos);
                let normalized = normalize_reference_label(label);
                if !normalized.is_empty() && reference_checker(&normalized) {
                    link_bracket_starts.push(pos);
                }
            }
        }
    }

    // Track bracket depth, but only for valid link brackets
    let mut bracket_depth = 0usize;
    let mut active_link_brackets: Vec<usize> = Vec::new();

    while i < bytes.len() {
        let b = bytes[i];

        // Track bracket depth for valid links only
        if b == b'[' && link_bracket_starts.contains(&i) {
            bracket_depth += 1;
            active_link_brackets.push(i);
            i += 1;
            continue;
        }
        if b == b']' && !active_link_brackets.is_empty() {
            bracket_depth = bracket_depth.saturating_sub(1);
            active_link_brackets.pop();
            i += 1;
            continue;
        }

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
                // Only scope by bracket depth when inside a valid link pattern.
                // This prevents emphasis from spanning link boundaries, but allows
                // emphasis to span brackets that don't form valid links.
                label_id: bracket_depth,
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

                // Search backward for the closest matching opener.
                // Per CommonMark spec, we find any matching opener first,
                // then determine strong vs regular based on both counts.
                for (pos, &opener_idx) in opener_stack.iter().enumerate().rev() {
                    let opener = &runs[opener_idx];
                    let closer = &runs[idx];

                    // Only match within same bracket scope (label_id).
                    // This prevents emphasis from spanning link boundaries.
                    if opener.label_id != closer.label_id {
                        continue;
                    }

                    if opener.kind != closer.kind || !opener.can_open || opener.count == 0 {
                        continue;
                    }

                    // Rule of 3: if (opener_count + closer_count) % 3 == 0 and
                    // the closer can open or the opener can close, skip unless
                    // both counts are divisible by 3
                    let opener_count = opener.count;
                    let closer_count = closer.count;
                    if (opener.can_close || closer.can_open)
                        && !closer_count.is_multiple_of(3)
                        && (opener_count + closer_count).is_multiple_of(3)
                    {
                        continue;
                    }

                    opener_stack_pos = Some(pos);
                    break;
                }

                let Some(pos) = opener_stack_pos else { break };
                let opener_idx = opener_stack[pos];
                let use_count = if runs[opener_idx].count >= 2 && runs[idx].count >= 2 {
                    2
                } else {
                    1
                };

                // Openers consume from END of run (leftover stays at beginning).
                // This ensures for `***foo***`, the inner `**` is consumed leaving `*` at start.
                let opener_start =
                    runs[opener_idx].start_offset + runs[opener_idx].count - use_count;
                // Closers consume from BEGINNING of what remains.
                let closer_start = runs[idx].start_offset;

                matches.push(EmphasisMatch {
                    opener_start,
                    closer_start,
                    is_strong: use_count == 2,
                });

                // Opener: reduce count but keep start_offset (leftover is at beginning)
                runs[opener_idx].count -= use_count;
                // Closer: reduce count and advance start_offset (leftover is at end)
                runs[idx].count -= use_count;
                runs[idx].start_offset += use_count;

                // Remove openers between the matched opener and this closer.
                opener_stack.truncate(pos + 1);
                if runs[opener_idx].count == 0 {
                    opener_stack.pop();
                }

                // Note: With the "consume from END" algorithm for openers,
                // crossing matches are no longer an issue because the leftover
                // chars end up at the beginning of the opener run (wrapping
                // around the inner match), not at the end (which would cross).

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

/// Information about a match found within a token's range.
/// Used when the opener doesn't start at the exact token boundary.
#[derive(Debug)]
struct OpenerMatch<'a> {
    /// The matched emphasis span
    matched: &'a EmphasisMatch,
    /// How many chars before opener_start (literal prefix to emit)
    prefix_len: usize,
}

impl EmphasisContext {
    /// Create a new emphasis context by analyzing the source text.
    /// The reference_checker function is used to determine if a bracket pattern
    /// is a valid shortcut reference link.
    pub(crate) fn new(
        source: &str,
        base_offset: usize,
        reference_checker: impl Fn(&str) -> bool,
    ) -> Self {
        let mut runs = collect_delimiter_runs(source, reference_checker);
        let matches = match_delimiters(&mut runs);
        Self {
            matches,
            base_offset,
        }
    }

    /// Find the *earliest* match whose opener_start is within [token_start, token_end)
    /// and matches the expected `is_strong` value.
    /// Returns None if no match found, or the match plus prefix length.
    ///
    /// This is used instead of exact offset matching because with the "consume from END"
    /// algorithm, an opener might start in the middle of a DOUBLE_STAR token.
    fn opener_within(
        &self,
        token_start: usize,
        token_len: usize,
        expect_strong: bool,
    ) -> Option<OpenerMatch<'_>> {
        let token_end = token_start + token_len;
        let mut best: Option<OpenerMatch<'_>> = None;

        for m in &self.matches {
            // Filter by expected emphasis type
            if m.is_strong != expect_strong {
                continue;
            }

            let abs_opener = m.opener_start + self.base_offset;
            if abs_opener >= token_start && abs_opener < token_end {
                let candidate = OpenerMatch {
                    matched: m,
                    prefix_len: abs_opener - token_start,
                };
                // Pick the earliest match (smallest prefix_len)
                if best
                    .as_ref()
                    .is_none_or(|b| candidate.prefix_len < b.prefix_len)
                {
                    best = Some(candidate);
                }
            }
        }

        best
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

    let ends_block = p.lookahead(|p| {
        p.bump(MD_HARD_LINE_LITERAL);
        p.at(NEWLINE) || p.at(EOF)
    });

    if ends_block {
        return super::parse_textual(p);
    }

    let m = p.start();
    p.bump(MD_HARD_LINE_LITERAL);
    Present(m.complete(p, MD_HARD_LINE))
}

/// Check if there's a matching closing backtick sequence before EOF/blank line.
///
/// Per CommonMark §6.1, a code span opener must have a matching closer with the
/// same number of backticks. If no match exists, the opener should be treated
/// as literal text, not an unclosed code span.
///
/// Returns false if no match found (opener should become literal text).

fn has_matching_code_span_closer(p: &mut MarkdownParser, opening_count: usize) -> bool {
    use crate::lexer::MarkdownLexContext;

    p.lookahead(|p| {
        // Skip the opening backticks (handle both BACKTICK and TRIPLE_BACKTICK)
        if p.at(T!["```"]) {
            p.bump(T!["```"]);
        } else {
            p.bump(BACKTICK);
        }

        loop {
            // EOF = no matching closer found
            if p.at(T![EOF]) {
                return false;
            }

            // Blank line = paragraph boundary, terminates search
            if p.at(NEWLINE) && p.at_blank_line() {
                return false;
            }

            // Per CommonMark §4.3, setext heading underlines take priority over
            // inline code spans. If crossing a newline would land on a setext
            // underline, the code span is invalid — the underline forms a heading.
            if p.at(NEWLINE) {
                p.bump_remap_with_context(MD_TEXTUAL_LITERAL, MarkdownLexContext::CodeSpan);
                if crate::syntax::at_setext_underline_after_newline(p).is_some() {
                    return false;
                }
                continue;
            }

            // Found backticks - check if they match (handle both BACKTICK and TRIPLE_BACKTICK)
            if p.at(BACKTICK) || p.at(T!["```"]) {
                let closing_count = p.cur_text().len();
                if closing_count == opening_count {
                    return true;
                }
                // Not matching - continue searching
                if p.at(T!["```"]) {
                    p.bump(T!["```"]);
                } else {
                    p.bump(BACKTICK);
                }
                continue;
            }

            // Consume token and continue (use CodeSpan context for proper backslash handling)
            p.bump_remap_with_context(MD_TEXTUAL_LITERAL, MarkdownLexContext::CodeSpan);
        }
    })
}

/// Parse inline code span (`` `code` `` or ``` `` `code` `` ```).
///
/// Grammar: MdInlineCode = l_tick: '`' content: MdInlineItemList r_tick: '`'
///
/// Per CommonMark §6.1:
/// - Code spans can use multiple backticks to allow literal backticks inside
/// - The opening and closing backtick strings must be the same length
/// - Backslash escapes are NOT processed inside code spans (\` is literal `\``)
/// - If no matching closer exists, the opener is treated as literal text
pub(crate) fn parse_inline_code(p: &mut MarkdownParser) -> ParsedSyntax {
    use crate::lexer::MarkdownLexContext;

    // Handle both BACKTICK and TRIPLE_BACKTICK (T!["```"]) as code span openers.
    // TRIPLE_BACKTICK can appear when backticks are at line start but info string
    // contains backticks, making it not a fenced code block (CommonMark examples 138, 145).
    let is_backtick = p.at(BACKTICK);
    let is_triple_backtick = p.at(T!["```"]);
    if !is_backtick && !is_triple_backtick {
        return Absent;
    }

    let opening_count = p.cur_text().len();

    // DESIGN PRINCIPLE #2 & #4: Check for matching closer BEFORE creating any nodes.
    // If no match exists, return Absent so backticks become literal text.
    // This avoids synthesizing MD_INLINE_CODE with missing r_tick_token.
    if !has_matching_code_span_closer(p, opening_count) {
        return Absent; // Caller will treat backtick as literal MD_TEXTUAL
    }

    // We have a valid code span - now parse it
    let m = p.start();

    // Opening backtick(s) - remap TRIPLE_BACKTICK to BACKTICK for consistency
    if is_triple_backtick {
        p.bump_remap(BACKTICK);
    } else {
        p.bump(BACKTICK);
    }

    // Content - parse until we find matching closing backticks
    // Per CommonMark, code spans can span multiple lines (newlines become spaces in output)
    // All content is lexed in CodeSpan context to keep backslash literal and avoid
    // hard-line-break detection.
    let content = p.start();
    loop {
        // EOF should not happen (lookahead guaranteed a closer), but handle defensively
        if p.at(T![EOF]) {
            break;
        }

        // DESIGN PRINCIPLE #3: Terminate on blank line (paragraph boundary)
        if p.at(NEWLINE) {
            if p.at_blank_line() {
                break; // Paragraph boundary - stop
            }
            // Soft line break - consume NEWLINE as content and continue
            // Use CodeSpan context so next token is also lexed without escape processing
            let text_m = p.start();
            p.bump_remap_with_context(MD_TEXTUAL_LITERAL, MarkdownLexContext::CodeSpan);
            text_m.complete(p, MD_TEXTUAL);
            continue;
        }

        // Found matching closing backticks (handle both BACKTICK and TRIPLE_BACKTICK)
        if (p.at(BACKTICK) || p.at(T!["```"])) && p.cur_text().len() == opening_count {
            break;
        }

        // DESIGN PRINCIPLE #1: Use CodeSpan context so backslash is literal
        let text_m = p.start();
        p.bump_remap_with_context(MD_TEXTUAL_LITERAL, MarkdownLexContext::CodeSpan);
        text_m.complete(p, MD_TEXTUAL);
    }
    content.complete(p, MD_INLINE_ITEM_LIST);

    // Closing backticks (guaranteed to exist due to lookahead check)
    // Remap TRIPLE_BACKTICK to BACKTICK for consistency
    if p.at(T!["```"]) {
        p.bump_remap(BACKTICK);
    } else {
        p.bump(BACKTICK);
    }

    Present(m.complete(p, MD_INLINE_CODE))
}

/// Parse emphasis using the delimiter stack matches.
fn parse_emphasis_from_context(p: &mut MarkdownParser, expect_strong: bool) -> ParsedSyntax {
    let context = match p.emphasis_context() {
        Some(context) => context,
        None => return Absent,
    };

    // Must be at an emphasis token
    if !p.at(DOUBLE_STAR) && !p.at(DOUBLE_UNDERSCORE) && !p.at(T![*]) && !p.at(UNDERSCORE) {
        return Absent;
    }

    // Get current token info BEFORE any re-lex
    let token_start = u32::from(p.cur_range().start()) as usize;
    let token_len: usize = p.cur_range().len().into();

    // Find match within current token's range that has the expected is_strong value
    let opener_match = match context.opener_within(token_start, token_len, expect_strong) {
        Some(m) => m,
        None => return Absent,
    };

    // If the opener doesn't start at the exact token boundary, return Absent.
    // The caller (parse_any_inline) will emit literal text, advancing the parser position.
    // On subsequent calls, we'll eventually be at the correct position with prefix_len == 0.
    if opener_match.prefix_len > 0 {
        return Absent;
    }

    // Extract values before dropping the borrow on context
    let use_count = if expect_strong { 2 } else { 1 };
    let closer_offset = opener_match.matched.closer_start + context.base_offset;
    // Use the correct delimiter character for error messages
    let is_underscore = p.at(DOUBLE_UNDERSCORE) || p.at(UNDERSCORE);
    let opener_text = match (expect_strong, is_underscore) {
        (true, true) => "__",
        (true, false) => "**",
        (false, true) => "_",
        (false, false) => "*",
    };

    let m = p.start();
    let opening_range = p.cur_range();

    // Consume opener tokens
    // For strong emphasis (use_count=2), we can bump DOUBLE_* directly if at one.
    // Only re-lex when we need to consume a partial token or single chars.
    if use_count == 2 && (p.at(DOUBLE_STAR) || p.at(DOUBLE_UNDERSCORE)) {
        // Bump the double token as a single unit
        p.bump_any();
    } else {
        // Consume individual tokens
        for _ in 0..use_count {
            if p.at(DOUBLE_STAR) || p.at(DOUBLE_UNDERSCORE) {
                p.force_relex_emphasis_inline();
            }
            p.bump_any();
        }
    }

    // Parse content until we reach the closer
    let content = p.start();
    loop {
        // EOF always ends content
        if p.at(T![EOF]) {
            break;
        }

        let current_offset = u32::from(p.cur_range().start()) as usize;
        let current_len: usize = p.cur_range().len().into();

        // Check if closer is AT or WITHIN current token
        if closer_offset >= current_offset && closer_offset < current_offset + current_len {
            break;
        }

        // Check if we've passed the closer (can happen when link parsing consumes past it)
        if current_offset > closer_offset {
            break;
        }

        // Handle NEWLINE: emphasis can span multiple lines per CommonMark
        // But blank lines end paragraphs, so stop there
        if p.at(NEWLINE) {
            if p.at_blank_line() {
                // Blank line = paragraph boundary, emphasis is unclosed
                break;
            }
            if closer_offset > current_offset {
                // Soft line break - consume NEWLINE as textual content and continue
                let text_m = p.start();
                p.bump_remap(MD_TEXTUAL_LITERAL);
                text_m.complete(p, MD_TEXTUAL);
                continue;
            }
            // Closer should have been at or before this newline - stop
            break;
        }

        if parse_any_inline(p).is_absent() {
            break;
        }
    }
    content.complete(p, MD_INLINE_ITEM_LIST);

    // Consume closer tokens (1 or 2)
    // Handle partial closer consumption (e.g., `*foo**` where closer might be at offset 4
    // but token DOUBLE_STAR spans 4-6)
    let current_offset = u32::from(p.cur_range().start()) as usize;
    let closer_prefix_len = closer_offset.saturating_sub(current_offset);

    if closer_prefix_len > 0 {
        // Closer starts AFTER token start - emit prefix as literal
        if p.at(DOUBLE_STAR) || p.at(DOUBLE_UNDERSCORE) {
            p.force_relex_emphasis_inline();
        }
        for _ in 0..closer_prefix_len {
            let text_m = p.start();
            p.bump_remap(MD_TEXTUAL_LITERAL);
            text_m.complete(p, MD_TEXTUAL);
        }
    }

    // Now consume actual closer delimiters
    // For strong emphasis (use_count=2), we can bump DOUBLE_* directly if at one.
    let mut consumed_closer = 0;
    if use_count == 2 && (p.at(DOUBLE_STAR) || p.at(DOUBLE_UNDERSCORE)) {
        p.bump_any();
        consumed_closer = 2;
    } else {
        for _ in 0..use_count {
            if p.at(DOUBLE_STAR) || p.at(DOUBLE_UNDERSCORE) {
                p.force_relex_emphasis_inline();
            }
            if p.at(T![*]) || p.at(UNDERSCORE) || p.at(DOUBLE_STAR) || p.at(DOUBLE_UNDERSCORE) {
                p.bump_any();
                consumed_closer += 1;
            } else {
                break;
            }
        }
    }

    if consumed_closer < use_count {
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

fn parse_inline_item_list_until_no_links(p: &mut MarkdownParser, stop: MarkdownSyntaxKind) -> bool {
    let m = p.start();
    let prev_context = set_inline_emphasis_context_until(p, stop);
    let mut bracket_depth = 0usize;
    let mut has_nested_link = false;

    loop {
        // Per CommonMark, link text can span lines, but blank lines end the link.
        // Check for blank line (NEWLINE followed by NEWLINE or EOF after optional whitespace)
        if p.at(NEWLINE) {
            if p.at_blank_line() {
                break; // Blank line ends link text
            }
            // Single newline inside link text - consume and continue
            let _ = super::parse_textual(p);
            continue;
        }

        if p.at(T![EOF]) {
            break;
        }

        // IMPORTANT: Parse constructs that can contain `]` BEFORE checking for stop token.
        // Per CommonMark, `]` inside code spans, autolinks, and HTML doesn't terminate links.

        // Code spans can contain `]`
        if p.at(BACKTICK) {
            if parse_inline_code(p).is_present() {
                continue;
            }
            let _ = super::parse_textual(p);
            continue;
        }

        // Autolinks and inline HTML can contain `]`
        if p.at(L_ANGLE) {
            if parse_autolink(p).is_present() {
                continue;
            }
            if parse_inline_html(p).is_present() {
                continue;
            }
            let _ = super::parse_textual(p);
            continue;
        }

        // NOW check for stop token (after constructs that can contain it)
        if p.at(stop) {
            if bracket_depth == 0 {
                break;
            }
            bracket_depth = bracket_depth.saturating_sub(1);
            let _ = super::parse_textual(p);
            continue;
        }

        if p.at(L_BRACK) {
            if !has_nested_link && nested_link_starts_here(p) {
                has_nested_link = true;
            }
            bracket_depth += 1;
            let _ = super::parse_textual(p);
            continue;
        }

        if parse_any_inline_no_links(p).is_absent() {
            break;
        }
    }

    m.complete(p, MD_INLINE_ITEM_LIST);
    p.set_emphasis_context(prev_context);
    has_nested_link
}

/// Parse inline items until `stop` token, allowing full inline parsing including links.
/// Used for image alt text where nested links/images should be fully parsed
/// so their text content can be extracted for the alt attribute.
fn parse_inline_item_list_until(p: &mut MarkdownParser, stop: MarkdownSyntaxKind) {
    let m = p.start();
    let prev_context = set_inline_emphasis_context_until(p, stop);
    let mut bracket_depth = 0usize;

    loop {
        if p.at(NEWLINE) {
            if p.at_blank_line() {
                break;
            }
            let _ = super::parse_textual(p);
            continue;
        }

        if p.at(T![EOF]) {
            break;
        }

        // Code spans can contain `]`
        if p.at(BACKTICK) {
            if parse_inline_code(p).is_present() {
                continue;
            }
            let _ = super::parse_textual(p);
            continue;
        }

        // Autolinks and inline HTML can contain `]`
        if p.at(L_ANGLE) {
            if parse_autolink(p).is_present() {
                continue;
            }
            if parse_inline_html(p).is_present() {
                continue;
            }
            let _ = super::parse_textual(p);
            continue;
        }

        if p.at(stop) {
            if bracket_depth == 0 {
                break;
            }
            bracket_depth = bracket_depth.saturating_sub(1);
            let _ = super::parse_textual(p);
            continue;
        }

        // For image alt: allow full inline parsing including links and images
        if p.at(L_BRACK) {
            let result = parse_link_or_image(p, LinkParseKind::Link);
            if result.is_present() {
                continue;
            }
            bracket_depth += 1;
            let _ = super::parse_textual(p);
            continue;
        }

        if p.at(BANG) && p.nth_at(1, L_BRACK) {
            let result = parse_link_or_image(p, LinkParseKind::Image);
            if result.is_present() {
                continue;
            }
            let _ = super::parse_textual(p);
            continue;
        }

        if parse_any_inline(p).is_absent() {
            break;
        }
    }

    m.complete(p, MD_INLINE_ITEM_LIST);
    p.set_emphasis_context(prev_context);
}

fn nested_link_starts_here(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        if !p.at(L_BRACK) {
            return false;
        }

        p.bump(L_BRACK);
        let mut depth = 0usize;

        loop {
            if p.at(EOF) || p.at_inline_end() {
                return false;
            }

            if p.at(L_BRACK) {
                depth += 1;
                p.bump(L_BRACK);
                continue;
            }

            if p.at(R_BRACK) {
                if depth > 0 {
                    depth -= 1;
                    p.bump(R_BRACK);
                    continue;
                }
                p.bump(R_BRACK);
                return p.at(L_PAREN) || p.at(L_BRACK);
            }

            p.bump(p.cur());
        }
    })
}

fn parse_any_inline_no_links(p: &mut MarkdownParser) -> ParsedSyntax {
    if p.at(L_BRACK) {
        return super::parse_textual(p);
    }

    if p.at(BANG) && p.nth_at(1, L_BRACK) {
        return parse_inline_image(p);
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
    // Create a reference checker closure that uses the parser's link reference definitions
    let context = EmphasisContext::new(inline_source, base_offset, |label| {
        p.has_link_reference_definition(label)
    });
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
    let has_nested_link = if matches!(kind, LinkParseKind::Image) {
        // For images, allow full inline parsing (including links) in alt text.
        // This lets nested links/images be parsed so their text can be extracted for alt.
        parse_inline_item_list_until(p, R_BRACK);
        false
    } else {
        parse_inline_item_list_until_no_links(p, R_BRACK)
    };

    // ] - if missing, rewind and treat [ as literal text.
    // Per CommonMark, if there's no valid ] to close the link (e.g., all ]
    // characters are inside code spans or HTML), the [ is literal text.
    // NOTE: We intentionally do NOT emit an "unclosed link" diagnostic here.
    // CommonMark treats unmatched `[` as literal text, not an error.
    if !p.eat(R_BRACK) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    // Per CommonMark, a link (not image) whose text contains another link must fail.
    // The inner link wins and the outer `[` becomes literal text.
    if matches!(kind, LinkParseKind::Link) && has_nested_link {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    // Now decide based on what follows ]
    let link_validation = if p.at(L_PAREN) {
        inline_link_is_valid(p)
    } else {
        InlineLinkValidation::Invalid
    };

    if matches!(
        link_validation,
        InlineLinkValidation::Valid | InlineLinkValidation::DepthExceeded
    ) {
        // Inline link/image: [text](url) or ![alt](url)
        // Bump past ( and lex the following tokens in LinkDefinition context
        // so whitespace separates destination and title.
        p.expect_with_context(L_PAREN, crate::lexer::MarkdownLexContext::LinkDefinition);

        let destination = p.start();
        let destination_result = parse_inline_link_destination_tokens(p);

        // When depth exceeded, destination is truncated but link is still valid.
        // Complete the destination and link immediately without looking for closing paren.
        if destination_result == DestinationScanResult::DepthExceeded {
            destination.complete(p, MD_INLINE_ITEM_LIST);
            return Present(m.complete(p, kind.inline_kind()));
        }

        let has_title = inline_title_starts_after_whitespace_tokens(p);
        while is_title_separator_token(p) {
            bump_link_def_separator(p);
        }
        if destination_result == DestinationScanResult::Invalid {
            destination.abandon(p);
            m.abandon(p);
            p.rewind(checkpoint);
            p.force_relex_regular();
            return Absent;
        }
        destination.complete(p, MD_INLINE_ITEM_LIST);

        if has_title {
            let title_m = p.start();
            let list_m = p.start();
            parse_title_content(p, get_title_close_char(p));
            list_m.complete(p, MD_INLINE_ITEM_LIST);
            title_m.complete(p, MD_LINK_TITLE);
        }

        // Skip trailing whitespace/newlines before closing paren without creating nodes
        // (creating nodes would violate the MD_INLINE_LINK grammar which expects exactly 7 children)
        while is_title_separator_token(p) {
            skip_link_def_separator_tokens(p);
        }

        if !p.eat(R_PAREN) {
            if p.at_inline_end() {
                kind.report_unclosed_destination(p, opening_range);
            }
            m.abandon(p);
            p.rewind(checkpoint);
            p.force_relex_regular();
            return Absent;
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
            // Return Absent - the caller will treat `[` as textual.
            // Don't consume the whole bracket sequence to avoid consuming
            // past emphasis closers.
            return Absent;
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
            // Return Absent - the caller will treat `[` as textual.
            // Don't consume the whole bracket sequence to avoid consuming
            // past emphasis closers.
            return Absent;
        }
        Present(m.complete(p, kind.reference_kind()))
    }
}

struct ReferenceLinkLookahead {
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

        let link_text = collect_link_text(p)?;

        // Link text must be non-empty after normalization (e.g., `[\n ]` normalizes to empty)
        let normalized_link = normalize_reference_label(&link_text);
        if normalized_link.is_empty() {
            return None;
        }

        p.bump(R_BRACK);

        if p.at(L_PAREN) {
            return None;
        }

        if p.at(L_BRACK) {
            p.bump(L_BRACK);
            let label_text = collect_label_text_simple(p);
            if let Some(label_text) = label_text {
                let label = if label_text.is_empty() {
                    link_text.clone()
                } else {
                    // Explicit label must also normalize to non-empty
                    let normalized_label = normalize_reference_label(&label_text);
                    if normalized_label.is_empty() {
                        return None;
                    }
                    label_text
                };
                p.bump(R_BRACK);
                return Some(ReferenceLinkLookahead {
                    label_raw: label,
                    is_shortcut: false,
                });
            }
        }

        Some(ReferenceLinkLookahead {
            label_raw: link_text,
            is_shortcut: true,
        })
    })
}

/// Collect text for a link label (e.g., the `label` in `[text][label]`).
///
/// Per CommonMark §4.7, link labels have specific rules:
/// - Unescaped square brackets are NOT allowed inside labels (see example 555)
/// - Backslash escapes ARE allowed (e.g., `\]` is a literal `]` in the label)
/// - No inline parsing (backticks, HTML, etc. are literal characters)
///
/// We stop at the first R_BRACK token (unescaped `]`). Escaped brackets like `\]`
/// are lexed as MD_TEXTUAL_LITERAL, not R_BRACK, so they're included in the label.
fn collect_label_text_simple(p: &mut MarkdownParser) -> Option<String> {
    let mut text = String::new();

    loop {
        if p.at(T![EOF]) || p.at_inline_end() {
            return None;
        }

        // Blank lines terminate
        if p.at(NEWLINE) && p.at_blank_line() {
            return None;
        }

        // R_BRACK token = unescaped `]` closes the label.
        // Note: Escaped brackets (`\]`) are lexed as MD_TEXTUAL_LITERAL,
        // not R_BRACK, so they're correctly included in the label text.
        if p.at(R_BRACK) {
            return Some(text);
        }

        text.push_str(p.cur_text());
        p.bump(p.cur());
    }
}

/// Collect text for link text (e.g., the `text` in `[text](url)` or `[text][label]`).
/// Per CommonMark, link text CAN contain inline elements - code spans, autolinks, HTML.
/// `]` inside these constructs does NOT close the link text.
fn collect_link_text(p: &mut MarkdownParser) -> Option<String> {
    let mut text = String::new();
    let mut bracket_depth = 0usize;

    loop {
        if p.at(T![EOF]) || p.at_inline_end() {
            return None;
        }

        // Per CommonMark, blank lines terminate link text
        if p.at(NEWLINE) && p.at_blank_line() {
            return None;
        }

        // Code spans can contain `]` - skip them entirely.
        // Per CommonMark, `]` inside code spans doesn't terminate link text.
        if p.at(BACKTICK) {
            let opening_count = p.cur_text().len();
            text.push_str(p.cur_text());
            p.bump(p.cur());

            // Find matching closing backticks
            let mut found_close = false;
            while !p.at(T![EOF]) && !p.at_inline_end() {
                if p.at(NEWLINE) && p.at_blank_line() {
                    break; // Blank line terminates
                }
                if p.at(BACKTICK) && p.cur_text().len() == opening_count {
                    text.push_str(p.cur_text());
                    p.bump(p.cur());
                    found_close = true;
                    break;
                }
                text.push_str(p.cur_text());
                p.bump(p.cur());
            }
            if !found_close {
                // Unclosed code span - treat opening backticks as literal
                // (already added to text, continue normally)
            }
            continue;
        }

        // Autolinks and inline HTML can contain `]` - skip them entirely.
        // Per CommonMark, `]` inside `<...>` constructs doesn't terminate link text.
        if p.at(L_ANGLE) {
            text.push_str(p.cur_text());
            p.bump(p.cur());

            // Consume until `>` or newline
            while !p.at(T![EOF]) && !p.at_inline_end() && !p.at(R_ANGLE) {
                if p.at(NEWLINE) {
                    // Newlines end autolinks/HTML tags
                    break;
                }
                text.push_str(p.cur_text());
                p.bump(p.cur());
            }
            if p.at(R_ANGLE) {
                text.push_str(p.cur_text());
                p.bump(p.cur());
            }
            continue;
        }

        if p.at(L_BRACK) {
            bracket_depth += 1;
            text.push_str(p.cur_text());
            p.bump(p.cur());
            continue;
        }

        if p.at(R_BRACK) {
            if bracket_depth == 0 {
                return Some(text);
            }
            bracket_depth -= 1;
            text.push_str(p.cur_text());
            p.bump(p.cur());
            continue;
        }

        text.push_str(p.cur_text());
        p.bump(p.cur());
    }
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
        let mut saw_whitespace = false;
        while is_title_separator_token(p) {
            bump_link_def_separator(p);
            saw_whitespace = true;
        }
        saw_whitespace && get_title_close_char(p).is_some()
    })
}

/// Result of validating an inline link.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InlineLinkValidation {
    /// Link is valid with complete destination
    Valid,
    /// Link is invalid
    Invalid,
    /// Link is valid but destination was truncated due to paren depth limit.
    /// The link should be closed immediately without looking for `)`.
    DepthExceeded,
}

fn inline_link_is_valid(p: &mut MarkdownParser) -> InlineLinkValidation {
    p.lookahead(|p| {
        if !p.at(L_PAREN) {
            return InlineLinkValidation::Invalid;
        }

        p.bump(L_PAREN);
        p.re_lex_link_definition();

        let destination_result = scan_inline_link_destination_tokens(p);

        // If depth exceeded, link is valid but truncated - no need to check for closing paren
        if destination_result == DestinationScanResult::DepthExceeded {
            return InlineLinkValidation::DepthExceeded;
        }

        if destination_result == DestinationScanResult::Invalid {
            return InlineLinkValidation::Invalid;
        }

        let mut saw_separator = false;
        while is_title_separator_token(p) {
            skip_link_def_separator_tokens(p);
            saw_separator = true;
        }
        let has_title = saw_separator && get_title_close_char(p).is_some();
        while is_title_separator_token(p) {
            skip_link_def_separator_tokens(p);
        }

        if has_title {
            scan_title_content(p, get_title_close_char(p));
        }

        while is_title_separator_token(p) {
            skip_link_def_separator_tokens(p);
        }

        if p.at(R_PAREN) {
            InlineLinkValidation::Valid
        } else {
            InlineLinkValidation::Invalid
        }
    })
}

/// Result of scanning a link destination.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DestinationScanResult {
    /// Destination is valid and complete
    Valid,
    /// Destination is invalid (contains invalid characters, etc.)
    Invalid,
    /// Destination was truncated because paren depth exceeded the limit.
    /// In this case, the link is considered valid but closed at the truncation point.
    DepthExceeded,
}

fn scan_inline_link_destination_tokens(p: &mut MarkdownParser) -> DestinationScanResult {
    const MAX_PAREN_DEPTH: i32 = super::MAX_LINK_DESTINATION_PAREN_DEPTH;
    // Skip leading whitespace to match parse_inline_link_destination_tokens behavior
    while is_title_separator_token(p) {
        skip_link_def_separator_tokens(p);
    }
    if p.at(L_ANGLE) {
        p.bump_link_definition();
        let mut pending_escape = false;
        loop {
            if p.at(EOF) || p.at(NEWLINE) {
                return DestinationScanResult::Invalid;
            }
            if p.at(R_ANGLE) {
                if pending_escape {
                    if !super::validate_link_destination_text(
                        p.cur_text(),
                        super::LinkDestinationKind::Enclosed,
                        &mut pending_escape,
                    ) {
                        return DestinationScanResult::Invalid;
                    }
                    p.bump_link_definition();
                    continue;
                }
                p.bump_link_definition();
                return DestinationScanResult::Valid;
            }
            if !super::validate_link_destination_text(
                p.cur_text(),
                super::LinkDestinationKind::Enclosed,
                &mut pending_escape,
            ) {
                return DestinationScanResult::Invalid;
            }
            p.bump_link_definition();
        }
    }

    let mut paren_depth: i32 = 0;
    let mut pending_escape = false;
    while !p.at(EOF) && !p.at(NEWLINE) {
        if is_whitespace_token(p) {
            break;
        }
        let text = p.cur_text();
        if !super::validate_link_destination_text(
            text,
            super::LinkDestinationKind::Raw,
            &mut pending_escape,
        ) {
            return DestinationScanResult::Invalid;
        }
        match super::try_update_paren_depth(text, paren_depth, MAX_PAREN_DEPTH) {
            super::ParenDepthResult::Ok(next_depth) => {
                paren_depth = next_depth;
                p.bump_link_definition();
            }
            super::ParenDepthResult::DepthExceeded => {
                // Paren depth exceeded - destination is truncated at this point.
                // Per CommonMark/cmark, the link is still valid but closed here.
                return DestinationScanResult::DepthExceeded;
            }
            super::ParenDepthResult::UnmatchedClose => {
                // Unmatched closing paren - destination ends here normally.
                // The `)` belongs to the enclosing construct (inline link closer).
                break;
            }
        }
    }
    if p.at(EOF) {
        return DestinationScanResult::Invalid;
    }
    if p.at(NEWLINE) {
        return if p.at_blank_line() {
            DestinationScanResult::Invalid
        } else {
            DestinationScanResult::Valid
        };
    }
    DestinationScanResult::Valid
}

fn scan_title_content(p: &mut MarkdownParser, close_char: Option<char>) {
    let Some(close_char) = close_char else {
        return;
    };

    let text = p.cur_text();
    let is_complete = text.len() >= 2 && super::ends_with_unescaped_close(text, close_char);

    p.bump_link_definition();
    if is_complete {
        return;
    }

    loop {
        // Stop on EOF or blank line (titles cannot span blank lines per CommonMark)
        if p.at(EOF) || p.at_blank_line() {
            return;
        }

        // Continue through single newlines (titles can span non-blank lines)
        if p.at(NEWLINE) {
            skip_link_def_separator_tokens(p);
            continue;
        }

        let text = p.cur_text();
        if super::ends_with_unescaped_close(text, close_char) {
            p.bump_link_definition();
            return;
        }

        p.bump_link_definition();
    }
}

fn skip_link_def_separator_tokens(p: &mut MarkdownParser) {
    if p.at(NEWLINE) {
        p.bump(NEWLINE);
    } else {
        p.bump_link_definition();
    }
}

fn is_title_separator_token(p: &MarkdownParser) -> bool {
    is_whitespace_token(p) || (p.at(NEWLINE) && !p.at_blank_line())
}

fn bump_link_def_separator(p: &mut MarkdownParser) {
    if p.at(NEWLINE) {
        let item = p.start();
        p.bump_remap(MD_TEXTUAL_LITERAL);
        item.complete(p, MD_TEXTUAL);
    } else {
        bump_textual_link_def(p);
    }
}

fn parse_inline_link_destination_tokens(p: &mut MarkdownParser) -> DestinationScanResult {
    p.re_lex_link_definition();
    const MAX_PAREN_DEPTH: i32 = super::MAX_LINK_DESTINATION_PAREN_DEPTH;

    if p.at(L_ANGLE) {
        bump_textual_link_def(p);
        let mut pending_escape = false;
        loop {
            if p.at(EOF) || p.at(NEWLINE) {
                return DestinationScanResult::Invalid;
            }
            if p.at(R_ANGLE) {
                if pending_escape {
                    if !super::validate_link_destination_text(
                        p.cur_text(),
                        super::LinkDestinationKind::Enclosed,
                        &mut pending_escape,
                    ) {
                        return DestinationScanResult::Invalid;
                    }
                    bump_textual_link_def(p);
                    continue;
                }
                bump_textual_link_def(p);
                return DestinationScanResult::Valid;
            }
            if !super::validate_link_destination_text(
                p.cur_text(),
                super::LinkDestinationKind::Enclosed,
                &mut pending_escape,
            ) {
                return DestinationScanResult::Invalid;
            }
            bump_textual_link_def(p);
        }
    }

    let mut paren_depth: i32 = 0;
    let mut pending_escape = false;
    while is_title_separator_token(p) {
        bump_link_def_separator(p);
    }
    while !p.at(EOF) && !p.at(NEWLINE) {
        if is_whitespace_token(p) {
            break;
        }

        let text = p.cur_text();
        if !super::validate_link_destination_text(
            text,
            super::LinkDestinationKind::Raw,
            &mut pending_escape,
        ) {
            return DestinationScanResult::Invalid;
        }
        match super::try_update_paren_depth(text, paren_depth, MAX_PAREN_DEPTH) {
            super::ParenDepthResult::Ok(next_depth) => {
                paren_depth = next_depth;
                bump_textual_link_def(p);
            }
            super::ParenDepthResult::DepthExceeded => {
                // Paren depth exceeded - destination is truncated at this point.
                return DestinationScanResult::DepthExceeded;
            }
            super::ParenDepthResult::UnmatchedClose => {
                // Unmatched closing paren - destination ends here normally.
                // The `)` belongs to the enclosing construct (inline link closer).
                break;
            }
        }
    }
    if p.at(EOF) {
        return DestinationScanResult::Invalid;
    }
    if p.at(NEWLINE) {
        return if p.at_blank_line() {
            DestinationScanResult::Invalid
        } else {
            DestinationScanResult::Valid
        };
    }
    DestinationScanResult::Valid
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
    let is_complete = text.len() >= 2 && super::ends_with_unescaped_close(text, close_char);

    bump_textual_link_def(p);
    if is_complete {
        return;
    }

    loop {
        // Stop on EOF or blank line (titles cannot span blank lines per CommonMark)
        if p.at(EOF) || p.at_blank_line() {
            return;
        }

        // Continue through single newlines (titles can span non-blank lines)
        if p.at(NEWLINE) {
            bump_link_def_separator(p);
            continue;
        }

        let text = p.cur_text();
        if super::ends_with_unescaped_close(text, close_char) {
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
pub(crate) fn is_inline_html(text: &str) -> Option<usize> {
    let bytes = text.as_bytes();
    if bytes.len() < 2 || bytes[0] != b'<' {
        return None;
    }

    // HTML comment: <!-- ... -->
    // Per CommonMark 0.31.2 §6.8, an HTML comment consists of `<!--` + text + `-->`,
    // where text does not start with `>` or `->`, and does not end with `-`.
    // Additionally, `<!-->` and `<!--->` are valid (degenerate) comments.
    if bytes.starts_with(b"<!--") {
        let rest = &bytes[4..];
        // Handle degenerate comments: <!-->  and  <!--->
        if rest.starts_with(b">") {
            return Some(5); // <!-->
        }
        if rest.starts_with(b"->") {
            return Some(6); // <!--->
        }
        // Find closing --> after <!--
        if let Some(pos) = text[4..].find("-->") {
            let body = &text[4..4 + pos];
            // Body must not end with '-'
            if body.ends_with('-') {
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

    let mut need_space = true;
    // We already know the boundary char was whitespace, so first iteration has space.
    let mut had_space = true;

    loop {
        if need_space {
            let s = skip_spaces(&mut i)?;
            had_space = had_space || s;
        }
        need_space = true;

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
        had_space = skip_spaces(&mut i)?;
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
            // After value, need to find whitespace at top of loop
            had_space = false;
        }
        // If no '=' was found, `had_space` from skip_spaces above carries over
        // as the separator for the next attribute (boolean attribute case).
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

    // Per CommonMark §4.3, setext heading underlines take priority over inline HTML.
    // If this HTML tag spans across a line that is a setext underline, treat `<` as literal.
    if crate::syntax::inline_span_crosses_setext(p, html_len) {
        return Absent;
    }

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

    // Content as inline item list containing textual nodes.
    // Autolinks don't process backslash escapes, but the lexer may combine
    // `\>` into a single escape token. We re-lex in CodeSpan context where
    // backslash is literal, so `\` and `>` are separate tokens.
    p.force_relex_code_span();

    let content_m = p.start();
    while !p.at(R_ANGLE) && !p.at(T![EOF]) && !p.at_inline_end() {
        let text_m = p.start();
        p.bump_remap_with_context(
            MD_TEXTUAL_LITERAL,
            crate::lexer::MarkdownLexContext::CodeSpan,
        );
        text_m.complete(p, MD_TEXTUAL);
    }
    content_m.complete(p, MD_INLINE_ITEM_LIST);

    // >
    p.expect(R_ANGLE);

    // Re-lex back to regular context
    p.force_relex_regular();

    Present(m.complete(p, MD_AUTOLINK))
}

/// Dispatch to the appropriate inline parser based on current token.
pub(crate) fn parse_any_inline(p: &mut MarkdownParser) -> ParsedSyntax {
    if p.at(MD_HARD_LINE_LITERAL) {
        parse_hard_line(p)
    } else if p.at(BACKTICK) || p.at(T!["```"]) {
        // Try code span, fall back to literal text if no matching closer exists.
        // T!["```"] can appear when backticks are at line start but info string
        // contains backticks, making it not a fenced code block (CommonMark examples 138, 145).
        let result = parse_inline_code(p);
        if result.is_absent() {
            super::parse_textual(p)
        } else {
            result
        }
    } else if p.at(DOUBLE_STAR) || p.at(DOUBLE_UNDERSCORE) {
        // For cases like `***foo***`, the em match starts at the exact token boundary
        // (prefix_len=0) while the strong match starts at offset 1 (prefix_len=1).
        // Try italic first to handle nested emphasis correctly, then try strong.
        let result = parse_inline_italic(p);
        if result.is_present() {
            return result;
        }
        let result = parse_inline_emphasis(p);
        if result.is_present() {
            return result;
        }
        // Neither matched - re-lex to single token and emit just one char as literal.
        // This handles cases like `**foo*` where opener is at offset 1.
        p.force_relex_emphasis_inline();
        super::parse_textual(p)
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
