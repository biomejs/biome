use crate::prelude::*;
use biome_formatter::{write, AttributePosition};
use biome_html_syntax::HtmlAttributeList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAttributeList;
impl FormatRule<HtmlAttributeList> for FormatHtmlAttributeList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &HtmlAttributeList, f: &mut HtmlFormatter) -> FormatResult<()> {
        let line_break = if f.options().attribute_position() == AttributePosition::Multiline {
            hard_line_break()
        } else {
            soft_line_break_or_space()
        };

        write!(
            f,
            [&group(&soft_block_indent(&format_with(|f| {
                f.join_with(&line_break)
                    .entries(node.iter().formatted())
                    .finish()
            })))]
        )
    }
}
