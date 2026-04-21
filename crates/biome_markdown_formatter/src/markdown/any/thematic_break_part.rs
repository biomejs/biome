//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_markdown_syntax::AnyMdThematicBreakPart;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMdThematicBreakPart;
impl FormatRule<AnyMdThematicBreakPart> for FormatAnyMdThematicBreakPart {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &AnyMdThematicBreakPart, f: &mut MarkdownFormatter) -> FormatResult<()> {
        match node {
            AnyMdThematicBreakPart::MdIndentToken(node) => node.format().fmt(f),
            AnyMdThematicBreakPart::MdThematicBreakChar(node) => node.format().fmt(f),
        }
    }
}
