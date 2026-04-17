use crate::prelude::*;
use biome_markdown_syntax::{MdContinuationIndent, MdContinuationIndentFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdContinuationIndent;
impl FormatNodeRule<MdContinuationIndent> for FormatMdContinuationIndent {
    fn fmt_fields(
        &self,
        node: &MdContinuationIndent,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        let MdContinuationIndentFields { indent } = node.as_fields();

        indent.format().fmt(f)
    }
}
