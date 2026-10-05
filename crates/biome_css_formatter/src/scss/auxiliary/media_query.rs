use crate::prelude::*;
use crate::utils::media_query_comments::{fmt_media_query_leading, fmt_media_query_node};
use biome_css_syntax::{ScssMediaQuery, ScssMediaQueryFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssMediaQuery;
impl FormatNodeRule<ScssMediaQuery> for FormatScssMediaQuery {
    fn fmt_node(&self, node: &ScssMediaQuery, f: &mut CssFormatter) -> FormatResult<()> {
        fmt_media_query_node(node.syntax(), f, |f| self.fmt_fields(node, f))
    }

    fn fmt_fields(&self, node: &ScssMediaQuery, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssMediaQueryFields { head, tail } = node.as_fields();

        write!(f, [head.format()])?;

        if let Some(tail) = tail {
            write!(f, [space(), tail.format()])?;
        }

        Ok(())
    }

    fn fmt_leading_comments(
        &self,
        node: &ScssMediaQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        fmt_media_query_leading(node.syntax(), f)
    }
}
