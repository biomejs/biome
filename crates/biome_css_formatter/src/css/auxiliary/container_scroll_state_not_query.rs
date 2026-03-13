use crate::prelude::*;
use biome_css_syntax::{CssContainerScrollStateNotQuery, CssContainerScrollStateNotQueryFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerScrollStateNotQuery;
impl FormatNodeRule<CssContainerScrollStateNotQuery> for FormatCssContainerScrollStateNotQuery {
    fn fmt_fields(
        &self,
        node: &CssContainerScrollStateNotQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssContainerScrollStateNotQueryFields { not_token, query } = node.as_fields();

        write!(f, [not_token.format(), space(), query.format()])
    }
}
