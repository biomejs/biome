use biome_markdown_syntax::T;
use biome_markdown_syntax::kind::MarkdownSyntaxKind::*;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax::{self, *};

use crate::MarkdownParser;
use crate::lexer::MarkdownLexContext;
use crate::syntax::inline_span_crosses_setext;

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
    if inline_span_crosses_setext(p, html_len) {
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
    p.relex_code_span();

    let content_m = p.start();
    while !p.at(R_ANGLE) && !p.at(T![EOF]) && !p.at_inline_end() {
        let text_m = p.start();
        p.bump_remap_with_context(MD_TEXTUAL_LITERAL, MarkdownLexContext::CodeSpan);
        text_m.complete(p, MD_TEXTUAL);
    }
    content_m.complete(p, MD_INLINE_ITEM_LIST);

    // >
    p.expect(R_ANGLE);

    // Re-lex back to regular context
    p.force_relex_regular();

    Present(m.complete(p, MD_AUTOLINK))
}
