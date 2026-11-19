use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{HtmlProcessingInstruction, HtmlProcessingInstructionFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlProcessingInstruction;
impl FormatNodeRule<HtmlProcessingInstruction> for FormatHtmlProcessingInstruction {
    fn fmt_fields(
        &self,
        node: &HtmlProcessingInstruction,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let HtmlProcessingInstructionFields {
            start_token,
            target,
            attributes,
            end_token,
        } = node.as_fields();

        write!(
            f,
            [
                start_token.format(),
                target.format(),
                space(),
                attributes.format(),
                space(),
                end_token.format()
            ]
        )
    }
}
