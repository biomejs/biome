use crate::prelude::*;
use biome_markdown_syntax::MdThematicBreakChar;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdThematicBreakChar;
impl FormatNodeRule<MdThematicBreakChar> for FormatMdThematicBreakChar {
    fn fmt_fields(
        &self,
        node: &MdThematicBreakChar,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        node.value().format().fmt(f)
    }
}
