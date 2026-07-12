use crate::prelude::*;
use crate::utils::media_query_comments::{fmt_media_query_leading, fmt_media_query_node};
use biome_css_syntax::{CssMediaTypeQuery, CssMediaTypeQueryFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaTypeQuery;
impl FormatNodeRule<CssMediaTypeQuery> for FormatCssMediaTypeQuery {
    fn fmt_node(&self, node: &CssMediaTypeQuery, f: &mut CssFormatter) -> FormatResult<()> {
        fmt_media_query_node(node.syntax(), f, |f| self.fmt_fields(node, f))
    }

    fn fmt_fields(&self, node: &CssMediaTypeQuery, f: &mut CssFormatter) -> FormatResult<()> {
        let CssMediaTypeQueryFields { modifier, ty } = node.as_fields();

        if modifier.is_some() {
            write!(f, [modifier.format(), space()])?;
        }

        write!(f, [ty.format()])
    }

    fn fmt_leading_comments(
        &self,
        node: &CssMediaTypeQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        fmt_media_query_leading(node.syntax(), f)
    }
}
