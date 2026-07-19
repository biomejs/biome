//! Reindenting support for literal (`|`) and folded (`>`) block scalars.
//!
//! The parser hands the formatter the entire scalar body as a single
//! `BLOCK_CONTENT_LITERAL` token whose text:
//!
//! - starts with the line break that ends the header line,
//! - contains every body byte: each line's indentation and any interior
//!   blank lines,
//! - carries no trailing terminator of its own; trailing blank lines are
//!   included only while they stay within the scalar's scope, otherwise the
//!   lexer attaches them as leading trivia of the following token.
//!
//! [`FormatBlockScalar`] rewrites that token so the body sits at the
//! configured indent width, mirroring how Prettier prints block scalars.
//! [`canonical_body`] returns [`Cow::Borrowed`] whenever the source body is
//! already canonical, so formatting already-formatted input emits the token
//! text zero-copy without allocating.

use std::borrow::Cow;

use crate::FormatYamlSyntaxToken;
use crate::prelude::*;
use biome_formatter::trivia::FormatToken;
use biome_formatter::{CstFormatContext, FormatOptions, write};
use biome_rowan::AstNode;
use biome_yaml_syntax::{
    AnyYamlBlockHeader, YamlBlockContent, YamlBlockHeaderList, YamlSyntaxKind, YamlSyntaxNode,
    YamlSyntaxToken,
};

/// True when a folded or literal block scalar can be safely reindented to
/// the canonical width: it sits at the document top level and its body has
/// a shape we can model (see [`has_well_indented_body`]). Anything else is
/// left verbatim so we never corrupt content we can't reproduce.
pub(crate) fn is_reindentable_block_scalar(
    scalar: &YamlSyntaxNode,
    headers: &YamlBlockHeaderList,
    content: &YamlBlockContent,
) -> bool {
    if !is_top_level(scalar) {
        return false;
    }
    let has_explicit_indent = headers
        .iter()
        .any(|h| matches!(h, AnyYamlBlockHeader::YamlIndentationIndicator(_)));
    has_well_indented_body(content, has_explicit_indent)
}

/// True when the scalar is the document's top-level node, i.e. not nested
/// inside a block mapping or sequence. The reindent logic rewrites the body
/// to a canonical column-0 offset, which is only correct at the top level;
/// a nested scalar's body indentation is relative to its enclosing node, so
/// rewriting it would shift the content out of place.
fn is_top_level(scalar: &YamlSyntaxNode) -> bool {
    !scalar.ancestors().any(|node| {
        matches!(
            node.kind(),
            YamlSyntaxKind::YAML_BLOCK_MAPPING | YamlSyntaxKind::YAML_BLOCK_SEQUENCE
        )
    })
}

/// A body is "well indented" when every non-blank line has at least one
/// leading space, and — if no explicit indentation indicator is present —
/// all non-blank lines share the same leading-space count. That second
/// constraint prevents us from re-indenting bodies where a parser quirk
/// (a `...` marker, or a comment at a shallower indent) pulled content
/// in at column 0 or some other rogue depth.
fn has_well_indented_body(content: &YamlBlockContent, has_explicit_indent: bool) -> bool {
    let Ok(token) = content.value_token() else {
        return false;
    };
    let text = token.text();
    let body_text = text
        .strip_prefix("\r\n")
        .or_else(|| text.strip_prefix('\n'))
        .unwrap_or(text);
    let mut indents = body_text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.bytes().take_while(|b| *b == b' ').count());
    let Some(first) = indents.next() else {
        return true;
    };
    if first == 0 {
        return false;
    }
    if has_explicit_indent {
        return true;
    }
    indents.all(|i| i == first)
}

/// The chomping mode selected by the block scalar's header: it controls how
/// line breaks at the end of the body are emitted (see [`canonical_body`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Chomp {
    Clip,
    Strip,
    Keep,
}

/// Formats a folded or literal block scalar, reindenting its body to the
/// canonical width. Only valid when [`is_reindentable_block_scalar`] returns
/// `true`; otherwise the scalar must be formatted verbatim.
pub(crate) struct FormatBlockScalar<'a> {
    pub(crate) opener: &'a YamlSyntaxToken,
    pub(crate) headers: &'a YamlBlockHeaderList,
    pub(crate) content: &'a YamlBlockContent,
}

