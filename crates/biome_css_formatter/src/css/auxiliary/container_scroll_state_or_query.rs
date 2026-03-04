use crate::prelude::*;
use biome_css_syntax::{CssContainerScrollStateOrQuery, CssContainerScrollStateOrQueryFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerScrollStateOrQuery;
impl FormatNodeRule<CssContainerScrollStateOrQuery> for FormatCssContainerScrollStateOrQuery {
    fn fmt_fields(
        &self,
        node: &CssContainerScrollStateOrQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssContainerScrollStateOrQueryFields {
            left,
            or_token,
            right,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                or_token.format(),
                space(),
                right.format()
            ]
        )
    }
}
