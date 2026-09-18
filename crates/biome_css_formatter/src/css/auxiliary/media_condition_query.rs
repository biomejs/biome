use crate::prelude::*;
use crate::utils::media_query_comments::{fmt_media_query_leading, fmt_media_query_node};
use biome_css_syntax::{CssMediaConditionQuery, CssMediaConditionQueryFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaConditionQuery;
impl FormatNodeRule<CssMediaConditionQuery> for FormatCssMediaConditionQuery {
    fn fmt_node(&self, node: &CssMediaConditionQuery, f: &mut CssFormatter) -> FormatResult<()> {
        fmt_media_query_node(node.syntax(), f, |f| self.fmt_fields(node, f))
    }

    fn fmt_fields(&self, node: &CssMediaConditionQuery, f: &mut CssFormatter) -> FormatResult<()> {
        let CssMediaConditionQueryFields { condition } = node.as_fields();

        write!(f, [condition.format()])
    }

    fn fmt_leading_comments(
        &self,
        node: &CssMediaConditionQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        fmt_media_query_leading(node.syntax(), f)
    }
}
