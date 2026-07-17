use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{MdReferenceLinkLabel, MdReferenceLinkLabelFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdReferenceLinkLabel;
impl FormatNodeRule<MdReferenceLinkLabel> for FormatMdReferenceLinkLabel {
    fn fmt_fields(
        &self,
        node: &MdReferenceLinkLabel,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        let MdReferenceLinkLabelFields {
            l_brack_token,
            label,
            r_brack_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_brack_token.format(),
                label.format(),
                r_brack_token.format()
            ]
        )
    }
}
