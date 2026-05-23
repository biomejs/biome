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
        && fields.directives.iter().next().is_none()
        && fields.dashdashdash_token.is_none()
        && fields.dotdotdot_token.is_none()
        && match fields.node {
            Some(AnyYamlBlockNode::YamlBlockInBlockNode(node)) => {
                is_bare_block_scalar_node(&node)
            }
            _ => false,
        }
}

/// True when the block-in-block node has no properties and wraps a folded
/// or literal scalar whose body has a shape this formatter can safely
/// reindent. See [`has_well_indented_body`] for the exact rules.
pub(crate) fn is_bare_block_scalar_node(node: &YamlBlockInBlockNode) -> bool {
    if node.properties().iter().next().is_some() {
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

#[derive(Clone, Copy)]
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

    let mut trailing_blanks = 0usize;
    for line in lines.iter().rev() {
        if line.trim().is_empty() {
            trailing_blanks += 1;
        } else {
            break;
        }
    }
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
            for _ in 0..canonical_indent {
                out.push(' ');
            }
            out.push_str(stripped);
        }
    }
    // File-terminating newline (one is always required; the chomping
    // indicator decides whether to add extra blank lines on top).
    out.push('\n');
    if matches!(chomp, Chomp::Keep) {
        for _ in 0..extra_blank_lines {
            out.push('\n');
        }
    }
    out
}
