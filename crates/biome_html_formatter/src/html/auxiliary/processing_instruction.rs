use crate::{html::lists::attribute_list::FormatHtmlAttributeListOptions, prelude::*};
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
            l_angle_token,
            target,
            attributes,
            question_mark_token,
            r_angle_token,
        } = node.as_fields();

        write!(
            f,
            [&group(&format_with(|f| {
                write!(f, [l_angle_token.format(), target.format()])?;
                attributes
                    .format()
                    .with_options(FormatHtmlAttributeListOptions {
                        is_canonical_html_element: false,
                        tag_name: None,
                    })
                    .fmt(f)?;
                write!(f, [question_mark_token.format(), r_angle_token.format()])
            }))]
        )
    }
}
