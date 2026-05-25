use crate::FormatYamlSyntaxToken;
use crate::prelude::*;
use biome_formatter::trivia::FormatToken;
use biome_formatter::{CstFormatContext, FormatOptions};
use biome_rowan::{AstNode, AstNodeList};
use biome_yaml_syntax::{
    AnyYamlBlockHeader, AnyYamlBlockInBlockContent, AnyYamlBlockNode, AnyYamlDocument,
    YamlBlockContent, YamlBlockHeaderList, YamlBlockInBlockNode, YamlDocument, YamlRoot,
    YamlSyntaxToken,
};

/// True when the document has no decorations (BOM, directives, document
/// markers) and wraps a single property-less block scalar. Only this exact
/// shape is supported by our partial delegation; other shapes fall back to
/// verbatim while the parent chain is still being built out.
pub(crate) fn is_bare_block_scalar_document(doc: &YamlDocument) -> bool {
    let fields = doc.as_fields();
    fields.bom_token.is_none()
        && fields.directives.is_empty()
        && fields.dashdashdash_token.is_none()
        && fields.dotdotdot_token.is_none()
        && match fields.node {
            Some(AnyYamlBlockNode::YamlBlockInBlockNode(node)) => is_bare_block_scalar_node(&node),
            _ => false,
        }
}

/// True when the block-in-block node has no properties and wraps a folded
/// or literal scalar whose body has a shape this formatter can safely
/// reindent. See [`has_well_indented_body`] for the exact rules.
pub(crate) fn is_bare_block_scalar_node(node: &YamlBlockInBlockNode) -> bool {
    if !node.properties().is_empty() {
        return false;
    }
    let (headers, content) = match node.content() {
        Ok(AnyYamlBlockInBlockContent::YamlFoldedScalar(scalar)) => {
            (scalar.headers(), scalar.content().ok())
        }
        Ok(AnyYamlBlockInBlockContent::YamlLiteralScalar(scalar)) => {
            (scalar.headers(), scalar.content().ok())
        }
        _ => return false,
    };
    let Some(content) = content else { return false };
    let has_explicit_indent = headers
        .iter()
        .any(|h| matches!(h, AnyYamlBlockHeader::YamlIndentationIndicator(_)));
    has_well_indented_body(&content, has_explicit_indent)
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
    let body_text = text.strip_prefix('\n').unwrap_or(text);
    let mut indents = body_text
        .split('\n')
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

/// True when the root contains exactly one bare block-scalar document.
pub(crate) fn is_bare_block_scalar_root(root: &YamlRoot) -> bool {
    let mut docs = root.documents().iter();
    matches!(
        (docs.next(), docs.next()),
        (Some(AnyYamlDocument::YamlDocument(doc)), None)
            if is_bare_block_scalar_document(&doc)
    )
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
    f.context()
        .comments()
        .mark_suppression_checked(content.syntax());

    let content_token = content.value_token()?;
    let canonical_indent = f.options().indent_width().value() as usize;
    let body = compute_body(
        content_token.text(),
        has_explicit_indent,
        chomp,
        canonical_indent,
    );
    let position = content_token.text_range().start();

    FormatYamlSyntaxToken.format_replaced(&content_token, &text(&body, position), f)
}

fn compute_body(
    content_text: &str,
    has_explicit_indent: bool,
    chomp: Chomp,
    canonical_indent: usize,
) -> String {
    let body_text = content_text.strip_prefix('\n').unwrap_or(content_text);
    let lines: Vec<&str> = body_text.split('\n').collect();

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
}
