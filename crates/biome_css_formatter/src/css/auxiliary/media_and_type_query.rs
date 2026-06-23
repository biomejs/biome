use crate::prelude::*;
use crate::utils::media_query_comments::{fmt_media_query_leading, fmt_media_query_node};
use biome_css_syntax::{CssMediaAndTypeQuery, CssMediaAndTypeQueryFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaAndTypeQuery;
impl FormatNodeRule<CssMediaAndTypeQuery> for FormatCssMediaAndTypeQuery {
    fn fmt_node(&self, node: &CssMediaAndTypeQuery, f: &mut CssFormatter) -> FormatResult<()> {
        fmt_media_query_node(node.syntax(), f, |f| self.fmt_fields(node, f))
    }

    fn fmt_fields(&self, node: &CssMediaAndTypeQuery, f: &mut CssFormatter) -> FormatResult<()> {
        let CssMediaAndTypeQueryFields {
            left,
            and_token,
            right,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                and_token.format(),
                space(),
                right.format()
            ]
        )
    }

    fn fmt_leading_comments(
        &self,
        node: &CssMediaAndTypeQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        fmt_media_query_leading(node.syntax(), f)
    }
}
