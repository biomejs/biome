use crate::prelude::*;
use biome_formatter::{write, AttributePosition};
use biome_html_syntax::HtmlAttributeList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAttributeList;
impl FormatRule<HtmlAttributeList> for FormatHtmlAttributeList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &HtmlAttributeList, f: &mut HtmlFormatter) -> FormatResult<()> {
        let attribute_count = node.len();
        let attribute_seperator = if f.options().attribute_position()
            == AttributePosition::Multiline
            && attribute_count > 1
        {
            hard_line_break()
        } else {
            soft_line_break_or_space()
        };

        if attribute_count > 0 {
            write!(
                f,
                [
                    space(),
                    &soft_line_indent_or_space(&format_with(|f| {
                        f.join_with(&attribute_seperator)
                            .entries(node.iter().formatted())
                            .finish()?;

                        Ok(())
                    }))
                ]
            )?;
        }

        Ok(())
    }
}
