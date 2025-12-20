//! Inline element parsing for Markdown.
//!
//! Handles inline code spans, emphasis (bold/italic), links, and images.

use biome_markdown_syntax::T;
use biome_markdown_syntax::kind::MarkdownSyntaxKind::*;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax::{self, *};

use crate::MarkdownParser;

/// Parse inline code span (`` `code` ``).
///
/// Grammar: MdInlineCode = l_tick: '`' content: MdInlineItemList r_tick: '`'
pub(crate) fn parse_inline_code(p: &mut MarkdownParser) -> ParsedSyntax {
    if !p.at(BACKTICK) {
        return Absent;
    }

    let m = p.start();

    // Opening backtick
    p.bump(BACKTICK);

    // Content - parse until closing backtick, newline, or EOF
    let content = p.start();
    while !p.at(BACKTICK) && !p.at(T![EOF]) && !p.has_preceding_line_break() {
        let text_m = p.start();
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);
    }
    content.complete(p, MD_INLINE_ITEM_LIST);

    // Closing backtick - emits diagnostic if missing
    p.expect(BACKTICK);

    Present(m.complete(p, MD_INLINE_CODE))
}

/// Parse inline emphasis (bold: `**text**` or `__text__`).
///
/// Grammar: MdInlineEmphasis = l_fence: ('**' | '__') content: MdInlineItemList r_fence: ('**' | '__')
pub(crate) fn parse_inline_emphasis(p: &mut MarkdownParser) -> ParsedSyntax {
    if !p.at(DOUBLE_STAR) && !p.at(DOUBLE_UNDERSCORE) {
        return Absent;
    }

    let m = p.start();
    let opener = p.cur();

    // Opening marker
    p.bump(opener);

    // Content
    let content = p.start();
    while !p.at(opener) && !p.at(T![EOF]) {
        // Check for newline in trivia
        if p.has_preceding_line_break() {
            break;
        }
        // Parse nested inline or textual
        if !parse_nested_inline(p) {
            let text_m = p.start();
            p.bump_remap(MD_TEXTUAL_LITERAL);
            text_m.complete(p, MD_TEXTUAL);
        }
    }
    content.complete(p, MD_INLINE_ITEM_LIST);

    // Closing marker - emits diagnostic if missing
    p.expect(opener);

    Present(m.complete(p, MD_INLINE_EMPHASIS))
}

/// Parse inline italic (`*text*` or `_text_`).
///
/// Grammar: MdInlineItalic = l_fence: ('*' | '_') content: MdInlineItemList r_fence: ('*' | '_')
pub(crate) fn parse_inline_italic(p: &mut MarkdownParser) -> ParsedSyntax {
    if !p.at(T![*]) && !p.at(UNDERSCORE) {
        return Absent;
    }

    let m = p.start();
    let opener = p.cur();

    // Opening marker
    p.bump(opener);

    // Content
    let content = p.start();
    while !p.at(opener) && !p.at(T![EOF]) {
        // Check for newline in trivia
        if p.has_preceding_line_break() {
            break;
        }
        // Parse nested inline or textual
        if !parse_nested_inline(p) {
            let text_m = p.start();
            p.bump_remap(MD_TEXTUAL_LITERAL);
            text_m.complete(p, MD_TEXTUAL);
        }
    }
    content.complete(p, MD_INLINE_ITEM_LIST);

    // Closing marker - emits diagnostic if missing
    p.expect(opener);

    Present(m.complete(p, MD_INLINE_ITALIC))
}

/// Parse inline link (`[text](url)`).
///
/// Grammar: `MdInlineLink = '[' text: MdInlineItemList ']' '(' source: MdInlineItemList ')'`
pub(crate) fn parse_inline_link(p: &mut MarkdownParser) -> ParsedSyntax {
    if !p.at(L_BRACK) {
        return Absent;
    }

    let m = p.start();

    // [
    p.bump(L_BRACK);

    // Link text
    let text = p.start();
    while !p.at(R_BRACK) && !p.at(T![EOF]) && !p.has_preceding_line_break() {
        let text_m = p.start();
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);
    }
    text.complete(p, MD_INLINE_ITEM_LIST);

    // ] - emits diagnostic if missing
    p.expect(R_BRACK);

    // ( - emits diagnostic if missing
    p.expect(L_PAREN);

    // URL/source
    let source = p.start();
    while !p.at(R_PAREN) && !p.at(T![EOF]) && !p.has_preceding_line_break() {
        let text_m = p.start();
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);
    }
    source.complete(p, MD_INLINE_ITEM_LIST);

    // ) - emits diagnostic if missing
    p.expect(R_PAREN);

    Present(m.complete(p, MD_INLINE_LINK))
}

/// Parse inline image (`![alt](url)`).
///
/// Grammar: `MdInlineImage = '!' '[' alt: MdInlineItemList ']' '(' source: MdInlineItemList ')'`
pub(crate) fn parse_inline_image(p: &mut MarkdownParser) -> ParsedSyntax {
    // Image starts with ![
    if !p.at(BANG) {
        return Absent;
    }

    // Check that [ follows !
    if !p.nth_at(1, L_BRACK) {
        return Absent;
    }

    let m = p.start();

    // !
    p.bump(BANG);

    // [
    p.bump(L_BRACK);

    // alt text
    let alt = p.start();
    while !p.at(R_BRACK) && !p.at(T![EOF]) && !p.has_preceding_line_break() {
        let text_m = p.start();
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);
    }
    alt.complete(p, MD_INLINE_ITEM_LIST);

    // ] - emits diagnostic if missing
    p.expect(R_BRACK);

    // ( - emits diagnostic if missing
    p.expect(L_PAREN);

    // source URL
    let source = p.start();
    while !p.at(R_PAREN) && !p.at(T![EOF]) && !p.has_preceding_line_break() {
        let text_m = p.start();
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);
    }
    source.complete(p, MD_INLINE_ITEM_LIST);

    // ) - emits diagnostic if missing
    p.expect(R_PAREN);

    Present(m.complete(p, MD_INLINE_IMAGE))
}

/// Try to parse a nested inline element (code, link, image).
/// Returns true if something was parsed, false otherwise.
///
/// Note: We intentionally don't parse nested emphasis here to avoid
/// ambiguity and potential infinite recursion. Emphasis markers inside
/// other emphasis are treated as literal text.
fn parse_nested_inline(p: &mut MarkdownParser) -> bool {
    if p.at(BACKTICK) {
        parse_inline_code(p).is_present()
    } else if p.at(BANG) && p.nth_at(1, L_BRACK) {
        parse_inline_image(p).is_present()
    } else if p.at(L_BRACK) {
        parse_inline_link(p).is_present()
    } else {
        false
    }
}

/// Dispatch to the appropriate inline parser based on current token.
pub(crate) fn parse_any_inline(p: &mut MarkdownParser) -> ParsedSyntax {
    if p.at(BACKTICK) {
        parse_inline_code(p)
    } else if p.at(DOUBLE_STAR) || p.at(DOUBLE_UNDERSCORE) {
        parse_inline_emphasis(p)
    } else if p.at(T![*]) || p.at(UNDERSCORE) {
        parse_inline_italic(p)
    } else if p.at(BANG) && p.nth_at(1, L_BRACK) {
        parse_inline_image(p)
    } else if p.at(L_BRACK) {
        parse_inline_link(p)
    } else {
        super::parse_textual(p)
    }
}