impl Format<YamlFormatContext> for FormatBlockScalar<'_> {
    fn fmt(&self, f: &mut YamlFormatter) -> FormatResult<()> {
        let mut has_explicit_indent = false;
        let mut chomp = Chomp::Clip;
        for header in self.headers {
            match header {
                AnyYamlBlockHeader::YamlIndentationIndicator(_) => has_explicit_indent = true,
                AnyYamlBlockHeader::YamlBlockKeepIndicator(_) => chomp = Chomp::Keep,
                AnyYamlBlockHeader::YamlBlockStripIndicator(_) => chomp = Chomp::Strip,
                AnyYamlBlockHeader::YamlBogusBlockHeader(_) => {}
            }
        }

        write!(f, [self.opener.format()])?;
        write!(f, [self.headers.format()])?;

        // We rewrite the content token directly rather than going through
        // FormatYamlBlockContent's rule, so run the suppression check here
        // ourselves; it doubles as the visited marker required by the debug
        // formatter's suppression bookkeeping.
        if f.context().comments().is_suppressed(self.content.syntax()) {
            return format_suppressed_node(self.content.syntax()).fmt(f);
        }

        let content_token = self.content.value_token()?;
        let canonical_indent = f.options().indent_width().value() as usize;
        let body = canonical_body(
            content_token.text(),
            has_explicit_indent,
            chomp,
            canonical_indent,
        );

        FormatYamlSyntaxToken.format_replaced(
            &content_token,
            &syntax_token_cow_slice(body, &content_token, content_token.text_range().start()),
            f,
        )
    }
}

/// Returns the canonical form of a block scalar body.
///
/// `content_text` is the full `BLOCK_CONTENT_LITERAL` token text (see the
/// module documentation for its shape). The canonical form:
///
/// - re-indents every non-blank line to `canonical_indent` spaces, unless an
///   explicit indentation indicator is present, in which case lines are kept
///   verbatim because over-indentation is real content there;
/// - empties whitespace-only lines;
/// - uses `\n` line breaks only: the printer converts them to the configured
///   line ending, so a literal `\r` here would double up to `\r\r\n`;
/// - ends with a single terminating line break, plus one extra break per
///   additional trailing blank line when the chomping mode is [`Chomp::Keep`].
///
/// Returns [`Cow::Borrowed`] when the source body is already canonical so
/// that already-formatted input is emitted zero-copy; `syntax_token_cow_slice`
/// debug-asserts that a borrowed body matches the token text.
fn canonical_body(
    content_text: &str,
    has_explicit_indent: bool,
    chomp: Chomp,
    canonical_indent: usize,
) -> Cow<'_, str> {
    if is_canonical(content_text, has_explicit_indent, chomp, canonical_indent) {
        return Cow::Borrowed(content_text);
    }
    Cow::Owned(rebuild_body(
        content_text,
        has_explicit_indent,
        chomp,
        canonical_indent,
    ))
}

/// True when [`rebuild_body`] would reproduce `content_text` byte for byte,
/// letting [`canonical_body`] skip the rebuild entirely.
///
/// Kept in sync with [`rebuild_body`] by the round-trip unit test below and
/// by the borrowed-text debug assertion inside `syntax_token_cow_slice`.
fn is_canonical(
    content_text: &str,
    has_explicit_indent: bool,
    chomp: Chomp,
    canonical_indent: usize,
) -> bool {
    if content_text.contains('\r') {
        return false;
    }
    // The rebuilt body always starts with the header-terminating line break.
    let Some(body_text) = content_text.strip_prefix('\n') else {
        return false;
    };
    let lines: Vec<&str> = body_text.split('\n').collect();
    let trailing_blanks = lines
        .iter()
        .rev()
        .take_while(|line| line.trim().is_empty())
        .count();
    // The rebuilt body ends with exactly one terminating line break; `keep`
    // preserves any additional trailing blank lines on top of it.
    let canonical_trailing = match chomp {
        Chomp::Keep => trailing_blanks >= 1,
        Chomp::Clip | Chomp::Strip => trailing_blanks == 1,
    };
    if !canonical_trailing {
        return false;
    }
    let (body_lines, trailing) = lines.split_at(lines.len() - trailing_blanks);
    // Trailing blank lines must be truly empty, not whitespace-only.
    if trailing.iter().any(|line| !line.is_empty()) {
        return false;
    }
    let mut min_indent: Option<usize> = None;
    for line in body_lines {
        if line.trim().is_empty() {
            // The rebuild empties interior whitespace-only lines.
            if !line.is_empty() {
                return false;
            }
        } else if !has_explicit_indent {
            let indent = line.bytes().take_while(|b| *b == b' ').count();
            min_indent = Some(min_indent.map_or(indent, |current| current.min(indent)));
        }
    }
    match min_indent {
        // Reindenting strips the smallest indent and prefixes the canonical
        // one, so the body only stays unchanged when the two are equal.
        Some(min_indent) => min_indent == canonical_indent,
        // Nothing needs reindenting: blank body, or explicit indicator.
        None => true,
    }
}

