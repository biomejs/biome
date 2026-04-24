use crate::prelude::*;
use biome_markdown_syntax::MdIndent;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdIndent;
impl FormatNodeRule<MdIndent> for FormatMdIndent {
    fn fmt_fields(&self, node: &MdIndent, f: &mut MarkdownFormatter) -> FormatResult<()> {
        node.value_token().format().fmt(f)
    }
}
