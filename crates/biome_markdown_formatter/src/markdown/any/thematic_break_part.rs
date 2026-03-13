use crate::prelude::*;
use biome_markdown_syntax::AnyMdThematicBreakPart;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMdThematicBreakPart;
impl FormatRule<AnyMdThematicBreakPart> for FormatAnyMdThematicBreakPart {
    type Context = MdFormatContext;
    fn fmt(&self, node: &AnyMdThematicBreakPart, f: &mut MarkdownFormatter) -> FormatResult<()> {
        match node {
            AnyMdThematicBreakPart::MdIndentToken(node) => node.format().fmt(f),
            AnyMdThematicBreakPart::MdThematicBreakChar(node) => node.format().fmt(f),
        }
    }
}
