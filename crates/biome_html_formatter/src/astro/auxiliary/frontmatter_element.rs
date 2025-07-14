use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{AstroFrontmatterElement, AstroFrontmatterElementFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAstroFrontmatterElement;
impl FormatNodeRule<AstroFrontmatterElement> for FormatAstroFrontmatterElement {
    fn fmt_fields(
        &self,
        node: &AstroFrontmatterElement,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let AstroFrontmatterElementFields {
            l_fence_token,
            content_token,
            r_fence_token,
        } = node.as_fields();

        write!(f, [l_fence_token.format(), hard_line_break()])?;

        if let Some(content_token) = content_token {
            write!(f, [content_token.format(), hard_line_break()])?;
        }

        write!(f, [r_fence_token.format()])?;

        Ok(())
    }
}