/// Rewrites a non-canonical body into the form described on
/// [`canonical_body`].
fn rebuild_body(
    content_text: &str,
    has_explicit_indent: bool,
    chomp: Chomp,
    canonical_indent: usize,
) -> String {
    let body_text = content_text
        .strip_prefix("\r\n")
        .or_else(|| content_text.strip_prefix('\n'))
        .unwrap_or(content_text);
    let lines: Vec<&str> = body_text
        .split('\n')
        .map(|line| line.strip_suffix('\r').unwrap_or(line))
        .collect();

    let trailing_blanks = lines
        .iter()
        .rev()
        .take_while(|line| line.trim().is_empty())
        .count();
    let body_lines = &lines[..lines.len() - trailing_blanks];
    let extra_blank_lines = trailing_blanks.saturating_sub(1);

    let inferred_indent = if has_explicit_indent {
        0
    } else {
        body_lines
            .iter()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.bytes().take_while(|b| *b == b' ').count())
            .min()
            .unwrap_or(0)
    };

    let indent_prefix = " ".repeat(canonical_indent);
    let mut out = String::with_capacity(content_text.len());
    for line in body_lines.iter() {
        out.push('\n');
        if line.trim().is_empty() {
            continue;
        }
        if has_explicit_indent {
            out.push_str(line);
        } else {
            let stripped = &line[inferred_indent.min(line.len())..];
            out.push_str(&indent_prefix);
            out.push_str(stripped);
        }
    }
    // File-terminating newline (one is always required; the chomping
    // indicator decides whether to add extra blank lines on top).
    out.push('\n');
    if chomp == Chomp::Keep {
        for _ in 0..extra_blank_lines {
            out.push('\n');
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // The content token always starts with the newline that follows the
    // header, and includes every body byte (interior whitespace, blank
    // lines, trailing blanks) up to the end of the scalar's scope.

    #[test]
    fn clip_reindents_to_canonical_width() {
        let body = canonical_body("\n    123\n    456\n    789\n\n\n", false, Chomp::Clip, 2);
        assert_eq!(body, "\n  123\n  456\n  789\n");
    }

    #[test]
    fn strip_drops_all_trailing_blanks() {
        let body = canonical_body("\n    123\n    456\n    789\n\n\n", false, Chomp::Strip, 2);
        assert_eq!(body, "\n  123\n  456\n  789\n");
    }

    #[test]
    fn keep_preserves_trailing_blank_lines() {
        let body = canonical_body("\n    123\n    456\n    789\n\n\n", false, Chomp::Keep, 2);
        assert_eq!(body, "\n  123\n  456\n  789\n\n\n");
    }

    #[test]
    fn explicit_indent_preserves_body_verbatim() {
        // With an explicit indentation indicator, lines that look "over-
        // indented" relative to the indicator are real content and must
        // not be normalized away.
        let body = canonical_body("\n    123\n   456\n  789\n\n\n", true, Chomp::Strip, 2);
        assert_eq!(body, "\n    123\n   456\n  789\n");
    }

    #[test]
    fn empty_body_produces_single_terminating_newline() {
        // Just `|-\n` or `>+\n` with no content lines at all. We still
        // need the file-terminating newline (the printer/option may strip
        // it later if `trailing_newline` is off).
        assert_eq!(canonical_body("\n", false, Chomp::Clip, 2), "\n");
        assert_eq!(canonical_body("\n", false, Chomp::Strip, 2), "\n");
        assert_eq!(canonical_body("\n", false, Chomp::Keep, 2), "\n");
    }

    #[test]
    fn keep_with_no_extra_blanks_emits_one_newline() {
        // Body ends with exactly one newline — `Keep` should not invent
        // extra blank lines that aren't in the source.
        let body = canonical_body("\n    abc\n", false, Chomp::Keep, 2);
        assert_eq!(body, "\n  abc\n");
    }

    #[test]
    fn interior_blank_line_is_preserved_as_empty() {
        // A blank line within the body becomes a bare `\n` in the
        // output — neither indented nor stripped.
        let body = canonical_body("\n    a\n\n    b\n\n", false, Chomp::Clip, 2);
        assert_eq!(body, "\n  a\n\n  b\n");
    }

    // CRLF (Windows) input must yield the same LF-only body as the
    // equivalent Unix input: the printer is responsible for converting
    // the `\n`s back to the configured line ending, so emitting a `\r`
    // here would double up to `\r\r\n`.

    #[test]
    fn crlf_clip_reindents_like_lf() {
        let body = canonical_body(
            "\r\n    123\r\n    456\r\n    789\r\n\r\n\r\n",
            false,
            Chomp::Clip,
            2,
        );
        assert_eq!(body, "\n  123\n  456\n  789\n");
    }

    #[test]
    fn crlf_keep_preserves_trailing_blank_lines() {
        let body = canonical_body(
            "\r\n    123\r\n    456\r\n    789\r\n\r\n\r\n",
            false,
            Chomp::Keep,
            2,
        );
        assert_eq!(body, "\n  123\n  456\n  789\n\n\n");
    }

    #[test]
    fn crlf_explicit_indent_preserves_body() {
        let body = canonical_body(
            "\r\n    123\r\n   456\r\n  789\r\n\r\n\r\n",
            true,
            Chomp::Strip,
            2,
        );
        assert_eq!(body, "\n    123\n   456\n  789\n");
    }

    // Already-canonical bodies must be recognized and borrowed so that
    // formatting an already-formatted file never allocates.

    #[test]
    fn canonical_input_is_borrowed() {
        for (source, has_explicit_indent, chomp) in [
            ("\n  123\n  456\n", false, Chomp::Clip),
            ("\n  123\n\n  456\n", false, Chomp::Strip),
            ("\n  123\n\n\n", false, Chomp::Keep),
            ("\n  deeper\n    than\n  canonical\n", false, Chomp::Clip),
            ("\n    123\n   456\n  789\n", true, Chomp::Strip),
            ("\n", false, Chomp::Clip),
            ("\n", false, Chomp::Keep),
        ] {
            let body = canonical_body(source, has_explicit_indent, chomp, 2);
            assert!(
                matches!(body, Cow::Borrowed(_)),
                "expected {source:?} to be canonical"
            );
        }
    }

    #[test]
    fn non_canonical_input_is_rewritten() {
        for (source, has_explicit_indent, chomp) in [
            // Wrong indent width.
            ("\n    123\n", false, Chomp::Clip),
            // Whitespace-only trailing line.
            ("\n  123\n  \n", false, Chomp::Keep),
            // Whitespace-only interior line.
            ("\n  a\n \n  b\n", false, Chomp::Clip),
            // Missing terminating newline.
            ("\n  123", false, Chomp::Clip),
            // Trailing blanks that clip must drop.
            ("\n  123\n\n\n", false, Chomp::Clip),
            // CRLF line endings.
            ("\r\n  123\r\n", false, Chomp::Clip),
        ] {
            let body = canonical_body(source, has_explicit_indent, chomp, 2);
            assert!(
                matches!(body, Cow::Owned(_)),
                "expected {source:?} to be rewritten"
            );
        }
    }

    #[test]
    fn rebuilt_bodies_round_trip_as_borrowed() {
        // Pins `is_canonical` to `rebuild_body`: every rewritten body must
        // itself be recognized as canonical, otherwise a second format pass
        // would disagree with the first.
        for (source, has_explicit_indent, chomp) in [
            ("\n    123\n    456\n    789\n\n\n", false, Chomp::Clip),
            ("\n    123\n    456\n    789\n\n\n", false, Chomp::Strip),
            ("\n    123\n    456\n    789\n\n\n", false, Chomp::Keep),
            ("\n    123\n   456\n  789\n\n\n", true, Chomp::Strip),
            ("\n", false, Chomp::Clip),
            ("\n    a\n\n    b\n\n", false, Chomp::Clip),
            ("\n  123\n  \n", false, Chomp::Keep),
            ("\r\n    123\r\n    456\r\n\r\n", false, Chomp::Keep),
            ("", false, Chomp::Clip),
        ] {
            let first = canonical_body(source, has_explicit_indent, chomp, 2);
            let second = canonical_body(first.as_ref(), has_explicit_indent, chomp, 2);
            assert_eq!(first, second, "rebuild of {source:?} must be stable");
            assert!(
                matches!(second, Cow::Borrowed(_)),
                "rebuild of {source:?} must be canonical"
            );
        }
    }
}
