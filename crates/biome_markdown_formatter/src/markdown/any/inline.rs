//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_markdown_syntax::AnyMdInline;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMdInline;
impl FormatRule<AnyMdInline> for FormatAnyMdInline {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &AnyMdInline, f: &mut MarkdownFormatter) -> FormatResult<()> {
        match node {
            AnyMdInline::MdAutolink(node) => node.format().fmt(f),
            AnyMdInline::MdEntityReference(node) => node.format().fmt(f),
            AnyMdInline::MdHardLine(node) => node.format().fmt(f),
            AnyMdInline::MdHtmlBlock(node) => node.format().fmt(f),
            AnyMdInline::MdInlineCode(node) => node.format().fmt(f),
            AnyMdInline::MdInlineEmphasis(node) => node.format().fmt(f),
            AnyMdInline::MdInlineHtml(node) => node.format().fmt(f),
            AnyMdInline::MdInlineImage(node) => node.format().fmt(f),
            AnyMdInline::MdInlineItalic(node) => node.format().fmt(f),
            AnyMdInline::MdInlineLink(node) => node.format().fmt(f),
            AnyMdInline::MdQuotePrefix(node) => node.format().fmt(f),
            AnyMdInline::MdReferenceImage(node) => node.format().fmt(f),
            AnyMdInline::MdReferenceLink(node) => node.format().fmt(f),
            AnyMdInline::MdSoftBreak(node) => node.format().fmt(f),
            AnyMdInline::MdTextual(node) => node.format().fmt(f),
        }
    }
}
