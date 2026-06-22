use crate::FormatYamlSyntaxToken;
use crate::prelude::*;
use biome_formatter::trivia::FormatToken;
use biome_formatter::{CstFormatContext, FormatOptions};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Chomp {
    Clip,
    Strip,
    Keep,
}

pub(crate) fn format_block_scalar(
    opener: &YamlSyntaxToken,
    headers: &YamlBlockHeaderList,
    content: &YamlBlockContent,
    f: &mut YamlFormatter,
) -> FormatResult<()> {
    let mut has_explicit_indent = false;
    let mut chomp = Chomp::Clip;
    for header in headers {
        match header {
            AnyYamlBlockHeader::YamlIndentationIndicator(_) => has_explicit_indent = true,
            AnyYamlBlockHeader::YamlBlockKeepIndicator(_) => chomp = Chomp::Keep,
            AnyYamlBlockHeader::YamlBlockStripIndicator(_) => chomp = Chomp::Strip,
            AnyYamlBlockHeader::YamlBogusBlockHeader(_) => {}
        }
    }

    biome_formatter::write!(f, [opener.format(), headers.format()])?;

    // We rewrite the content token directly rather than going through
    // FormatYamlBlockContent's rule, so manually mark the node as visited
    // for the suppression bookkeeping the debug formatter enforces.
    f.context().comments().is_suppressed(content.syntax());

    let content_token = content.value_token()?;
    let canonical_indent = f.options().indent_width().value() as usize;
    let body = compute_body(
        content_token.text(),
        has_explicit_indent,
        chomp,
        canonical_indent,
    );
    let position = content_token.text_range().start();

    FormatYamlSyntaxToken.format_replaced(&content_token, &text(&body, Some(position)), f)
}

fn compute_body(
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
        let body = compute_body("\n    123\n    456\n    789\n\n\n", false, Chomp::Clip, 2);
        assert_eq!(body, "\n  123\n  456\n  789\n");
    }

    #[test]
    fn strip_drops_all_trailing_blanks() {
        let body = compute_body("\n    123\n    456\n    789\n\n\n", false, Chomp::Strip, 2);
        assert_eq!(body, "\n  123\n  456\n  789\n");
    }

    #[test]
    fn keep_preserves_trailing_blank_lines() {
        let body = compute_body("\n    123\n    456\n    789\n\n\n", false, Chomp::Keep, 2);
        assert_eq!(body, "\n  123\n  456\n  789\n\n\n");
    }

    #[test]
    fn explicit_indent_preserves_body_verbatim() {
        // With an explicit indentation indicator, lines that look "over-
        // indented" relative to the indicator are real content and must
        // not be normalized away.
        let body = compute_body("\n    123\n   456\n  789\n\n\n", true, Chomp::Strip, 2);
        assert_eq!(body, "\n    123\n   456\n  789\n");
    }

    #[test]
    fn empty_body_produces_single_terminating_newline() {
        // Just `|-\n` or `>+\n` with no content lines at all. We still
        // need the file-terminating newline (the printer/option may strip
        // it later if `trailing_newline` is off).
        assert_eq!(compute_body("\n", false, Chomp::Clip, 2), "\n");
        assert_eq!(compute_body("\n", false, Chomp::Strip, 2), "\n");
        assert_eq!(compute_body("\n", false, Chomp::Keep, 2), "\n");
    }

    #[test]
    fn keep_with_no_extra_blanks_emits_one_newline() {
        // Body ends with exactly one newline — `Keep` should not invent
        // extra blank lines that aren't in the source.
        let body = compute_body("\n    abc\n", false, Chomp::Keep, 2);
        assert_eq!(body, "\n  abc\n");
    }

    #[test]
    fn interior_blank_line_is_preserved_as_empty() {
        // A blank line within the body becomes a bare `\n` in the
        // output — neither indented nor stripped.
        let body = compute_body("\n    a\n\n    b\n\n", false, Chomp::Clip, 2);
        assert_eq!(body, "\n  a\n\n  b\n");
    }

    // CRLF (Windows) input must yield the same LF-only body as the
    // equivalent Unix input: the printer is responsible for converting
    // the `\n`s back to the configured line ending, so emitting a `\r`
    // here would double up to `\r\r\n`.

    #[test]
    fn crlf_clip_reindents_like_lf() {
        let body = compute_body(
            "\r\n    123\r\n    456\r\n    789\r\n\r\n\r\n",
            false,
            Chomp::Clip,
            2,
        );
        assert_eq!(body, "\n  123\n  456\n  789\n");
    }

    #[test]
    fn crlf_keep_preserves_trailing_blank_lines() {
        let body = compute_body(
            "\r\n    123\r\n    456\r\n    789\r\n\r\n\r\n",
            false,
            Chomp::Keep,
            2,
        );
        assert_eq!(body, "\n  123\n  456\n  789\n\n\n");
    }

    #[test]
    fn crlf_explicit_indent_preserves_body() {
        let body = compute_body(
            "\r\n    123\r\n   456\r\n  789\r\n\r\n\r\n",
            true,
            Chomp::Strip,
            2,
        );
        assert_eq!(body, "\n    123\n   456\n  789\n");
    }
}
