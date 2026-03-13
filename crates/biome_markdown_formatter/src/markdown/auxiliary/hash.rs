use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::MdHash;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdHash;
impl FormatNodeRule<MdHash> for FormatMdHash {
    fn fmt_fields(&self, node: &MdHash, f: &mut MarkdownFormatter) -> FormatResult<()> {
        write!(f, [node.hash_token().format()])
    }
}
