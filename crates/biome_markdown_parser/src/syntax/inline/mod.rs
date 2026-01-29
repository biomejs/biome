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
use biome_parser::prelude::ParsedSyntax;

use crate::MarkdownParser;

mod code_span;
mod emphasis;
mod entities;
mod html;
mod links;

pub(crate) use emphasis::EmphasisContext;
pub(crate) use html::is_inline_html;

enum InlineLinksPolicy {
    NoLinks,
    FullLinks,
}

struct InlineListUntilResult {
    has_nested_link: bool,
}

fn parse_inline_item_list_until_impl(
    p: &mut MarkdownParser,
    stop: MarkdownSyntaxKind,
    policy: InlineLinksPolicy,
) -> InlineListUntilResult {
    let m = p.start();
    let prev_context = emphasis::set_inline_emphasis_context_until(p, stop);
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
            if code_span::parse_inline_code(p).is_present() {
                continue;
            }
            let _ = super::parse_textual(p);
            continue;
        }

        // Autolinks and inline HTML can contain `]`
        if p.at(L_ANGLE) {
            if html::parse_autolink(p).is_present() {
                continue;
            }
            if html::parse_inline_html(p).is_present() {
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
            match policy {
                InlineLinksPolicy::NoLinks => {
                    if !has_nested_link && nested_link_starts_here(p) {
                        has_nested_link = true;
                    }
                    bracket_depth += 1;
                    let _ = super::parse_textual(p);
                    continue;
                }
                InlineLinksPolicy::FullLinks => {
                    let result = links::parse_link_or_reference(p);
                    if result.is_present() {
                        continue;
                    }
                    bracket_depth += 1;
                    let _ = super::parse_textual(p);
                    continue;
                }
            }
        }

        if matches!(policy, InlineLinksPolicy::FullLinks) && p.at(BANG) && p.nth_at(1, L_BRACK) {
            let result = links::parse_image_or_reference(p);
            if result.is_present() {
                continue;
            }
            let _ = super::parse_textual(p);
            continue;
        }

        let parsed = match policy {
            InlineLinksPolicy::NoLinks => parse_any_inline_no_links(p),
            InlineLinksPolicy::FullLinks => parse_any_inline(p),
        };
        if parsed.is_absent() {
            break;
        }
    }

    m.complete(p, MD_INLINE_ITEM_LIST);
    p.set_emphasis_context(prev_context);
    InlineListUntilResult { has_nested_link }
}

fn parse_inline_item_list_until_no_links(p: &mut MarkdownParser, stop: MarkdownSyntaxKind) -> bool {
    parse_inline_item_list_until_impl(p, stop, InlineLinksPolicy::NoLinks).has_nested_link
}

/// Parse inline items until `stop` token, allowing full inline parsing including links.
/// Used for image alt text where nested links/images should be fully parsed
/// so their text content can be extracted for the alt attribute.
fn parse_inline_item_list_until(p: &mut MarkdownParser, stop: MarkdownSyntaxKind) {
    let _ = parse_inline_item_list_until_impl(p, stop, InlineLinksPolicy::FullLinks);
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
        return links::parse_inline_image(p);
    }

    parse_any_inline(p)
}

/// Dispatch to the appropriate inline parser based on current token.
pub(crate) fn parse_any_inline(p: &mut MarkdownParser) -> ParsedSyntax {
    if p.at(MD_HARD_LINE_LITERAL) {
        code_span::parse_hard_line(p)
    } else if p.at(BACKTICK) || p.at(T!["```"]) {
        // Try code span, fall back to literal text if no matching closer exists.
        // T!["```"] can appear when backticks are at line start but info string
        // contains backticks, making it not a fenced code block (CommonMark examples 138, 145).
        let result = code_span::parse_inline_code(p);
        if result.is_absent() {
            super::parse_textual(p)
        } else {
            result
        }
    } else if p.at(DOUBLE_STAR) || p.at(DOUBLE_UNDERSCORE) {
        // For cases like `***foo***`, the em match starts at the exact token boundary
        // (prefix_len=0) while the strong match starts at offset 1 (prefix_len=1).
        // Try italic first to handle nested emphasis correctly, then try strong.
        let result = emphasis::parse_inline_italic(p);
        if result.is_present() {
            return result;
        }
        let result = emphasis::parse_inline_emphasis(p);
        if result.is_present() {
            return result;
        }
        // Neither matched - re-lex to single token and emit just one char as literal.
        // This handles cases like `**foo*` where opener is at offset 1.
        p.force_relex_emphasis_inline();
        super::parse_textual(p)
    } else if p.at(T![*]) || p.at(UNDERSCORE) {
        // Try italic, fall back to literal text if flanking rules fail
        let result = emphasis::parse_inline_italic(p);
        if result.is_absent() {
            super::parse_textual(p)
        } else {
            result
        }
    } else if p.at(BANG) && p.nth_at(1, L_BRACK) {
        // Try image, fall back to literal text if parsing fails
        let result = links::parse_inline_image(p);
        if result.is_absent() {
            super::parse_textual(p)
        } else {
            result
        }
    } else if p.at(L_BRACK) {
        // Try link, fall back to literal text if parsing fails
        let result = links::parse_inline_link(p);
        if result.is_absent() {
            super::parse_textual(p)
        } else {
            result
        }
    } else if p.at(L_ANGLE) {
        // Try autolink first (takes priority per CommonMark)
        let result = html::parse_autolink(p);
        if result.is_present() {
            return result;
        }
        // Then try inline HTML
        let result = html::parse_inline_html(p);
        if result.is_present() {
            return result;
        }
        // Fall back to textual
        super::parse_textual(p)
    } else if p.at(MD_ENTITY_LITERAL) {
        // Entity or numeric character reference (already validated by lexer)
        entities::parse_entity_reference(p)
    } else {
        super::parse_textual(p)
    }
}
