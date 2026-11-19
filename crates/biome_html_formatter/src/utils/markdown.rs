use crate::prelude::*;
use biome_html_syntax::{HtmlRoot, HtmlSyntaxNode};

/// Returns `true` when `element` is the first element of an HTML block of a
/// Markdown file: the element whose opening tag is on the block's first line.
///
/// The formatter prints the "root" element in block style. The rest of the child
/// elements are formatted normally. Because of that, we need to check:
/// - if the document is embedded in markdown
/// - it doesn't have sibling before it
/// - its grandparent is a root
///
/// Markdown reads a block that starts with a tag such as `<span>` or `<img>`
/// as HTML only while the block's first line holds that tag and nothing else.
/// If the formatter joined the tag with its children, or wrapped its
/// attributes, Markdown would read the block as a paragraph with inline HTML.
pub(crate) fn starts_markdown_block(element: &HtmlSyntaxNode, f: &HtmlFormatter) -> bool {
    f.options()
        .file_source()
        .as_embedding_kind()
        .is_markdown_block()
        && element.prev_sibling().is_none()
        && element
            .grand_parent()
            .is_some_and(|node| HtmlRoot::can_cast(node.kind()))
}
