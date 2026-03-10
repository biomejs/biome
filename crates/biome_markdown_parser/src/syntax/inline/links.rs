use biome_markdown_syntax::MarkdownSyntaxKind;
use biome_markdown_syntax::T;
use biome_markdown_syntax::kind::MarkdownSyntaxKind::*;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax::{self, *};
use biome_rowan::TextRange;

use crate::MarkdownParser;
use crate::lexer::MarkdownLexContext;
use crate::syntax::inline::{parse_inline_item_list_until, parse_inline_item_list_until_no_links};
use crate::syntax::parse_error::{unclosed_image, unclosed_link};
use crate::syntax::reference::normalize_reference_label;
use crate::syntax::{
    LinkDestinationKind, MAX_LINK_DESTINATION_PAREN_DEPTH, ParenDepthResult,
    ends_with_unescaped_close, try_update_paren_depth, validate_link_destination_text,
};

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
            Self::Link => p.error(unclosed_link(p, opening_range, "expected `)` to close URL")),
            Self::Image => p.error(unclosed_image(
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
        p.expect_with_context(L_PAREN, MarkdownLexContext::LinkDefinition);

        let destination = p.start();
        let destination_result = parse_inline_link_destination_tokens(p);

        // When depth exceeded, destination is truncated but link is still valid.
        // Complete the destination and link immediately without looking for closing paren.
        if destination_result == DestinationScanResult::DepthExceeded {
            destination.complete(p, MD_INLINE_ITEM_LIST);
            p.force_relex_regular();
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
        p.has_link_reference_definition(normalized.as_ref())
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

        // Skip leading whitespace before angle bracket check
        // (parse_inline_link_destination_tokens only skips whitespace in the raw path).
        while is_title_separator_token(p) {
            bump_link_def_separator(p);
        }

        let destination_result = parse_inline_link_destination_tokens(p);

        // If depth exceeded, link is valid but truncated - no need to check for closing paren
        if destination_result == DestinationScanResult::DepthExceeded {
            return InlineLinkValidation::DepthExceeded;
        }

        if destination_result == DestinationScanResult::Invalid {
            return InlineLinkValidation::Invalid;
        }

        let mut saw_separator = false;
        while is_title_separator_token(p) {
            bump_link_def_separator(p);
            saw_separator = true;
        }
        let has_title = saw_separator && get_title_close_char(p).is_some();

        if has_title {
            parse_title_content(p, get_title_close_char(p));
        }

        while is_title_separator_token(p) {
            bump_link_def_separator(p);
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

fn parse_inline_link_destination_tokens(p: &mut MarkdownParser) -> DestinationScanResult {
    const MAX_PAREN_DEPTH: i32 = MAX_LINK_DESTINATION_PAREN_DEPTH;

    p.re_lex_link_definition();

    // Enclosed destination: <url>
    if p.at(L_ANGLE) {
        bump_textual_link_def(p);
        let mut pending_escape = false;
        loop {
            if p.at(EOF) || p.at(NEWLINE) {
                return DestinationScanResult::Invalid;
            }
            if p.at(R_ANGLE) {
                if pending_escape {
                    if !validate_link_destination_text(
                        p.cur_text(),
                        LinkDestinationKind::Enclosed,
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
            if !validate_link_destination_text(
                p.cur_text(),
                LinkDestinationKind::Enclosed,
                &mut pending_escape,
            ) {
                return DestinationScanResult::Invalid;
            }
            bump_textual_link_def(p);
        }
    }

    // Raw destination (no angle brackets)
    let mut paren_depth: i32 = 0;
    let mut pending_escape = false;

    // Skip leading whitespace in raw path.
    while is_title_separator_token(p) {
        bump_link_def_separator(p);
    }

    while !p.at(EOF) && !p.at(NEWLINE) {
        if is_whitespace_token(p) {
            break;
        }
        let text = p.cur_text();
        if !validate_link_destination_text(text, LinkDestinationKind::Raw, &mut pending_escape) {
            return DestinationScanResult::Invalid;
        }
        match try_update_paren_depth(text, paren_depth, MAX_PAREN_DEPTH) {
            ParenDepthResult::Ok(next_depth) => {
                paren_depth = next_depth;
                bump_textual_link_def(p);
            }
            ParenDepthResult::DepthExceeded => {
                return DestinationScanResult::DepthExceeded;
            }
            ParenDepthResult::UnmatchedClose => {
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
    let is_complete = text.len() >= 2 && ends_with_unescaped_close(text, close_char);

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
        if ends_with_unescaped_close(text, close_char) {
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
