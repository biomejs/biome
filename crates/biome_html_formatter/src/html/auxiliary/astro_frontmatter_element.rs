use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{HtmlAstroFrontmatterElement, HtmlAstroFrontmatterElementFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAstroFrontmatterElement;
impl FormatNodeRule<HtmlAstroFrontmatterElement> for FormatHtmlAstroFrontmatterElement {
    fn fmt_fields(
        &self,
        node: &HtmlAstroFrontmatterElement,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let HtmlAstroFrontmatterElementFields {
            l_fence_token,
            content_token,
            r_fence_token,
        } = node.as_fields();

        write!(f, [l_fence_token.format(), hard_line_break(),])?;

        if let Some(content_token) = content_token {
            write!(f, [content_token.format(), hard_line_break()])?;
        }
        write!(f, [r_fence_token.format(), hard_line_break()])
    }
}
