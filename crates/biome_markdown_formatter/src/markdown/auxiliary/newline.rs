use crate::prelude::*;
use biome_markdown_syntax::MdNewline;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdNewline;
impl FormatNodeRule<MdNewline> for FormatMdNewline {
    fn fmt_fields(&self, node: &MdNewline, f: &mut MarkdownFormatter) -> FormatResult<()> {
        node.value_token().format().fmt(f)
    }
}
