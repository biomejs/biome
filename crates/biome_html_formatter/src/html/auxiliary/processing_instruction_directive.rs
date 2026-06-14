use crate::{html::lists::attribute_list::FormatHtmlAttributeListOptions, prelude::*};
use biome_formatter::write;
use biome_html_syntax::{
    HtmlProcessingInstructionDirective, HtmlProcessingInstructionDirectiveFields,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlProcessingInstructionDirective;

impl FormatNodeRule<HtmlProcessingInstructionDirective>
    for FormatHtmlProcessingInstructionDirective
{
    fn fmt_fields(
        &self,
        node: &HtmlProcessingInstructionDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let HtmlProcessingInstructionDirectiveFields {
            l_angle_token,
            opening_question_mark_token,
            target,
            attributes,
            closing_question_mark_token,
            r_angle_token,
        } = node.as_fields();

        write!(
            f,
            [&group(&format_with(|f| {
                write!(
                    f,
                    [
                        l_angle_token.format(),
                        opening_question_mark_token.format(),
                        target.format()
                    ]
                )?;
                attributes
                    .format()
                    .with_options(FormatHtmlAttributeListOptions {
                        is_canonical_html_element: false,
                        tag_name: None,
                    })
                    .fmt(f)?;
                write!(
                    f,
                    [closing_question_mark_token.format(), r_angle_token.format()]
                )
            }))]
        )
    }
}
