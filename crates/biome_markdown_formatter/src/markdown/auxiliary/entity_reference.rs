use crate::prelude::*;
use biome_markdown_syntax::MdEntityReference;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdEntityReference;
impl FormatNodeRule<MdEntityReference> for FormatMdEntityReference {
    fn fmt_fields(&self, node: &MdEntityReference, f: &mut MarkdownFormatter) -> FormatResult<()> {
        node.value_token().format().fmt(f)
    }
}
