use crate::prelude::*;
use crate::{
    context::ProseWrap,
    markdown::auxiliary::link_title::{FormatMdLinkTitleOptions, is_empty_link_title},
};
use biome_formatter::write;
use biome_markdown_syntax::{MdLinkReferenceDefinition, MdLinkReferenceDefinitionFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdLinkReferenceDefinition;
impl FormatNodeRule<MdLinkReferenceDefinition> for FormatMdLinkReferenceDefinition {
    fn fmt_fields(
        &self,
        node: &MdLinkReferenceDefinition,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        let MdLinkReferenceDefinitionFields {
            indent: indent_tokens,
            l_brack_token,
            label,
            r_brack_token,
            colon_token,
            destination,
            title,
        } = node.as_fields();

        if f.options().prose_wrap() == ProseWrap::Always {
            let formatted_title = format_with(|f| {
                if let Some(title) = &title
                    && !is_empty_link_title(title)
                {
                    write!(
                        f,
                        [
                            soft_line_break_or_space(),
                            title.format().with_options(FormatMdLinkTitleOptions {
                                leading_space: false,
                            })
                        ]
                    )?;
                }
                Ok(())
            });
            return write!(
                f,
                [group(&biome_formatter::format_args![
                    indent_tokens.format(),
                    l_brack_token.format(),
                    label.format(),
                    r_brack_token.format(),
                    colon_token.format(),
                    indent(&biome_formatter::format_args![
                        soft_line_break_or_space(),
                        destination.format(),
                        formatted_title
                    ])
                ])]
            );
        }

        let formatted_title = format_with(|f| {
            if let Some(title) = &title {
                write!(f, [title.format()])?;
            }
            Ok(())
        });
        write!(
            f,
            [
                indent_tokens.format(),
                l_brack_token.format(),
                label.format(),
                r_brack_token.format(),
                colon_token.format(),
                space(),
                destination.format(),
                formatted_title,
            ]
        )
    }
}
