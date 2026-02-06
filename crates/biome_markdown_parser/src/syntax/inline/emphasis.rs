use biome_markdown_syntax::MarkdownSyntaxKind;
use biome_markdown_syntax::T;
use biome_markdown_syntax::kind::MarkdownSyntaxKind::*;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax::{self, *};
use biome_unicode_table::is_unicode_punctuation;

use crate::MarkdownParser;
use crate::syntax::parse_error::unclosed_emphasis;
use crate::syntax::reference::normalize_reference_label;

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

fn backtick_run_len(bytes: &[u8], start: usize) -> usize {
    let mut count = 1;
    while start + count < bytes.len() && bytes[start + count] == b'`' {
        count += 1;
    }
    count
}

fn skip_code_span(bytes: &[u8], i: &mut usize) {
    let backtick_count = backtick_run_len(bytes, *i);
    *i += backtick_count;

    while *i < bytes.len() {
        if bytes[*i] == b'`' {
            let close_count = backtick_run_len(bytes, *i);
            *i += close_count;
            if close_count == backtick_count {
                break;
            }
        } else {
            *i += 1;
        }
    }
}

fn skip_angle_bracket(bytes: &[u8], i: &mut usize) {
    *i += 1;
    while *i < bytes.len() && bytes[*i] != b'>' && bytes[*i] != b'\n' {
        *i += 1;
    }
    if *i < bytes.len() && bytes[*i] == b'>' {
        *i += 1;
    }
}

fn is_flanking_delimiter(primary: Option<char>, secondary: Option<char>) -> bool {
    match primary {
        None => false,                        // At start/end of input, can't be flanking
        Some(c) if is_whitespace(c) => false, // Next to whitespace
        Some(c) if is_emphasis_marker(c) => true,
        Some(c) if is_punctuation(c) => {
            // Only flanking if the other side is whitespace or punctuation
            match secondary {
                None => true, // Boundary counts as whitespace
                Some(s) => is_whitespace(s) || is_punctuation(s),
            }
        }
        Some(_) => true, // Not next to whitespace or punctuation = flanking
    }
}

/// Check if an opening delimiter is left-flanking per CommonMark rules.
/// A left-flanking delimiter run is one that is:
/// - Not followed by Unicode whitespace, AND
/// - Either (a) not followed by punctuation, OR (b) preceded by whitespace/punctuation
fn is_left_flanking_delimiter(char_after: Option<char>, char_before: Option<char>) -> bool {
    is_flanking_delimiter(char_after, char_before)
}

/// Check if a closing delimiter is right-flanking per CommonMark rules.
/// A right-flanking delimiter run is one that is:
/// - Not preceded by Unicode whitespace, AND
/// - Either (a) not preceded by punctuation, OR (b) followed by whitespace/punctuation
fn is_right_flanking_delimiter(char_before: Option<char>, char_after: Option<char>) -> bool {
    is_flanking_delimiter(char_before, char_after)
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
    /// Whether this is a valid inline link `[...](` or full reference `[...][]`
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
                skip_code_span(bytes, &mut i);
                continue;
            }
            b'<' => {
                // Skip potential HTML/autolinks
                skip_angle_bracket(bytes, &mut i);
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
                if !normalized.is_empty() && reference_checker(normalized.as_ref()) {
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
            skip_code_span(bytes, &mut i);
        } else if b == b'<' {
            // Skip potential HTML tags and autolinks
            skip_angle_bracket(bytes, &mut i);
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

        // Parse nested inline content
        let result = super::parse_any_inline(p);
        if result.is_absent() {
            break;
        }
    }
    content.complete(p, MD_INLINE_ITEM_LIST);

    // Consume closer tokens
    let mut consumed_closer = 0;
    while consumed_closer < use_count {
        if use_count == 2 && (p.at(DOUBLE_STAR) || p.at(DOUBLE_UNDERSCORE)) {
            p.bump_any();
            consumed_closer += 2;
            continue;
        }
        if p.at(DOUBLE_STAR) || p.at(DOUBLE_UNDERSCORE) {
            p.force_relex_emphasis_inline();
        }
        if p.at(T![*]) || p.at(UNDERSCORE) {
            p.bump_any();
            consumed_closer += 1;
        } else {
            break;
        }
    }

    if consumed_closer < use_count {
        p.error(unclosed_emphasis(p, opening_range, opener_text));
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

pub(crate) fn set_inline_emphasis_context_until(
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
